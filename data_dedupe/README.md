
# Data Dedupe

This project is a command-line tool for finding duplicate files on your filesystem. It recursively searches a specified path, computes checksums for each file, and identifies files with identical content.

## Features

- Walks through directories and collects all files
- Computes MD5 checksums for file contents
- Groups files by checksum to find duplicates
- Outputs lists of duplicate files

## Usage

Run the tool with the `--path` argument to specify the directory to search:

```
deduper --path /your/directory
```

The tool will print the number of files found, the number of duplicates, and the paths of duplicate files.

