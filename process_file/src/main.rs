use glob::glob;
use std::env;
use std::fs::{read_dir, read_to_string, File};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::path::PathBuf;
use yaml_rust2::{YamlEmitter, YamlLoader};

fn main() {
    println!("Hello, world!");
}
fn path_operation() -> anyhow::Result<()> {
    println!("{:?}", env::current_dir()?);
    println!("{}", PathBuf::from("aaa").exists());

    for f in read_dir(".")? {
        println!("{:?}", f?.file_name())
    }
    for f in glob(r"D:\project\rust\rs-df/**/*.rs")? {
        let p = f?;
        println!("{:?} {}", p.file_name().unwrap(), p.is_dir())
    }
    Ok(())
}

fn read_file() {
    let file = read_to_string(r"D:\OneDrive\python\tool.py").unwrap();
    println!("{}", file)
}

fn read_file_line() -> anyhow::Result<()> {
    let file = File::open(r"D:\OneDrive\python\tool.py").unwrap();
    for l in BufReader::new(file).lines() {
        let line = l?;
        println!("{line}")
    }
    Ok(())
}

fn write_file() -> anyhow::Result<()> {
    let mut file = File::create("data.txt")?;
    file.write_all("999".as_bytes())?;
    Ok(())
}

fn write_file_line() {
    let file = File::create("data.txt").unwrap();
    let mut writer = LineWriter::new(file);
    for _ in 1..10 {
        writer
            .write_all("ppp\n".as_bytes())
            .expect("error when write");
    }
}

fn read_yaml() -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    let file = read_to_string(r"C:\Users\sharp\AppData\Local\Programs\clash_win\config.yaml")?;
    #[cfg(target_os = "macos")]
    let file = read_to_string("/Users/sharp/.config/clash/config.yaml")?;
    let data = YamlLoader::load_from_str(&file)?;

    println!("{:?}", &data[0]["dns"]["nameserver"].as_str());
    Ok(())
}

fn write_yaml() -> anyhow::Result<()> {
    let input_string = "a: b\nc: d";
    let yaml = YamlLoader::load_from_str(input_string).unwrap();

    let mut output = String::new();
    YamlEmitter::new(&mut output).dump(&yaml[0]).unwrap();

    assert_eq!(
        output,
        r#"---
    a: b
    c: d"#
    );

    Ok(())
}
