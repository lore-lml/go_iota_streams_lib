use iota_streams_lib::channel::tangle_channel_writer::ChannelWriter;
use iota_streams_lib::user_builders::author_builder::AuthorBuilder;
use std::ptr::{null, null_mut};
use tokio::runtime::Runtime;
use std::ffi::{CString, CStr};
use crate::api::utils::{ChannelInfo, RawPacket, KeyNonce, ChannelState};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn new_channel_writer() -> *mut ChannelWriter{
    let author = AuthorBuilder::new().build();
    let channel = ChannelWriter::new(author);
    Box::into_raw(Box::new(channel))
}

#[no_mangle]
pub extern "C" fn drop_channel_writer(channel: *mut ChannelWriter){
    unsafe {
        channel.drop_in_place()
    }
}

#[no_mangle]
pub unsafe extern "C" fn open_channel_writer(channel: *mut ChannelWriter) -> *const ChannelInfo{
    let ch = match channel.as_mut(){
        None => return null(),
        Some(ch) => ch
    };

    Runtime::new().unwrap().block_on(async {
        match ch.open().await{
            Ok((channel_id, announce_id)) => {
                let channel_id = CString::new(channel_id).unwrap().into_raw();
                let announce_id = CString::new(announce_id).unwrap().into_raw();
                let res = ChannelInfo{channel_id, announce_id};
                Box::into_raw(Box::new(res))
            },
            Err(_) => null()
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn open_channel_writer_and_save(channel: *mut ChannelWriter, state_psw: *const c_char) -> *const ChannelInfo{
    let ch = match channel.as_mut(){
        None => return null(),
        Some(ch) => ch
    };

    let state_psw = match CStr::from_ptr(state_psw).to_str(){
        Ok(state_psw) => state_psw,
        Err(_) => return null()
    };

    Runtime::new().unwrap().block_on(async {
        match ch.open_and_save(state_psw).await{
            Ok((channel_id, announce_id, _)) => {
                let channel_id = CString::new(channel_id).unwrap().into_raw();
                let announce_id = CString::new(announce_id).unwrap().into_raw();
                let res = ChannelInfo{channel_id, announce_id};
                Box::into_raw(Box::new(res))
            },
            Err(_) => null()
        }
    })
}

#[no_mangle]
pub extern "C" fn send_raw_data(channel: *mut ChannelWriter, packet: *const RawPacket, key_nonce: *const KeyNonce) -> *const c_char{
    unsafe {
        let ch = channel.as_mut();
        let p = packet.as_ref();
        let kn = key_nonce.as_ref();

        match (&ch, &p){
            (None, _) => return null(),
            (_, None) => return null(),
            _ => {}
        }

        let ch = ch.unwrap();
        let p = p.unwrap();
        let public = p.public();
        let masked = p.masked();
        let opt_kn = match kn{
            None => None,
            Some(kn) => Some((kn.key.clone(), kn.nonce.clone()))
        };

        let res = Runtime::new().unwrap().block_on(async {
            ch.send_signed_raw_data(public, masked, opt_kn).await
        });

        match res{
            Ok(res) => CString::new(res).map_or(null(), |h| h.into_raw()),
            Err(_) => null()
        }
    }
}

#[no_mangle]
pub extern "C" fn export_channel_to_file(channel: *mut ChannelWriter, file_path: *const c_char, psw: *const c_char) -> i32{
    unsafe {
        let ch = match channel.as_mut(){
            None => return -1,
            Some(ch) => ch,
        };

        let path = CStr::from_ptr(file_path).to_str();
        let psw = CStr::from_ptr(psw).to_str();

        let (path, psw) = match (path, psw){
            (Ok(path), Ok(psw)) => (path, psw),
            _ => return -1
        };

        match ch.export_to_file(psw, path){
            Ok(_) => 1,
            Err(_) => -1
        }
    }
}

#[no_mangle]
pub extern "C" fn export_channel_to_bytes(channel: *mut ChannelWriter, psw: *const c_char) -> *const ChannelState{
    unsafe {
        let ch = match channel.as_mut(){
            None => return null(),
            Some(ch) => ch,
        };

        let psw = CStr::from_ptr(psw).to_str();

        let psw = match psw{
            Ok(psw) => psw,
            _ => return null()
        };

        match ch.export_to_bytes(psw){
            Ok(state) => {
                let res = ChannelState::new(state);
                Box::into_raw(Box::new(res))
            }
            Err(_) => null()
        }
    }
}

#[no_mangle]
pub extern "C" fn import_channel_from_file(file_path: *const c_char, psw: *const c_char, node_url: *const c_char) -> *mut ChannelWriter{
    unsafe {
        if file_path == null() || psw == null(){
            return null_mut();
        }
        let path = CStr::from_ptr(file_path).to_str();
        let psw = CStr::from_ptr(psw).to_str();
        let node = if node_url == null() {None} else {
            match CStr::from_ptr(node_url).to_str(){
                Ok(node) => Some(node),
                Err(_) => None,
            }
        };

        let (path, psw) = match (path, psw){
            (Ok(path), Ok(psw)) => (path, psw),
            _ => return null_mut()
        };

        Runtime::new().unwrap().block_on(async {
            match ChannelWriter::import_from_file(path, psw, node, None).await{
                Ok(ch) => Box::into_raw(Box::new(ch)),
                Err(_) => null_mut()
            }
        })
    }
}

#[no_mangle]
pub unsafe extern "C" fn import_channel_from_bytes(byte_state: *const u8, len: usize, psw: *const c_char, node_url: *const c_char) -> *mut ChannelWriter{
    if byte_state == null() || psw == null(){
        return null_mut();
    }

    let state = std::slice::from_raw_parts(byte_state, len).to_vec();
    let psw = CStr::from_ptr(psw).to_str();
    let node = if node_url == null() {None} else {
        match CStr::from_ptr(node_url).to_str(){
            Ok(node) => Some(node),
            Err(_) => None,
        }
    };

    let psw = match psw{
        Ok(psw) => psw,
        _ => return null_mut()
    };

    Runtime::new().unwrap().block_on(async {
        match ChannelWriter::import_from_bytes(&state, psw, node, None).await{
            Ok(ch) => Box::into_raw(Box::new(ch)),
            Err(_) => null_mut()
        }
    })
}

#[no_mangle]
pub unsafe extern "C" fn import_channel_from_tangle(channel_id: *const c_char, announce_id: *const c_char, psw: *const c_char, node_url: *const c_char) -> *mut ChannelWriter{
    if channel_id == null() || announce_id == null() || psw == null(){
        return null_mut();
    }

    let channel_id = CStr::from_ptr(channel_id).to_str();
    let announce_id = CStr::from_ptr(announce_id).to_str();
    let psw = CStr::from_ptr(psw).to_str();
    let node = if node_url == null() {None} else {
        match CStr::from_ptr(node_url).to_str(){
            Ok(node) => Some(node),
            Err(_) => None,
        }
    };

    let (channel_id, announce_id, psw) = match (channel_id, announce_id, psw){
        (Ok(channel_id), Ok(announce_id), Ok(psw)) => (channel_id, announce_id, psw),
        _ => return null_mut()
    };

    Runtime::new().unwrap().block_on(async {
        match ChannelWriter::import_from_tangle(channel_id, announce_id, psw, node, None).await{
            Ok(ch) => Box::into_raw(Box::new(ch)),
            Err(_) => null_mut()
        }
    })
}

#[no_mangle]
pub extern "C" fn channel_info(channel: *mut ChannelWriter) -> *const ChannelInfo{
    if channel == null_mut(){
        return null();
    }

    unsafe {
        let ch = match channel.as_mut() {
            None => return null(),
            Some(ch) => ch,
        };

        let (channel_id, announce_id) = ch.channel_address();
        let channel_id = CString::new(channel_id).unwrap().into_raw();
        let announce_id = CString::new(announce_id).unwrap().into_raw();
        let res = ChannelInfo{channel_id, announce_id};
        Box::into_raw(Box::new(res))
    }
}
