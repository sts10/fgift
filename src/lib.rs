extern crate csv;
extern crate rand;
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Person {
    pub name: String,
    pub family_number: Option<usize>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Assignment {
    pub giver: Person,
    pub receiver: Person,
}

pub fn find_gift_givers(
    names: &[Person],                 // this is like &Vec<Person>
    previous_years_giving: &[String], // and this is like &Vec<String> , but it's a slice I guess
    special_requests: &[String],
) -> Option<Vec<Assignment>> {
    // receiving_vec is simply a vector of the names who are already receiving
    // let mut receiving_vec: Vec<String> = [].to_vec();
    // let mut special_request_givers_vec: Vec<String> = [].to_vec();
    let mut assignment_pairs: Vec<Assignment> = [].to_vec();

    // first, handle special requests
    for request in special_requests {
        let request_vec: Vec<&str> = request.split(" gives to ").collect();
        let request_giver_name = request_vec[0].to_string();
        let request_receiver_name = request_vec[1].to_string();
        // special_request_givers_vec.push(request_giver_name.clone());
        // receiving_vec.push(request_receiver_name.clone());
        let giver = Person {
            name: request_giver_name,
            family_number: None,
        };
        let receiver = Person {
            name: request_receiver_name,
            family_number: None,
        };
        assignment_pairs.push(Assignment { giver, receiver });
    }
    println!(
        "assignment_pairs len after special requests is {:?}",
        assignment_pairs.len()
    );

    // println!("Special request }", assignment_pairs);
    println!("names len is currently {:?}", names.len());

    'giver: for giver in names {
        // if special_request_givers_vec.contains(&giver.name) {
        for existing_assignment in &assignment_pairs {
            if existing_assignment.giver.name == giver.name {
                // println!("found a special giver");
                continue 'giver;
            }
        }
        // if we're here, we didn't find a special request of who they should give to,
        // so we need to find a receiver for them

        match find_receiver_for(giver, &names, &assignment_pairs, previous_years_giving) {
            Some(receiver) => {
                assignment_pairs.push(Assignment {
                    giver: giver.clone(),
                    receiver,
                });
            }
            None => return None, // println!("Couldn't find solution. Please run program again."),
        }
    }
    Some(assignment_pairs)
}

fn find_receiver_for(
    giver: &Person,
    names: &[Person],
    existing_assignment_pairs: &Vec<Assignment>,
    previous_years_giving: &[String],
) -> Option<Person> {
    let mut rng = thread_rng();
    let mut potential_receiver: Person;

    for _n in 0..1000 {
        potential_receiver = names[rng.gen_range(0, names.len())].clone();

        // What makes a GOOD receiver?
        //   - potential receiver is NOT already receiving
        //   - potential receiver is NOT this giver
        //   - potential receiver is NOT in this giver's family
        //   - potential receiver has NOT given to this person in previous years

        // I have to compare `.name`s here because the Persons generated from special requests
        // have None for family_number!
        if !existing_assignment_pairs
            .iter()
            .any(|pair| pair.receiver.name.to_lowercase() == potential_receiver.name.to_lowercase())
            && potential_receiver.name != giver.name
            && giver.family_number != potential_receiver.family_number
            && !previous_years_giving.contains(&format!(
                "{} gives to {}",
                giver.name, potential_receiver.name
            ))
        {
            return Some(potential_receiver);
        } else {
            // return to top of loop and randomly choose another potential_receiver
            continue;
        }
    }
    None
}

pub fn verify_assignments(names: &[Person], assignment_pairs: &[Assignment]) -> bool {
    if assignment_pairs.len() != names.len() {
        return false;
    }
    for name in names {
        let mut gives: bool = false;
        let mut receives: bool = false;
        for assignment in assignment_pairs {
            if name == &assignment.giver {
                gives = true;
            }
            if name == &assignment.receiver {
                receives = true;
            }
        }
        if !gives || !receives {
            return false;
        }
    }
    true
}

pub fn get_file_path() -> String {
    let file_path = gets().unwrap();
    let file_path = file_path.trim_matches(|c| c == '\'' || c == ' ');
    file_path.to_string()
}

pub fn read_csv(file_path: &PathBuf) -> Vec<Vec<String>> {
    let mut names: Vec<Vec<String>> = [].to_vec();

    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        let record = result.expect("a CSV record");
        let mut family_vec_strings: Vec<String> = [].to_vec();
        for name in record.iter() {
            if name.len() > 1 {
                family_vec_strings.push(name.to_string());
            }
        }

        names.push(family_vec_strings);
    }
    names
}

pub fn make_persons(families: Vec<Vec<String>>) -> Vec<Person> {
    let mut flat_names: Vec<Person> = vec![];

    for (number, family) in families.iter().enumerate() {
        for name in family {
            flat_names.push(Person {
                name: name.to_string(),
                family_number: Some(number),
            });
        }
    }
    flat_names
}

pub fn shuffle_persons(mut v: Vec<Person>) -> Vec<Person> {
    let mut rng = thread_rng();
    v.shuffle(&mut rng);
    v
}

pub fn sort_assignments_alphabetically(mut assignments: Vec<Assignment>) -> Vec<Assignment> {
    assignments
        .sort_by(|assignment1, assignment2| assignment1.giver.name.cmp(&assignment2.giver.name));
    assignments
}

// helper functions (also in sts10/eyeoh)
pub fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches('\n').to_string()),
        Err(error) => Err(error),
    }
}

pub fn read_by_line(file_path: &PathBuf) -> io::Result<Vec<String>> {
    let mut vec = Vec::new();
    let f = match File::open(file_path) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line {
            Ok(l) => vec.push(l.trim().to_string()),
            Err(e) => {
                eprintln!("Error reading a line in file: {}", e);
                return Err(e);
            }
        }
    }
    Ok(vec)
}
