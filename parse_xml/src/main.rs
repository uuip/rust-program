use anyhow::{anyhow, Result};
use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use roxmltree::{ExpandedName, Node};

#[derive(Debug)]
struct Version {
    version: String,
    short_version: String,
    channel: String,
    pub_date: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rsp = reqwest::get("https://www.macbartender.com/B2/updates/AppcastB5.xml")
        .await?
        .text()
        .await?;
    let v = parse_appcast(&rsp)?;
    println!("{:#?}", v);
    Ok(())
}

fn parse_appcast(text: &str) -> Result<Version> {
    let doc = roxmltree::Document::parse(text)?;
    let sparkle = doc
        .root_element()
        .namespaces()
        .find(|ns| ns.name() == Some("sparkle"))
        .map(|t| t.uri());
    println!("{:?}", 3333);
    let mut versions: Vec<Version> = doc
        .descendants()
        .filter(|e| e.has_tag_name("item"))
        .filter_map(|item| parse_item(item, sparkle).ok())
        .collect();
    println!("{:?}", versions);
    versions.sort_by(|a, b| a.pub_date.cmp(&b.pub_date));
    versions
        .into_iter()
        .filter(|x| x.channel != "beta")
        .last()
        .ok_or_else(|| anyhow!("Failed to parse version"))
}

fn parse_item(item: Node, sparkle: Option<&str>) -> Result<Version> {
    let pub_date = find_text(&item, "pubDate").unwrap_or_default();
    let version1 = find_text(&item, "title").unwrap_or_default();
    let mut version2 = String::new();
    let mut version3 = String::new();
    let mut channel = String::from("release");
    let mut short_version = String::new();

    if let Some(ns) = sparkle {
        channel = find_sparkle_text(&item, "channel", ns).unwrap_or_else(|| "release".to_string());
        version2 = find_sparkle_text(&item, ns, "version").unwrap_or_default();
        short_version = find_sparkle_text(&item, ns, "shortVersionString").unwrap_or_default();

        if let Some(t) = item.descendants().find(|e| e.has_tag_name("enclosure")) {
            for attr in t
                .attributes()
                .filter(|a| a.namespace().unwrap_or_default() == ns)
            {
                match attr.name() {
                    "version" => version3 = attr.value().to_string(),
                    "shortVersionString" => short_version = attr.value().to_string(),
                    _ => (),
                }
            }
        }
    }
    let version = if !version3.is_empty() {
        version3
    } else if !version2.is_empty() {
        version2
    } else {
        version1
    };

    Ok(Version {
        version,
        short_version,
        channel,
        pub_date: parse_dt(&pub_date)?,
    })
}

fn find_text(item: &Node, tag: &str) -> Option<String> {
    item.descendants()
        .find(|e| e.has_tag_name(tag))
        .and_then(|e| e.text())
        .map(|t| t.trim().to_owned())
}

fn find_sparkle_text(item: &Node, tag: &str, ns: &str) -> Option<String> {
    let name = ExpandedName::from((ns, tag));
    item.descendants()
        .find(|e| e.has_tag_name(name))
        .and_then(|e| e.text())
        .map(|t| t.trim().to_owned())
}

fn parse_dt(pub_date: &str) -> Result<DateTime<Utc>, ParseError> {
    DateTime::parse_from_rfc3339(pub_date)
        .or_else(|_| DateTime::parse_from_rfc2822(pub_date))
        .or_else(|_| DateTime::parse_from_str(pub_date, "%d, %a %b %Y %H:%M:%S %z"))
        .or_else(|_| DateTime::parse_from_str(pub_date, "%B %d, %Y %H:%M:%S %z"))
        .map(|d| d.to_utc())
        .or_else(|_| {
            NaiveDateTime::parse_from_str(pub_date, "%Y-%m-%d %H:%M:%S").map(|d| d.and_utc())
        })
}
