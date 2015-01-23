// sdl stuff here?
use libc::{c_int, c_char};

#[repr(C)]
struct IPaddress { host: u32, port: u16 }
struct TCPsocket;
struct SDLNet_SocketSet;

// Linking setup (using https://github.com/xsleonard/rust-sdl2_image/ as an example)
#[cfg(target_os="macos")]
mod mac {
    #[cfg(mac_framework)]
    #[link(kind="framework", name="SDL2_net")]
    extern {}

    #[cfg(not(mac_framework))]
    #[link(name="SDL2_net")]
    extern {}
}
#[cfg(any(target_os="linux", target_os="freebsd", target_os="windows"))]
mod others {
    #[link(name="SDL2_net")]
    extern {}
}

extern "C" {
    //General
    pub fn SDLNet_Init() -> ();
    pub fn SDLNet_Quit() -> ();
    pub fn SDLNet_GetError() -> *mut c_char;

    // Name Resolution
    pub fn SDLNet_ResolveHost(address: *mut IPaddress, host: *const c_char, port: u16) -> c_int;
    pub fn SDLNet_ResolveIP(address: *mut IPaddress) -> *const c_char;

    // TCP Sockets
    pub fn SDLNet_TCP_Open(ip: *mut IPaddress) -> *mut TCPsocket;
    pub fn SDLNet_TCP_Close(sock: *mut TCPsocket) -> ();
    pub fn SDLNet_TCP_Accept(server: *mut TCPsocket) -> *mut TCPsocket;
    pub fn SDLNet_TCP_GetPeerAddress(sock: *mut TCPsocket) -> *mut IPaddress;
    pub fn SDLNet_TCP_Send(sock: *mut TCPsocket, data : *const void, len: c_int) -> ();
    pub fn SDLNet_TCP_Recv(sock: *mut TCPsocket, data: *mut void, maxlen: c_int) -> c_int;

    // UDP Sockets

    // UDP Packets

    // Socket Sets
    pub fn SDLNet_AllocSocketSet(maxsockets: c_int) -> *mut SDLNet_SocketSet;
    pub fn SDLNet_FreeSocketSet(set: *mut SDLNet_SocketSet) -> ();
    pub fn SDLNet_TCP_AddSocket(set: *mut SDLNet_SocketSet, sock: *mut TCPsocket) -> c_int;
    pub fn SDLNet_TCP_DelSocket(set: *mut SDLNet_SocketSet, sock: *mut TCPsocket) -> c_int;
    pub fn SDLNet_CheckSockets(set: *mut SDLNet_SocketSet, timeout: u32) -> c_int;
    pub fn SDLNet_SocketReady(sock: *mut TCPsocket) -> c_int; // documentation unclear as to what it takes in - assmuing tcp for now
}

