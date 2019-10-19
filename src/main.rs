#[macro_use]
extern crate structopt;

use std::error::Error;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::process::Command;
use structopt::StructOpt;

const BASE_DIR: &str = "./notes";

#[derive(Debug, StructOpt)]
#[structopt(name = "CLI for notes taking app")]
enum Operations {
    #[structopt(name = "create")]
    Create { name: String },

    #[structopt(name = "open")]
    Open { name: String },

    #[structopt(name = "delete")]
    Delete { name: String },
}

fn main() -> std::io::Result<()> {
    let operation = Operations::from_args();

    if !Path::new(&BASE_DIR).exists() {
        fs::create_dir(&BASE_DIR).expect("Failed to create a new dir. Please check and try again");
    }

    match operation {
        Operations::Create { name } => {
            println!("Creating a new notes");
            create(&name)?;
        }
        Operations::Open { name } => {
            println!("Opening a notes");
            open(&name);
        }
        Operations::Delete { name } => {
            println!("Deleting a notes");
            delete(&name)?;
        }
    }

    Ok(())
}

fn execute_command(file_path: &String) {
    let cmd = format!("notepad {}", file_path);
    Command::new("cmd")
        .args(&["/C", &cmd[..]])
        .output()
        .expect("failed to execute process");
}

fn create(name: &String) -> std::io::Result<()> {
    let file_path = format!("./{}/{}", BASE_DIR, name);
    let path = Path::new(&file_path);
    let display = path.display();

    if !path.exists() {
        let mut _file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };
    }

    execute_command(&file_path);

    Ok(())
}

fn open(name: &String) {
    let file_path = format!("./{}/{}", BASE_DIR, name);
    let path = Path::new(&file_path);

    if path.exists() {
        execute_command(&file_path);
    } else {
        println!("File not found to open");
    }
}

fn delete(name: &String) -> std::io::Result<()> {
    let file_path = format!("./{}/{}", BASE_DIR, name);
    let path = Path::new(&file_path);

    if path.exists() {
        fs::remove_file(file_path)?;
        println!("Deleted file : {}", name);
    } else {
        println!("File not found to delete");
    }
    Ok(())
}
