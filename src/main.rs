use std::io::Write;
use std::net::TcpListener;

fn main() {
	let srv = TcpListener::bind("::1:9999").unwrap();
	loop {
		let (mut cl, _) = srv.accept().unwrap();
		let msg = b"Hello, client!";
		let f = |buf| -> Result<_, mhttp::Exhausted> {
			let r = mhttp::ResponseBuilder::new(buf, mhttp::Status::Ok)?
				.add_header("Content-Length", &msg.len().to_string())?
				.finish();
			Ok(r.0)
		};
		cl.write(f(&mut [0; 1024]).unwrap()).unwrap();
		cl.write(msg).unwrap();
		drop(cl);

		let (mut cl, _) = srv.accept().unwrap();
		let msg = b"Hello, server!";
		let f = |buf| -> Result<_, mhttp::Exhausted> {
			let r = mhttp::RequestBuilder::new(buf, "/how/do/you/do.q?", mhttp::Method::Get)?
				.add_header("Content-Length", &msg.len().to_string())?
				.finish();
			Ok(r.0)
		};
		cl.write(f(&mut [0; 1024]).unwrap()).unwrap();
		cl.write(msg).unwrap();
		drop(cl);
	}
}
