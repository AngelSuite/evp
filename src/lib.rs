#![deny(missing_docs)]
#![deny(clippy::missing_docs_in_private_items)]
#![deny(unsafe_code)]
#![warn(clippy::pedantic)]
#![allow(clippy::too_many_lines)]
#![allow(clippy::cast_possible_truncation)]
#![allow(clippy::cast_sign_loss)]

//! # `evp`
//!
//! `evp` is a library that reads and produces Evidence Packages, as
//! defined by Internet Draft [draft-hopkins-evp-spec](https://hpkns.uk/evp)

/// Exporters allow packages and test cases to be exported to different file formats.
pub mod exporters;
/// Locking file
mod lock_file;
/// The types of data in a package
pub mod package;
/// The results of this crate
pub mod result;
/// Open a ZIP file in a fashion that allows it to be switched between reading and writing.
mod zip_read_writer;

/// Import useful items from the `evp` package in one go.
pub mod prelude {
    pub use super::package::{
        Author, CustomMetadataField, Evidence, EvidenceData, EvidencePackage, MediaFile, Metadata,
        TestCase, TestCaseMetadata, TestCasePassStatus,
    };
    pub use super::result::{Error, Result};
    /// Items relating to creating and parsing attestations
    pub mod attesting {
        /// The attestation JWS type
        pub type Attestation = biscuit::jws::Compact<Vec<u8>, biscuit::Empty>;
        pub use biscuit::errors::*;
        pub use biscuit::jwa::SignatureAlgorithm as Algorithm;
        /// The attestation JWS header
        pub type Header = biscuit::jws::Header<biscuit::Empty>;
        pub use biscuit::jws::{RegisteredHeader, Secret};
        /// The attestation JWS set for validation
        pub type JWKSet = biscuit::jwk::JWKSet<biscuit::Empty>;
        /// The attestation JWS for validation
        pub type JWK = biscuit::jwk::JWK<biscuit::Empty>;
        pub use biscuit::jwk::*;
    }
}
