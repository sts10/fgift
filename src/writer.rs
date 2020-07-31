use fgift::gets;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::io::Write;

#[derive(Debug)]
pub enum Destination {
    Terminal,
    FilePath(String),
}

pub fn create_destination(output: &Option<String>) -> Destination {
    let output_dest: Destination = match output {
        Some(file_path) => Destination::FilePath(file_path.to_string()),
        None => Destination::Terminal,
    };

    match &output_dest {
        Destination::FilePath(file_path) => {
            create_file(&Destination::FilePath(file_path.to_string()))
                .expect("Couldn't write to file");
        }
        Destination::Terminal => (),
    }

    output_dest
}
pub fn write_to<StringLike: Into<String>>(
    dest: &Destination,
    output: StringLike,
) -> std::io::Result<()> {
    match dest {
        Destination::FilePath(file_path) => {
            let mut f = OpenOptions::new().append(true).open(file_path).unwrap();
            writeln!(f, "{}", &output.into())
        }
        Destination::Terminal => {
            // println!("{}", &output);
            let stdout = io::stdout(); // get the global stdout entity
            let mut handle = stdout.lock(); // acquire a lock on it
            writeln!(handle, "{}", output.into())
        }
    }
}
fn create_file(dest: &Destination) -> std::io::Result<()> {
    match dest {
        Destination::FilePath(file_path) => {
            match File::open(file_path) {
                Ok(f) => {
                    eprintln!("File where you want to write, {:?}, already exists. Would you like to overwrite? (y/N)", f);
                    if gets().unwrap() == "y" {
                        File::create(file_path)?;
                    } else {
                        panic!("OK, exiting");
                    }
                }
                Err(_e) => {
                    File::create(file_path)?;
                }
            }
            Ok(())
        }
        Destination::Terminal => Ok(()),
    }
}
