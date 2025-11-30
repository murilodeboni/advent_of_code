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

/// Resolve the input path for a given day name (e.g., `d01`), automatically
/// selecting the test file when running under `cargo test` . 
/// `base_dir` should be the year crate's `CARGO_MANIFEST_DIR`, 
/// since a library cannot know the caller's manifest.
/// Input files are expected at `<base_dir>/src/bin/inputs/{name}.txt` and
/// `{name}_test.txt`.
pub fn input_path(base_dir: &str, name: &str) -> String {
    let base = format!("{base_dir}/src/bin/inputs/{name}");
    let use_test = cfg!(test);
    if use_test {
        format!("{base}_test.txt")
    } else {
        format!("{base}.txt")
    }
}

/// Convenience wrapper to read the resolved input for a given day name.
pub fn read_input(base_dir: &str, name: &str) -> Vec<String> {
    read_lines_unwrap(input_path(base_dir, name))
}
