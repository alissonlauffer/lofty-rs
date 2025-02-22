use crate::error::{ErrorKind, LoftyError, Result};
use crate::macros::try_vec;

use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

#[derive(Eq, PartialEq, Debug, Clone)]
/// Represents an `MP4` atom identifier
pub enum AtomIdent {
	/// A four byte identifier
	///
	/// Many FOURCCs start with `0xA9` (©), and should be human-readable.
	Fourcc([u8; 4]),
	/// A freeform identifier
	///
	/// # Example
	///
	/// ```text
	/// ----:com.apple.iTunes:SUBTITLE
	/// ─┬── ────────┬─────── ───┬────
	///  ╰freeform identifier    ╰name
	///              |
	///              ╰mean
	/// ```
	Freeform {
		/// A string using a reverse DNS naming convention
		mean: String,
		/// A string identifying the atom
		name: String,
	},
}

pub(crate) struct AtomInfo {
	pub(crate) start: u64,
	pub(crate) len: u64,
	pub(crate) extended: bool,
	pub(crate) ident: AtomIdent,
}

impl AtomInfo {
	pub(crate) fn read<R>(data: &mut R) -> Result<Self>
	where
		R: Read + Seek,
	{
		let start = data.stream_position()?;

		let len = data.read_u32::<BigEndian>()?;

		let mut ident = [0; 4];
		data.read_exact(&mut ident)?;

		let mut atom_ident = AtomIdent::Fourcc(ident);

		// Encountered a freeform identifier
		if &ident == b"----" {
			atom_ident = parse_freeform(data)?;
		}

		let (len, extended) = match len {
			// The atom extends to the end of the file
			0 => {
				let pos = data.stream_position()?;
				let end = data.seek(SeekFrom::End(0))?;

				data.seek(SeekFrom::Start(pos))?;

				(end - pos, false)
			},
			// There's an extended length
			1 => (data.read_u64::<BigEndian>()?, true),
			_ if len < 8 => {
				return Err(LoftyError::new(ErrorKind::BadAtom(
					"Found an invalid length (< 8)",
				)))
			},
			_ => (u64::from(len), false),
		};

		Ok(Self {
			start,
			len,
			extended,
			ident: atom_ident,
		})
	}
}

fn parse_freeform<R>(data: &mut R) -> Result<AtomIdent>
where
	R: Read + Seek,
{
	let mean = freeform_chunk(data, b"mean")?;
	let name = freeform_chunk(data, b"name")?;

	Ok(AtomIdent::Freeform { mean, name })
}

fn freeform_chunk<R>(data: &mut R, name: &[u8]) -> Result<String>
where
	R: Read + Seek,
{
	let atom = AtomInfo::read(data)?;

	match atom.ident {
		AtomIdent::Fourcc(ref fourcc) if fourcc == name => {
			// Version (1)
			// Flags (3)
			data.seek(SeekFrom::Current(4))?;

			// Already read the size, identifier, and version/flags (12 bytes)
			let mut content = try_vec![0; (atom.len - 12) as usize];
			data.read_exact(&mut content)?;

			String::from_utf8(content).map_err(|_| {
				LoftyError::new(ErrorKind::BadAtom(
					"Found a non UTF-8 string while reading freeform identifier",
				))
			})
		},
		_ => Err(LoftyError::new(ErrorKind::BadAtom(
			"Found freeform identifier \"----\" with no trailing \"mean\" or \"name\" atoms",
		))),
	}
}
