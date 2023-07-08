# Twitch Recover RS

Inspired by [twitch_recover](https://github.com/pravindoesstuff/twitch_recover).

Twitch Recover is a free tool for recovering direct m3u8 links (compatible with sub-only VODs)

## **How to use**

``` text
Usage: twitch_recover_rs [OPTIONS]

Options:
  -d, --days <DAYS>  Duration in days to retrieve the streams (30 days by default)
  -h, --help         Print help
  -V, --version      Print version
```

## **Build instructions**

Clone the repository:
```
git clone https://github.com/Mageas/twitch_recover_rs
```

Move into the project directory:
```
cd twitch_recover_rs
```

Build the project with cargo:
```
cargo build --release
```
The binary is located in `./target/release/twitch_recover_rs`

## **Install instructions**

Install the project with cargo:
```
cargo install --path=.
```

## **TODO**

- Add a config file to control (USER_AGENTS, DOMAINS, days, page_count)

