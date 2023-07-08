use thiserror::Error;

#[derive(Error, Debug)]
pub enum TwitchRecoverError {
    #[error("User agent not found")]
    UserAgent,

    #[error("Failed to parse the stream title")]
    ParseStreamTitle,

    #[error("The streamer '{0}' could not be found")]
    StreamerNotFound(String),

    #[error("The stream could not be found")]
    StreamNotFound,

    #[error("The request was unable to be performed correctly")]
    HttpRequest(#[source] reqwest::Error),

    #[error("The response code {0} was received for '{1}'")]
    HttpResponseCode(String, String),

    #[error("The HTTP response cannot be serialized")]
    SerializeHttpResponse(#[source] reqwest::Error),
}

pub type TwitchRecoverResult<T = ()> = Result<T, TwitchRecoverError>;
