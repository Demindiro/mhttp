use super::{Exhausted, header::{HeadersBuilder, HeadersParser, InvalidHeader}};

/// All methods that may be used in requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Method {
	Get,
	Head,
	Post,
	Put,
	Delete,
	Connect,
	Options,
	Trace,
	Patch,
}

/// Utility for creating HTTP requests.
pub struct RequestBuilder<'a>(HeadersBuilder<'a>);

impl<'a> RequestBuilder<'a> {
	/// Begin creating a new HTTP request.
	pub fn new(buffer: &'a mut [u8], path: &str, method: Method) -> Result<Self, Exhausted> {
		let s = match method {
			Method::Get => "GET",
			Method::Head => "HEAD",
			Method::Post => "POST",
			Method::Put => "PUT",
			Method::Delete => "DELETE",
			Method::Connect => "CONNECT",
			Method::Options => "OPTIONS",
			Method::Trace => "TRACE",
			Method::Patch => "PATCH",
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

	/// Append a single header.
	#[inline]
	pub fn add_header(self, header: &str, value: &str) -> Result<Self, Exhausted> {
		self.0.add_header(header, value).map(Self)
	}

	/// Construct the final request, returning the slice with the headers and the remainder
	/// of the buffer.
	#[inline]
	pub fn finish(self) -> (&'a [u8], &'a mut [u8]) {
		self.0.finish()
	}
}

/// Utility for parsing HTTP requests.
#[derive(Debug)]
pub struct RequestParser<'a, 'b> {
	pub method: Method,
	pub path: &'a str,
	headers: HeadersParser<'a, 'b>,
}

impl<'a, 'b> RequestParser<'a, 'b> {
	/// Parse a HTTP request, returning the method, path, headers and any additional data if successful.
	pub fn parse(data: &'a [u8], storage: &'b mut [&'a str]) -> Result<(Self, &'a [u8]), InvalidRequest<'a>> {
		for (i, w) in data.windows(2).enumerate() {
			if w == b"\r\n" {
				let (h, d) = data.split_at(i);
				let mut h = h.split(|&c| c == b' ');
				let method = h.next().ok_or(InvalidRequest::Truncated)?;
				let path = h.next().ok_or(InvalidRequest::Truncated)?;
				let version = h.next().ok_or(InvalidRequest::Truncated)?;
				if let Some(g) = h.next() {
					return Err(InvalidRequest::TrailingGarbage(g));
				}

				match version {
					b"HTTP/1.1" | b"HTTP/1.0" => (),
					v => return Err(InvalidRequest::UnsupportedVersion(v)),
				}
				let method = match method {
					b"GET" => Method::Get,
					b"HEAD" => Method::Head,
					b"POST" => Method::Post,
					b"PUT" => Method::Put,
					b"DELETE" => Method::Delete,
					b"CONNECT" => Method::Connect,
					b"OPTIONS" => Method::Options,
					b"TRACE" => Method::Trace,
					b"PATCH" => Method::Patch,
					m => return Err(InvalidRequest::InvalidMethod(m)),
				};
				let path = core::str::from_utf8(path)
					.map_err(|_| InvalidRequest::InvalidPath(path))?;
				let (headers, d) = HeadersParser::parse(&d[2..], storage)?;

				return Ok((Self {
					method,
					path,
					headers,
				}, d));
			}
		}
		Err(InvalidRequest::Truncated)
	}

	/// Get the value of the header with the given name.
	#[inline]
	pub fn header(&self, header: &str) -> Option<&'a str> {
		self.headers.get(header)
	}
}

/// Errors that may occur while parsing a request.
#[derive(Debug)]
pub enum InvalidRequest<'a> {
	InvalidMethod(&'a [u8]),
	InvalidPath(&'a [u8]),
	UnsupportedVersion(&'a [u8]),
	TrailingGarbage(&'a [u8]),
	Truncated,
	InvalidUTF8,
	NoValue,
}

impl From<InvalidHeader> for InvalidRequest<'_> {
	fn from(h: InvalidHeader) -> Self {
		match h {
			InvalidHeader::Truncated => Self::Truncated,
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
		let (r, d) = RequestParser::parse(
			b"GET /a/b/cde HTTP/1.1\r\n\
			Host: the.moon\r\n\
			ETag: \"beep\"\r\n\
			\r\n\
			Hello from Earth!",
			&mut s,
		).unwrap();
		assert_eq!(d, b"Hello from Earth!");
		assert_eq!(r.method, Method::Get);
		assert_eq!(r.path, "/a/b/cde");
		assert_eq!(r.header("Host"), Some("the.moon"));
		assert_eq!(r.header("host"), Some("the.moon"));
		assert_eq!(r.header("HOST"), Some("the.moon"));
		assert_eq!(r.header("hOsT"), Some("the.moon"));
		assert_eq!(r.header("ETag"), Some("\"beep\""));
		assert_eq!(r.header("etag"), Some("\"beep\""));
		assert_eq!(r.header("ETAG"), Some("\"beep\""));
		assert_eq!(r.header("EtaG"), Some("\"beep\""));
		assert_eq!(r.header("Non-Existent-Header"), None);
	}
}
