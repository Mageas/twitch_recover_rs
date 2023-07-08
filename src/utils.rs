use std::io::{stdout, Write};

use rand::seq::SliceRandom;
use rand::thread_rng;

use reqwest::header::USER_AGENT;
use reqwest::Response;

use crate::constants::USER_AGENTS;
use crate::{TwitchRecoverError, TwitchRecoverResult};

/// Get a random header
fn get_random_header() -> Result<&'static str, TwitchRecoverError> {
    match USER_AGENTS.choose(&mut thread_rng()) {
        Some(user_agent) => Ok(user_agent),
        None => Err(TwitchRecoverError::UserAgent),
    }
}

/// Request that returns a TwitchRecoverResult
pub async fn request(url: &str) -> TwitchRecoverResult<Response> {
    let header = get_random_header()?;
    let client = reqwest::Client::new();
    let response = client
        .get(url)
        .header(USER_AGENT, header)
        .send()
        .await
        .map_err(TwitchRecoverError::HttpRequest)?;

    if response.status() != reqwest::StatusCode::OK {
        return Err(TwitchRecoverError::HttpResponseCode(
            response.status().to_string(),
            url.to_string(),
        ));
    }

    Ok(response)
}

/// Progress bar
pub struct ProgressBar {
    /// Increment to track the progress
    pub inc: usize,
}

impl Default for ProgressBar {
    fn default() -> Self {
        Self { inc: 1 }
    }
}

impl ProgressBar {
    /// Update the progress bar and increment the inc by 1.
    pub fn progress(&mut self, message: impl std::fmt::Display) {
        let mut stdout = stdout();
        print!("\r{message}");
        stdout.flush().unwrap();
        self.inc += 1;
    }

    /// Insert a new line and consume `self`
    pub fn reset(self) {
        println!();
    }
}
