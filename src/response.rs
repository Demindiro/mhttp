use super::{Exhausted, header::{HeadersBuilder, HeadersParser, InvalidHeader}};

/// All status codes that responses can return.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Status {
	// 1xx
	Continue,
	SwitchingProtocols,
	Processing,
	EarlyHints,
	// 2xx
	Ok,
	Created,
	Accepted,
	NonAuthoritativeInformation,
	NoContent,
	ResetContent,
	PartialContent,
	MultiStatus,
	AlreadyReported,
	ImUsed,
	// 3xx
	MultipleChoice,
	MovedPermanently,
	Found,
	SeeOther,
	NotModified,
	UseProxy,
	Unused,
	TemporaryRedirect,
	PermanentRedirect,
	// 4xx
	BadRequest,
	Unauthorized,
	PaymentRequired,
	Forbidden,
	NotFound,
	MethodNotAllowed,
	NotAcceptable,
	ProxyAuthenticationRequired,
	RequestTimeout,
	Conflict,
	Gone,
	LengthRequired,
	PreconditionFailed,
	PayloadTooLarge,
	UriTooLong,
	UnsupportedMediaType,
	RangeNotSatisfiable,
	ExpectationFailed,
	ImATeapot,
	MisdirectedRequest,
	UnprocessableEntity,
	Locked,
	FailedDependency,
	TooEarly,
	UpgradeRequired,
	PreconditionRequired,
	TooManyRequests,
	RequestHeaderFieldsTooLarge,
	UnavailableForLegalReasons,
	// 5xx
	InternalServerError,
	NotImplemented,
	BadGateway,
	ServiceUnavailable,
	GatewayTimeout,
	HttpVersionNotSupported,
	VariantAlsoNegotiates,
	InsufficientStorage,
	LoopDetected,
	NotExtended,
	NetworkAuthenticationRequired,
}

/// Utility for creating HTTP responses.
pub struct ResponseBuilder<'a>(HeadersBuilder<'a>);

impl<'a> ResponseBuilder<'a> {
	/// Create a new HTTP response by writing into the given buffer.
	pub fn new(buffer: &'a mut [u8], status: Status) -> Result<Self, Exhausted> {
		let s = match status {
			// 1xx
			Status::Continue => "100 Continue",
			Status::SwitchingProtocols => "101 Switching Protocols",
			Status::Processing => "102 Processing",
			Status::EarlyHints => "103 EarlyHints",
			// 2xx
			Status::Ok => "200 OK",
			Status::Created => "201 Created",
			Status::Accepted => "202 Accepted",
			Status::NonAuthoritativeInformation => "203 Non-Authoritative Information",
			Status::NoContent => "204 No Content",
			Status::ResetContent => "205 Reset Content",
			Status::PartialContent => "206 Partial Content",
			Status::MultiStatus => "207 Multi-Status",
			Status::AlreadyReported => "208 Already Reported",
			Status::ImUsed => "226 IM Used",
			// 3xx
			Status::MultipleChoice => "300 Multiple Choice",
			Status::MovedPermanently => "301 Moved Permanently",
			Status::Found => "302 Found",
			Status::SeeOther => "303 See Other",
			Status::NotModified => "304 Not Modified",
			Status::UseProxy => "305 Use Proxy",
			Status::Unused => "306 unused",
			Status::TemporaryRedirect => "307 Temporary Redirect",
			Status::PermanentRedirect => "308 Permanent Redirect",
			// 4xx
			Status::BadRequest => "400 Bad Request",
			Status::Unauthorized => "401 Unauthorized",
			Status::PaymentRequired => "402 Payment Required",
			Status::Forbidden => "403 Forbidden",
			Status::NotFound => "404 Not Found",
			Status::MethodNotAllowed => "405 Method Not Allowed",
			Status::NotAcceptable => "406 Not Acceptable",
			Status::ProxyAuthenticationRequired => "407 Proxy Authentication Required",
			Status::RequestTimeout => "408 Request Timeout",
			Status::Conflict => "409 Conflict",
			Status::Gone => "410 Gone",
			Status::LengthRequired => "411 Length Required",
			Status::PreconditionFailed => "412 Precondition Failed",
			Status::PayloadTooLarge => "413 Payload Too Large",
			Status::UriTooLong => "414 URI Too Long",
			Status::UnsupportedMediaType => "415 Unsupported Media Type",
			Status::RangeNotSatisfiable => "416 Range Not Satisfiable",
			Status::ExpectationFailed => "417 Expectation Failed",
			Status::ImATeapot => "418 I'm a teapot",
			Status::MisdirectedRequest => "421 Misdirected Request",
			Status::UnprocessableEntity => "422 Unprocessable Entity",
			Status::Locked => "423 Locked",
			Status::FailedDependency => "424 Failed Dependency",
			Status::TooEarly => "425 Too Early",
			Status::UpgradeRequired => "426 Upgrade Required",
			Status::PreconditionRequired => "428 Precondition Required",
			Status::TooManyRequests => "429 Too Many Requests",
			Status::RequestHeaderFieldsTooLarge => "431 Request Header Fields Too Large",
			Status::UnavailableForLegalReasons => "451 Unavailable For Legal Reasons",
			// 5xx
			Status::InternalServerError => "500 Internal Server Error",
			Status::NotImplemented => "501 Not Implemented",
			Status::BadGateway => "502 Bad Gateway",
			Status::ServiceUnavailable => "503 Service Unavailable",
			Status::GatewayTimeout => "504 Gateway Timeout",
			Status::HttpVersionNotSupported => "505 HTTP Version Not Supported",
			Status::VariantAlsoNegotiates => "506 Variant Also Negotiates",
			Status::InsufficientStorage => "507 Insufficient Storage",
			Status::LoopDetected => "508 Loop Detected",
			Status::NotExtended => "510 Not Extended",
			Status::NetworkAuthenticationRequired => "511 Network Authentication Required",
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

	/// Append a single header.
	#[inline]
	pub fn add_header(self, header: &str, value: &str) -> Result<Self, Exhausted> {
		self.0.add_header(header, value).map(Self)
	}

	/// Finish constructing the HTTP response, returning the slice with the headers and
	/// the remaining buffer slice.
	#[inline]
	pub fn finish(self) -> (&'a [u8], &'a mut [u8]) {
		self.0.finish()
	}
}

/// Utility for parsing HTTP responses.
#[derive(Debug)]
pub struct ResponseParser<'a, 'b> {
	pub status: Status,
	headers: HeadersParser<'a, 'b>,
}

impl<'a, 'b> ResponseParser<'a, 'b> {
	/// Parse a HTTP response, returning the status and any additional data if successful.
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
					// 1xx
					b"100" => Status::Continue,
					b"101" => Status::SwitchingProtocols,
					b"102" => Status::Processing,
					b"103" => Status::EarlyHints,
					// 2xx
					b"200" => Status::Ok,
					b"201" => Status::Created,
					b"202" => Status::Accepted,
					b"203" => Status::NonAuthoritativeInformation,
					b"204" => Status::NoContent,
					b"205" => Status::ResetContent,
					b"206" => Status::PartialContent,
					b"207" => Status::MultiStatus,
					b"208" => Status::AlreadyReported,
					b"226" => Status::ImUsed,
					// 3xx
					b"300" => Status::MultipleChoice,
					b"301" => Status::MovedPermanently,
					b"302" => Status::Found,
					b"303" => Status::SeeOther,
					b"304" => Status::NotModified,
					b"305" => Status::UseProxy,
					b"306" => Status::Unused,
					b"307" => Status::TemporaryRedirect,
					b"308" => Status::PermanentRedirect,
					// 4xx
					b"400" => Status::BadRequest,
					b"401" => Status::Unauthorized,
					b"402" => Status::PaymentRequired,
					b"403" => Status::Forbidden,
					b"404" => Status::NotFound,
					b"405" => Status::MethodNotAllowed,
					b"406" => Status::NotAcceptable,
					b"407" => Status::ProxyAuthenticationRequired,
					b"408" => Status::RequestTimeout,
					b"409" => Status::Conflict,
					b"410" => Status::Gone,
					b"411" => Status::LengthRequired,
					b"412" => Status::PreconditionFailed,
					b"413" => Status::PayloadTooLarge,
					b"414" => Status::UriTooLong,
					b"415" => Status::UnsupportedMediaType,
					b"416" => Status::RangeNotSatisfiable,
					b"417" => Status::ExpectationFailed,
					b"418" => Status::ImATeapot,
					b"421" => Status::MisdirectedRequest,
					b"422" => Status::UnprocessableEntity,
					b"423" => Status::Locked,
					b"424" => Status::FailedDependency,
					b"425" => Status::TooEarly,
					b"426" => Status::UpgradeRequired,
					b"428" => Status::PreconditionRequired,
					b"429" => Status::TooManyRequests,
					b"431" => Status::RequestHeaderFieldsTooLarge,
					b"451" => Status::UnavailableForLegalReasons,
					// 5xx
					b"500" => Status::InternalServerError,
					b"501" => Status::NotImplemented,
					b"502" => Status::BadGateway,
					b"503" => Status::ServiceUnavailable,
					b"504" => Status::GatewayTimeout,
					b"505" => Status::HttpVersionNotSupported,
					b"506" => Status::VariantAlsoNegotiates,
					b"507" => Status::InsufficientStorage,
					b"508" => Status::LoopDetected,
					b"510" => Status::NotExtended,
					b"511" => Status::NetworkAuthenticationRequired,
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

	/// Get the value of the header with the given name.
	#[inline]
	pub fn header(&self, header: &str) -> Option<&'a str> {
		self.headers.get(header)
	}
}

/// Errors that may occur while parsing a response.
#[derive(Debug)]
pub enum InvalidResponse<'a> {
	InvalidStatus(&'a [u8]),
	UnsupportedVersion(&'a [u8]),
	Truncated,
	InvalidUTF8,
	NoValue,
}

impl From<InvalidHeader> for InvalidResponse<'_> {
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
