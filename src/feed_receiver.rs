use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct FeedEntry {
    pub entry: feed_rs::model::Entry,
    pub published: i64,
    pub webhook_url: String,
    pub link: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedReceiver {
    url: String,
    last_published: Option<i64>,
    webhook_url: String,
}

impl FeedReceiver {
    /*
    pub fn new(url: &str, webhook_url: &str) -> Self {
        Self {
            url: url.to_string(),
            last_published: None,
            webhook_url: webhook_url.to_string(),
        }
    }
    */

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub async fn receive(&mut self) -> Result<Vec<FeedEntry>, crate::Error> {
        let res = reqwest::get(self.url.clone()).await?;
        let xml = res.text().await?;
        let res = feed_rs::parser::parse(xml.as_bytes())?;
        let mut latest_feed = self.last_published.unwrap_or(0);
        let criteria = latest_feed;
        let mut ret = vec![];
        for entry in res.entries {
            if entry.published.is_none() {
                log::warn!("Entry hasn't published datetime: Skipped");
                continue;
            }
            let published = entry.published.unwrap().timestamp();
            if published <= criteria {
                continue;
            }
            ret.push(FeedEntry {
                entry: entry.clone(),
                published,
                webhook_url: self.webhook_url.clone(),
                link: res
                    .links
                    .first()
                    .map(|x| x.href.to_string())
                    .unwrap_or(self.url.clone()),
                title: res
                    .title
                    .clone()
                    .map(|x| x.content.to_string())
                    .unwrap_or("RSS Bot".to_string()),
            });
            if latest_feed <= published {
                latest_feed = published;
            }
        }
        self.last_published = Some(latest_feed);
        Ok(ret)
    }
}
