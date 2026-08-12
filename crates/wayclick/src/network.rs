use std::sync::mpsc::Sender;
use wayclick_schema::ServerResponse;

pub fn check_update(tx: Sender<ServerResponse>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let resp = reqwest::Client::new()
            .get("https://wayclick.dragmine.me/git.php/changelog")
            .header(
                "User-Agent",
                format!("wayclick {}", env!("CARGO_PKG_VERSION")),
            )
            .send()
            .await
            .expect("Failed to check for update")
            .json::<ServerResponse>()
            .await
            .expect("Failed to convert to json");
        _ = tx.send(resp);
    })
}
