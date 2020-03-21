# casclib-rs

Command line tool to extract files from a CASC archive.

## Usage

Create a `config.toml` file:

```toml
[storage]
path = "/Applications/Warcraft III/Data"
listfile = "./temp/listfile.txt"
[extract]
globs = [
  "war3.w3mod:customkeys.txt",
  "war3.w3mod:units/*func.txt"
]
out_dir = "./temp/files"
```

Then run this program:

```
Fluxs-MacBook-Pro:casc-extract fluxxu$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
     Running `target/debug/casc-extract`
creating output dir: ./temp/files
opening storage: /Applications/Warcraft III/Data
extracting files matching globs:
- war3.w3mod:customkeys.txt
- war3.w3mod:units/*func.txt
reading listfile: ./temp/listfile.txt
listfile entries: 110740
done. 110740 files scanned, 24 extracted, 0 skipped.
```
