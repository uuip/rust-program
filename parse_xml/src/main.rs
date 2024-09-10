use anyhow::{anyhow, Error};
use chrono::{DateTime, NaiveDateTime, ParseError, Utc};
use roxmltree::ExpandedName;

#[derive(Debug)]
struct Version {
    version: String,
    short_version: String,
    channel: String,
    pub_date: DateTime<Utc>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let rsp = reqwest::get("https://releases.eggerapps.at/postico2/appcast.xml?update_channel=2")
        .await?
        .text()
        .await?;
    let v = parse_appcast(&rsp)?;
    println!("{:#?}", v);
    Ok(())
}

fn parse_appcast(text: &str) -> Result<Version, Error> {
    let mut versions: Vec<Version> = vec![];

    let doc = roxmltree::Document::parse(text)?;
    let sparkle = doc
        .root_element()
        .namespaces()
        .find(|ns| ns.name() == Some("sparkle"));

    let items = doc.descendants().filter(|e| e.has_tag_name("item"));
    for item in items {
        let mut pub_date = String::new();
        let mut version1 = String::new();
        let mut version2 = String::new();
        let mut version3 = String::new();
        let mut channel = String::from("release");
        let mut short_version = String::new();

        if let Some(Some(t)) = item
            .descendants()
            .find(|e| e.has_tag_name("pubDate"))
            .map(|e| e.text())
        {
            pub_date = t.trim().to_owned();
        }

        if let Some(Some(t)) = item
            .descendants()
            .find(|e| e.has_tag_name("title"))
            .map(|e| e.text())
        {
            version1 = t.trim().to_owned();
        }

        if let Some(ns) = sparkle {
            let name = ExpandedName::from((ns.uri(), "channel"));
            if let Some(Some(t)) = item
                .descendants()
                .find(|e| e.has_tag_name(name))
                .map(|e| e.text())
            {
                channel = t.trim().to_owned();
            }

            let name = ExpandedName::from((ns.uri(), "version"));
            if let Some(Some(t)) = item
                .descendants()
                .find(|e| e.has_tag_name(name))
                .map(|e| e.text())
            {
                version2 = t.trim().to_owned();
            }

            let name = ExpandedName::from((ns.uri(), "shortVersionString"));
            if let Some(Some(t)) = item
                .descendants()
                .find(|e| e.has_tag_name(name))
                .map(|e| e.text())
            {
                short_version = t.trim().to_owned();
            }

            if let Some(t) = item.descendants().find(|e| e.has_tag_name("enclosure")) {
                for a in t
                    .attributes()
                    .filter(|a| a.namespace().unwrap_or_default() == ns.uri())
                {
                    if a.name() == "version" {
                        version3 = a.value().to_owned();
                    } else if a.name() == "shortVersionString" {
                        short_version = a.value().to_owned();
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

        versions.push(Version {
            version,
            short_version,
            channel,
            pub_date: parse_dt(&pub_date)?,
        });
    }
    let mut rc = versions
        .into_iter()
        .filter(|x| x.channel != "beta")
        .collect::<Vec<_>>();
    rc.sort_by(|a, b| a.pub_date.cmp(&b.pub_date));
    rc.into_iter().last().ok_or(anyhow!("parse version error"))
}

fn parse_dt(pub_date: &str) -> Result<DateTime<Utc>, ParseError> {
    DateTime::parse_from_rfc3339(pub_date)
        .or_else(|_| DateTime::parse_from_rfc2822(pub_date))
        .map(|d| d.to_utc())
        .or_else(|_| {
            NaiveDateTime::parse_from_str(pub_date, "%Y-%m-%d %H:%M:%S").map(|d| d.and_utc())
        })
}
