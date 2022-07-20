# Error reproduction with multiple parameters

## Case 1: Without flags

```sh
cargo run -- 3 4 5 6 7 8
```

```
input: ["3", "4", "5", "6", "7"]
output: "8"
one: None
two: None
yes: Some(false)
```

## Case 2: with one flag

```sh
cargo run -- --one 1 3 4 5 6 7 8
```

```
input: ["3", "4", "5", "6", "7"]
output: "8"
one: Some("1")
two: None
yes: Some(false)
```

with second parameter everything works too

```sh
cargo run -- --two 1 3 4 5 6 7 8
```

```
input: ["3", "4", "5", "6", "7"]
output: "8"
one: None
two: Some("1")
yes: Some(false)
```

or with boolean parameter

```sh
cargo run -- --yes 3 4 5 6 7 8
```

```
input: ["3", "4", "5", "6", "7"]
output: "8"
one: None
two: None
yes: Some(true)
```

## Case 3: with many flags

```sh
cargo run -- --one 1 --two 2 3 4 5 6 7 8
```

```
error: Found argument '4' which wasn't expected, or isn't valid in this context

USAGE:
    clap-multi-arg-repro [OPTIONS] <input>... <output>
```

The second parameter can be a boolean type. It doesn't matter.

```sh
cargo run -- --yes --two 2 3 4 5 6 7 8
```

```
error: Found argument '4' which wasn't expected, or isn't valid in this context

USAGE:
    clap-multi-arg-repro [OPTIONS] <input>... <output>
```
