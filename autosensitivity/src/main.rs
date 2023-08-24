use clap::Parser;
use home::home_dir;
use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
use std::{
    path::PathBuf,
    process::{Command, Stdio},
};

#[derive(Parser, Debug)]
#[command(author, version, about="A sensitivity config manager for X11", long_about=None, arg_required_else_help = true)]
struct Args {
    /// Save given xinput device config
    #[arg(short, long)]
    save: Option<String>,

    /// Load all saved config
    #[arg(short, long, action)]
    load: bool,
}

fn main() {
    let args = Args::parse();

    let config_dir = load_config_directory();

    if let Some(save) = args.save {
        println!("Saving config {}", save);
        let _ = save_config(save, config_dir);
    } else if args.load {
        println!("Loading config files...");
        let _ = load_configs(config_dir);
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

fn load_configs(config_dir: PathBuf) -> std::io::Result<()> {
    let config_paths = fs::read_dir(config_dir).unwrap();

    for path_result in config_paths {
        let result = path_result.unwrap();
        let path = result.path();
        let input_name = path.file_stem().unwrap().to_str();
        let contents =
            fs::read_to_string(path.to_owned()).expect("Should have been able to read the file");
        match input_name {
            Some(input) => {
                let input_id = find_id(input.to_string());
                match input_id {
                    Ok(v) => {
                        load_config(v, contents);
                        println!("Loaded config for device {}", input);
                    }
                    Err(_) => println!("Device {} is not present", input),
                }
            }
            None => panic!("Couldn't print filename"),
        }
    }
    return Ok(());
}

fn load_config(input_id: String, contents: String) {
    let vec = contents.split("\t").collect::<Vec<&str>>();
    let key = vec[0];
    let value = vec[1];

    let set_input = Command::new("xinput")
        .arg("--set-prop")
        .arg(input_id)
        .arg(key)
        .arg(value)
        .output()
        .expect("Failed to set value, is this input id used ?");

    if !set_input.status.success() {
        panic!("Xinput: {}", String::from_utf8_lossy(&set_input.stderr));
    }
}

fn find_id(input_name: String) -> Result<String, Error> {
    let output_list = Command::new("xinput")
        .arg("list")
        .arg("--id-only")
        .arg(format!("pointer:{}", input_name))
        .output()
        .expect("failed to run xinput, is it installed ?");
    if !output_list.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            "Couldn't find id for input name",
        ));
    }

    let mut input_id = String::from_utf8_lossy(&output_list.stdout).to_string();
    input_id.pop();
    return Ok(input_id);
}

fn save_config(input_name: String, config_dir: PathBuf) -> std::io::Result<()> {
    let input_id_res = find_id(input_name.clone());

    if !input_id_res.is_ok() {
        panic!("Couldn't find id for input {}", input_name.clone());
    }

    let input_id = input_id_res.unwrap();

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
        panic!(
            "Xinput: {}",
            String::from_utf8_lossy(&output_accel_speed.stderr)
        );
    }

    let binding = String::from_utf8_lossy(&output_accel_speed.stdout);
    let accel_speed_option = binding.split("\t").last();
    if let Some(accel_speed) = accel_speed_option {
        let mut input_path = config_dir.to_owned();
        input_path.push(input_name);

        let mut config_file = File::create(input_path)?;
        write!(config_file, "libinput Accel Speed\t{}", accel_speed)
    } else {
        panic!("Couldn't save the config to a file")
    }
}
