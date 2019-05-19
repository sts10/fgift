extern crate structopt;
use fgift::*;
use std::path::PathBuf;
use structopt::StructOpt;

pub mod writer;
use crate::writer::create_destination;
use crate::writer::write_to;

/// fgift: Family Gift List Maker
#[derive(StructOpt, Debug)]
#[structopt(name = "fgift")]
struct Opt {
    /// Give verbose output
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Provide file with previous years giving
    #[structopt(short = "p", long = "previous", parse(from_os_str))]
    previous_years_file: Option<PathBuf>,

    /// Provide file with special requests (assignments that must be made)
    #[structopt(short = "s", long = "special", parse(from_os_str))]
    special_requests_file: Option<PathBuf>,

    /// Print assignments to a file, rather than the terminal
    #[structopt(short = "o", long = "output")]
    output: Option<String>,

    /// CSV of family names
    #[structopt(name = "NAMES CSV FILE", parse(from_os_str))]
    names_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    if opt.verbose {
        println!("{:?}", opt);
    }
    let names: Vec<Vec<String>> = read_csv(opt.names_file);
    let names = flatten_and_shuffle(names);

    let previous_years_giving: Vec<String> = match opt.previous_years_file {
        Some(file_path) => read_by_line(file_path).unwrap(),
        None => vec![],
    };

    let special_requests: Vec<String> = match opt.special_requests_file {
        Some(file_path) => read_by_line(file_path).unwrap(),
        None => vec![],
    };

    let output_dest = create_destination(opt.output);

    println!("\n");
    // loop until we get a good solution
    loop {
        match find_gift_givers(&names, &previous_years_giving, &special_requests) {
            Some(assignment_pairs) => {
                // sort list alphabetically to cover evidence of special requests
                let assignment_pairs = sort_assignments_alphabetically(assignment_pairs);
                for assignment in assignment_pairs {
                    write_to(
                        &output_dest,
                        format!(
                            "{} gives to {}",
                            assignment.giver.name, assignment.receiver.name
                        ),
                    )
                    .expect("Error writing to output");
                }
                break;
            }
            None => {
                println!("\n------------------");
                println!("Got a bad solution\nGoing to try again");
                println!("------------------\n");
                continue;
            }
        };
    }
    println!("\n------------------\nDone!");
}
