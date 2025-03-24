// here we use a struct to define our data types. PathBuf is like a String, but for system paths that works cross platform
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}


fn main() {
    // first and second argument in the command line (cargo run -- some-pattern some-file) with some exceptions
    let pattern = std::env::args().nth(1).expect("no pattern given");
    let path = std::env::args().nth(2).expect("no path given");

    println!("pattern: {pattern}, path: {path}")
}