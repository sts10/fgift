
```rust
        let potential_receiver_family_number = rng.gen_range(0, names.len());
        let potential_receiver_member_number = rng.gen_range(0, names[potential_receiver_family_number].len());
        potential_receiver_name = &names[potential_receiver_family_number][potential_receiver_member_number];
```


```rust

fn _sort_families(mut names: Vec<Vec<String>>) -> Vec<Vec<String>> {
    names.sort_by(|family1, family2| family1.len().cmp(&family2.len()));
    names.reverse();
    names
}

fn _shuffle_families(families: Vec<Vec<String>>) -> Vec<Vec<String>> {
    let mut shuffled_families: Vec<Vec<String>> = vec![];
    let mut rng = thread_rng();

    for mut family in families {
        rng.shuffle(&mut family);
        shuffled_families.push(family);
    }
    rng.shuffle(&mut shuffled_families);
    shuffled_families
}


```

```rust
pub fn _get_file_path() -> String {
    // let file_path = gets().unwrap();
    // let file_path = file_path.trim_matches(|c| c == '\'' || c == ' ');
    // file_path.to_string()
    gets()
        .unwrap()
        .trim_matches(|c| c == '\'' || c == ' ')
        .to_string()
}
```

```rust
// here's the old special request handler logic
for request in special_requests {
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
```
