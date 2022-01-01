use super::Exhausted;

pub(crate) struct HeadersBuilder<'a> {
	pub(crate) buffer: &'a mut [u8],
	pub(crate) index: usize,
}

impl<'a> HeadersBuilder<'a> {
	#[inline]
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
