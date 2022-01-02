#![no_std]

mod header;
mod request;
mod response;

use core::fmt;

pub use request::{Method, RequestBuilder, RequestParser, InvalidRequest};
pub use response::{Status, ResponseBuilder, ResponseParser, InvalidResponse};

/// An error that is returned if the buffer is too small.
pub struct Exhausted;

impl fmt::Debug for Exhausted {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		"buffer exhausted".fmt(f)
	}
}
