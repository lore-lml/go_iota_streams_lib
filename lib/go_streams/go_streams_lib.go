package go_streams_lib

/*
#cgo LDFLAGS: -L./.. -lc_streams_lib
#include "../c_streams.h"
*/
import "C"
import "unsafe"

type ChannelWriter struct {
	channel *C.channel_writer_t
}

type ChannelInfo struct{
	ChannelId string
	AnnounceId string
}

type KeyNonce struct{
	keyNonce *C.key_nonce_t
}

type RawPacket struct{
	packet *C.raw_packet_t
}

func NewChannelWriter() *ChannelWriter{
	var channel = C.new_channel_writer()
	return &ChannelWriter{channel: channel}
}

func (ch *ChannelWriter) Open() ChannelInfo{
	var info = C.open_channel_writer(ch.channel)
	defer C.drop_channel_info(info)
	return ChannelInfo{ChannelId: C.GoString(info.channel_id), AnnounceId: C.GoString(info.announce_id)}
}

func (ch *ChannelWriter) SendRawData(packet *RawPacket, keyNonce *KeyNonce) string{
	var msgid = C.send_raw_data(ch.channel, packet.packet, keyNonce.keyNonce)
	defer C.drop_str(msgid)
	return C.GoString(msgid)
}

func (ch *ChannelWriter) Close(){
	C.drop_channel_writer(ch.channel)
}

func NewRawPacket(pubData, maskData []byte) *RawPacket{
	p_len := C.ulong(len(pubData))
	m_len := C.ulong(len(maskData))
	c_pub := (*C.uchar)(unsafe.Pointer(&pubData[0]))
	c_mask := (*C.uchar)(unsafe.Pointer(&maskData[0]))

	var packet = C.new_raw_packet(c_pub, p_len, c_mask, m_len)
	return &RawPacket{packet: packet}
}

func (packet *RawPacket) Drop(){
	C.drop_raw_packet(packet.packet)
}

func CreateEncryptionKeyNonce(key, nonce string) *KeyNonce{
	return &KeyNonce{
		keyNonce: C.create_encryption_key_nonce(C.CString(key), C.CString(nonce)),
	}
}

func (keyNonce *KeyNonce) Drop(){
	C.drop_key_nonce(keyNonce.keyNonce)
}

func HashString(str string) string{
	c_str := C.CString(str)
	hash := C.hash_string(c_str)
	defer C.drop_str(hash)
	return C.GoString(hash)
}
