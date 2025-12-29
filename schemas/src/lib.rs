//! Statically embedded XML schemas for publishing standards.
//!
//! This is an umbrella crate that re-exports all available schema crates.
//! Use feature flags to include only the schemas you need.
//!
//! # Features
//!
//! - `full` - Include all schemas
//! - `dita` - OASIS DITA 1.2
//! - `dita13` - OASIS DITA 1.3
//! - `dita-lce` - DITA Learning Content Education
//! - `niso-sts` - NISO Standards Tag Suite 1.0
//! - `jats` - JATS 1.4 (Journal Article Tag Suite)
//! - `bits` - BITS 2.2 (Book Interchange Tag Suite)
//! - `docbook` - DocBook 5.1
//! - `akoma-ntoso` - Akoma Ntoso 3.0 (Legal Documents)
//! - `tei` - TEI P5 (Text Encoding Initiative)
//! - `spl` - FDA SPL (Pharmaceutical Package Inserts)
//!
//! # Example
//!
//! ```toml
//! [dependencies]
//! schemas = { version = "0.1", features = ["dita13", "jats"] }
//! ```
//!
//! ```ignore
//! use schemas::prelude::*;
//!
//! // Access DITA 1.3 schemas
//! println!("{} files", Dita13::file_count());
//!
//! // Access JATS schemas
//! for file in Jats14::files_by_extension("xsd") {
//!     println!("{}", file.path().display());
//! }
//! ```

// Re-export core types (always available)
pub use schemas_core::{
    self as core, BundleSummary, Dir, DirEntry, File, SchemaBundle, SchemaBundleExt, SchemaError,
};

// Conditionally re-export schema crates
#[cfg(feature = "dita")]
pub use schemas_dita::{self as dita, Dita12};

#[cfg(feature = "dita13")]
pub use schemas_dita13::{self as dita13, Dita13};

#[cfg(feature = "dita-lce")]
pub use schemas_dita_lce::{self as dita_lce, DitaLce};

#[cfg(feature = "niso-sts")]
pub use schemas_niso_sts::{self as niso_sts, NisoSts};

#[cfg(feature = "jats")]
pub use schemas_jats::{self as jats, Jats14};

#[cfg(feature = "bits")]
pub use schemas_bits::{self as bits, Bits22};

#[cfg(feature = "docbook")]
pub use schemas_docbook::{self as docbook, DocBook51};

#[cfg(feature = "akoma-ntoso")]
pub use schemas_akoma_ntoso::{self as akoma_ntoso, AkomaNtoso30};

#[cfg(feature = "tei")]
pub use schemas_tei::{self as tei, TeiP5};

#[cfg(feature = "spl")]
pub use schemas_spl::{self as spl, Spl};

/// Prelude module for convenient imports.
///
/// ```ignore
/// use schemas::prelude::*;
/// ```
pub mod prelude {
    pub use crate::{BundleSummary, SchemaBundle, SchemaBundleExt, SchemaError};

    #[cfg(feature = "dita")]
    pub use crate::Dita12;

    #[cfg(feature = "dita13")]
    pub use crate::Dita13;

    #[cfg(feature = "dita-lce")]
    pub use crate::DitaLce;

    #[cfg(feature = "niso-sts")]
    pub use crate::NisoSts;

    #[cfg(feature = "jats")]
    pub use crate::Jats14;

    #[cfg(feature = "bits")]
    pub use crate::Bits22;

    #[cfg(feature = "docbook")]
    pub use crate::DocBook51;

    #[cfg(feature = "akoma-ntoso")]
    pub use crate::AkomaNtoso30;

    #[cfg(feature = "tei")]
    pub use crate::TeiP5;

    #[cfg(feature = "spl")]
    pub use crate::Spl;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_core_types_available() {
        // Core types should always be available
        fn _check_types<T: SchemaBundle>() {}
    }

    #[cfg(feature = "dita")]
    #[test]
    fn test_dita_available() {
        assert!(Dita12::file_count() > 0);
    }

    #[cfg(feature = "dita13")]
    #[test]
    fn test_dita13_available() {
        assert!(Dita13::file_count() > 0);
    }

    #[cfg(feature = "jats")]
    #[test]
    fn test_jats_available() {
        assert!(Jats14::file_count() > 0);
    }

    #[cfg(feature = "full")]
    #[test]
    fn test_all_schemas_available() {
        use crate::prelude::*;

        assert!(Dita12::file_count() > 0);
        assert!(Dita13::file_count() > 0);
        assert!(DitaLce::file_count() > 0);
        assert!(NisoSts::file_count() > 0);
        assert!(Jats14::file_count() > 0);
        assert!(Bits22::file_count() > 0);
        assert!(DocBook51::file_count() > 0);
        assert!(AkomaNtoso30::file_count() > 0);
        assert!(TeiP5::file_count() > 0);
        assert!(Spl::file_count() > 0);
    }
}
