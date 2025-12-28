//! NISO Standards Tag Suite (STS) Schemas
//!
//! This crate provides statically embedded NISO STS 1.0 XSD schema files.
//! NISO STS is a standard for the XML encoding of standards documents.
//!
//! # Included Tag Sets
//!
//! - **Interchange**: For exchange between organizations
//! - **Extended**: With additional elements for publishing
//!
//! Both tag sets include MathML 3.0 support.
//!
//! # Example
//!
//! ```
//! use schemas_niso_sts::NisoSts;
//! use schemas_core::SchemaBundle;
//!
//! // List all schema files
//! for path in NisoSts::list_paths() {
//!     println!("{}", path.display());
//! }
//!
//! // Find interchange schemas
//! for file in NisoSts::interchange_files() {
//!     println!("{}", file.path().display());
//! }
//! ```
//!
//! # License
//!
//! NISO STS schemas are made openly available for public use.
//! See the LICENSE file for details.

pub use schemas_core::{BundleSummary, Dir, File, SchemaBundle, SchemaBundleExt, SchemaError};

use include_dir::include_dir;

static SCHEMA_DIR: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/../niso/schemas");

/// NISO STS Schema Bundle
pub struct NisoSts;

impl SchemaBundle for NisoSts {
    const NAME: &'static str = "NISO STS";
    const VERSION: &'static str = "1.0";
    const LICENSE: &'static str = "NISO";

    fn dir() -> &'static Dir<'static> {
        &SCHEMA_DIR
    }
}

impl NisoSts {
    /// Get interchange tag set schemas only.
    pub fn interchange_files() -> impl Iterator<Item = &'static File<'static>> {
        Self::files().filter(|f| f.path().to_string_lossy().contains("interchange"))
    }

    /// Get extended tag set schemas only.
    pub fn extended_files() -> impl Iterator<Item = &'static File<'static>> {
        Self::files().filter(|f| f.path().to_string_lossy().contains("extended"))
    }

    /// Get MathML schemas.
    pub fn mathml_files() -> impl Iterator<Item = &'static File<'static>> {
        Self::files().filter(|f| f.path().to_string_lossy().contains("mathml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_count() {
        assert!(NisoSts::file_count() > 0, "Should have schema files");
    }

    #[test]
    fn test_has_interchange_and_extended() {
        let interchange: Vec<_> = NisoSts::interchange_files().collect();
        let extended: Vec<_> = NisoSts::extended_files().collect();

        assert!(!interchange.is_empty(), "Should have interchange schemas");
        assert!(!extended.is_empty(), "Should have extended schemas");
    }

    #[test]
    fn test_summary() {
        let summary = NisoSts::summary();
        assert_eq!(summary.name, "NISO STS");
        assert_eq!(summary.version, "1.0");
        assert!(summary.file_count > 0);
    }
}
