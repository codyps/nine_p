extern crate rotor;

enum NineServConnState {
    
}

struct NineServConn {
    stream: rotor::mio::tcp::TcpStream,
    state: NineServConnState,
}

enum NineServ {
    Server(rotor::mio::tcp::TcpListener),
    Connection(rotor::mio::tcp::TcpStream),
}

impl NineServ {
    pub fn new(sock: rotor::mio::tcp::TcpListener,
               scope: &mut rotor::EarlyScope) 
        -> rotor::Response<NineServ, rotor::void::Void>
    {
        scope.register(&sock, rotor::EventSet::readable(),
            rotor::PollOpt::edge()).unwrap();
        rotor::Response::ok(NineServ::Server(sock)
    }

    fn accept(self) -> Response<NineServ, TcpStream> {
        match self {
            NineServ::Server(sock) => {
                match sock.accept() {
                    Ok(some((conn, _))) => {
                        rotor::Response::spawn(NineServ::Server(sock), conn)
                    },
                    Ok(None) => {
                        rotor::Response::ok(NineServ::Server(sock))
                    }
                    Err(e) => {
                        writeln!(&mut strerr(), "Error: {}", e).ok();
                        rotor::Response::ok(NineServ::Server(sock))
                    }
                }
            },

            _ => unreachable!();
        }
    }
}

