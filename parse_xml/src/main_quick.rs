use chrono::{DateTime, Utc};
use quick_xml::events::Event;
use quick_xml::Reader;

#[derive(Debug)]
struct Version {
    version: String,
    channel: String,
    pub_date: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rsp = reqwest::get("https://linearmouse.app/appcast.xml")
        .await?
        .text()
        .await?;
    let mut reader = Reader::from_str(rsp.as_str());
    reader.config_mut().trim_text(true);

    let mut versions: Vec<Version> = vec![];
    loop {
        match reader.read_event()? {
            Event::Start(e) => {
                if e.name().as_ref() == b"item" {
                    let mut pub_date = String::new();
                    let mut version = String::new();
                    let mut channel = String::from("release");
                    loop {
                        match reader.read_event()? {
                            Event::Start(inner) => match inner.name().as_ref() {
                                b"pubDate" => {
                                    pub_date = reader.read_text(inner.name())?.into();
                                }
                                b"title" => {
                                    if version.is_empty() {
                                        version = reader.read_text(inner.name())?.into();
                                    }
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
                                        version = attr
                                            .decode_and_unescape_value(reader.decoder())?
                                            .into();
                                    }
                                }
                            }
                            Event::End(element) => {
                                if element.name().as_ref() == b"item" {
                                    break;
                                }
                            }
                            _ => {}
                        }
                    }
                    versions.push(Version {
                        version,
                        channel,
                        pub_date: DateTime::parse_from_rfc3339(&pub_date)
                            .or_else(|_| DateTime::parse_from_rfc2822(&pub_date))?
                            .into(),
                    });
                }
            }

            Event::Eof => break,
            _ => (),
        }
    }
    println!("{:#?}", versions.len());
    let mut rc = versions
        .into_iter()
        .filter(|x| x.channel != "beta")
        .collect::<Vec<_>>();
    rc.sort_by(|a, b| a.pub_date.cmp(&b.pub_date));
    println!("{:#?}", rc.last());
    Ok(())
}
