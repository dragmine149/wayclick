#[cfg(feature = "ui")]
use wayclick_frontend;

pub fn main() {
    #[cfg(feature = "ui")]
    wayclick_frontend::main(dirs::config_dir().unwrap().join("wayclick"));
}
