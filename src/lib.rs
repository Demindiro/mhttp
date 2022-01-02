#![no_std]

mod client;
mod server;
mod header;
mod request;
mod response;

use core::fmt;

pub use request::{Method, RequestBuilder, RequestParser, InvalidRequest};
pub use response::{Status, ResponseBuilder, ResponseParser, InvalidResponse};

pub struct Exhausted;

impl fmt::Debug for Exhausted {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		"buffer exhausted".fmt(f)
	}
}
