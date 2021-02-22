# Words

simple rust library to iterate over all words in a given string


## Usage

```rust
use words::*;

let iter_over_words = "hello world".words();
```

Now you can do fun stuff like this:
```
for word in iter_over_words {
    println!("{}", word);
}
```

