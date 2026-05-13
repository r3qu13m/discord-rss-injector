mod error;
mod feed_receiver;
pub use crate::error::Error;
use crate::feed_receiver::FeedReceiver;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct App {
    receivers: Vec<FeedReceiver>,
    #[serde(skip)]
    filename: Option<String>,
}

impl App {
    pub fn new(filename: &str) -> Result<Self, crate::Error> {
        if std::fs::exists(filename)? {
            let json = std::fs::read_to_string(filename)?;
            let ret: Self = serde_json::from_str(&json)?;
            Ok(Self {
                filename: Some(filename.to_string()),
                ..ret
            })
        } else {
            Ok(Self {
                receivers: vec![],
                filename: Some(filename.to_string()),
            })
        }
    }

    pub fn save(&self) -> Result<(), crate::Error> {
        let json = serde_json::to_string_pretty(self)?;
        if let Some(filename) = &self.filename {
            std::fs::write(filename, json)?;
        }
        Ok(())
    }

    pub async fn run(&mut self) {
        let client = reqwest::Client::new();
        let mut queue = vec![];
        for receiver in &mut self.receivers {
            let res = receiver.receive().await;
            match res {
                Err(e) => {
                    log::error!("Can't receive a feed: {}", receiver.get_url());
                    log::error!("Error string: {e:?}");
                }
                Ok(v) => {
                    queue.extend(v);
                }
            }
        }
        queue.sort_by_key(|x| x.published);
        for entry in queue {
            if entry.entry.title.is_none() {
                log::warn!("title is none: {:?}", entry.entry);
                continue;
            }
            let title = entry.entry.title.clone().unwrap().content;
            let link = entry.entry.links.first();
            let avatar_url = format!(
                "https://t0.gstatic.com/faviconV2?client=SOCIAL&type=FAVICON&fallback_opts=TYPE,SIZE,URL&url={}&size=256",
                entry.link,
            );
            let content = match link {
                Some(x) => format!("{} - {}", title, x.href),
                None => title,
            };
            let req = serde_json::json! {{
                "content": content,
                "username": entry.title,
                "avatar_url": avatar_url
            }};
            let mut retry_count = 0;
            loop {
                let res = client
                    .post(entry.webhook_url.clone())
                    .body(req.to_string())
                    .header(reqwest::header::CONTENT_TYPE, "application/json")
                    .send()
                    .await;
                if retry_count == 10 {
                    log::error!("retry count exceeded");
                    log::error!("{:?}", entry);
                    panic!();
                }
                if res.is_err() {
                    log::warn!(
                        "Webhook send error: {:?} (retry_count = {retry_count})",
                        res
                    );
                    tokio::time::sleep(tokio::time::Duration::from_secs(3 * (retry_count + 1)))
                        .await;
                    retry_count += 1;
                    continue;
                }
                break;
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    rustls::crypto::CryptoProvider::install_default(rustls::crypto::ring::default_provider())
        .expect("Error: Can't set the default crypto provider");

    let mut app = App::new("rss.json")?;
    loop {
        app.run().await;
        app.save()?;
        tokio::time::sleep(tokio::time::Duration::from_mins(10)).await;
    }
}
