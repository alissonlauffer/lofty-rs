pub(super) mod properties;

#[cfg(feature = "vorbis_comments")]
use super::tag::VorbisComments;
use crate::error::Result;
use crate::file::{AudioFile, FileType, TaggedFile};
use crate::ogg::constants::SPEEXHEADER;
use crate::properties::FileProperties;
use crate::tag::TagType;
use properties::SpeexProperties;

use std::io::{Read, Seek};

/// An OGG Speex file
pub struct SpeexFile {
	#[cfg(feature = "vorbis_comments")]
	/// The vorbis comments contained in the file
	///
	/// NOTE: While a metadata packet is required, it isn't required to actually have any data.
	pub(crate) vorbis_comments: VorbisComments,
	/// The file's audio properties
	pub(crate) properties: SpeexProperties,
}

impl From<SpeexFile> for TaggedFile {
	fn from(input: SpeexFile) -> Self {
		Self {
			ty: FileType::Speex,
			properties: FileProperties::from(input.properties),
			#[cfg(feature = "vorbis_comments")]
			tags: vec![input.vorbis_comments.into()],
			#[cfg(not(feature = "vorbis_comments"))]
			tags: Vec::new(),
		}
	}
}

impl AudioFile for SpeexFile {
	type Properties = SpeexProperties;

	fn read_from<R>(reader: &mut R, read_properties: bool) -> Result<Self>
	where
		R: Read + Seek,
	{
		let file_information = super::read::read_from(reader, SPEEXHEADER, &[])?;

		Ok(Self {
            properties: if read_properties { properties::read_properties(reader, &file_information.1)? } else { SpeexProperties::default() },
            #[cfg(feature = "vorbis_comments")]
            // Safe to unwrap, a metadata packet is mandatory in Speex
            vorbis_comments: file_information.0.unwrap(),
        })
	}

	fn properties(&self) -> &Self::Properties {
		&self.properties
	}

	fn contains_tag(&self) -> bool {
		true
	}

	fn contains_tag_type(&self, tag_type: TagType) -> bool {
		tag_type == TagType::VorbisComments
	}
}

impl SpeexFile {
	#[cfg(feature = "vorbis_comments")]
	/// Returns a reference to the Vorbis comments tag
	pub fn vorbis_comments(&self) -> &VorbisComments {
		&self.vorbis_comments
	}

	#[cfg(feature = "vorbis_comments")]
	/// Returns a mutable reference to the Vorbis comments tag
	pub fn vorbis_comments_mut(&mut self) -> &mut VorbisComments {
		&mut self.vorbis_comments
	}
}
