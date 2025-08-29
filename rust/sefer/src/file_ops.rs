use std::fs;
use std::io;

/// Read a file and return its contents as a String
pub fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}

/// Write content to a file
pub fn write_file(path: &str, content: &str) -> Result<(), io::Error> {
    fs::write(path, content)
}

/// Generate an output filename by replacing or adding the specified extension
pub fn generate_output_filename(input_path: &str, extension: &str) -> String {
    if input_path.contains('.') {
        let base = input_path.rsplit_once('.').unwrap().0;
        format!("{}.{}", base, extension)
    } else {
        format!("{}.{}", input_path, extension)
    }
}

/// Check if a file exists
pub fn file_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

/// Get file size in bytes
pub fn file_size(path: &str) -> Result<u64, io::Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_output_filename() {
        assert_eq!(
            generate_output_filename("test.json", "btrl"),
            "test.btrl"
        );
        assert_eq!(
            generate_output_filename("test", "btrl"),
            "test.btrl"
        );
        assert_eq!(
            generate_output_filename("path/to/test.json", "books.json"),
            "path/to/test.books.json"
        );
    }
}