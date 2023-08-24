use clap::Parser;
use home::home_dir;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::{path::PathBuf, process::{Command, Stdio}};

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
        let _ = save_config(save, config_dir);
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

fn save_config(input_name: String, config_dir: PathBuf) -> std::io::Result<()> {
    let output_list = Command::new("xinput")
        .arg("list")
        .arg("--id-only")
        .arg(format!("pointer:{}", input_name))
        .output()
        .expect("failed to run xinput, is it installed ?");
    if !output_list.status.success() {
        panic!("Xinput: {}", String::from_utf8_lossy(&output_list.stderr));
    }

    let mut input_id = String::from_utf8_lossy(&output_list.stdout).to_string();
    input_id.pop();

    let list_props = Command::new("xinput")
        .arg("list-props")
        .arg(input_id)
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let grep_accel_speed = Command::new("grep")
        .arg("libinput Accel Speed (")
        .stdin(Stdio::from(list_props.stdout.unwrap()))
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let output_accel_speed = grep_accel_speed.wait_with_output().unwrap();

    if !output_accel_speed.status.success() {
        panic!("Xinput: {}", String::from_utf8_lossy(&output_accel_speed.stderr));
    }


    let binding = String::from_utf8_lossy(&output_accel_speed.stdout);
    let accel_speed_option = binding.split("\t").last();
    if let Some(accel_speed) = accel_speed_option {
        let mut input_path = config_dir;
        input_path.push(input_name);

        let mut config_file = File::create(input_path)?;
        write!(config_file, "libinput Accel Speed\t{}", accel_speed)

    } else {
        panic!("Couldn't save the config to a file")
    }
}
