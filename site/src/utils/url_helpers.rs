/*!
 * URL Parsing Utilities
 *
 * Shared utilities for parsing Bible application URLs and extracting
 * book, chapter, and verse information from URL paths and parameters.
 */

use urlencoding::decode;

/// Parse book and chapter information from URL pathname
///
/// Extracts the book name and chapter number from URL paths like:
/// - `/Genesis/1` -> Some(("Genesis", 1))
/// - `/1%20Timothy/3` -> Some(("1 Timothy", 3))
/// - `/invalid` -> None
///
/// # Arguments
/// * `pathname` - URL pathname to parse (e.g., "/Genesis/1")
///
/// # Returns
/// * `Some((book_name, chapter_number))` if parsing succeeds
/// * `None` if the path format is invalid
pub fn parse_book_chapter_from_url(pathname: &str) -> Option<(String, u32)> {
    let path_parts: Vec<&str> = pathname.trim_start_matches('/').split('/').collect();

    if path_parts.len() == 2 {
        // Decode URL-encoded book name (handles spaces as %20, etc.)
        let book_name = if let Ok(decoded) = decode(path_parts[0]) {
            decoded.into_owned()
        } else {
            // Fallback: replace underscores with spaces
            path_parts[0].replace('_', " ")
        };

        // Parse chapter number
        if let Ok(chapter_num) = path_parts[1].parse::<u32>() {
            return Some((book_name, chapter_num));
        }
    }

    None
}

/// Validate if a URL path represents a valid Bible chapter
///
/// Checks if the given pathname follows the expected format
/// for Bible chapter URLs: `/{book_name}/{chapter_number}`
///
/// # Arguments
/// * `pathname` - URL pathname to validate
///
/// # Returns
/// * `true` if the path has the correct format
/// * `false` if the path is invalid or incomplete
pub fn is_valid_chapter_path(pathname: &str) -> bool {
    parse_book_chapter_from_url(pathname).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_book_chapter_basic() {
        assert_eq!(
            parse_book_chapter_from_url("/Genesis/1"),
            Some(("Genesis".to_string(), 1))
        );
    }

    #[test]
    fn test_parse_book_chapter_url_encoded() {
        assert_eq!(
            parse_book_chapter_from_url("/1%20Timothy/3"),
            Some(("1 Timothy".to_string(), 3))
        );
    }

    #[test]
    fn test_parse_book_chapter_underscore_fallback() {
        assert_eq!(
            parse_book_chapter_from_url("/1_Timothy/3"),
            Some(("1 Timothy".to_string(), 3))
        );
    }

    #[test]
    fn test_parse_book_chapter_invalid_paths() {
        assert_eq!(parse_book_chapter_from_url("/Genesis"), None);
        assert_eq!(parse_book_chapter_from_url("/Genesis/invalid"), None);
        assert_eq!(parse_book_chapter_from_url("/"), None);
        assert_eq!(parse_book_chapter_from_url(""), None);
    }

    #[test]
    fn test_is_valid_chapter_path() {
        assert!(is_valid_chapter_path("/Genesis/1"));
        assert!(is_valid_chapter_path("/1%20Timothy/3"));
        assert!(!is_valid_chapter_path("/Genesis"));
        assert!(!is_valid_chapter_path("/invalid"));
    }
}
