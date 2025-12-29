# schemas-rs

Rust crates providing statically embedded XML schemas for various standards.

## Crates

| Crate | Description | License |
|-------|-------------|---------|
| `schemas-core` | Core traits and types | MIT/Apache-2.0 |
| `schemas-dita` | OASIS DITA 1.2 schemas | OASIS IPR |
| `schemas-dita13` | OASIS DITA 1.3 schemas | Apache-2.0 |
| `schemas-dita-lce` | DITA Learning Content Education | Apache-2.0 |
| `schemas-niso-sts` | NISO Standards Tag Suite 1.0 | NISO (open use) |
| `schemas-jats` | JATS 1.4 (Journal Article Tag Suite) | Public Domain |
| `schemas-bits` | BITS 2.2 (Book Interchange Tag Suite) | Public Domain |
| `schemas-docbook` | DocBook 5.1 (RNG/Schematron) | BSD-2-Clause |
| `schemas-akoma-ntoso` | Akoma Ntoso 3.0 (Legal Documents) | CC-BY-4.0 |
| `schemas-tei` | TEI P5 (Text Encoding Initiative) | BSD-2-Clause |
| `schemas-spl` | FDA SPL (Pharmaceutical Package Inserts) | BSD-3-Clause |

## Usage

Add the desired schema crate to your `Cargo.toml`:

```toml
[dependencies]
schemas-dita = "0.1"
# or
schemas-niso-sts = "0.1"
```

### List Schema Files

```rust
use schemas_dita::Dita12;
use schemas_core::SchemaBundle;

// List all files
for path in Dita12::list_paths() {
    println!("{}", path.display());
}

// Get file count and total size
println!("{} files, {} bytes",
    Dita12::file_count(),
    Dita12::total_size());
```

### Access Schema Content

```rust
use schemas_dita::Dita12;
use schemas_core::SchemaBundle;

// Get a specific file
if let Some(file) = Dita12::get_file("xsd1.2/base/xsd/basemap.xsd") {
    let content = std::str::from_utf8(file.contents()).unwrap();
    println!("Content: {}", &content[..200]);
}

// Find all XSD files
for file in Dita12::files_by_extension("xsd") {
    println!("{}: {} bytes", file.path().display(), file.contents().len());
}
```

### Write Schemas to Disk

```rust
use schemas_dita::Dita12;
use schemas_core::SchemaBundle;
use std::path::Path;

// Extract all schemas to a directory
let count = Dita12::write_to_directory(Path::new("./schemas")).unwrap();
println!("Wrote {} files", count);
```

## Implementation

This crate uses [`include_dir`](https://crates.io/crates/include_dir) for zero-copy static embedding of schema files. No build.rs code generation required.

## License Compliance

The schema files retain their original licenses:

- **DITA 1.2**: OASIS Intellectual Property Rights Policy
- **DITA 1.3**: Apache License 2.0
- **DITA LCE**: Apache License 2.0 (Copyright Birgit Strackenbrock)
- **NISO STS**: Open use encouraged by NISO
- **JATS 1.4**: Public Domain (NLM/NIH)
- **BITS 2.2**: Public Domain (NLM/NIH)
- **DocBook 5.1**: BSD-2-Clause (OASIS DocBook TC)
- **Akoma Ntoso 3.0**: CC-BY-4.0 (OASIS Open)
- **TEI P5**: BSD-2-Clause (TEI Consortium)
- **SPL**: BSD-3-Clause (HL7 / FDA)

The wrapper Rust code (`schemas-core`) is dual-licensed under MIT/Apache-2.0.

## Not Included

**S1000D** schemas are NOT included due to licensing restrictions that prohibit redistribution. Users needing S1000D schemas should download them directly from [users.s1000d.org](https://users.s1000d.org/).
