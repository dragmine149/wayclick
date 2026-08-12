<?php

declare(strict_types=1);

require __DIR__ . '/vendor/autoload.php';

use Dotenv\Dotenv;

$dotenv = Dotenv::createImmutable(dirname(__FILE__, 2));
$dotenv->load();
$debug_data = [];

/**
 * Utility functions used everywhere.
 */
class Utils
{
    /**
     * Stores information in an array to output in the result later.
     *
     * @param mixed $data Data to output
     * @param string $tag Debug information to place before hand to make it easier to know what came from where.
     */
    public static function debug(mixed $data, string $tag)
    {
        $output = is_object($data) ? print_r($data, true) : $data;
        $output = is_array($data) ? implode(',', $output) : $output;

        global $debug_data;
        if (!array_key_exists($tag, $debug_data)) {
            $debug_data[$tag] = [];
        }
        array_push($debug_data[$tag], $output);
    }

    /**
     * Gets the version of the server, otherwise know as the last date this file was edited.
     *
     * WARNING: This might make some stuff confused if a file is updated multiple times on one day.
     *
     * @return string The version of the file as per the modification time.
     */
    public static function get_version(): string
    {
        // the file exists so we don't have to worry about that.
        return date('ym.d', filemtime(__FILE__));
    }
}

// just in case it's needed for whatever reason.
Utils::debug(Utils::get_version(), 'server version');

/**
 * Custom class to quickly return an error of a specific type.
 */
class Errors
{
    /**
     * Generic "catch-all" method for any error that occurs.
     *
     * @param string $msg The custom message to return for the error.
     * @param mixed $extra Optional extra data to provide.
     */
    static function error(string $msg, mixed $extra = null): void
    {
        $data = ['error' => $msg, 'extra' => $extra];
        Network::json(400, $data);
    }

    /**
     * Function to return if a request has resulted in something not found.
     *
     * @param string $path The path that was called to result in not found.
     */
    static function not_found(string $path): void
    {
        Network::json(404, ['error' => '`' . $path . '` has not been found']);
    }

    /**
     * Function to call when a feature is planned but not yet worked on.
     *
     * @param string $feature The nae of the feature in question.
     */
    static function not_implemented(string $feature): void
    {
        Network::json(501, ['error' => '`' . $feature . '` has not yet been implemented']);
    }

    static function unprocessed(string $found, string $expected): void
    {
        Network::json(422, [
            'error' => 'Request contained data that could not be processed!',
            'expected' => $expected,
            'found' => $found,
        ]);
    }

    /**
     * Helper function to check if a given URI arg has a value.
     *
     * @param array<string, ?string> $args Every single arg in the URL.
     * @param string $key The key of the arg to look for.
     * @return string The value in the args array on success. Network error on fail.
     */
    static function verify_uri_arg(array $args, string $key): string
    {
        $value = $args[$key];
        if (is_null($value)) {
            Errors::error('Missing value for URI query `' . $key . '`', [
                'args' => $args,
                'key' => $key,
                'value' => $value,
            ]);
        }
        return $value;
    }
}

/**
 * Main class dealing with all the parsing and translation of the inputted url.
 */
class Network
{
    /**
     * Parse the query args from `$_SERVER['REQUEST_URI']`
     *
     * @return array<string, ?string> The query args in the format of `key, value`
     */
    static function parse_query_args(): array
    {
        // get and separate the args.
        $args = parse_url($_SERVER['REQUEST_URI'], PHP_URL_QUERY);
        if (!is_string($args)) {
            return [];
        }
        $args = explode('&', $args);
        if (!is_array($args)) {
            Errors::error('UNREACHABLE SPLIT ISSUE');
            return [];
        }

        // format the args in a more php friendly array and return them.
        $return_args = [];
        foreach ($args as $arg) {
            $arg = explode('=', $arg, 2);
            match (sizeof($arg)) {
                1 => $return_args[$arg[0]] = null,
                2 => $return_args[$arg[0]] = $arg[1],
            };
        }

        return $return_args;
    }

    /**
     * Quick hand to stop the process and return a json response.
     * @param array<mixed> $data The json data to return.
     */
    static function json(int $status, array $data): void
    {
        if ($_ENV['debug']) {
            global $debug_data;
            $data['debug'] = $debug_data;
        }

        http_response_code($status);
        header('Content-Type: application/json');
        echo json_encode($data);
        exit();
    }

    /**
     * Process the path used and return the data as required.
     */
    static function process_path(): void
    {
        $path = parse_url($_SERVER['REQUEST_URI'], PHP_URL_PATH);
        // remove the file itself from the path.
        $path = str_replace('git.php', '', $path);
        $path = trim($path, '/');
        $args = Network::parse_query_args();

        match ($path) {
            'health' => Network::json(200, ['status' => 'ok']),
            'changelog' => new Repo()->fetch_changelog(),
            default => Errors::not_found($path),
        };
    }
}

class Repo
{
    /** @var ExternalRequest The client used to communicate with github. */
    private $github;
    /** @var RawRepo The information about the repo itself. */
    private $repo;
    /** @var ?mysqli Connection information of the database. */
    private $database;

    public function __construct()
    {
        if (function_exists('mysqli_connect')) {
            $this->database = mysqli_connect('localhost', $_ENV['USERNAME'], $_ENV['PASSWORD'], 'dragmine');
            if ($this->database == false) {
                $this->database == null;
            }
        }
        $this->github = new ExternalRequest()
            ->authenticate($_ENV['GITHUB'])
            ->cache($this->database);
    }

    function fetch_changelog()
    {
        $release = $this->github
            ->set_url('https://api.github.com/repos/dragmine149/wayclick/releases/latest')
            ->execute()
            ->json();
        Network::json(200, [
            'release_notes' => $release->body,
            'version' => $release->tag_name,
        ]);
    }
}

/**
 * Wrapper for curl to make making external requests slightly nicer to work with.
 */
class ExternalRequest
{
    /** @var ?CurlHandle Reference to the curl object itself. */
    private $curl;
    /** @var string[] Headers to provide to the network. */
    private $headers = [
        'Accept: application/json',
    ];
    protected null|string|false $response;
    private ?mysqli $database = null;
    private string $url;

    function __construct()
    {
        // Use information from the server to set our user agent. (i have a felling other scripts will use this file.)
        $server_name = $_SERVER['HTTP_HOST'];
        if ($server_name === 'localhost:8080') {
            // in case of local development, redirect to the main website instead of a random localhost.
            $server_name = 'dragmine.me';
        }
        $agent_name = ucfirst(explode('.', $server_name)[0]);

        array_push(
            $this->headers,
            'User-Agent: ' . $agent_name . '-server-' . Utils::get_version() . ' (https://' . $server_name . ')',
        );
    }

    /**
     * Add an authentication token to the headers to use.
     */
    function authenticate(string $token): self
    {
        array_push($this->headers, 'Authorization: Bearer ' . $token);
        return $this;
    }

    /**
     * Setup the initial request information.
     * @param string $url The url to request
     */
    function set_url(string $url): self
    {
        $this->url = $url;
        $this->curl = curl_init($url);
        curl_setopt_array($this->curl, [
            CURLOPT_RETURNTRANSFER => true,
            CURLOPT_HTTPHEADER => $this->headers,
            CURLOPT_FOLLOWLOCATION => true,
            CURLOPT_TIMEOUT => 30,
            CURLOPT_SSL_VERIFYPEER => true,
        ]);
        return $this;
    }

    /**
     * Add a database to the class to use as a caching solution.
     *
     * This probably doesn't follow the official caching spec of whatever, but it's my code so i don't care that much.
     */
    function cache(?mysqli $database): self
    {
        if (is_null($database)) {
            return $this;
        }
        $this->database = $database;
        return $this;
    }

    /**
     * Execute the stored curl information.
     *
     * @see ExternalRequest::response for the results of this function.
     */
    function execute(): self
    {
        $update = false;

        // check to see if we have a database cache.
        if (!is_null($this->database)) {
            // execute a basic query to get data.
            $stmt = $this->database->prepare('SELECT data, requested_on FROM cache WHERE url = ?');
            $stmt->bind_param('s', $this->url);
            $stmt->execute();
            // both a "query pass" check and "exists" check.
            $res = $stmt->get_result();
            if ($res != false) {
                // Check if the cache has expired.
                $data = $res->fetch_assoc();
                if (!is_null($data)) {
                    // Utils::debug($data, 'Select cache data');
                    // Utils::debug($data['requested_on'], 'Select cache data');
                    if (($data['requested_on'] + 3600) > time()) {
                        $this->response = $data['data'];
                        return $this;
                    }
                    Utils::debug('Cache expired', 'External Request: ' . $this->url);
                    $update = true;
                }
            }
            Utils::debug('No cache / failed to get.', 'External Request: ' . $this->url);
        }

        // checks to see if we have a curl handler
        if (is_null($this->curl)) {
            Utils::debug('Curl execute called without handler', 'curl');
            $this->response = false;
            return $this;
        }

        // executes the request.
        $this->response = curl_exec($this->curl);
        // checks if invalid request, just for more debugging.
        if ($this->response == false) {
            Utils::debug(curl_error($this->curl), 'curl');
            return $this;
        }

        // cache the request if possible.
        if (!is_null($this->database)) {
            $stmt;
            $epoch = time();
            if ($update) {
                $stmt = $this->database->prepare('UPDATE cache SET data = ?, requested_on = ? WHERE url = ? ');
                $stmt->bind_param('sds', $this->response, $epoch, $this->url);
            } else {
                $stmt = $this->database->prepare('INSERT INTO cache (url, data, requested_on) VALUES (?, ?, ?)');
                $stmt->bind_param('ssd', $this->url, $this->response, $epoch);
            }
            Utils::debug($stmt->execute(), 'External Request: ' . $this->url);
            Utils::debug($stmt->affected_rows, 'External Request: ' . $this->url);
        }

        return $this;
    }

    /**
     * @return mixed A json decoded version of the response.
     */
    function json()
    {
        if (is_null($this->response)) {
            return false;
        }
        return json_decode($this->response);
    }
}

Network::process_path();
