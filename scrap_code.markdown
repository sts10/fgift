
```rust
        let potential_receiver_family_number = rng.gen_range(0, names.len());
        let potential_receiver_member_number = rng.gen_range(0, names[potential_receiver_family_number].len());
        potential_receiver_name = &names[potential_receiver_family_number][potential_receiver_member_number];
```
