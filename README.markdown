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

## Testing for bias

To see if the program indeed had such a bias, I decided to try to write a series of tests in which I aim to perform a [Pearson chi-squared test](https://en.wikipedia.org/wiki/Pearson%27s_chi-squared_test). The tests and their associated helper function are currently in `src/lib.rs` (the first one is called `chi_squared_test_claire`). Note that I am not a statistician and relied mostly on my decades-old AP Stats knowledge in making this choice... so if you have a better idea of how to test for this bias please leave and issue! 

As the code stands, the tests pass when testing "givers" in the _largest_ family (Claire and Phil are examples, but **fail** for givers in the smaller families (Cameron and Manny are the two I test currently). I can't figure out how to get these two tests to pass.

## Attempts to pass my bias tests

In an effort to make these two tests pass (as written), I had the code shuffle both the order of families and the order of names within the families for each trial (see: `fn shuffle_families` in `src/lib.rs`). I also tried assigning receivers to the two small families first and _then_ assigning members of the large family. Neither made the tests pass. 

Given that one of the requirements of the system is that you cannot be assigned to give to a member of your own immediate family (and that nuclear families will almost certainly be of different sizes), I'm not sure if it's even possible to make the chi-squared tests pass as I've written them. 

That said... 

### Other ideas for passing my bias tests that I haven't tried

One idea, though: the program should do a full restart whenever a "bad" choice (i.e. picks herself, picks someone in her family, or someone she has given to in the past) is made randomly. That way, when we finally get a "good" run-through, we're more assured that every selection was freely random. 
