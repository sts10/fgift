# FGift: Family Gift List Maker

Takes an input of a CSV file, describing an extended family, with each line an immediate family. Outputs a list of who should give to who, ensuring that no one gives to anyone in their immediate family, but otherwise selecting randomly. One common use-case for this would be where you need to create "Secret Santa" give assignments.

Obviously you can substitute the concept of "families" for any small groups of people, such as teams at your workplace.

## Usage

```text
Usage: fgift [OPTIONS] <NAMES CSV FILE>

Arguments:
  <NAMES CSV FILE>  CSV of family names

Options:
  -v, --verbose...
          Prints verbose output, including parameters as received. Can accept either 
          one or two count
  -p, --previous <PREVIOUS_YEARS_FILE>
          Provide file with previous years giving
  -s, --special <SPECIAL_REQUESTS_FILE>
          Provide file with special requests (assignments that must be made)
  -o, --output <OUTPUT>
          Print assignments to a file, rather than to the terminal
  -h, --help
          Print help
  -V, --version
          Print version
```

## Example input CSV

To begin, you'll need a CSV where each row is an immediate (nuclear) family to provide as an input to this tool.

For example, when you give this program a CSV file that looks like this...

```csv
Names,,,,
Claire,Phil,Haley,Alex,Luke
Cameron,Mitchell,Lily,,
Jay,Gloria,Manny,,
```

Running `fgift names.csv` gives you an output like this:

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

The gift assignments are randomized, _except_ that no one gives to anyone in their immediate family. For example, Claire does not give to Phil, Haley, Alex, or Luke, since they are in the same immediate family. This information -- that Claire is in an immediate family with Phil, Haley, Alex, and Luke -- is established (or encoded) in the inputted CSV file, by the fact that all those names are in the same row. 

## Installation 

### Using Rust
1. [Install Rust](https://www.rust-lang.org/tools/install) if you haven't already
2. Install FGift: `cargo install --git https://github.com/sts10/fgift --branch main`

You should now be able to run `fgift` from anywhere in your terminal. Run `fgift --help` for help.

To **upgrade** your installation of FGift, run `cargo install --force --git https://github.com/sts10/fgift --branch main`. 

### From a GitHub release
Alternatively, if you don't want to install Rust and Cargo, I periodically publish Releases to GitHub, each with an executable file that you can download.

To install the executable on a Linux/macOS machine, download the `fgift` executable and move it to somewhere in your `$PATH`, like `$HOME/.local/bin` (you can do this on the command line with something like `mv ~/Downloads/fgift ~/.local/bin/`). You may also need to give the executable permissions with something like `chmod +x ~/.local/bin/fgift`.

### Uninstalling FGift and Rust

To uninstall FGift, run: `cargo uninstall fgift`

To uninstall Rust/`cargo` (if you install Rust solely to run FGift, for example), as of 2023, run: `rustup self uninstall` ([source](https://www.rust-lang.org/tools/install)).

## Examples

- `fgift tests/test-files/test-names.csv` Creates random gift assignments from Names file `test-names.csv`, without assigning anyone to give to members of their immediate family (as denoted by rows in the `test-names.csv` file (see above for examples).

- `fgift -p=tests/test-files/previous-years-giving-list-test.txt tests/test-files/test-names.csv` Creates gift assignments without repeating any assignments found in `previous-years-giving-list-test.txt`

- `fgift -p=tests/test-files/previous-years-giving-list-test.txt -o=this-years-assignments.txt tests/test-files/test-names.csv` Creates gift assignments without repeating any assignments found in `previous-years-giving-list-test.txt`. Writes created assignments to text file `this-years-giving.txt` instead of to the terminal.

- `fgift -p=tests/test-files/previous-years-giving-list-test.txt tests/test-files/test-names.csv > this-years-assignments.txt` Same as above; just uses `>` to write to an output file.

- `fgift -p tests/test-files/previous-years-giving-list-test.txt -s tests/test-files/special-requests-test.txt tests/test-files/test-names.csv` Creates gift assignments that respects special requests made in `special-requests-test.txt` file, then avoids repeating any assignments found in `previous-years-giving-list-test.txt`.

## Options

As shown in the above examples, you can provide (a) a text file of who has given to who in previous years, if you want to avoid reassignments, and/or (b) a file with special requests, which will be fulfilled as specified. 

Both of these optional files should look just like the output text (i.e. "Claire gives to Cameron"), with each assignment on its own line.

## Notes about the NAMES CSV file

Given the way I have the Rust code reading the CSV, it will ignore the first line of the CSV file. So just keep that as a generic title, like "Names".

Generally I'd recommend creating and editing the CSV files in a spreadsheet editor like Microsoft Excel or [LibreOffice](https://www.libreoffice.org/) Calc, rather than a text editor like Vim or Sublime Text.

## Notes on randomness

For an investigation into how "random" the selections this programs makes, see ["Notes on Randomness" document](./notes-on-randomness.markdown).
