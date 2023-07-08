use std::{fmt, str::FromStr};

use chrono::{prelude::*, Duration};

use crate::{streamer::ModelStream, TwitchRecoverError};

/// Stream
#[derive(Debug)]
pub struct Stream {
    /// Stream Title
    title: Title,

    /// Stream Date
    date: String,

    /// Stream id
    stream_id: usize,

    /// Stream timestamp
    timestamp: i64,
}

impl Stream {
    /// Instantiate a new `Stream`
    pub fn new(title: Title, stream_id: usize, date: String, timestamp: i64) -> Self {
        Self {
            title,
            stream_id,
            date,
            timestamp,
        }
    }

    /// Return the id
    pub fn id(&self) -> usize {
        self.stream_id
    }

    /// Return the timestamp
    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }
}

impl TryFrom<ModelStream> for Stream {
    type Error = TwitchRecoverError;

    /// Try to instantiate `Stream` from a given `ModelStream`
    fn try_from(input: ModelStream) -> Result<Self, Self::Error> {
        let date: DateTime<Utc> = DateTime::from_str(input.date.as_str()).unwrap();
        Ok(Self::new(
            Title::try_from(input.infos.as_str())?,
            input.id,
            format!(
                "{} ({})",
                date.format("%B %d, %Y at %H:%M:%S"),
                convert_minutes(input.length)
            ),
            date.timestamp(),
        ))
    }
}

impl fmt::Display for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n  📅 {}{}",
            self.date, self.title
        )
    }
}

/// Title of the stream
#[derive(Debug)]
pub struct Title(Vec<String>);

impl TryFrom<&str> for Title {
    type Error = TwitchRecoverError;

    /// Try to instantiate `Title` from a given `&str`
    ///
    /// # Arguments
    ///
    /// * `input` - A string that contains a teast 3 pipes (|) as a separator. The first element is the `Stream name`, and the second element is the `Stream game`
    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let input: Vec<&str> = input.split('|').collect();

        let mut output = Vec::new();

        for i in (0..input.len()).step_by(3) {
            // + 2 is used for searching the third element, even if it is useless
            if i + 2 < input.len() {
                output.push(input[i].to_string());
            }
        }

        if output.is_empty() {
            return Err(TwitchRecoverError::ParseStreamTitle);
        }

        Ok(Self(output))
    }
}

impl fmt::Display for Title {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, item| {
            result.and_then(|_| write!(f, "\n  📝 {}", item))
        })
    }
}

/// Convert minutes to hours:minutes
fn convert_minutes(minutes: usize) -> String {
    let duration = Duration::minutes(minutes as i64);
    let hours = duration.num_hours();
    let remaining_minutes = duration.num_minutes() % 60;

    format!("{:02}h{:02}s", hours, remaining_minutes)
}
