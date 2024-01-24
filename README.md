# Rust CSV Manipulation CLI

This Rust command-line tool provides a set of CSV file manipulation commands, allowing users to perform operations such as displaying, modifying, deleting rows/columns/entries, merging files, sorting, adding rows, and more.

## Features

- **Display**: View the contents of your CSV file with various options.
- **Modify**: Edit rows, columns, and entries in a straightforward manner.
- **Delete**: Delete rows, columns, and entries in a straightforward manner.
- **Merge**: Combine two CSV files into a single cohesive dataset.
- **Sort and Display**: Arrange rows in ascending order for easy analysis.
- **Add**: Append new rows and columns effortlessly.

## Install(As a Crate)
### Add to your Cargo.toml
```bash
csvr = "0.1.0"
```

### Use in code
```bash
use csvr::{ CSVFile, CSVError, FileDataUtils };
```

## Usage(Locally)
### Load File
```bash
cargo run csvfile.csv
```
(The CLI opens up if the file is loaded successfully. **Does not load empty files.**)

## Examples

#### View Commands and Usage

```bash
>>> help
```

#### Display Row

```bash
>>> display row 1
```

#### Display Column

```bash
>>> -d col 3
```

#### Delete Column

```bash
>>> delete col 3
```

#### Delete Item

```bash
>>> -r item 3 5
```

#### Merge File

```bash
>>> merge secondcsv.csv
```

#### Write to Loaded File

```bash
>>> write
```

#### Write to New File(Creates if Doesn't Exist)

```bash
>>> write fileName.csv
```

#### Exit the CLI

```bash
>>> exit
```

## Upcoming Updates
- **Merging** multiple files with same dimensions.
- **Deleting** multiple rows, cols and entries at once.
