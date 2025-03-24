use clap::Parser; // clap is a arg parse tool we can use

// here we use a struct to define our data types. 

#[derive(Parser)] // reads the command-line arguments, matches them to the fields in your struct, and parses them into Rust types automatically
struct Cli {
    /// the pattern to look for in our file
    pattern: String,
    /// the path to the file to read, PathBuf is like a String, but for system paths that works cross platform
    path: std::path::PathBuf,
}


fn main() {
    let args = Cli::parse();

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path)
}