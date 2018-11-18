# Family Gift (Secret Santa) List Maker

Takes an input of a CSV file, describing an extended family, with each line an immediate family. Outputs a list of who should give to who, ensuring that no one gives to anyone in their immediate family, but otherwise selecting randomly.

Obviously you could substitute the concept of "families" for small teams at your work.

## Example CSV

Make a CSV where each row is an immediate family. 

For example, a CSV like this...

```csv
Names,,,,
Claire,Phil,Haley,Alex,Luke
Cameron,Mitchell,Lily,,
Jay,Gloria,Manny,,
```

Could give you an output like this:

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

`test-names.csv` provides a template for this CSV input file-- basically each immediate family goes on its own line, with the first row being a title (it doesn't matter what the title is).

Optionally, you can enter a text file listing who gave to who in previous years. This file should look just like the output text (i.e. "Claire gives to Cameron").

You can also optionally input a text file containing "special requests," i.e. people who _must_ give to another given person. This file should look just like the output text (i.e. "Claire gives to Cameron").

Once your CSV file (and other optional files) is ready, run `cargo run`, enter the requested file location(s), and see who's giving to who!

## Notes about the CSV input file

Given the way I have the Rust code reading the CSV, it will ignore the first line of the CSV file. So just keep that as a generic title, like "Names".

Generally I'd recommend creating and editing the CSV files in a spreadsheet editor like Microsoft Excel or [LibreOffice](https://www.libreoffice.org/) Calc, rather than a text editor like Vim or Sublime Text.

## Notes on "Randomness"

Are the results of this program truly random? Or in other words, is it somehow bias in choosing who gives to whom? At first I thought, yes, sure, but after watching [this video](https://www.youtube.com/watch?v=5kC5k5QBqcc) I'm not so sure.

For example, maybe the program should do a full restart whenever a "bad" choice (i.e. picks herself, picks someone in her family, or someone she has given to in the past) is made randomly. That way, when we finally get a "good" run-through, we're more assured that every selection was freely random. 
