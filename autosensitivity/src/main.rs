use clap::Parser;
use home::home_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::{path::PathBuf, process::Command};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None, arg_required_else_help = true)]
struct Args {
    /// Save config
    #[arg(short, long)]
    save: Option<String>,

    /// Load config
    #[arg(short, long, action)]
    load: bool,
}

fn main() {
    let args = Args::parse();

    let config_dir = load_config_directory();

    if let Some(save) = args.save {
        println!("Saving config {}", save);
        save_config(save, config_dir);
    }

    if args.load {
        println!("Loading file")
    }
}

fn load_config_directory() -> PathBuf {
    let home_dir = home_dir();

    if let Some(mut config_dir) = home_dir {
        config_dir.push(".config");
        config_dir.push("autosensitivity");
        let _ = fs::create_dir_all(config_dir.as_path());
        return config_dir;
    }
    panic!("Couldn't load config directory")
}

fn save_config(input_name: String, config_dir: PathBuf) {
    let output = Command::new("xinput")
        .arg("list")
        .arg("--id-only")
        .arg(format!("pointer:{}", input_name))
        .output()
        .expect("failed to run xinput, is it installed ?");
    println!("output: {}!", String::from_utf8_lossy(&output.stdout));
    let mut input_path = config_dir;
    input_path.push(input_name);

    let config_file = File::create(input_path);
    let _ = config_file.expect("Should exist").write_all(b"accel_speed");
}
