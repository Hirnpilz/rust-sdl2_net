extern crate libc;
// export it for now

use libc::{c_int, c_char, c_void};
use std::ffi::CString;

pub mod ffi;
pub use ffi::{IPaddress, _TCPsocket, _SDLNet_SocketSet};

pub struct TCPsocket {
    opaquePtr: *mut _TCPsocket,
}
pub struct SocketSet {
    opaquePtr: *mut _SDLNet_SocketSet,
}

pub fn init() -> () { unsafe { ffi::SDLNet_Init(); } }
pub fn quit() -> () { unsafe { ffi::SDLNet_Quit(); } }
pub fn get_error() -> String {
    unsafe {
        let raw = &ffi::SDLNet_GetError();
        std::str::from_utf8(std::ffi::c_str_to_bytes(raw)).unwrap().to_string()
    }
}

pub fn resolve_host(mut address: IPaddress, host: &str, port: u16) -> i32 {
    unsafe {
        ffi::SDLNet_ResolveHost(&mut address, CString::from_slice(host.as_bytes()).as_ptr(), port)
    }
}

pub fn resolve_ip(mut address: IPaddress) -> String {
    unsafe {
        let raw = &ffi::SDLNet_ResolveIP(&mut address);
        std::str::from_utf8(std::ffi::c_str_to_bytes(raw)).unwrap().to_string()
    }
}

pub fn tcp_open(mut ip: IPaddress) -> TCPsocket {
    unsafe {
        TCPsocket { opaquePtr: ffi::SDLNet_TCP_Open(&mut ip) }
    }
}

pub fn tcp_close(sock: TCPsocket) -> () {
    unsafe {
        ffi::SDLNet_TCP_Close(sock.opaquePtr)
    }
}

pub fn tcp_accept(server: TCPsocket) -> TCPsocket {
    unsafe {
        TCPsocket { opaquePtr: ffi::SDLNet_TCP_Accept(server.opaquePtr) }
    }
}

pub fn tcp_get_peer_address(sock: TCPsocket) -> Box<*mut IPaddress> {
    unsafe {
        Box::new(ffi::SDLNet_TCP_GetPeerAddress(sock.opaquePtr))
    }
}

pub fn tcp_send(sock: TCPsocket, data: &[u8]) -> () {
    unsafe {
        ffi::SDLNet_TCP_Send(sock.opaquePtr, data.as_ptr() as *const c_void, data.len() as i32);
    }
}

pub fn tcp_recv(sock: TCPsocket, maxlen: i32) -> Vec<u8> {
    let mut data: Vec<u8> = Vec::with_capacity(maxlen as usize);
    unsafe {
        let read_ammnt = ffi::SDLNet_TCP_Recv(sock.opaquePtr, data.as_mut_ptr() as *mut c_void, data.len() as i32);
        data.set_len(read_ammnt as usize);
    }
    data
}

pub fn alloc_socket_set(maxsockets: i32) -> SocketSet {
    unsafe {
        SocketSet { opaquePtr: ffi::SDLNet_AllocSocketSet(maxsockets) }
    }
}

pub fn free_socket_set(set: SocketSet) -> () {
    unsafe {
        ffi::SDLNet_FreeSocketSet(set.opaquePtr);
    }
}

pub fn tcp_add_socket(set: SocketSet, sock: TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_TCP_AddSocket(set.opaquePtr, sock.opaquePtr)
    }
}

// Maybe should take in the box here as it may get deleted.... not sure
pub fn tcp_del_socket(set: SocketSet, sock: TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_TCP_DelSocket(set.opaquePtr, sock.opaquePtr)
    }
}

pub fn check_sockets(set: SocketSet, timeout: u32) -> i32 {
    unsafe {
        ffi::SDLNet_CheckSockets(set.opaquePtr, timeout)
    }
}

pub fn socket_ready(sock: TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_SocketReady(sock.opaquePtr)
    }
}
