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
    let mut assignment_pairs: Vec<Assignment> = Vec::new();

    // First, handle special requests
    assignment_pairs = add_special_assignments(special_requests, assignment_pairs);

    // Now do the rest of the random assignments, with consideration for avoiding previous years'
    // assignments
    for giver in names {
        // skip givers who were assigned in special requests (ugh, yes, this isn't great)
        if assignment_pairs
            .iter()
            .any(|pair| pair.giver.name.to_lowercase() == giver.name.to_lowercase())
        {
            continue;
        }

        // If we're here, we didn't find a special request of who they should give to,
        // so we need to find a receiver for them
        match find_receiver_for(giver, &names, &assignment_pairs, previous_years_giving) {
            Some(receiver) => {
                assignment_pairs.push(Assignment {
                    giver: giver.clone(),
                    receiver,
                });
            }
            None => return None,
        }
    }
    Some(assignment_pairs)
}

fn add_special_assignments(
    special_requests: &[String],
    existing_assignment_pairs: Vec<Assignment>,
) -> Vec<Assignment> {
    let mut new_assignments = existing_assignment_pairs;

    for request in special_requests {
        // I'd love to compress these 3 lines into 1
        let request_vec: Vec<&str> = request.split(" gives to ").collect();
        let request_giver_name = request_vec[0].to_string();
        let request_receiver_name = request_vec[1].to_string();

        let giver = Person {
            name: request_giver_name,
            family_number: None,
        };
        let receiver = Person {
            name: request_receiver_name,
            family_number: None,
        };
        new_assignments.push(Assignment { giver, receiver });
    }
    new_assignments
}

fn find_receiver_for(
    giver: &Person,
    names: &[Person],
    existing_assignment_pairs: &[Assignment],
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
    // Unable to find a receiver for this giver. Return None.
    None
}

pub fn verify_assignments(names: &[Person], assignment_pairs: &[Assignment]) -> bool {
    if assignment_pairs.len() != names.len() {
        return false;
    }
    for name in names {
        // look through assignments to make sure this name gives to someone...
        let gives: bool = assignment_pairs.iter().any(|pair| &pair.giver == name);
        // and receives from someone
        let receives: bool = assignment_pairs.iter().any(|pair| &pair.receiver == name);

        // If we at any point have a name that doesn't give or doesn't receive
        // we know this is a bad and unverified list of assignments
        if !gives || !receives {
            return false;
        }
    }
    true
}

pub fn read_csv(file_path: &PathBuf) -> Vec<Vec<String>> {
    let mut names: Vec<Vec<String>> = [].to_vec();

    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    // Loop over each "record", or row in the CSV file
    for result in rdr.records() {
        let record = result.expect("Error reading a record from a CSV file");
        let mut family_vec_strings: Vec<String> = [].to_vec();
        for name in record.iter() {
            if name != "" {
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
