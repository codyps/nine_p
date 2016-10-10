extern crate futures;
extern crate tokio_core;
extern crate nine_p;

use futures::Future;
use futures::stream::Stream; /* incoming().for_each() */
use tokio_core::io::Io; /* split() */

fn main() {
    let addr = ::std::env::args().nth(1).expect("1 argument required: <bind addr>:<port>");
    let addr = addr.parse::<std::net::SocketAddr>().unwrap();

    let mut l = tokio_core::reactor::Core::new().unwrap();
    let handle = l.handle();

    let socket = tokio_core::net::TcpListener::bind(&addr, &handle).unwrap();
    
    println!("Listening on: {}", addr);

    let done = socket.incoming().for_each(|(socket, addr)| {
        let wrt = tokio_core::io::write_all(socket, b"hello\n");
        handle.spawn(wrt.then(move |res| {
            println!("result {:?}: {:?}", res, addr);
            Ok(())
        }));

        Ok(())
    });

    l.run(done).unwrap();
}
