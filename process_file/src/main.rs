mod yaml;

use std::fs::{File, read_to_string};
use std::io::{BufRead, BufReader, LineWriter, Write};

fn main() {
    println!("Hello, world!");
}

fn read_file() -> anyhow::Result<()> {
    let file = read_to_string(r"D:\OneDrive\python\tool.py")?;
    println!("{}", file);
    Ok(())
}

fn read_file_line() -> anyhow::Result<()> {
    let file = File::open(r"D:\OneDrive\python\tool.py")?;
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

fn write_file_line() -> anyhow::Result<()> {
    let file = File::create("data.txt")?;
    let mut writer = LineWriter::new(file);
    for _ in 1..10 {
        writer
            .write_all("ppp\n".as_bytes())
            .expect("error when write");
    }
    Ok(())
}
