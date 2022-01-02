use super::Exhausted;

pub(crate) struct HeadersBuilder<'a> {
	pub(crate) buffer: &'a mut [u8],
	pub(crate) index: usize,
}

impl<'a> HeadersBuilder<'a> {
	pub fn add_header(mut self, header: &str, value: &str) -> Result<Self, Exhausted> {
		let size = header.len() + 2 + value.len() + 2;
		if self.buffer.len() - self.index < size + 2 {
			return Err(Exhausted);
		}

		let b = &mut self.buffer[self.index..];

		b[..header.len()].copy_from_slice(header.as_bytes());
		let b = &mut b[header.len()..];

		b[..2].copy_from_slice(b": ");
		let b = &mut b[2..];

		b[..value.len()].copy_from_slice(value.as_bytes());
		let b = &mut b[value.len()..];

		b[..2].copy_from_slice(b"\r\n");

		self.index += size;
		Ok(self)
	}

	#[inline]
	pub fn finish(self) -> (&'a [u8], &'a mut [u8]) {
		self.buffer[self.index..][..2].copy_from_slice(b"\r\n");
		let (l, r) = self.buffer.split_at_mut(self.index + 2);
		(l, r)
	}
}

#[derive(Debug)]
pub(crate) struct HeadersParser<'a, 'b> {
	headers: &'b [&'a str],
}

impl<'a, 'b> HeadersParser<'a, 'b> {
	pub fn parse(mut data: &'a [u8], storage: &'b mut [&'a str]) -> Result<(Self, &'a [u8]), InvalidHeader> {
		'l: for index in 0..storage.len() {
			if data.len() < 2 {
				return Err(InvalidHeader::Truncated);
			}
			if &data[..2] == b"\r\n" {
				return Ok((Self { headers: &storage[..index] }, &data[2..]));
			}
			for (i, w) in data.windows(2).enumerate() {
				if w == b"\r\n" {
					let (h, d) = data.split_at(i);
					if !h.contains(&b':') {
						return Err(InvalidHeader::NoValue);
					}
					storage[index] = core::str::from_utf8(h).map_err(|_| InvalidHeader::InvalidUTF8)?;
					data = &d[2..];
					continue 'l;
				}
			}
			break;
		}
		return Err(InvalidHeader::Truncated);
	}

	pub fn get(&self, name: &str) -> Option<&'a str> {
		'l: for &h in self.headers {
			// Iterating over bytes is significantly simpler than over chars
			for (h, n) in h.bytes().zip(name.bytes().map(|c| c.to_ascii_lowercase())) {
				debug_assert!(b'a' <= n && n <= b'z' || b'0' <= n && n <= b'9' || n == b'-');
				if h.to_ascii_lowercase() != n {
					continue 'l;
				}
			}
			if h.as_bytes()[name.len()] != b':' {
				continue;
			}
			return Some(h[name.len() + 1..].trim_start())
		}
		None
	}
}

pub enum InvalidHeader {
	Truncated,
	Exhausted,
	InvalidUTF8,
	NoValue,
}
