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
Claire gives to Cameron
Phil gives to Mitchell
Haley gives to Manny
Alex gives to Jay
Luke gives to Gloria
Baby gives to Lily
Jay gives to Haley
Gloria gives to Alex
Manny gives to Baby
Cameron gives to Phil
Mitchell gives to Luke
Lily gives to Claire
```

## How to use

[Install Rust](https://www.rust-lang.org/en-US/install.html). Clone down this directory, `cd` into it, then run `cargo run`. You'll be prompted to enter the file location of your CSV file.

`test-names.csv` provides a template for this CSV input file-- basically each immediate family goes on its own line.

Optionally, you can enter a text file listing who gave to who in previous years. This file should look just like the output text (i.e. "Claire gives to Cameron").

Once your CSV file is ready, run `cargo run`, enter the file location, and see who's giving to who!

## Notes about the CSV input file

Given the way I have the Rust code reading the CSV, it will ignore the first line of the CSV file. So just keep that as a generic title, like "Names".

Generally I'd recommend creating and editing the CSV files in a spreadsheet editor like Microsoft Excel or [LibreOffice](https://www.libreoffice.org/) Calc, rather than a text editor like Vim or Sublime Text.
