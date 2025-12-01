use std::{env, fs::read_to_string, io, path::Path};

/// Read input lines as owned `String`s, preserving line order.
pub fn read_lines(path: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let content = read_to_string(path)?;
    Ok(content.lines().map(|line| line.to_string()).collect())
}

/// Convenience wrapper that panics on I/O errors.
pub fn read_lines_unwrap(path: impl AsRef<Path>) -> Vec<String> {
    read_lines(path).expect("failed to read input file")
}

pub fn input_path(base_dir: &str, name: &str, test: bool) -> String {
    let base = format!("{base_dir}/src/bin/inputs/{name}");
    if test {
        format!("{base}_test.txt")
    } else {
        format!("{base}.txt")
    }
}

/// Convenience wrapper to read the resolved input for a given day name.
pub fn read_input(base_dir: &str, name: &str, test: bool) -> Vec<String> {
    read_lines_unwrap(input_path(base_dir, name, test))
}
