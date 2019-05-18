mod integration_tests {
    use fgift::*;
    use std::path::PathBuf;

    fn make_a_list(
        names_file_path: PathBuf,
        previous_years_file: Option<PathBuf>, // and this is like &Vec<String> , but it's a slice I guess
        special_requests_file: Option<PathBuf>,
    ) -> Vec<Assignment> {
        let names: Vec<Person> = flatten_and_shuffle(read_csv(names_file_path));

        let previous_years_giving: Vec<String> = match previous_years_file {
            Some(file_path) => read_by_line(file_path).unwrap(),
            None => vec![],
        };

        let special_requests: Vec<String> = match special_requests_file {
            Some(file_path) => read_by_line(file_path).unwrap(),
            None => vec![],
        };

        // loop until we get a good solution
        loop {
            match find_gift_givers(&names, &previous_years_giving, &special_requests) {
                Some(assignment_pairs) => {
                    return assignment_pairs;
                }
                None => {
                    continue;
                }
            };
        }
    }

    #[test]
    fn claire_gives() {
        let assignment_pairs = make_a_list(
            PathBuf::from("tests/test-files/test-names.csv"),
            Some(PathBuf::from(
                "tests/test-files/previous-years-giving-list-test.txt",
            )),
            Some(PathBuf::from("tests/test-files/special-requests-test.txt")),
        );
        assert_eq!(assignment_pairs[0].giver.name, "Claire");
    }

    #[test]
    fn can_fulfill_special_request() {
        for _ in 0..1000 {
            let assignment_pairs = make_a_list(
                PathBuf::from("tests/test-files/test-names.csv"),
                Some(PathBuf::from(
                    "tests/test-files/previous-years-giving-list-test.txt",
                )),
                Some(PathBuf::from("tests/test-files/special-requests-test.txt")),
            );
            assert_eq!(assignment_pairs[0].giver.name, "Claire");
            assert_eq!(assignment_pairs[0].receiver.name, "Jay");

            assert_eq!(assignment_pairs[1].giver.name, "Alex");
            assert_eq!(assignment_pairs[1].receiver.name, "Gloria");

            assert_eq!(assignment_pairs[2].giver.name, "Haley");
            assert_eq!(assignment_pairs[2].receiver.name, "Manny");
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

    fn get_givers_vec(assignments: Vec<Assignment>) -> Vec<String> {
        let mut givers_names = vec![];
        for assignment in assignments {
            givers_names.push(assignment.giver.name);
        }
        givers_names
    }

    #[test]
    fn no_repeat_givers() {
        for _ in 0..1000 {
            let assignments = make_a_list(
                PathBuf::from("tests/test-files/test-names.csv"),
                Some(PathBuf::from(
                    "tests/test-files/previous-years-giving-list-test.txt",
                )),
                Some(PathBuf::from("tests/test-files/special-requests-test.txt")),
            );
            let givers_names = get_givers_vec(assignments);
            assert!(has_unique_elements(givers_names));
        }
    }

    fn get_receivers_vec(assignments: Vec<Assignment>) -> Vec<String> {
        let mut receivers_names = vec![];
        for assignment in assignments {
            receivers_names.push(assignment.receiver.name);
        }
        receivers_names
    }

    #[test]
    fn no_repeat_receivers() {
        for _ in 0..1000 {
            let assignments = make_a_list(
                PathBuf::from("tests/test-files/test-names.csv"),
                Some(PathBuf::from(
                    "tests/test-files/previous-years-giving-list-test.txt",
                )),
                Some(PathBuf::from("tests/test-files/special-requests-test.txt")),
            );
            let receivers_names = get_receivers_vec(assignments);
            assert!(has_unique_elements(receivers_names));
        }
    }

    #[test]
    fn no_one_gives_to_own_family_member() {
        for _ in 0..1000 {
            let assignments =
                make_a_list(PathBuf::from("tests/test-files/test-names.csv"), None, None);
            for assignment in assignments {
                assert!(assignment.giver.family_number != assignment.receiver.family_number);
            }
        }
    }

    #[test]
    fn no_assignments_from_previous_years_are_given() {
        let previous_years_file =
            PathBuf::from("tests/test-files/previous-years-giving-list-test.txt");

        // let previous_years_giving: Vec<String> = match previous_years_file {
        //     Some(file_path) => read_by_line(file_path).unwrap(),
        //     None => vec![],
        // };
        let previous_years_giving: Vec<String> = read_by_line(previous_years_file.clone()).unwrap();

        for _ in 0..1000 {
            let assignments = make_a_list(
                PathBuf::from("tests/test-files/test-names.csv"),
                Some(previous_years_file.clone()),
                None,
            );

            for assignment in assignments {
                // if we ever have a match with a previous year, test fails.
                // so `assert` that there is no match ever time. If it ever fails, test fails.
                assert!(!previous_years_giving.contains(&format!(
                    "{} gives to {}",
                    assignment.giver.name, assignment.receiver.name
                )))
            }
        }
    }

    #[test]
    fn sufficiently_random_basic_test() {
        for _ in 0..1000 {
            let assignment_pairs = make_a_list(
                PathBuf::from("tests/test-files/test-names.csv"),
                Some(PathBuf::from(
                    "tests/test-files/previous-years-giving-list-test.txt",
                )),
                Some(PathBuf::from("tests/test-files/special-requests-test.txt")),
            );
            let mut pair_one_count: f64 = 0 as f64;
            let mut pair_two_count: f64 = 0 as f64;
            let phil = Person {
                name: "Phil".to_string(),
                family_number: None,
            };
            let claire = Person {
                name: "Claire".to_string(),
                family_number: None,
            };
            let cameron = Person {
                name: "Cameron".to_string(),
                family_number: None,
            };
            let manny = Person {
                name: "Manny".to_string(),
                family_number: None,
            };
            if assignment_pairs.contains(&Assignment {
                giver: phil,
                receiver: cameron,
            }) {
                pair_one_count = pair_one_count + 1.0;
            }

            if assignment_pairs.contains(&Assignment {
                giver: manny,
                receiver: claire,
            }) {
                pair_two_count = pair_two_count + 1.0;
            }
            assert!(
                pair_one_count <= pair_two_count * 1.0005
                    || pair_one_count >= pair_two_count * 1.0005
            );
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
    fn individual_giver_chi_test(giver: Person, upper_tail_critical: f64) -> bool {
        // run 1000 trials to get a Vector of observed values

        let mut observed_receivers_hashmap: HashMap<Person, usize> = HashMap::new();

        for _ in 0..1000 {
            let assignment_pairs =
                make_a_list(PathBuf::from("tests/test-files/test-names.csv"), None, None);
            for assignment in assignment_pairs {
                if assignment.giver.name == giver.name {
                    observed_receivers_hashmap
                        .entry(assignment.receiver)
                        .and_modify(|count| *count += 1)
                        .or_insert(1);
                }
            }
        }
        // this won't find cases where Claire _never_ gives to a legitimate receiver... _probably_ not a huge deal

        // convert the Hashmap to a Vector to make it easier to iterate through
        let observed_receivers_vec: Vec<(&Person, &usize)> =
            observed_receivers_hashmap.iter().collect();

        // now calculate the chi-squared statistic
        let mut chi_squared_statistic: f64 = 0.0;
        println!("For {}... ", giver.name);
        for (_n, observed_receiver_name_and_count) in observed_receivers_vec.iter().enumerate() {
            let receiver = observed_receiver_name_and_count.0;
            let observed_count = observed_receiver_name_and_count.1;

            let expected_count: f64 =
                1000.0 / (look_up_number_of_potential_receivers(&giver.name) as f64);

            println!(
                "We expected {} to give to {} {} times out of 1000; Observed: {} times out of 1000",
                giver.name, receiver.name, expected_count, observed_count
            );

            chi_squared_statistic = chi_squared_statistic
                + (*observed_count as f64 - expected_count as f64).powf(2.0)
                    / expected_count as f64;
        }

        println!(
            "For {}, found a chi squared of {}",
            giver.name, chi_squared_statistic
        );

        chi_squared_statistic < upper_tail_critical
    }

    // https://en.wikibooks.org/wiki/Engineering_Tables/Chi-Squared_Distibution
    #[test]
    fn chi_squared_test_claire() {
        let claire = Person {
            name: "Claire".to_string(),
            family_number: None,
        };
        assert!(individual_giver_chi_test(claire, 11.070))
    }

    #[test]
    fn chi_squared_test_phil() {
        let phil = Person {
            name: "Phil".to_string(),
            family_number: None,
        };
        assert!(individual_giver_chi_test(phil, 11.070))
    }

    #[test]
    fn chi_squared_test_cameron() {
        let cameron = Person {
            name: "Cameron".to_string(),
            family_number: None,
        };
        assert!(individual_giver_chi_test(cameron, 14.067))
    }

    #[test]
    fn chi_squared_test_manny() {
        let manny = Person {
            name: "Manny".to_string(),
            family_number: None,
        };
        assert!(individual_giver_chi_test(manny, 14.067))
    }
}
