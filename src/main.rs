extern crate structopt;
use family_gift_list_maker::*;
use std::path::PathBuf;
use structopt::StructOpt;

/// FGift
#[derive(StructOpt, Debug)]
#[structopt(name = "fgift")]
struct Opt {
    /// Give verbose output
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,

    /// Provide file with previous years giving
    #[structopt(short = "p", long = "previous", parse(from_os_str))]
    previous_years_file: Option<PathBuf>,

    /// Provide file with special requests
    /// Special requests are assignments that must be made
    #[structopt(short = "s", long = "special", parse(from_os_str))]
    special_requests_file: Option<PathBuf>,

    /// Optionally print gift assignments to a file
    #[structopt(short = "o", long = "output")]
    output: Option<String>,

    /// CSV of family names
    #[structopt(name = "FAMILY NAMES", parse(from_os_str))]
    names_file: PathBuf,
}

fn main() {
    let opt = Opt::from_args();
    let names: Vec<Vec<String>> = read_csv(opt.names_file);
    let names = flatten_and_shuffle(names);

    // println!("\nOptionally, enter file path for a text list of previous years' giving\n(Hit <enter> if you do not want to enter such a file)");
    // let previous_years_file_path = get_file_path();
    // let previous_years_giving: Vec<String> = if previous_years_file_path.is_empty() {
    //     [].to_vec()
    // } else {
    //     read_by_line(&previous_years_file_path).unwrap()
    // };

    let previous_years_giving: Vec<String> = match opt.previous_years_file {
        Some(file_path) => read_by_line(file_path).unwrap(),
        None => vec![],
    };
    // println!("\nOptionally, enter file path for a text list of special requests\n(Hit <enter> if you do not want to enter such a file)");
    // let special_requests_file_path = get_file_path();
    // let special_requests: Vec<String> = if special_requests_file_path.is_empty() {
    //     [].to_vec()
    // } else {
    //     read_by_line(&special_requests_file_path).unwrap()
    // };

    let special_requests: Vec<String> = match opt.special_requests_file {
        Some(file_path) => read_by_line(file_path).unwrap(),
        None => vec![],
    };
    println!("\n");

    // loop until we get a good solution
    loop {
        match find_gift_givers(&names, &previous_years_giving, &special_requests) {
            Some(assignment_pairs) => {
                // sort list alphabetically to cover evidence of special requests
                let assignment_pairs = sort_assignments_alphabetically(assignment_pairs);
                for assignment in assignment_pairs {
                    println!(
                        "{} gives to {}",
                        assignment.giver.name, assignment.receiver.name
                    );
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
    println!("\n------------------");
    println!("Done!");
}
