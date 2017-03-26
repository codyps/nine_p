extern crate futures;
extern crate tokio_core;
extern crate tokio_proto;
extern crate tokio_service;

extern crate nine_p;

use std::io;
use std::str;
use tokio_core::io::{Codec,EasyBuf};

#[derive(Debug)]
struct Message {
    mtype: u8,
    tag: u16,
    data: Vec<u8>
}

/*
 *
 */
#[derive(Debug)]
struct MessageCodec;

impl Codec for MessageCodec {
    type In = Message;
    type Out = Message;

    fn decode(&mut self, buf: &mut EasyBuf) -> io::Result<Option<Self::In>> {
        if buf.len() < 4 {
            return Ok(None)
        }

        let size: u32 = buf[0] | buf[1] << 8 | buf[2] << 16 | buf[3] << 24;

        if buf.len() < size {
            return Ok(None)
        }

        let mtype = buf[4];
        let tag:u16 = buf[5] | buf[6] << 8;

        buf.drain_to(7)
        let data = buf.drain_to(size - 7);

        Ok(Some(Message {
            mtype: mtype,
            tag: tag,
            data: data,
        })
    }

    fn encode(&mut self, msg: Self::Out, buf: &mut Vec<u8>)
        -> io::Result<()>
    {
        let l = msg.data.len() + 7;
        let size = [l as u8, l >> 8 as u8, l >> 16 as u8, l >> 24 as u8];
        buf.append(size);
        buf.append(msg.mtype);
        buf.append([msg.tag as u8, msg.tag >> 8 as u8]);
        buf.append(msg.data);

        Ok(())
    }
}

use tokio_proto::pipeline::ServerProto;

pub struct MessageProto;
use tokio_core::io::{Io, Framed};

impl <T: Io + 'static> ServerProto<T> for MessageProto {
    type Request = Message;
    type Response = Message;

    type Transport = Framed<T, LineCodec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(MessageCodec))
    }
}

use tokio_service::Service;

pub struct Rpc;

use futures::{future, Future, BoxFuture};

impl Service for Rpc {
    type Request = Message;
    type Response = Message;

    type Error = io::Error;

    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        future::ok(req).boxed()
    }
}

pub struct EchoRev;

impl Service for EchoRev {
    type Request = String;
    type Response = String;
    type Error = io::Error;
    type Future = BoxFuture<Self::Response, Self::Error>;

    fn call(&self, req: Self::Request) -> Self::Future {
        let rev: String = req.chars().rev().collect();
        future::ok(rev).boxed()
    }
}

use tokio_proto::TcpServer;

fn main() {
    let addr = ::std::env::args().nth(1).expect("1 argument required: <bind addr>:<port>");
    let addr = addr.parse::<std::net::SocketAddr>().unwrap();

    let server = TcpServer::new(LineProto, addr);

    server.serve(|| Ok(EchoRev));
}
