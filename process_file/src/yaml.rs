use std::fs::read_to_string;
use yaml_rust2::{YamlEmitter, YamlLoader};

fn main() {
    println!("Hello, world!");
}

fn read_yaml() -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    let file = read_to_string(r"C:\Users\sharp\AppData\Local\Programs\clash_win\config.yaml")?;
    #[cfg(target_os = "macos")]
    let file = read_to_string("/Users/sharp/.config/clash/config.yaml")?;
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let file = read_to_string(
        std::path::Path::new(&std::env::var("HOME")?).join(".config/clash/config.yaml"),
    )?;
    let data = YamlLoader::load_from_str(&file)?;

    println!("{:?}", data[0]["dns"]["nameserver"].as_str());
    Ok(())
}

fn write_yaml() -> anyhow::Result<()> {
    let input_string = "a: b\nc: d";
    let yaml = YamlLoader::load_from_str(input_string)?;

    let mut output = String::new();
    YamlEmitter::new(&mut output).dump(&yaml[0])?;

    assert_eq!(
        output,
        r#"---
a: b
c: d"#
    );

    Ok(())
}

#[test]
fn yaml_output_matches_example() {
    write_yaml().unwrap();
}
