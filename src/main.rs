use clap::Parser;
use fgift::*;
use std::path::PathBuf;
pub mod writer;
use crate::writer::create_destination;
use crate::writer::write_to;

/// Family Gift List Maker
#[derive(Parser, Debug)]
#[clap(version, name = "fgift")]
struct Args {
    /// Prints verbose output, including parameters as received
    #[clap(short = 'v', long = "verbose")]
    verbose: bool,

    /// Provide file with previous years giving
    #[clap(short = 'p', long = "previous")]
    previous_years_file: Option<PathBuf>,

    /// Provide file with special requests (assignments that must be made)
    #[clap(short = 's', long = "special")]
    special_requests_file: Option<PathBuf>,

    /// Print assignments to a file, rather than to the terminal
    #[clap(short = 'o', long = "output")]
    output: Option<String>,

    /// CSV of family names
    #[clap(name = "NAMES CSV FILE")]
    names_file: PathBuf,
}

fn main() {
    let opt = Args::parse();
    let names: Vec<Vec<String>> = read_csv(&opt.names_file);
    let persons = make_persons(names);
    let persons = shuffle_persons(persons);

    let special_requests: Option<Vec<String>> =
        opt.special_requests_file.as_ref().map(|file_path| {
            read_by_line(file_path).expect("Unable to read a line in the special requests file.")
        });

    // I should probably make this an Option too, but it's more cumbersome than with
    // special_requests
    let previous_years_giving: Vec<String> = match &opt.previous_years_file {
        Some(file_path) => {
            read_by_line(file_path).expect("Unable to read a line in the previous_years_file")
        }
        None => vec![],
    };

    let output_dest = create_destination(&opt.output);

    if opt.verbose {
        println!("Parameters received: {:?}", opt);
    }

    println!();
    // loop until we get a good solution
    loop {
        match find_gift_givers(&persons, &previous_years_giving, &special_requests) {
            Some(assignment_pairs) => {
                // Verify that everyone gives and everyone receives
                assert!(verify_assignments(&persons, &assignment_pairs), "Was unable to verify that everyone gives and receives. Something wrong with inputs or code.");
                // If we made it here, we know the assignments were verified as good
                println!("Assignments have been verified ({} persons, {} assignment pairs, and all give and all receive)\n", persons.len(), assignment_pairs.len());

                // Sort list alphabetically to cover evidence of special requests
                for assignment in sort_assignments_alphabetically(assignment_pairs) {
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
                if opt.verbose {
                    eprintln!("\n------------------\nGot a bad solution. Going to try again\n------------------\n");
                }
                continue;
            }
        };
    }
    if opt.verbose {
        println!("------------------\nDone!");
    }
}
