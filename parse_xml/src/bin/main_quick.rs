use anyhow::{Result, anyhow};
use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use quick_xml::events::Event;
use quick_xml::name::QName;
use quick_xml::{Reader, XmlVersion};

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
                if e.name().as_ref() == "item"
                    && let Ok(version) = parse_item(&mut reader)
                {
                    versions.push(version);
                }
            }
            Event::Eof => break,
            _ => (),
        }
    }
    println!("{:#?}", versions.len());
    versions.sort_by_key(|a| a.pub_date);
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
                "pubDate" => {
                    pub_date = read_text(reader, inner.name())?;
                }
                "title" if version.is_empty() => {
                    version = read_text(reader, inner.name())?;
                }
                "sparkle:channel" => {
                    channel = read_text(reader, inner.name())?;
                }
                "sparkle:version" => {
                    version = read_text(reader, inner.name())?;
                }
                _ => (),
            },
            Event::Empty(e) => {
                for attr in e.attributes().flatten() {
                    if attr.key.as_ref() == "sparkle:version" {
                        version = attr.normalized_value(XmlVersion::Implicit1_0)?.into();
                    }
                }
            }
            Event::End(element) if element.name().as_ref() == "item" => {
                break;
            }
            Event::Eof => return Err(anyhow!("Unexpected EOF while parsing item")),
            _ => {}
        }
    }

    Ok(Version {
        version,
        channel,
        pub_date: parse_dt(&pub_date)?,
    })
}

fn read_text(reader: &mut Reader<&[u8]>, end: QName) -> Result<String> {
    let raw = reader.read_text(end)?;
    // Text is already decoded, but XML entities still need unescaping.
    Ok(quick_xml::escape::unescape(raw.as_ref())?.into_owned())
}

fn parse_dt(pub_date: &str) -> Result<DateTime<Utc>, ParseError> {
    DateTime::parse_from_rfc3339(pub_date)
        .or_else(|_| DateTime::parse_from_rfc2822(pub_date))
        .map(|d| d.to_utc())
        .or_else(|_| {
            NaiveDateTime::parse_from_str(pub_date, "%Y-%m-%d %H:%M:%S").map(|d| d.and_utc())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unknown_text_entity_returns_an_error() {
        let mut reader = Reader::from_str("<title>&unknown;</title>");
        let Event::Start(start) = reader.read_event().unwrap() else {
            panic!("expected a title element");
        };
        assert!(read_text(&mut reader, start.name()).is_err());
    }

    #[test]
    fn text_and_attribute_versions_unescape_entities() {
        for content in [
            "<title>版本 &amp; 2</title>",
            "<sparkle:version>版本 &amp; 2</sparkle:version><title>fallback</title>",
            "<title>fallback</title><enclosure sparkle:version=\"&#x7248;本 &amp; 2\"/>",
        ] {
            let xml = format!(
                "<item>{content}<sparkle:channel>beta &amp; dev</sparkle:channel>\
                 <pubDate>2025-01-01T00:00:00Z</pubDate></item>"
            );
            let mut reader = Reader::from_str(&xml);
            reader.read_event().unwrap();
            let item = parse_item(&mut reader).unwrap();
            assert_eq!(item.version, "版本 & 2");
            assert_eq!(item.channel, "beta & dev");
        }
    }

    #[test]
    fn truncated_item_returns_an_error() {
        let (tx, rx) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut reader = Reader::from_str("<item>");
            reader.read_event().unwrap();
            tx.send(parse_item(&mut reader).is_err()).unwrap();
        });
        assert!(rx.recv_timeout(std::time::Duration::from_secs(1)).unwrap());
    }

    #[test]
    fn item_parsing_leaves_the_reader_at_the_next_item() {
        let mut reader = Reader::from_str(
            "<item><title>2.0</title><pubDate>2025-01-01T00:00:00Z</pubDate></item><item/>",
        );
        reader.read_event().unwrap();
        let item = parse_item(&mut reader).unwrap();
        assert_eq!(item.version, "2.0");
        assert_eq!(item.channel, "release");
        assert_eq!(
            item.pub_date,
            "2025-01-01T00:00:00Z".parse::<DateTime<Utc>>().unwrap()
        );
        assert!(matches!(reader.read_event().unwrap(), Event::Empty(_)));
    }
}
