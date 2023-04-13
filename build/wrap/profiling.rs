use std::{fs::File, io::{Read, Write}};

use regex::Regex;

/// Finds all `pub fn` in a file and wraps them with `#[profiling::function]`
pub fn auto_profile_exported_fns(rust_file_path: &str) {
    // Open the file
    let mut file = File::open(rust_file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Find all `pub fn` and wrap them with `#[profiling::function]`
    let exported_fn_re = Regex::new(r"\s{4}pub fn").unwrap();
    let mut new_contents = String::new();
    for line in contents.lines() {
        if exported_fn_re.is_match(line) {
            new_contents.push_str(&format!("    #[profiling::function]\n{}\n", line));
        } else {
            new_contents.push_str(&format!("{}\n", line));
        }
    }

    // Re-write the file
    let mut file = File::create(rust_file_path).unwrap();
    file.write_all(new_contents.as_bytes()).unwrap();
}
