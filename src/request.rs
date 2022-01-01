use super::{Exhausted, header::HeadersBuilder};

pub enum Method {
	Get,
}

pub struct RequestBuilder<'a>(HeadersBuilder<'a>);

impl<'a> RequestBuilder<'a> {
	#[inline]
	pub fn new(buffer: &'a mut [u8], path: &str, method: Method) -> Result<Self, Exhausted> {
		let s = match method {
			Method::Get => "GET",
		};
		let size = s.len() + 1 + path.len() + 1 + "HTTP/1.1\r\n".len();
		if buffer.len() < size + 2 {
			return Err(Exhausted);
		}

		let b = &mut buffer[..];

		b[..s.len()].copy_from_slice(s.as_bytes());
		let b = &mut b[s.len()..];

		b[..1].copy_from_slice(b" ");
		let b = &mut b[1..];

		b[..path.len()].copy_from_slice(path.as_bytes());
		let b = &mut b[path.len()..];

		b[..1].copy_from_slice(b" ");
		let b = &mut b[1..];

		b[.."HTTP/1.1\r\n".len()].copy_from_slice(b"HTTP/1.1\r\n");

		Ok(Self(HeadersBuilder { buffer, index: size }))
	}

	#[inline]
	pub fn add_header(self, header: &str, value: &str) -> Result<Self, Exhausted> {
		self.0.add_header(header, value).map(Self)
	}

	#[inline]
	pub fn finish(self) -> (&'a [u8], &'a mut [u8]) {
		self.0.finish()
	}
}
