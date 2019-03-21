extern crate csv;
extern crate rand;
use rand::prelude::*;
// use std::collections::hash_map::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;

pub fn find_gift_givers<'a>(
    names: &[(String, usize)],        // this is like &Vec<Vec<String>>
    previous_years_giving: &[String], // and this is like &Vec<String> , but it's a slice I guess
    special_requests: &[String],
) -> Option<Vec<(String, String)>> {
    let mut receiving_vec: Vec<String> = [].to_vec();
    let mut givers_vec: Vec<String> = [].to_vec();
    let mut pairs: Vec<(String, String)> = [].to_vec();

    // first, handle special requests
    for request in special_requests {
        // need to find receiver's name here

        let request_vec: Vec<&str> = request.split(' ').collect();
        givers_vec.push(request_vec[0].to_string());
        receiving_vec.push(request_vec[3].to_string());
        pairs.push((request_vec[0].to_string(), request_vec[3].to_string()));
    }

    for giver in names {
        if givers_vec.contains(&giver.0) {
            continue;
        }
        // if we're here, we didn't find a special request of who they should give to,
        // so we need to find a receiver for them

        match find_receiver_for(
            &giver.0,
            giver.1,
            &names,
            &receiving_vec,
            previous_years_giving,
        ) {
            Some(name) => {
                receiving_vec.push(name.clone());
                pairs.push((giver.0.clone(), name));
            }
            None => return None, // println!("Couldn't find solution. Please run program again."),
        }
    }
    Some(pairs)
}

fn find_receiver_for(
    giver_name: &str,
    giver_family_number: usize,
    // names: &Vec<(String, usize)>,
    names: &[(String, usize)],

    receiving_vec: &[String],
    previous_years_giving: &[String],
) -> Option<String> {
    let mut rng = thread_rng();
    let mut potential_receiver: (String, usize);
    let mut loop_counter = 0;

    loop {
        loop_counter += 1;
        if loop_counter > 1000 {
            // We painted ourselves into a corner!
            return None;
        }

        let names_length = names.len();
        potential_receiver = names[rng.gen_range(0, names_length)].clone();

        // What makes a bad receiver?
        //   - potential receiver is already receiving
        //   - potential receiver IS this giver
        //   - potential receiver is in this giver's family
        //   - potential receiver has given to this person in previous years

        if receiving_vec.contains(&potential_receiver.0)
            || potential_receiver.0 == giver_name
            || giver_family_number == potential_receiver.1
            || previous_years_giving
                .contains(&format!("{} gives to {}", giver_name, potential_receiver.0))
        {
            // go to the next iteration of the loop and find a new potential_receiver
            continue;
        } else {
            // if I'm here, I know I have got a valid receiver for this giver. let's break out of the loop and return
            // the receiver's name!
            break;
        }
    }

    Some(potential_receiver.0)
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

pub fn shuffle_families(families: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut shuffled_families: Vec<Vec<String>> = vec![];
    let mut rng = thread_rng();

    for mut family in families {
        rng.shuffle(&mut family);
        shuffled_families.push(family);
    }
    rng.shuffle(&mut shuffled_families);
    shuffled_families
}

pub fn flatten_and_shuffle(families: Vec<Vec<String>>) -> Vec<(String, usize)> {
    let mut rng = thread_rng();
    let mut flat_names: Vec<(String, usize)> = vec![];

    for (number, family) in families.iter().enumerate() {
        for name in family {
            flat_names.push((name.to_string(), number));
        }
    }
    rng.shuffle(&mut flat_names);
    flat_names
}

// helper functions (also in sts10/eyeoh)
pub fn gets() -> io::Result<String> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_n) => Ok(input.trim_end_matches('\n').to_string()),
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

#[cfg(test)]
mod integration_tests {
    use super::*;

    fn make_a_list() -> Vec<(String, String)> {
        let names_file_path = "test-files/test-names.csv";
        let names: Vec<Vec<String>> = read_csv(&names_file_path);
        let names = flatten_and_shuffle(names);

        let previous_years_file_path = "test-files/previous-years-giving-list-test.txt";
        let previous_years_giving: Vec<String> = if previous_years_file_path.is_empty() {
            [].to_vec()
        } else {
            read_by_line(&previous_years_file_path).unwrap()
        };

        let special_requests_file_path = "test-files/special-requests-test.txt";
        let special_requests: Vec<String> = if special_requests_file_path.is_empty() {
            [].to_vec()
        } else {
            read_by_line(&special_requests_file_path).unwrap()
        };

        // loop until we get a good solution
        loop {
            match find_gift_givers(&names, &previous_years_giving, &special_requests) {
                Some(pairs) => {
                    return pairs;
                }
                None => {
                    continue;
                }
            };
        }
    }

    #[test]
    fn claire_gives() {
        let pairs = make_a_list();
        assert_eq!(pairs[0].0, "Claire");
    }

    #[test]
    fn can_fulfill_special_request() {
        for _ in 0..1000 {
            let pairs = make_a_list();
            assert_eq!(pairs[0].0, "Claire");
            assert_eq!(pairs[0].1, "Jay");

            assert_eq!(pairs[1].0, "Alex");
            assert_eq!(pairs[1].1, "Gloria");

            assert_eq!(pairs[2].0, "Haley");
            assert_eq!(pairs[2].1, "Manny");
        }
    }
    use std::collections::HashSet;
    use std::hash::Hash;
    fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
    {
        let mut uniq = HashSet::new();
        iter.into_iter().all(move |x| uniq.insert(x))
    }

    fn get_givers_vec(pairs: Vec<(String, String)>) -> Vec<String> {
        let mut givers = vec![];
        for pair in pairs {
            givers.push(pair.0);
        }
        givers
    }

    #[test]
    fn no_repeat_givers() {
        for _ in 0..1000 {
            let pairs = make_a_list();
            let givers = get_givers_vec(pairs);
            assert!(has_unique_elements(givers));
        }
    }

    fn get_receivers_vec(pairs: Vec<(String, String)>) -> Vec<String> {
        let mut receivers = vec![];
        for pair in pairs {
            receivers.push(pair.1);
        }
        receivers
    }

    #[test]
    fn no_repeat_receivers() {
        for _ in 0..1000 {
            let pairs = make_a_list();
            let receivers = get_receivers_vec(pairs);
            assert!(has_unique_elements(receivers));
        }
    }

    #[test]
    fn sufficiently_random_basic_test() {
        for _ in 0..1000 {
            let pairs = make_a_list();
            let mut pair_one_count: f64 = 0 as f64;
            let mut pair_two_count: f64 = 0 as f64;
            if pairs.contains(&("Phil".to_string(), "Cameron".to_string())) {
                pair_one_count = pair_one_count + 1.0;
            }

            if pairs.contains(&("Manny".to_string(), "Claire".to_string())) {
                pair_two_count = pair_two_count + 1.0;
            }
            assert!(
                pair_one_count <= pair_two_count * 1.0005
                    || pair_one_count >= pair_two_count * 1.0005
            );
        }
    }

    fn make_a_list_no_requests_or_previous_years() -> Vec<(String, String)> {
        let names_file_path = "test-files/test-names.csv";
        let names: Vec<Vec<String>> = read_csv(&names_file_path);
        let names = flatten_and_shuffle(names);

        let previous_years_file_path = "";
        let previous_years_giving: Vec<String> = if previous_years_file_path.is_empty() {
            [].to_vec()
        } else {
            read_by_line(&previous_years_file_path).unwrap()
        };

        let special_requests_file_path = "";
        let special_requests: Vec<String> = if special_requests_file_path.is_empty() {
            [].to_vec()
        } else {
            read_by_line(&special_requests_file_path).unwrap()
        };

        // loop until we get a good solution
        loop {
            match find_gift_givers(&names, &previous_years_giving, &special_requests) {
                Some(pairs) => {
                    return pairs;
                }
                None => {
                    continue;
                }
            };
        }
    }
    fn look_up_number_of_potential_receivers(giver_name: &str) -> usize {
        // hard code this I think?!
        if giver_name == "Claire".to_string() {
            6
        } else if giver_name == "Phil".to_string() {
            6
        } else if giver_name == "Cameron".to_string() {
            8
        } else if giver_name == "Manny".to_string() {
            8
        } else {
            0
        }

        // match giver_name {
        //     ("Claire".to_string()) => 7,
        //     "Manny".to_string() => 5,
        // }
    }

    use std::collections::HashMap;
    fn individual_giver_chi_test(giver_name: String, upper_tail_critical: f64) -> bool {
        // run 1000 trials to get a Vector of observed values

        let mut observed_receivers_hashmap: HashMap<String, usize> = HashMap::new();

        for _ in 0..1000 {
            let pairs = make_a_list_no_requests_or_previous_years();
            for pair in pairs {
                if pair.0 == giver_name {
                    observed_receivers_hashmap
                        .entry(pair.1)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }
        // this won't find cases where Claire _never_ gives to a legitimate receiver... _probably_ not a huge deal

        // convert the Hashmap to a Vector to make it easier to iterate through
        let observed_receivers_vec: Vec<(&String, &usize)> =
            observed_receivers_hashmap.iter().collect();

        // now calculate the chi-squared statistic
        let mut chi_squared_statistic: f64 = 0.0;
        println!("For {}... ", giver_name);
        for (_n, observed_receiver_name_and_count) in observed_receivers_vec.iter().enumerate() {
            let receiver_name = observed_receiver_name_and_count.0;
            let observed_count = observed_receiver_name_and_count.1;

            let expected_count: f64 =
                1000.0 / (look_up_number_of_potential_receivers(&giver_name) as f64);

            println!(
                "We expected {} to give to {} {} times out of 1000; Observed: {} times out of 1000",
                giver_name, receiver_name, expected_count, observed_count
            );

            chi_squared_statistic = chi_squared_statistic
                + (*observed_count as f64 - expected_count as f64).powf(2.0)
                    / expected_count as f64;
        }

        println!(
            "For {}, found a chi squared of {}",
            giver_name, chi_squared_statistic
        );

        chi_squared_statistic < upper_tail_critical
    }

    // https://en.wikibooks.org/wiki/Engineering_Tables/Chi-Squared_Distibution
    #[test]
    fn chi_squared_test_claire() {
        assert!(individual_giver_chi_test("Claire".to_string(), 11.070))
    }

    #[test]
    fn chi_squared_test_phil() {
        assert!(individual_giver_chi_test("Phil".to_string(), 11.070))
    }

    #[test]
    fn chi_squared_test_cameron() {
        assert!(individual_giver_chi_test("Cameron".to_string(), 14.067))
    }

    #[test]
    fn chi_squared_test_manny() {
        assert!(individual_giver_chi_test("Manny".to_string(), 14.067))
    }
    // Other test ideas
    // 1. Check that no previous year pairs are assigned
    // 2. Check that no one gets someone in their own family
    // 3. Somehow check for randomness or 10,000 executions
}
