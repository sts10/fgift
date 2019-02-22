extern crate csv;
extern crate rand;
use rand::prelude::*;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
pub fn find_gift_givers(
    names: &[Vec<String>],            // this is like &Vec<Vec<String>>
    previous_years_giving: &[String], // and this is like &Vec<String> , but it's a slice I guess
    special_requests: &[String],
) -> Option<Vec<String>> {
    let mut receiving_vec: Vec<String> = [].to_vec();

    for (family_number, family) in names.iter().enumerate() {
        // family_number is a counter here... it's like an each_with_index

        for giver_name in family {
            let mut found_a_receiver = false;
            // Check the special_requests vec to see if this giver has a special request
            for request in special_requests {
                // need to find receiver's name here
                let request_vec: Vec<&str> = request.split(' ').collect();
                if request_vec[0] == giver_name {
                    receiving_vec.push(request_vec[3].to_string());
                    println!("{}", request);
                    found_a_receiver = true;
                    break;
                }
            }
            // if we're here, we didn't find a special request of who they should give to,
            // so we need to find a receiver for them
            if !found_a_receiver {
                match find_receiver_for(
                    giver_name,
                    family_number,
                    &names,
                    &receiving_vec,
                    previous_years_giving,
                ) {
                    Some(name) => receiving_vec.push(name),
                    None => return None, // println!("Couldn't find solution. Please run program again."),
                }
            }
        }
    }
    Some(receiving_vec)
}

fn find_receiver_for(
    giver_name: &str,
    giver_family_number: usize,
    names: &[Vec<String>],
    receiving_vec: &[String],
    previous_years_giving: &[String],
) -> Option<String> {
    let mut rng = thread_rng();
    let mut potential_receiver_name;
    let mut loop_counter = 0;
    loop {
        loop_counter += 1;
        if loop_counter > 1000 {
            // We painted ourselves into a corner!
            return None;
        }

        let potential_receiver_family_number = rng.gen_range(0, names.len());
        let potential_receiver_member_number =
            rng.gen_range(0, names[potential_receiver_family_number].len());
        potential_receiver_name =
            &names[potential_receiver_family_number][potential_receiver_member_number];

        // what makes a bad receiver?
        //   - potential receiver is already receiving
        //   - potential receiver IS this giver
        //   - potential receiver is in this giver's family
        //   - potential receiver has given to this person in previous years
        if receiving_vec.contains(&potential_receiver_name.to_string())
            || potential_receiver_name == giver_name
            || giver_family_number == potential_receiver_family_number
            || previous_years_giving.contains(&format!(
                "{} gives to {}",
                giver_name, potential_receiver_name
            ))
        {
            // go to the next iteration of the loop
            continue;
        } else {
            // if I'm here, I know I have got a good one. let's break out of the loop and push
            break;
        }
    }

    println!("{} gives to {}", giver_name, potential_receiver_name);

    Some(potential_receiver_name.to_string())
}

pub fn get_file_path() -> String {
    let file_path = gets().unwrap();
    let file_path = file_path.trim_matches(|c| c == '\'' || c == ' ');
    file_path.to_string()
}

pub fn read_csv(file_path: &str) -> Vec<Vec<String>> {
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

pub fn sort_families(mut names: Vec<Vec<String>>) -> Vec<Vec<String>> {
    names.sort_by(|family1, family2| family1.len().cmp(&family2.len()));
    names.reverse();
    names
}

// helper functions (also in sts10/eyeoh)
pub fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches("\n").to_string()),
        Err(error) => Err(error),
    }
}

pub fn read_by_line(file_path: &str) -> io::Result<Vec<String>> {
    let mut vec = Vec::new();
    let f = match File::open(file_path.trim_matches(|c| c == '\'' || c == ' ')) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let file = BufReader::new(&f);
    for line in file.lines() {
        match line {
            Ok(l) => vec.push(l.trim().to_string()),
            Err(e) => {
                eprintln!("Error reading a line in the {}: {}", file_path, e);
                return Err(e);
            }
        }
    }
    Ok(vec)
}

// an idea for a test
//
// println!("Read file as {:?}", previous_years_giving);
// println!("\n");
// let g = "Claire";
// let r = "Cameron";
// if previous_years_giving.contains(&format!("{} gives to {}", g, r)) {
//     println!("Tripped the easy test");
// }
