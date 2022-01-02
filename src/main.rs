use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
	let mut buf = [0; 0x1_0000];

	// Client test
	{
		let mut cl = TcpStream::connect("93.184.216.34:80").unwrap();
		let pkt = mhttp::RequestBuilder::new(&mut buf, "/", mhttp::Method::Get).unwrap()
			.add_header("Host", "www.example.com").unwrap()
			.finish().0;
		cl.write(pkt).unwrap();
		let l = cl.read(&mut buf).unwrap();
		println!("{}", core::str::from_utf8(&buf[..l]).unwrap());
	}

	// Server test
	let srv = TcpListener::bind("::1:9999").unwrap();
	loop {
		let (mut cl, _) = srv.accept().unwrap();
		let l = cl.read(&mut buf).unwrap();
		let msg = &buf[..l];
		dbg!(mhttp::RequestParser::parse(msg, &mut [""; 16]).unwrap());
		let f = |buf| -> Result<_, mhttp::Exhausted> {
			let r = mhttp::ResponseBuilder::new(buf, mhttp::Status::Ok)?
				.add_header("Content-Length", &msg.len().to_string())?
				.add_header("Content-Type", "text/plain")?
				.finish();
			Ok(r.0)
		};
		cl.write(f(&mut [0; 1024]).unwrap()).unwrap();
		cl.write(msg).unwrap();
		drop(cl);
	}
}
