use serde::Deserialize;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

/// This exists just to serialize the inputted JSON file
#[derive(Deserialize, Debug)]
pub struct Names {
    pub names: Vec<Vec<String>>,
}

/// Read NAMES file. Can handle either CSV or JSON. It tells the difference based solely on the
/// file extension.
pub fn read_file(names_file: &Path) -> Vec<Vec<String>> {
    if names_file
        .extension()
        .expect("Unable to detect file extension of inputted NAMES file")
        .eq_ignore_ascii_case("csv")
    {
        read_csv(names_file)
    } else if names_file
        .extension()
        .expect("Unable to detect file extension of inputted NAMES file")
        .eq_ignore_ascii_case("json")
    {
        read_json(names_file)
    } else {
        panic!(
            "Unable to detect file type of Names file. This program requires either a .csv or .json file. Check file extension."
        );
    }
}

/// Read inputted CSV file
fn read_csv(file_path: &Path) -> Vec<Vec<String>> {
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

/// Read inputted JSON file
fn read_json(file_path: &Path) -> Vec<Vec<String>> {
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    let data: Names = serde_json::from_reader(reader).unwrap();
    data.names
}
