use libc::{c_int, c_char, c_void};

#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct IPaddress { pub host: u32, pub port: u16 }
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct _TCPsocket;
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct _UDPsocket;
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct _UDPpacket;
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct _SDLNet_SocketSet;
#[allow(missing_copy_implementations)]
#[repr(C)]
pub struct _SDLNet_GenericSocket;
//unsafe impl Sync for *const _TCPsocket {}
//unsafe impl Sync for *const _UDPsocket {}
//unsafe impl Sync for *const _SDLNet_SocketSet {}

// Linking setup (using https://github.com/xsleonard/rust-sdl2_image/ as an example)
#[cfg(target_os="macos")]
mod mac {
    //#[cfg(mac_framework)]
    //#[link(kind="framework", name="SDL2_net")]
    //extern {}

    //#[cfg(not(mac_framework))]
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
    pub fn SDLNet_Init() -> c_int;
    pub fn SDLNet_Quit() -> ();
    pub fn SDLNet_GetError() -> *const c_char;

    // Name Resolution
    pub fn SDLNet_ResolveHost(address: *mut IPaddress, host: *const c_char, port: u16) -> c_int;
    pub fn SDLNet_ResolveIP(address: *mut IPaddress) -> *const c_char;

    // TCP Sockets
    pub fn SDLNet_TCP_Open(ip: *mut IPaddress) -> *mut _TCPsocket;
    pub fn SDLNet_TCP_Close(sock: *const _TCPsocket) -> ();
    pub fn SDLNet_TCP_Accept(server: *const _TCPsocket) -> *mut _TCPsocket;
    pub fn SDLNet_TCP_GetPeerAddress(sock: *const _TCPsocket) -> *mut IPaddress;
    pub fn SDLNet_TCP_Send(sock: *const _TCPsocket, data : *const c_void, len: c_int) -> c_int;
    pub fn SDLNet_TCP_Recv(sock: *const _TCPsocket, data: *mut c_void, maxlen: c_int) -> c_int;

    // UDP Sockets
    pub fn SDLNet_UDP_Open(port: u16) -> *mut _UDPsocket;
    pub fn SDLNet_UDP_Close(sock: *const _UDPsocket) -> ();
    pub fn SDLNet_UDP_Bind(sock: *const _UDPsocket, channel: i32, ip: *mut IPaddress) -> c_int;
    pub fn SDLNet_UDP_Unbind(sock: *const _UDPsocket, channel: i32) -> ();
    pub fn SDLNet_UDP_GetPeerAddress(sock: *const _UDPsocket, channel: i32) -> *mut IPaddress;
    pub fn SDLNet_UDP_Send(sock: *const _UDPsocket, channel: i32, packet: *mut _UDPpacket) ->
        c_int;
    pub fn SDLNet_UDP_Recv(sock: *const _UDPsocket, packet: *mut _UDPpacket) -> c_int;
//  pub fn SDLNet_UDP_SendV ...
//  pub fn SDLNet_UDP_RecvV ...

    // UDP Packets
    pub fn SDLNet_AllocPacket(size: i32) -> *mut _UDPpacket;
    pub fn SDLNet_ResizePacket(packet: *mut _UDPpacket, size: i32) -> c_int;
    pub fn SDLNet_FreePacket(packet: *mut _UDPpacket) -> ();
//  pub fn SDLNet_AllocPacketV ...
//  pub fn SDLNet_FreePacketV ...


    // Socket Sets
    pub fn SDLNet_AllocSocketSet(maxsockets: c_int) -> *const _SDLNet_SocketSet;
    pub fn SDLNet_FreeSocketSet(set: *const _SDLNet_SocketSet) -> ();
    pub fn SDLNet_AddSocket(set: *const _SDLNet_SocketSet, sock: *const _TCPsocket) -> c_int;
    pub fn SDLNet_DelSocket(set: *const _SDLNet_SocketSet, sock: *const _TCPsocket) -> c_int;
    pub fn SDLNet_CheckSockets(set: *const _SDLNet_SocketSet, timeout: u32) -> c_int;
    pub fn SDLNet_SocketReady(sock: *mut _SDLNet_GenericSocket) -> c_int; // documentation unclear as to what it takes in - assmuing tcp for now
}

