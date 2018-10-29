# Family Gift List Maker

Takes an input of a CSV file, describing an extended family, with each line an immediate family. Outputs a list of who should give to who, ensuring that no one gives to anyone in their immediate family, but otherwise selecting randomly.

## Example CSV

A CSV like this...

```csv
Names,,,,
Claire,Phil,Haley,Alex,Luke
Cameron,Mitchell,Lily,,
Jay,Gloria,Manny,,
```

Will give you an output like this:

```
Claire is giving to Manny
Phil is giving to Jay
Haley is giving to Cameron
Alex is giving to Gloria
Luke is giving to Lily
Cameron is giving to Luke
Mitchell is giving to Alex
Lily is giving to Haley
Jay is giving to Claire
Gloria is giving to Mitchell
Manny is giving to Phil
```

## How to use

[Install Rust](https://www.rust-lang.org/en-US/install.html). 

Then either (a) change the included `test-names.csv` file to your extended family OR (b) create your own CSV file and change any and all references to `test-names.csv` to your file name.

Then run `cargo run` and see who's giving to who!

## Notes about the CSV input file

Given the way I have the Rust code reading the CSV, it will ignore the first line of the CSV file. So just keep that as a generic title, like "Names".

Generally I'd recommend creating and editing the CSV files in a spreadsheet editor like Excel or LibreOffice Calc, rather than a text editor like Vim or Sublime Text.
