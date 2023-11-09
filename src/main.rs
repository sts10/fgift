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
    /// Prints verbose output, including parameters as received. Can
    /// accept either one or two count.
    #[clap(short = 'v', long = "verbose", action = clap::ArgAction::Count)]
    verbose: u8,

    /// Optionally provide file with previous years giving
    #[clap(short = 'p', long = "previous")]
    previous_years_file: Option<PathBuf>,

    /// Optionally provide file with special requests (assignments that _must_ be made)
    #[clap(short = 's', long = "special")]
    special_requests_file: Option<PathBuf>,

    /// Print assignments to a file, rather than to the terminal
    #[clap(short = 'o', long = "output")]
    output: Option<String>,

    /// File containing names and family information. Can be CSV or JSON file.
    #[clap(name = "NAMES FILE")]
    names_file: PathBuf,
}

fn main() {
    let opt = Args::parse();

    // We can accept CSV or JSON. Figure out which we received by looking at the file extension
    let names: Vec<Vec<String>> = if opt
        .names_file
        .extension()
        .expect("Unable to detect file extension of inputted NAMES file")
        .to_ascii_lowercase()
        == "csv"
    {
        read_csv(&opt.names_file)
    } else if opt
        .names_file
        .extension()
        .expect("Unable to detect file extension of inputted NAMES file")
        .to_ascii_lowercase()
        == "json"
    {
        read_json(&opt.names_file)
    } else {
        panic!("Unable to detect file type of Names file. This program requires either a .csv or .json file. Check file extension.");
    };
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

    if opt.verbose >= 2 {
        println!("Parameters received: {:?}", opt);
    }

    // loop until we get a good solution
    loop {
        match find_gift_givers(&persons, &previous_years_giving, &special_requests) {
            Some(assignment_pairs) => {
                // Verify that everyone gives and everyone receives.
                // This line will panic the entire program if assignments fail verification,
                // which is the desired result in this case.
                assert!(verify_assignments(&persons, &assignment_pairs), "Was unable to verify that everyone gives and receives. Something wrong with inputs or code.");
                // If we made it here, we know the assignments were verified as good
                if opt.verbose >= 1 {
                    println!("Assignments have been verified ({} persons, {} assignment pairs, and all give and all receive)\n", persons.len(), assignment_pairs.len());
                }

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
                if opt.verbose >= 2 {
                    eprintln!("\n------------------\nGot a bad solution. Going to try again\n------------------\n");
                }
                continue;
            }
        };
    }
    if opt.verbose >= 2 {
        println!("------------------\nDone!");
    }
}
