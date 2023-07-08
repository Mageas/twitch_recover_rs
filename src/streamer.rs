use crate::stream::Stream;
use crate::utils::{request, ProgressBar};
use crate::{TwitchRecoverError, TwitchRecoverResult};

pub struct StreamerOpt {
    days: usize,
}

impl StreamerOpt {
    pub fn new(days: usize) -> Self {
        Self { days }
    }
}

impl Default for StreamerOpt {
    fn default() -> Self {
        Self { days: 30 }
    }
}

/// Informations of the streamer
#[derive(Default)]
pub struct Streamer {
    /// Name of the streamer
    name: String,

    /// Id of the streamer
    id: Option<usize>,

    /// Streams of the streamer
    streams: Vec<Stream>,

    /// Days
    options: StreamerOpt,
}

impl Streamer {
    /// Instantiate a new `Streamer`
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into().to_lowercase(),
            ..Default::default()
        }
    }

    /// Instantiate a new `Streamer` with options
    pub fn with_options(name: impl Into<String>, options: StreamerOpt) -> Self {
        Self {
            name: name.into().to_lowercase(),
            options,
            ..Default::default()
        }
    }

    /// Return the streams
    pub fn streams(&self) -> &Vec<Stream> {
        &self.streams
    }

    /// Return the name
    pub fn name(&self) -> &String {
        &self.name
    }

    /// Fetch the streamer informations
    pub async fn fetch(&mut self) -> TwitchRecoverResult {
        self.fetch_id().await?;

        for stream in self.fetch_streams().await? {
            if let Ok(stream) = Stream::try_from(stream) {
                self.streams.push(stream);
            }
        }

        Ok(())
    }

    /// Retrieve the streamer id
    async fn fetch_id(&mut self) -> TwitchRecoverResult {
        let response = request(&format!(
            "https://sullygnome.com/api/standardsearch/{}/true/true/false/true",
            self.name
        ))
        .await?
        .json::<Vec<ModelStreamer>>()
        .await
        .map_err(TwitchRecoverError::SerializeHttpResponse)?;

        match response
            .iter()
            .find(|r| r.name.to_lowercase() == self.name)
            .map(|r| r.id)
        {
            Some(id) => {
                self.id = Some(id);
                Ok(())
            }
            None => Err(TwitchRecoverError::StreamerNotFound(self.name.to_owned())),
        }
    }

    /// Retrieve the streams from the last x days, make multiple requests if necessary
    async fn fetch_streams(&mut self) -> TwitchRecoverResult<Vec<ModelStream>> {
        let page_count = 50;
        let mut offset = 0;
        let mut response = Vec::new();
        let mut bar = ProgressBar::default();

        while let Ok(res) = request(&format!(
            "https://sullygnome.com/api/tables/channeltables/streams/{}/{}/%20/1/1/desc/{}/{page_count}",
            self.options.days,
            self.id.unwrap(),
            offset
        ))
        .await?
        .json::<ModelStreamList>()
        .await
        .map_err(TwitchRecoverError::SerializeHttpResponse)
        {
            bar.progress(format!(
                "[{}/{}] Requesting streams for '{}'",
                bar.inc,
                (res.records as f32 / page_count as f32).ceil(),
                self.name
            ));

            response.extend(res.data);

            // If all the data has been collected
            if response.len() >= res.records {
                bar.reset();
                break;
            }

            offset += page_count;
        }

        Ok(response)
    }
}

/// Response model for the `Streamer`
#[derive(serde::Deserialize, Debug)]
struct ModelStreamer {
    #[serde(rename = "siteurl")]
    name: String,
    #[serde(rename = "value")]
    id: usize,
}

/// Response model for streams
#[derive(serde::Deserialize, Debug)]
struct ModelStreamList {
    #[serde(rename = "recordsTotal")]
    records: usize,
    data: Vec<ModelStream>,
}

/// Response model for a stream
#[derive(serde::Deserialize, Debug)]
pub struct ModelStream {
    #[serde(rename = "gamesplayed")]
    pub infos: String,
    #[serde(rename = "streamId")]
    pub id: usize,
    #[serde(rename = "startDateTime")]
    pub date: String,
    pub length: usize,
}
