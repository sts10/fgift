extern crate csv;
extern crate rand;
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;

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
    persons: &[Person],               // this is like &Vec<Person>
    previous_years_giving: &[String], // and this is like &Vec<String> , but it's a slice I guess
    special_requests: &Option<Vec<String>>,
) -> Option<Vec<Assignment>> {
    // First, handle special requests
    let assignment_pairs_after_special_requests: Vec<Assignment> = match special_requests {
        Some(requests) => add_special_assignments(persons, requests, Vec::new()),
        None => Vec::new(),
    };
    finish_assignments_consider_previous_years(
        persons,
        assignment_pairs_after_special_requests,
        previous_years_giving,
    )
}

fn finish_assignments_consider_previous_years(
    persons: &[Person],
    existing_assignment_pairs: Vec<Assignment>,
    previous_years_giving: &[String],
) -> Option<Vec<Assignment>> {
    let mut assignment_pairs = existing_assignment_pairs;
    // Now do the rest of the random assignments, with consideration for avoiding previous years'
    // assignments
    for giver in persons {
        // Skip givers who were assigned in special requests -- we've already added them to
        // assignment_pairs in the add_special_assignments function (ugh, yes, this isn't great)
        if assignment_pairs.iter().any(|pair| &pair.giver == giver) {
            continue;
        }

        // If we're here, we didn't find a special request of who they should give to,
        // so we need to find a receiver for them
        match find_receiver_for(giver, persons, &assignment_pairs, previous_years_giving) {
            Some(receiver) => {
                assignment_pairs.push(Assignment {
                    giver: giver.clone(),
                    receiver,
                });
            }
            None => {
                return None;
            }
        }
    }
    Some(assignment_pairs)
}

fn add_special_assignments(
    persons: &[Person],
    special_requests: &[String],
    existing_assignment_pairs: Vec<Assignment>,
) -> Vec<Assignment> {
    let mut new_assignments = existing_assignment_pairs;

    for request_string in special_requests {
        let request_vec: Vec<&str> = request_string.split(" gives to ").collect();
        let request_giver_name = request_vec[0].to_string();
        let request_receiver_name = request_vec[1].to_string();

        let this_giver: Option<&Person> = persons.iter().find(|n| n.name == request_giver_name);
        let this_receiver: Option<&Person> =
            persons.iter().find(|n| n.name == request_receiver_name);
        match (this_giver, this_receiver) {
            (Some(g), Some(r)) => new_assignments.push(Assignment {
                giver: g.clone(),
                receiver: r.clone(),
            }),
            _ => panic!("Found a name in submitted special request file that I could not also find in the submitted names CSV file. Please check spelling and capitalization of names in both files."),
        };
    }

    new_assignments
}

fn find_receiver_for(
    giver: &Person,
    persons: &[Person],
    existing_assignment_pairs: &[Assignment],
    previous_years_giving: &[String],
) -> Option<Person> {
    let mut rng = thread_rng();
    let mut potential_receiver: Person;

    for _n in 0..1000 {
        potential_receiver = persons[rng.gen_range(0, persons.len())].clone();

        // What makes a GOOD receiver?
        //   - potential receiver is NOT already receiving
        //   - potential receiver is NOT this giver
        //   - potential receiver is NOT in this giver's family
        //   - potential receiver has NOT given to this person in previous years

        if !existing_assignment_pairs
            .iter()
            .any(|pair| pair.receiver == potential_receiver)
            && &potential_receiver != giver
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
    // Failed to find a receiver for this giver even after 1,000 attempts.
    // Think this means we painted ourselves in a corner given the given
    // restrictions. So we need to start over.
    // Return None and handle this issue elsewhere.
    None
}

pub fn verify_assignments(persons: &[Person], assignment_pairs: &[Assignment]) -> bool {
    if assignment_pairs.len() != persons.len() {
        return false;
    }
    for name in persons {
        // Look through assignments to make sure this name is a giver (gives to someone)...
        let gives: bool = assignment_pairs.iter().any(|pair| &pair.giver == name);
        // and is a receiver (receives from someone)
        let receives: bool = assignment_pairs.iter().any(|pair| &pair.receiver == name);

        // If we at any point have a name that doesn't give or doesn't receive
        // we know this is a bad and unverified list of assignments
        if !gives || !receives {
            return false;
        }
    }
    true
}

pub fn read_csv(file_path: &Path) -> Vec<Vec<String>> {
    let mut names: Vec<Vec<String>> = Vec::new();

    let file = File::open(file_path).expect("Could not open CSV file");
    let mut rdr = csv::Reader::from_reader(file);
    // Loop over each "record", or row in the CSV file
    for result in rdr.records() {
        let record = result.expect("Error reading a record from a CSV file");
        let mut family_vec_strings: Vec<String> = Vec::new();
        for name in record.iter() {
            if !name.is_empty() {
                family_vec_strings.push(name.to_string());
            }
        }

        names.push(family_vec_strings);
    }
    names
}

pub fn make_persons(families: Vec<Vec<String>>) -> Vec<Person> {
    let mut persons: Vec<Person> = Vec::new();

    for (number, family) in families.iter().enumerate() {
        for name in family {
            persons.push(Person {
                name: name.to_string(),
                family_number: Some(number),
            });
        }
    }
    persons
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

pub fn read_by_line(file_path: &Path) -> io::Result<Vec<String>> {
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
