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

## Is this library performant?
Yes.

A simple benchmark I ran on the entirety of _alice in wonderland_ (148574 bytes, 27345 words) took 598μs on an 2019 MacBook Pro (2,6 GHz 6-Core Intel Core i7)



## Why was this created?
This library was made in order to aid with document analyses, specifically search engine related research.
