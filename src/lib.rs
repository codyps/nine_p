#[repr(C)]
pub enum MsgId {
    Tversion = 100,
    Rversion = 101,
    Tauth = 102,
    Rauth = 103,
    Tattach = 104,
    Rattach = 105,
    Terror = 106,
    Rerror = 107,
    Tflush = 108,
    Rflush = 109,
    Twalk = 110,
    Rwalk = 111,
    Topen = 112,
    Ropen = 113,
    Tcreate = 114,
    Rcreate = 115,
    Tread = 116,
    Rread = 117,
    Twrite = 118,
    Rwrite = 119,
    Tclunk = 120,
    Rclunk = 121,
    Tremove = 122,
    Rremove = 123,
    Tstat = 124,
    Rstat = 125,
    Twstat = 126,
    Rwstat = 127
}

pub const NO_TAG : u16 = !0u16;
pub const NO_FID : u32 = !0u32;

// size[4] {T,R}version tag[2] msize[4] version[s]
//  client must emit Tversion as the first message of a connection
//  client must not emit other messages prior to recieving Rversion
//  tag must be NO_TAG
//  
//  msize = maximum message size, establish a lower bound
//  version = "unknown" | "9P2000" | "9P2000.u"
//
//  version string from client must begin with "9P"
//
//  server is permitted reply with:
//   - "unknown", if version sent by client is unrecognized
//   - exact version requested by client
//   - version requested by client with items after the first "." removed
//   - "9Pnnnn", where nnnn are digits, the server may use a lower number than the client.

pub enum NineT {
    Version{msize: u32, version: String}
}

pub enum NineReply {
    Version{msize: u32, version: String}
}


//#[cfg(feature = "rotor")]
//mod rotor;
// rotor-stream?

//#[cfg(feature = "futures")]
//mod futures;

//#[cfg(feature = "tokio")]
//mod tokio;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
