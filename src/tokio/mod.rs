#[macro_use]
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

#[macro_use]
extern crate futures;

pub struct Parser;

impl tokio_proto::Parse for Parser {
    type Out = tokio_proto::pipeline::Frame

}

/*
// based on tokio-socks5
use tokio_core::read_exact;

struct Client {
    /// maximum message length to accept
    max_len: usize,
    buffer: Rc<RefCell<Vec<u8>>>,
}

impl Client {
    fn serve(self, conn: tokio_core::net::TcpStream)
        -> Box<futures::Future<Item=(u64,u64), Error=std::io::Error>>
    {
        Box::new(tokio_core::read_exact(conn, [0u8; 2]).and_then(|(conn, buf)| {
            /* the first 4 bytes are the message length, in little endian */
            /* the length is of the entire message, including the length itself. do a small amount
             * of validation here */
            let msg_len = buf[0] | buf[1] << 8 |
                          buf[2] << 16 | buf[3] << 24;
            if (msg_len < 5 && msg_len > self.max_len) {
                /* error */
                Err(std::io::Error::other("bad message length"))
            }

            /* the first message is required to be a Tversion message */ 

            tokio_core::read_exact(conn, [0u8; msg_len - 4]).and_then(|(conn, buf)| {
                /* now we've got the full message to work with */
                /* TODO: consider avoiding reading the whole message into memory, can we do better?
                 * */
                 
            }
        }))
    }
}
*/
