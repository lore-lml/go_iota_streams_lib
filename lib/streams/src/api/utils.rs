use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use iota_streams_lib::utility::iota_utility;
use std::ptr::null;

#[repr(C)]
pub struct ChannelInfo{
    pub channel_id: *const c_char,
    pub announce_id: *const c_char
}



#[repr(C)]
pub struct KeyNonce{
    pub key: [u8; 32],
    pub nonce: [u8; 24],
}

#[repr(C)]
pub struct RawPacket{
    pub public: *const u8,
    pub p_len: usize,
    pub masked: *const u8,
    pub m_len: usize
}

#[repr(C)]
pub struct ChannelState{
    pub byte_state: *const u8,
    pub len: usize,
}

impl RawPacket{
    pub fn public(&self) -> Vec<u8>{
        unsafe{
            let p = std::slice::from_raw_parts(self.public, self.p_len);
            p.to_vec()
        }
    }

    pub fn masked(&self) -> Vec<u8>{
        unsafe{
            let m = std::slice::from_raw_parts(self.masked, self.m_len);
            m.to_vec()
        }
    }
}

impl ChannelState{
    pub fn new(byte_state: Vec<u8>) -> Self {
        let mut buf = byte_state.into_boxed_slice();
        let data = buf.as_mut_ptr();
        let len = buf.len();
        std::mem::forget(buf);
        ChannelState { byte_state: data, len }
    }
}

#[no_mangle]
pub extern "C" fn new_raw_packet(public: *mut u8, p_len: u64,
                                 masked: *mut u8, m_len: u64) -> *const RawPacket{
    let p_len = p_len as usize;
    let m_len = m_len as usize;
    let packet = RawPacket{public, p_len, masked, m_len};
    Box::into_raw(Box::new(packet))
}

#[no_mangle]
pub extern "C" fn hash_string(cs: *const c_char) -> *const c_char{
    unsafe {
        let s = CStr::from_ptr(cs).to_str().unwrap();
        let hash = iota_utility::hash_string(s);
        CString::new(hash).map_or(null(), |h| h.into_raw())
    }
}

#[no_mangle]
pub extern "C" fn create_encryption_key_nonce(key: *const c_char, nonce: *const c_char) -> *const KeyNonce{
    unsafe {
        let k = CStr::from_ptr(key).to_str().unwrap();
        let n = CStr::from_ptr(nonce).to_str().unwrap();
        let k = iota_utility::create_encryption_key(k);
        let n = iota_utility::create_encryption_nonce(n);

        Box::into_raw(Box::new(KeyNonce{key: k, nonce: n}))
    }
}

#[no_mangle]
pub extern "C" fn drop_channel_info(info: *mut ChannelInfo){
    unsafe {
        Box::from_raw(info);
    }
}

#[no_mangle]
pub extern "C" fn drop_key_nonce(kn: *const KeyNonce) {
    unsafe {
        Box::from_raw(kn as *mut KeyNonce);
    }
}

#[no_mangle]
pub extern "C" fn drop_raw_packet(packet: *mut RawPacket){
    unsafe {
        Box::from_raw(packet);
    }
}

#[no_mangle]
pub extern "C" fn drop_channel_state(state: *mut ChannelState){
    unsafe {
        Box::from_raw(state);
    }
}

#[no_mangle]
pub extern "C" fn drop_str(s: *const c_char) {
    unsafe {
        CString::from_raw(s as *mut c_char);
    }
}
