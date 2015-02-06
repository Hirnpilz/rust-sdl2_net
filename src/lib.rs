#![feature(libc)]
#![feature(std_misc)]
#![feature(core)]

extern crate libc;

use libc::c_void;
use std::ffi::CString;
use std::ptr;

pub mod ffi;
pub use ffi::{IPaddress, _TCPsocket, _SDLNet_SocketSet, _SDLNet_GenericSocket};

#[allow(missing_copy_implementations)]
pub struct TCPsocket {
    pub opaque_ptr: *const _TCPsocket,
}
#[allow(missing_copy_implementations)]
pub struct SocketSet {
    pub opaque_ptr: *const _SDLNet_SocketSet,
}
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct CustGeneralSocket {
    ready: i32,
}

//pub static EMPTY_SOCKET_SET: SocketSet = SocketSet {opaque_ptr: 0 as *const _SDLNet_SocketSet};
//pub static EMPTY_TCPSOCKET: TCPsocket = TCPsocket {opaque_ptr: 0 as *const _TCPsocket};

pub fn init() -> bool { unsafe { ffi::SDLNet_Init() == 0 } }
pub fn quit() -> () { unsafe { ffi::SDLNet_Quit(); } }
pub fn get_error() -> String {
    unsafe {
        let raw = &ffi::SDLNet_GetError();
        std::str::from_utf8(std::ffi::c_str_to_bytes(raw)).unwrap().to_string()
    }
}

// Returns an IPaddress struct if successful in becoming a host on the given port
pub fn become_host(port: u16) -> Option<ffi::IPaddress> {
    let mut address = ffi::IPaddress { host: 0, port: 0};
    let result = unsafe { ffi::SDLNet_ResolveHost(&mut address, ptr::null() , port) };
    if result == 0 {
        Some(address)
    } else {
        None
    }
}

// Resolves the host from the given IP and port
pub fn resolve_host(host: &str, port: u16) -> Option<ffi::IPaddress> {
    let mut address = ffi::IPaddress { host: 0, port: 0 };
    let result = unsafe { ffi::SDLNet_ResolveHost(&mut address, CString::from_slice(host.as_bytes()).as_ptr(), port) };
    if result == 0 {
        Some(address)
    } else {
        None
    }
}

// Resolves the given IP from the given IPaddress struct
pub fn resolve_ip(mut address: IPaddress) -> String {
    unsafe {
        let raw = &ffi::SDLNet_ResolveIP(&mut address);
        std::str::from_utf8(std::ffi::c_str_to_bytes(raw)).unwrap().to_string()
    }
}

// Returns a TCPsocket from the data given in the IPaddress object
pub fn tcp_open(address: &mut IPaddress) -> Option<TCPsocket> {
    unsafe {
        let socket = ffi::SDLNet_TCP_Open(address);
        if socket as *const _TCPsocket != ptr::null() {
            Some(TCPsocket { opaque_ptr: socket })
        } else {
            None
        }    
    }
}

// Closes the given TCPsocket
pub fn tcp_close(sock: &TCPsocket) -> () {
    unsafe {
        ffi::SDLNet_TCP_Close(sock.opaque_ptr)
    }
}

// Accepts a TCP connection on the given socket, returning the new socket of the connection
pub fn tcp_accept(server: &TCPsocket) -> Option<TCPsocket> {
    unsafe {
        let socket =  ffi::SDLNet_TCP_Accept(server.opaque_ptr);
        if socket as *const _TCPsocket != ptr::null() {
            Some(TCPsocket { opaque_ptr: socket})
        } else {
            None
        }
    }
}

pub fn tcp_get_peer_address(sock: &TCPsocket) -> Option<IPaddress> {
    unsafe {
        let possible_addr = ffi::SDLNet_TCP_GetPeerAddress(sock.opaque_ptr);
        if possible_addr as *const IPaddress == ptr::null() {
            None
        } else {
            Some(ptr::read(possible_addr))
        }
    }
}

// Writes the data out
pub fn tcp_send(sock: &TCPsocket, data: &mut [u8]) -> i32 {
    unsafe {
        ffi::SDLNet_TCP_Send(sock.opaque_ptr, &data[0] as *const u8 as *const c_void, data.len() as i32)
    }
}

// Receives incoming data
pub fn tcp_recv(sock: &TCPsocket, data: &mut [u8]) -> i32 {
    unsafe {
        ffi::SDLNet_TCP_Recv(sock.opaque_ptr, &mut data[0] as *mut u8 as *mut c_void, data.len() as i32)
    }
}

// Allocates a socket set to hold the given number of sockets
pub fn alloc_socket_set(maxsockets: i32) -> SocketSet {
    unsafe {
        SocketSet { opaque_ptr: ffi::SDLNet_AllocSocketSet(maxsockets) }
    }
}

// Frees the given socket set
pub fn free_socket_set(set: &SocketSet) -> () {
    unsafe {
        ffi::SDLNet_FreeSocketSet(set.opaque_ptr);
    }
}

// Adds a given socket to the given socket set
pub fn add_socket(set: &SocketSet, sock: &TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_AddSocket(set.opaque_ptr, sock.opaque_ptr)
    }
}

// Maybe should take in the box here as it may get deleted.... not sure
pub fn del_socket(set: &SocketSet, sock: &TCPsocket) -> i32 {
    unsafe {
        ffi::SDLNet_DelSocket(set.opaque_ptr, sock.opaque_ptr)
    }
}

// Returns the incoming data amount  on the given socketset
pub fn check_sockets(set: &SocketSet, timeout: u32) -> i32 {
    unsafe {
        ffi::SDLNet_CheckSockets(set.opaque_ptr, timeout)
    }
}

pub fn socket_ready(sock: &TCPsocket) -> bool {
    unsafe {
        (std::mem::transmute::<*const ffi::_TCPsocket,&CustGeneralSocket>(sock.opaque_ptr)).ready != 0
    }
}
