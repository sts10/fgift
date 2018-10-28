# Family Gift List Maker

Takes an input of a csv file, describing an extended family, with each line an immediate family. Outputs a list of who should give to who, ensuring that no one gives to anyone in their immediate family, but otherwise selecting randomly.

## Example CSV

```csv
Names,,,,
Claire,Phil,Haley,Alex,Luke
Cameron,Mitchell,Lily,,
Jay,Gloria,Manny,,
```

## How to use

[Install Rust](https://www.rust-lang.org/en-US/install.html). 

Then either (a) change the included "test-names.csv" file to your extended family OR (b) create your own csv file and change any and all references to "test-names.csv" to your file name.

Then run `cargo run` and see who's giving to who!


