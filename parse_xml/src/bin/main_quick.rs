use anyhow::{Result, anyhow};
use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use quick_xml::Reader;
use quick_xml::events::Event;

#[derive(Debug)]
struct Version {
    version: String,
    channel: String,
    pub_date: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let rsp = reqwest::get("https://releases.eggerapps.at/postico2/appcast.xml?update_channel=2")
        .await?
        .text()
        .await?;

    let mut reader = Reader::from_str(&rsp);
    reader.config_mut().trim_text(true);

    let mut versions: Vec<Version> = vec![];
    loop {
        match reader.read_event()? {
            Event::Start(e) => {
                if e.name().as_ref() == b"item"
                    && let Ok(version) = parse_item(&mut reader) {
                        versions.push(version);
                    }
            }
            Event::Eof => break,
            _ => (),
        }
    }
    println!("{:#?}", versions.len());
    versions.sort_by(|a, b| a.pub_date.cmp(&b.pub_date));
    let rc = versions
        .into_iter()
        .rfind(|x| x.channel != "beta")
        .ok_or_else(|| anyhow!("Failed to parse version"));

    println!("{:#?}", rc?);
    Ok(())
}

fn parse_item(reader: &mut Reader<&[u8]>) -> Result<Version> {
    let mut pub_date = String::new();
    let mut version = String::new();
    let mut channel = String::from("release");

    loop {
        match reader.read_event()? {
            Event::Start(inner) => match inner.name().as_ref() {
                b"pubDate" => {
                    pub_date = reader.read_text(inner.name())?.into();
                }
                b"title" if version.is_empty() => {
                    version = reader.read_text(inner.name())?.into();
                }
                b"sparkle:channel" => {
                    channel = reader.read_text(inner.name())?.into();
                }
                b"sparkle:version" => {
                    version = reader.read_text(inner.name())?.into();
                }
                _ => (),
            },
            Event::Empty(e) => {
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == b"sparkle:version" {
                        version = attr.decode_and_unescape_value(reader.decoder())?.into();
                    }
                }
            }
            Event::End(element) if element.name().as_ref() == b"item" => {
                break;
            }
            _ => {}
        }
    }

    Ok(Version {
        version,
        channel,
        pub_date: parse_dt(&pub_date)?,
    })
}

fn parse_dt(pub_date: &str) -> Result<DateTime<Utc>, ParseError> {
    DateTime::parse_from_rfc3339(pub_date)
        .or_else(|_| DateTime::parse_from_rfc2822(pub_date))
        .map(|d| d.to_utc())
        .or_else(|_| {
            NaiveDateTime::parse_from_str(pub_date, "%Y-%m-%d %H:%M:%S").map(|d| d.and_utc())
        })
}
