# Family Gift List Maker

Takes an input of a CSV file, describing an extended family, with each line an immediate family. Outputs a list of who should give to who, ensuring that no one gives to anyone in their immediate family, but otherwise selecting randomly.

## Example CSV

```csv
Names,,,,
Claire,Phil,Haley,Alex,Luke
Cameron,Mitchell,Lily,,
Jay,Gloria,Manny,,
```

## How to use

[Install Rust](https://www.rust-lang.org/en-US/install.html). 

Then either (a) change the included `test-names.csv` file to your extended family OR (b) create your own CSV file and change any and all references to `test-names.csv` to your file name.

Then run `cargo run` and see who's giving to who!

## Notes about the CSV input file

Given the way I have the Rust code reading the CSV, it will ignore the first line of the CSV file. So just keep that as a generic title, like "Names".

Generally I'd recommend creating and editing the CSV files in a spreadsheet editor like Excel or LibreOffice Calc, rather than a text editor like Vim or Sublime Text.
