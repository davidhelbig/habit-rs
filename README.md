# habit-rs

A simple habit tracker written in Rust. Currently support rudimentary command line interface and data is managed via json files.

Add a new habit and track it in file `habits.json`:

``` rust
habit-rs -f habits.json new --name reading
```

Mark a date as completed:

``` rust
habit-rs -f habits.json done reading 2022-04-30
```
