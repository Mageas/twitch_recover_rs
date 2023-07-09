#![windows_subsystem = "windows"]

use dialoguer::console::Term;
use dialoguer::{theme::ColorfulTheme, Input, Select};

use color_eyre::eyre::{Context, Result};

use clap::Parser;

mod constants;
mod error;
mod recover;
mod stream;
mod streamer;
mod utils;

pub use error::{TwitchRecoverError, TwitchRecoverResult};
use recover::Recover;
use streamer::{Streamer, StreamerOpt};

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let args = Args::parse();

    let streamer_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Streamer")
        .interact_text()
        .context("Failed to retrieve the streamer name from your input")?;

    let mut streamer = match args.days {
        Some(days) => Streamer::with_options(streamer_name, StreamerOpt::new(days)),
        None => Streamer::new(streamer_name),
    };

    streamer
        .fetch()
        .await
        .context("Failed to fetch the streamer data")?;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Stream")
        .items(streamer.streams())
        .default(0)
        .max_length(3)
        .interact_on_opt(&Term::stderr())
        .context("Failed to retrieve the stream from your input")?;

    let stream = match selection {
        Some(i) => Recover::new(
            streamer.name().to_owned(),
            streamer.streams()[i].timestamp(),
            streamer.streams()[i].id(),
        ),
        None => std::process::exit(0x0),
    };

    let url = stream
        .get_url()
        .await
        .context("The stream could not be found; it may have expired (Twitch usually removes VODs 30 days after the stream)")?;

    println!("{url}");

    Ok(())
}

/// Twitch Recover is a free tool for recovering direct m3u8 links (compatible with sub-only VODs)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Duration in days to retrieve the streams (30 days by default)
    #[arg(short, long)]
    pub days: Option<usize>,
}
