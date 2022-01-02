use super::{Exhausted, header::{HeadersBuilder, HeadersParser, InvalidHeader}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
	Ok,
	Forbidden,
	NotFound,
}

pub struct ResponseBuilder<'a>(HeadersBuilder<'a>);

impl<'a> ResponseBuilder<'a> {
	pub fn new(buffer: &'a mut [u8], status: Status) -> Result<Self, Exhausted> {
		let s = match status {
			Status::Ok => "200 OK",
			Status::Forbidden => "403 Forbidden",
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

#[derive(Debug)]
pub struct ResponseParser<'a, 'b> {
	pub status: Status,
	headers: HeadersParser<'a, 'b>,
}

impl<'a, 'b> ResponseParser<'a, 'b> {
	pub fn parse(data: &'a [u8], storage: &'b mut [&'a str]) -> Result<(Self, &'a [u8]), InvalidResponse<'a>> {
		for (i, w) in data.windows(2).enumerate() {
			if w == b"\r\n" {
				let (h, d) = data.split_at(i);
				let mut h = h.split(|&c| c == b' ');
				let version = h.next().ok_or(InvalidResponse::Truncated)?;
				let status = h.next().ok_or(InvalidResponse::Truncated)?;

				match version {
					b"HTTP/1.1" | b"HTTP/1.0" => (),
					v => return Err(InvalidResponse::UnsupportedVersion(v)),
				}
				let status = match status {
					b"200" => Status::Ok,
					b"403" => Status::Forbidden,
					b"404" => Status::NotFound,
					s => return Err(InvalidResponse::InvalidStatus(s)),
				};
				let (headers, d) = HeadersParser::parse(&d[2..], storage)?;

				return Ok((Self {
					status,
					headers,
				}, d));
			}
		}
		Err(InvalidResponse::Truncated)
	}

	#[inline]
	pub fn header(&self, header: &str) -> Option<&'a str> {
		self.headers.get(header)
	}
}

#[derive(Debug)]
pub enum InvalidResponse<'a> {
	InvalidStatus(&'a [u8]),
	UnsupportedVersion(&'a [u8]),
	Truncated,
	Exhausted,
	InvalidUTF8,
	NoValue,
}

impl From<InvalidHeader> for InvalidResponse<'_> {
	fn from(h: InvalidHeader) -> Self {
		match h {
			InvalidHeader::Truncated => Self::Truncated,
			InvalidHeader::Exhausted => Self::Exhausted,
			InvalidHeader::InvalidUTF8 => Self::InvalidUTF8,
			InvalidHeader::NoValue => Self::NoValue,
		}
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn parse() {
		let mut s = [""; 16];
		let (r, d) = ResponseParser::parse(
			b"HTTP/1.1 200 OK\r\n\
			Content-Length: 19\r\n\
			ETag: \"beep\"\r\n\
			\r\n\
			Hello from the Moon!",
			&mut s,
		).unwrap();
		assert_eq!(d, b"Hello from the Moon!");
		assert_eq!(r.status, Status::Ok);
		assert_eq!(r.header("Content-Length"), Some("19"));
		assert_eq!(r.header("content-length"), Some("19"));
		assert_eq!(r.header("CONTENT-LENGTH"), Some("19"));
		assert_eq!(r.header("ConTENt-LengTH"), Some("19"));
		assert_eq!(r.header("ETag"), Some("\"beep\""));
		assert_eq!(r.header("etag"), Some("\"beep\""));
		assert_eq!(r.header("ETAG"), Some("\"beep\""));
		assert_eq!(r.header("EtaG"), Some("\"beep\""));
		assert_eq!(r.header("Non-Existent-Header"), None);
	}
}
