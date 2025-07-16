
# Vehicle Manufacturers Search CLI

This project is a command-line tool written in Rust for searching vehicle manufacturers using the NHTSA public API. It allows users to search for manufacturers by name, common name, or country.

## How it works

- Fetches manufacturer data from the NHTSA API (`vpic.nhtsa.dot.gov`)
- Accepts a search term as a command-line argument
- Filters manufacturers whose name, common name, or country contains the search term
- Displays details for each matching manufacturer

## Usage

Run the tool with a search term:

```
cargo run -- <search term>
```

Example:

```
cargo run -- BMW
```

This will display all manufacturers matching "BMW" in their name, common name, or country.

## References

* [NHTSA Vehicle API](https://vpic.nhtsa.dot.gov/api/)
* [rust-cli-template](https://github.com/kbknapp/rust-cli-template)
