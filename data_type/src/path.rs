use camino::Utf8PathBuf;
use directories::BaseDirs;
use glob::glob;
use std::env;
use std::fs::read_dir;
use std::path::PathBuf;

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
    let p = Utf8PathBuf::from("/User/");
    println!("{}", p);

    let p = PathBuf::from("/User/");
    println!("{}", p.to_string_lossy());

    if let Some(base_dirs) = BaseDirs::new() {
        println!("{}", base_dirs.config_dir().display())
    }

    Ok(())
}
