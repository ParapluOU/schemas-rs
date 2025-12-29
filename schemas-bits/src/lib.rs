//! BITS (Book Interchange Tag Suite) 2.2 XML Schemas
//!
//! This crate provides statically embedded BITS 2.2 XSD schema files
//! with MathML 3 support that can be accessed at runtime or written to disk.
//!
//! # Example
//!
//! ```
//! use schemas_bits::Bits22;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in Bits22::list_paths() {
//!     println!("{}", path.display());
//! }
//!
//! // Get a specific file
//! if let Some(file) = Bits22::get_file("BITS-book2-2.xsd") {
//!     println!("Content: {} bytes", file.contents().len());
//! }
//!
//! // Find all XSD files
//! for file in Bits22::files_by_extension("xsd") {
//!     println!("{}: {} bytes", file.path().display(), file.contents().len());
//! }
//! ```
//!
//! # License
//!
//! The BITS schemas are in the public domain (NLM/NIH).
//! See the LICENSE file for details.

pub use schemas_core::{BundleSummary, Dir, File, SchemaBundle, SchemaBundleExt, SchemaError};

use include_dir::include_dir;

static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../bits/schemas");

/// BITS 2.2 Schema Bundle (Book Interchange Tag Suite)
pub struct Bits22;

impl SchemaBundle for Bits22 {
    const NAME: &'static str = "BITS";
    const VERSION: &'static str = "2.2";
    const LICENSE: &'static str = "Public Domain";

    fn dir() -> &'static Dir<'static> {
        &SCHEMA_DIR
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(Bits22::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_list_paths() {
        let paths: Vec<_> = Bits22::list_paths().collect();
        assert!(!paths.is_empty());
        assert!(paths.iter().any(|p| p.extension().is_some_and(|e| e == "xsd")));
    }

    #[test]
    fn test_files_by_extension() {
        let xsd_files: Vec<_> = Bits22::files_by_extension("xsd").collect();
        assert!(!xsd_files.is_empty());
        for file in &xsd_files {
            assert!(file.path().extension().is_some_and(|e| e == "xsd"));
        }
    }

    #[test]
    fn test_summary() {
        let summary = Bits22::summary();
        assert_eq!(summary.name, "BITS");
        assert_eq!(summary.version, "2.2");
        assert!(summary.file_count > 0);
        assert!(summary.total_size > 0);
    }
}
