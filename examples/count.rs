use word_iter::*;

// Count the amount of words in a given file
fn main() {
    let filename = std::env::args().nth(1).unwrap_or_else(|| {
        eprintln!("expected input file name as first argument");
        std::process::exit(0)
    });

    let content = std::fs::read_to_string(filename).expect("to read file");

    let bytes = content.as_bytes().len();
    println!("bytes read: {bytes}");

    let start = std::time::Instant::now();

    let count = content.words().count();

    let duration = std::time::Instant::now().duration_since(start);

    println!("words counted: {count}");
    println!("elapsed time: {}μs", duration.as_micros());
}
