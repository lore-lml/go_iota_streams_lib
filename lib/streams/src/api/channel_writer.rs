use iota_streams_lib::channel::tangle_channel_writer::ChannelWriter;
use iota_streams_lib::user_builders::author_builder::AuthorBuilder;
use std::ptr::null;
use tokio::runtime::Runtime;
use std::ffi::CString;
use crate::api::utils::{ChannelInfo, RawPacket, KeyNonce};
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn new_channel_writer() -> *mut ChannelWriter{
    let author = AuthorBuilder::new().build();
    let channel = ChannelWriter::new(author);
    println!("Channel created");
    Box::into_raw(Box::new(channel))
}

#[no_mangle]
pub extern "C" fn drop_channel_writer(channel: *mut ChannelWriter){
    unsafe {
        channel.drop_in_place()
    }
}

#[no_mangle]
pub extern "C" fn open_channel_writer(channel: *mut ChannelWriter) -> *const ChannelInfo{
    unsafe {
        let ch = match channel.as_mut(){
            None => return null(),
            Some(ch) => ch
        };

        let res = Runtime::new().unwrap().block_on(async {
            match ch.open().await{
                Ok((channel_id, announce_id)) => {
                    let channel_id = CString::new(channel_id).unwrap().into_raw();
                    let announce_id = CString::new(announce_id).unwrap().into_raw();
                    let res = ChannelInfo{channel_id, announce_id};
                    Box::into_raw(Box::new(res))
                },
                Err(_) => null()
            }
        });
        res
    }
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

