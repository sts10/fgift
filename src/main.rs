extern crate csv;
extern crate rand;

use std::fs::File;

use rand::prelude::*;

fn main() {
    let names: Vec<Vec<String>> = read_csv();
    let mut receiving_vec: Vec<String> = [].to_vec();
    let mut family_number = 0;
    for family in &names {
        for giver in family {
            match find_receiver_for(giver, family_number, &names, &receiving_vec) {
                Some(name) => receiving_vec.push(name),
                None => println!("Couldn't find solution. Please run program again."),
            }
        }
        family_number += 1;
    }
}

fn find_receiver_for(
    giver_name: &str,
    giver_family_number: usize,
    names: &Vec<Vec<String>>,
    receiving_vec: &Vec<String>,
) -> Option<String> {
    // find rand number between 1 and number_of_familes that is NOT `family_number`
    let mut rng = thread_rng();
    let mut potential_receiver_family_number;
    let mut potential_receiver_member_number;
    let mut potential_receiver_name;
    let mut loop_counter = 0;
    loop {
        loop_counter += 1;
        if loop_counter > 1000 {
            // We painted ourselves into a corner!
            return None;
        }

        potential_receiver_family_number = rng.gen_range(0, names.len());
        potential_receiver_member_number =
            rng.gen_range(0, names[potential_receiver_family_number].len());
        potential_receiver_name =
            &names[potential_receiver_family_number][potential_receiver_member_number];

        // what makes a bad receiver?
        //   - potential receiver is already receiving
        //   - potential receiver IS this giver
        //   - potential receiver is in this giver's family
        if receiving_vec.contains(&potential_receiver_name.to_string())
            || potential_receiver_name == giver_name
            || giver_family_number == potential_receiver_family_number
        {
            // go to the next iteration of the loop
            continue;
        } else {
            // if I'm here, I know I have got a good one. let's break out of the loop and push
            break;
        }
    }

    println!("{} is giving to {}", giver_name, potential_receiver_name);

    Some(potential_receiver_name.to_string())
}

fn read_csv() -> Vec<Vec<String>> {
    let mut names: Vec<Vec<String>> = [].to_vec();

    let file_path = "test-names.csv";

    let file = File::open(file_path).unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    // Loop over each record.
    for result in rdr.records() {
        // An error may occur, so abort the program in an unfriendly way.
        let record = result.expect("a CSV record");
        let mut family_vec_strings: Vec<String> = [].to_vec();
        // let family_vec: Vec<&str> = record.iter().collect();
        for name in record.iter() {
            if name.len() > 1 {
                family_vec_strings.push(name.to_string());
            }
        }

        names.push(family_vec_strings);
    }
    names
}
