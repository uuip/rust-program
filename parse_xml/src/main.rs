use chrono::{DateTime, Utc};
use roxmltree::ExpandedName;

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
    let doc = roxmltree::Document::parse(&rsp)?;
    let sparkle = doc
        .root_element()
        .namespaces()
        .find(|ns| ns.name() == Some("sparkle"));

    let items = doc.descendants().filter(|e| e.has_tag_name("item"));
    for item in items {
        if let Some(Some(t)) = item
            .descendants()
            .find(|e| e.has_tag_name("pubDate"))
            .map(|e| e.text())
        {
            println!("pubDate {}", t.trim().to_owned());
        }
        if let Some(Some(t)) = item
            .descendants()
            .find(|e| e.has_tag_name("title"))
            .map(|e| e.text())
        {
            println!("title {}", t.trim().to_owned());
        }
        if let Some(ns) = sparkle {
            let name = ExpandedName::from((ns.uri(), "channel"));
            if let Some(Some(t)) = item
                .descendants()
                .find(|e| e.has_tag_name(name))
                .map(|e| e.text())
            {
                println!("sparkle:channel {}", t.trim().to_owned());
            }

            let name = ExpandedName::from((ns.uri(), "version"));
            if let Some(Some(t)) = item
                .descendants()
                .find(|e| e.has_tag_name(name))
                .map(|e| e.text())
            {
                println!("sparkle:version {}", t.trim().to_owned());
            }

            if let Some(t) = item.descendants().find(|e| e.has_tag_name("enclosure")) {
                if let Some(attr) = t.attributes().find(|a| {
                    a.namespace().unwrap_or_default() == ns.uri() && a.name() == "version"
                }) {
                    println!("sparkle:version {}", attr.value().to_owned());
                }
            }
        }
    }

    Ok(())
}
