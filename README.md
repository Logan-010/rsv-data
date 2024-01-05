RSV is a minimal bianary alternative to CSV solving some of its problems, escape charecters and encoding. RSV aims to solve this by representing new data and new rows with special bytes that no utf8 string can contain. This allows for strings with no escape charecters. This code was adapted from the code at https://github.com/Stenway/RSV-Challenge/blob/main/Rust/src/main.rs

Heres some examples on how to use this:

Open RSV file and assign some data to it
```rust
let mut data: Rsv = Rsv::open("myRsv.rsv").unwrap();
data.set_data(vec![
    vec![Some("Hello user".to_string()), None],
    vec![Some("\n\\\'\"".to_string()), Some("😁🔃📖".to_string())]
]);

data.save().unwrap();
```

Create RSV file and assign some data to it
```rust
let mut data: Rsv = Rsv::create("myRsv.rsv").unwrap();
data.set_data(vec![
    vec![Some("Hello user".to_string()), None],
    vec![Some("\n\\\'\"".to_string()), Some("😁🔃📖".to_string())]
]);

data.save().unwrap();
```

Go to https://docs.rs/rsv-data/