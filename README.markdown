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

[Install Rust](https://www.rust-lang.org/en-US/install.html). Clone down this directory, `cd` into it, then run `cargo run`. You'll be prompted to enter the file location of your CSV file.

`test-names.csv` provides a template for this CSV input file-- basically each immediate family goes on its own line.

You can either edit that `test-names.csv` file and submit that file's path, OR create your own CSV file with your family's names.

Once your CSV file is ready, run `cargo run`, enter the file location, and see who's giving to who!

## Notes about the CSV input file

Given the way I have the Rust code reading the CSV, it will ignore the first line of the CSV file. So just keep that as a generic title, like "Names".

Generally I'd recommend creating and editing the CSV files in a spreadsheet editor like Microsoft Excel or [LibreOffice](https://www.libreoffice.org/) Calc, rather than a text editor like Vim or Sublime Text.
