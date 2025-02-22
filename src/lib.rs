//! [![GitHub Workflow Status](https://img.shields.io/github/workflow/status/Serial-ATA/lofty-rs/CI?style=for-the-badge&logo=github)](https://github.com/Serial-ATA/lofty-rs/actions/workflows/ci.yml)
//! [![Downloads](https://img.shields.io/crates/d/lofty?style=for-the-badge&logo=rust)](https://crates.io/crates/lofty)
//! [![Version](https://img.shields.io/crates/v/lofty?style=for-the-badge&logo=rust)](https://crates.io/crates/lofty)
//!
//! Parse, convert, and write metadata to audio formats.
//!
//! # Supported Formats
//!
//! | File Format | Metadata Format(s)                   |
//! |-------------|--------------------------------------|
//! | Ape         | `APEv2`, `APEv1`, `ID3v2`\*, `ID3v1` |
//! | AIFF        | `ID3v2`, `Text Chunks`               |
//! | FLAC        | `Vorbis Comments`, `ID3v2`\*         |
//! | MP3         | `ID3v2`, `ID3v1`, `APEv2`, `APEv1`   |
//! | MP4         | `iTunes-style ilst`                  |
//! | Opus        | `Vorbis Comments`                    |
//! | Ogg Vorbis  | `Vorbis Comments`                    |
//! | Speex       | `Vorbis Comments`                    |
//! | WAV         | `ID3v2`, `RIFF INFO`                 |
//!
//! \* The tag will be **read only**, due to lack of official support
//!
//! # Examples
//!
//! ## Reading a generic file
//!
//! It isn't always convenient to [use concrete file types](#using-concrete-file-types), which is where [`TaggedFile`]
//! comes in.
//!
//! ### Using a path
//!
//! ```rust
//! # use lofty::LoftyError;
//! # fn main() -> Result<(), LoftyError> {
//! use lofty::{read_from_path, Probe};
//!
//! // First, create a probe.
//! // This will guess the format from the extension
//! // ("mp3" in this case), but we can guess from the content if we want to.
//! let tagged_file = read_from_path("tests/files/assets/minimal/full_test.mp3", false)?;
//!
//! // Let's guess the format from the content just in case.
//! // This is not necessary in this case!
//! let tagged_file2 = Probe::open("tests/files/assets/minimal/full_test.mp3")?
//! 	.guess_file_type()?
//! 	.read(false)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Using an existing reader
//!
//! ```rust
//! # use lofty::LoftyError;
//! # fn main() -> Result<(), LoftyError> {
//! use lofty::read_from;
//! use std::fs::File;
//!
//! // Let's read from an open file
//! let mut file = File::open("tests/files/assets/minimal/full_test.mp3")?;
//!
//! // Here, we have to guess the file type prior to reading
//! let tagged_file = read_from(&mut file, false)?;
//! # Ok(())
//! # }
//! ```
//!
//! ### Accessing tags
//!
//! ```rust
//! # use lofty::LoftyError;
//! # fn main() -> Result<(), LoftyError> {
//! use lofty::read_from_path;
//!
//! let tagged_file = read_from_path("tests/files/assets/minimal/full_test.mp3", false)?;
//!
//! // Get the primary tag (ID3v2 in this case)
//! let id3v2 = tagged_file.primary_tag();
//!
//! // If the primary tag doesn't exist, or the tag types
//! // don't matter, the first tag can be retrieved
//! let unknown_first_tag = tagged_file.first_tag();
//! # Ok(())
//! # }
//! ```
//!
//! ## Using concrete file types
//!
//! ```rust
//! # use lofty::LoftyError;
//! # fn main() -> Result<(), LoftyError> {
//! use lofty::mp3::Mp3File;
//! use lofty::{AudioFile, TagType};
//! use std::fs::File;
//!
//! let mut file_content = File::open("tests/files/assets/minimal/full_test.mp3")?;
//!
//! // We are expecting an MP3 file
//! let mpeg_file = Mp3File::read_from(&mut file_content, true)?;
//!
//! assert_eq!(mpeg_file.properties().channels(), 2);
//!
//! // Here we have a file with multiple tags
//! assert!(mpeg_file.contains_tag_type(TagType::Id3v2));
//! assert!(mpeg_file.contains_tag_type(TagType::Ape));
//! # Ok(())
//! # }
//! ```
//!
//! # Features
//!
//! ## Individual metadata formats
//! These features are available if you have a specific use case, or just don't want certain formats.
//!
//! * `aiff_text_chunks`
//! * `ape`
//! * `id3v1`
//! * `id3v2`
//! * `mp4_ilst`
//! * `riff_info_list`
//! * `vorbis_comments`
//!
//! ## Utilities
//! * `id3v2_restrictions` - Parses ID3v2 extended headers and exposes flags for fine grained control
//!
//! # Important format-specific notes
//!
//! All formats have their own quirks that may produce unexpected results between conversions.
//! Be sure to read the module documentation of each format to see important notes and warnings.
#![forbid(clippy::dbg_macro, clippy::string_to_string)]
#![deny(
	clippy::pedantic,
	clippy::all,
	missing_docs,
	rustdoc::broken_intra_doc_links,
	rust_2018_idioms,
	trivial_casts,
	trivial_numeric_casts,
	unused_import_braces,
	explicit_outlives_requirements
)]
// TODO: This had multiple FPs right now, remove this when it is fixed
#![allow(clippy::needless_borrow)]
#![allow(
	clippy::too_many_lines,
	clippy::cast_precision_loss,
	clippy::cast_sign_loss,
	clippy::cast_possible_wrap,
	clippy::cast_possible_truncation,
	clippy::module_name_repetitions,
	clippy::must_use_candidate,
	clippy::doc_markdown,
	clippy::let_underscore_drop,
	clippy::match_wildcard_for_single_variants,
	clippy::semicolon_if_nothing_returned,
	clippy::new_without_default,
	clippy::from_over_into,
	clippy::upper_case_acronyms,
	clippy::single_match_else,
	clippy::similar_names,
	clippy::tabs_in_doc_comments,
	clippy::len_without_is_empty,
	clippy::needless_late_init
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod ape;
pub mod error;
pub(crate) mod file;
pub mod flac;
pub mod id3;
pub mod iff;
pub(crate) mod macros;
pub mod mp3;
pub mod mp4;
pub mod ogg;
pub(crate) mod picture;
mod probe;
pub(crate) mod properties;
pub(crate) mod tag;
mod traits;

pub use crate::error::{LoftyError, Result};

pub use crate::probe::{read_from, read_from_path, Probe};

pub use crate::file::{AudioFile, FileType, TaggedFile};
pub use crate::picture::{MimeType, Picture, PictureType};
pub use crate::properties::FileProperties;
pub use crate::tag::{Tag, TagType};
pub use tag::item::{ItemKey, ItemValue, TagItem};

pub use crate::traits::{Accessor, TagExt};

#[cfg(feature = "vorbis_comments")]
pub use picture::PictureInformation;
