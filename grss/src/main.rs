use clap::Parser; // clap is a arg parse tool we can use
use anyhow::{Context, Result}; // anyhow is useful for custom errors

// here we use a struct to define our data types. 

#[derive(Parser)] // reads the command-line arguments, matches them to the fields in your struct, and parses them into Rust types automatically
struct Cli {
    // the pattern to look for in our file
    pattern: String,
    // the path to the file to read, PathBuf is like a String, but for system paths that works cross platform
    path: std::path::PathBuf,
}


fn main() -> Result<()> {
    // using clap, we get some automatic help messages if we put no paths in
    let args = Cli::parse();

    // lets start our file reading with some error handling
    // If an error occurs, add context to the error message with the file path for easier debugging.
    // The "?" operator at the end propagates the error, making the function return early if reading fails.
    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;


    find_matches(&content, &args.pattern, &mut std::io::stdout());

    Ok(())
}

// add third parameter writer, which can be any type that has implement Write. This can be a file, stdout, memory buffer... anything that can write
fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line); // writeln! writes formatted data out
        }
    }
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\ndolor sit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n");
}