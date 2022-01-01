use super::{Exhausted, header::HeadersBuilder};

pub enum Status {
	Ok,
	Forbidden,
	NotFound,
}

pub struct ResponseBuilder<'a>(HeadersBuilder<'a>);

impl<'a> ResponseBuilder<'a> {
	#[inline]
	pub fn new(buffer: &'a mut [u8], status: Status) -> Result<Self, Exhausted> {
		let s = match status {
			Status::Ok => "200 OK",
			Status::Forbidden => "200 Forbidden",
			Status::NotFound => "404 Not Found",
		}.as_bytes();
		let size = "HTTP/1.1 ".len() + s.len() + "\r\n".len();
		if buffer.len() < size + 2 {
			return Err(Exhausted);
		}

		let b = &mut buffer[..];

		b[.."HTTP/1.1 ".len()].copy_from_slice(b"HTTP/1.1 ");
		let b = &mut b["HTTP/1.1 ".len()..];

		b[..s.len()].copy_from_slice(s);
		let b = &mut b[s.len()..];
		
		b[.."\r\n".len()].copy_from_slice(b"\r\n");

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
