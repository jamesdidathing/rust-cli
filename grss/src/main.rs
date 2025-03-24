use clap::Parser; // clap is a arg parse tool we can use

// here we use a struct to define our data types. 

#[derive(Parser)] // reads the command-line arguments, matches them to the fields in your struct, and parses them into Rust types automatically
struct Cli {
    // the pattern to look for in our file
    pattern: String,
    // the path to the file to read, PathBuf is like a String, but for system paths that works cross platform
    path: std::path::PathBuf,
}


fn main() {
    // using clap, we get some automatic help messages if we put no paths in
    let args = Cli::parse();

    // lets start our file reading
    let content = std::fs::read_to_string(&args.path).expect("Could not read file!");

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
}