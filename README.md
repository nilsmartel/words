# Word Iter

simple rust library to iterate over all words in a given string


## Usage

```rust
// by using this line, you can call the .words() method in strings
use word_iter::*;

let iter_over_words = "hello world".words();
```

Now you can do fun stuff like this:
```rust
for word in iter_over_words {
    println!("{}", word);
}
```
