use std::sync::{Arc, Mutex};

use crypto::{digest::Digest, sha1::Sha1};

use crate::{TwitchRecoverError, TwitchRecoverResult};

// #[derive(Debug)]
pub struct Recover {
    /// Streamer name
    name: String,

    /// Stream timestamp
    timestamp: i64,

    /// Stream id
    id: usize,
}

/// Urls related section
impl Recover {
    pub fn new(name: String, timestamp: i64, id: usize) -> Self {
        Self {
            name,
            timestamp,
            id,
        }
    }

    /// Get the vod url
    pub async fn get_url(&self) -> TwitchRecoverResult<String> {
        let urls = self.generate_all_urls();
        Self::find_valid_url(urls).await
    }

    /// Generate all the possible urls
    fn generate_all_urls(&self) -> Vec<String> {
        let base_url = format!("{}_{}_{}", self.name, self.id, self.timestamp);
        let mut hasher = Sha1::new();
        hasher.input_str(&base_url);
        let hash = hasher.result_str()[..20].to_owned();

        crate::constants::DOMAINS
            .iter()
            .map(|domain| format!("{}{}_{}/chunked/index-dvr.m3u8", domain, &hash, &base_url))
            .collect()
    }

    /// Find a valid url
    async fn find_valid_url(urls: Vec<String>) -> TwitchRecoverResult<String> {
        let valid_url: Arc<Mutex<Option<String>>> = Arc::new(Mutex::new(None));

        let mut joinhandles = Vec::new();
        let shared_urls = Arc::new(urls);

        for url in shared_urls.iter() {
            let child_valid_url = valid_url.clone();
            let child_url = url.clone();
            joinhandles.push(tokio::spawn(async move {
                let response = reqwest::get(&child_url).await.unwrap();

                if response.status() == reqwest::StatusCode::OK {
                    let mut guard = child_valid_url.lock().unwrap();
                    *guard = Some(child_url);
                }
            }));
        }

        for handle in joinhandles.into_iter() {
            handle.await.unwrap();

            let guard = valid_url.lock().unwrap();
            if let Some(ref v) = *guard {
                return Ok(v.to_owned());
            }
        }

        Err(TwitchRecoverError::StreamNotFound)
    }
}
