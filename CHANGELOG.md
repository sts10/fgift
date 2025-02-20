# v0.3.6

* Update project to Rust [Edition 2024](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0.html). 

# v0.3.5
* 60e29ed - adds dependency checker to readme 
* a8303e0 - upgrades cargo dist version used to create releases 
* f7f82e9 - upgrades rand dependency to v0.9.0 

# v0.3.4

* Upgrade clap and serde dependencies
* Sets `cargo-dist` version used to package releases to v0.15.0.

# v0.3.0

Can now accept NAMES file in JSON format. [#5](https://github.com/sts10/fgift/pull/5)

# v0.2.100

Upgrade cargo-dist version to 0.4.2.

# v0.2.99 

Add SHELL script installer option to cargo-dist configuration.

# v0.2.97 

First release using [cargo-dist](https://opensource.axo.dev/cargo-dist/book/introduction.html). Seems to have gone smoothly.


# v0.2.92

Nothing huge -- mostly dependency upgrades. 

* d9c524b upgrades clap version (previously used version had a security warning from cargo-audit)
* a2fe030 - bumps csv dependency to 1.2.2 
* 54d4291 - grammar fix in readme 
* 0a7533c - moves notes on randomness to a separate markdown file
* 6eb44eb - removes scrap_code file from git

**Full Changelog**: https://github.com/sts10/fgift/compare/v0.2.90...v0.2.92

# v0.2.90

* Switches project's license from MIT License to Mozilla Public License v2.0 (`MPL-2.0`).
* Re-organizes installation section of README to be more clear.

# v0.2.88

Upgrade some dependencies. 
* fcadefe - upgrades Clap dependency 
* 0d593ae - upgrades csv dependency  
* 2679b43 - fixes years in LICENSE  

# v0.2.87
* Creates two levels of verbosity. 
* Makes default output minimally verbose, allowing users to pass output to other programs, like cat or echo.
* Upgrades clap dependency to 4.0.32 and rand to 0.8.5.
* Increases number of trials of two tests, in an effort to make their success less susceptible to chance failures. Not sure how effective this was/is.

# v0.2.82
* Upgrades rand crate to v0.8.4.
* Fixes potential bug in random selection. Now uses `potential_receiver = match persons.choose(&mut rng)` rather than `rng.gen_range(0, persons.len())`, which my have had an off-by-one error?
* Moves a `clone` call to improve performance
* Takes other suggestions from Clippy.

# v0.2.8
A nice, hopefully stable release. 

## Features
* Accepts previous years giving as file
* Accepts special requests as file
