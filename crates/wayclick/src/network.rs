use std::sync::mpsc::Sender;
use wayclick_schema::ServerResponse;

pub fn check_update(tx: Sender<Result<ServerResponse, String>>) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        let resp = reqwest::blocking::Client::new()
            .get("https://wayclick.dragmine.me/git.php/changelog")
            .header(
                "User-Agent",
                format!("wayclick {}", env!("CARGO_PKG_VERSION")),
            )
            .send()
            .map_err(|e| e.to_string())
            .map(|v| v.json().map_err(|e| e.to_string()))
            .flatten();
        _ = tx.send(resp);
    })
}
