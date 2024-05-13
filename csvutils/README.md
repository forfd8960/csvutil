# csvutils

## Merge

```sh
~/D/r/c/csvutils> cargo run -- merge --inputs test.csv --inputs test1.csv --output merge.csv
   Compiling csvutils v0.1.0 (~/Documents/rust-code/csvutil/csvutils)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.74s
     Running `target/debug/csvutils merge --inputs test.csv --inputs test1.csv --output merge.csv`
Options { cmd: Merge(MergeOpts { inputs: ["test.csv", "test1.csv"], output: "merge.csv" }) }
MergeOpts { inputs: ["test.csv", "test1.csv"], output: "merge.csv" }
StringRecord(["f1", "f2", "f3"])
StringRecord(["f1", "f2", "f3"])
process_merge: ()
```

## Filter

```sh
~/D/r/c/csvutils>
cargo run -- filter --input merge.csv --output filter.csv --exps f1=111 --exps f2=e
   Compiling csvutils v0.1.0 (~/Documents/rust-code/csvutil/csvutils)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/csvutils filter --input merge.csv --output filter.csv --exps f1=111 --exps f2=e`

Options { cmd: Filter(FilterOpts { input: "merge.csv", output: "filter.csv", exps: ["f1=111", "f2=e"] }) }
FilterOpts { input: "merge.csv", output: "filter.csv", exps: ["f1=111", "f2=e"] }
fields: {"f2": "e", "f1": "111"}
StringRecord(["f1", "f2", "f3"])
idx_value: {1: "e", 0: "111"}
process_filter: ()
```
