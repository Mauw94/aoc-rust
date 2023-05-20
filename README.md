## Usage

### Scaffold a day
```sh
# example: `cargo scaffold 1`
cargo scaffold <day>
```
### Download input & description for a day
```sh
# example: `cargo download 1`
cargo download <day>
```
### Run solutions for a day
```sh
# example: `cargo solve 01`
cargo solve <day>
```
### Run all solutions

```sh
cargo all
```
### Read puzzle description in terminal

```sh
# example: `cargo read 1`
cargo read <day>
```
To read inputs for previous years, append the `--year/-y` flag. _(example: `cargo read 1 --year 2020`)_

### Run all solutions against the example input

```sh
cargo test
```

To run tests for a specific day, append `--bin <day>`, e.g. `cargo test --bin 01`. You can further scope it down to a specific part, e.g. `cargo test --bin 01 part_one`.