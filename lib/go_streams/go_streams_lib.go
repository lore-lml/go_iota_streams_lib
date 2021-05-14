package go_streams_lib

/*
#cgo LDFLAGS: -L./.. -lc_streams_lib
#include "../c_streams.h"
*/
import "C"
import (
	"unsafe"
)

type ChannelWriter struct {
	channel *C.channel_writer_t
}

type ChannelInfo struct {
	ChannelId  string
	AnnounceId string
}

type KeyNonce struct {
	keyNonce *C.key_nonce_t
}

type RawPacket struct {
	packet *C.raw_packet_t
}

func NewChannelWriter() *ChannelWriter {
	var channel = C.new_channel_writer()
	return &ChannelWriter{channel: channel}
}

func ImportChannelWriterFromFileWithNode(filePath string, psw string, nodeUrl string) *ChannelWriter {
	cPath := C.CString(filePath)
	cPsw := C.CString(psw)
	cNode := C.CString(nodeUrl)

	channel := C.import_channel_from_file(cPath, cPsw, cNode)
	if channel == nil {
		return nil
	}
	return &ChannelWriter{channel: channel}
}

func ImportChannelWriterFromFile(filePath string, psw string) *ChannelWriter {
	cPath := C.CString(filePath)
	cPsw := C.CString(psw)

	channel := C.import_channel_from_file(cPath, cPsw, nil)
	if channel == nil {
		return nil
	}
	return &ChannelWriter{channel: channel}
}

func ImportChannelWriterFromBytes(byteState []byte, psw string) *ChannelWriter {
	cPsw := C.CString(psw)
	cByteState := (*C.uchar)(unsafe.Pointer(&byteState[0]))
	length := C.int(len(byteState))
	channel := C.import_channel_from_bytes(cByteState, length, cPsw, nil)
	if channel == nil {
		return nil
	}
	return &ChannelWriter{channel: channel}
}

func (ch *ChannelWriter) Open() ChannelInfo {
	var info = C.open_channel_writer(ch.channel)
	defer C.drop_channel_info(info)
	return ChannelInfo{ChannelId: C.GoString(info.channel_id), AnnounceId: C.GoString(info.announce_id)}
}

func (ch *ChannelWriter) SendRawData(packet *RawPacket, keyNonce *KeyNonce) string {
	var kn *C.key_nonce_t = nil
	if keyNonce != nil {
		kn = keyNonce.keyNonce
	}

	var msgId = C.send_raw_data(ch.channel, packet.packet, kn)
	defer C.drop_str(msgId)
	return C.GoString(msgId)
}

func (ch *ChannelWriter) Close() {
	C.drop_channel_writer(ch.channel)
}

func (ch *ChannelWriter) ExportToFile(filePath string, psw string) bool {
	cPath := C.CString(filePath)
	cPsw := C.CString(psw)

	return C.export_channel_to_file(ch.channel, cPath, cPsw) != -1
}

func (ch *ChannelWriter) ExportToBytes(psw string) []byte {
	cPsw := C.CString(psw)
	channelState := C.export_channel_to_bytes(ch.channel, cPsw)
	defer C.drop_channel_state(channelState)
	length := channelState.len
	return C.GoBytes(unsafe.Pointer(channelState.byte_state), length)
}

func (ch *ChannelWriter) ChannelInfo() ChannelInfo {
	var info = C.channel_info(ch.channel)
	defer C.drop_channel_info(info)
	return ChannelInfo{ChannelId: C.GoString(info.channel_id), AnnounceId: C.GoString(info.announce_id)}
}

func NewRawPacket(pubData, maskData []byte) *RawPacket {
	p_len := C.ulong(len(pubData))
	m_len := C.ulong(len(maskData))
	c_pub := (*C.uchar)(unsafe.Pointer(&pubData[0]))
	c_mask := (*C.uchar)(unsafe.Pointer(&maskData[0]))

	var packet = C.new_raw_packet(c_pub, p_len, c_mask, m_len)
	return &RawPacket{packet: packet}
}

func (packet *RawPacket) Drop() {
	C.drop_raw_packet(packet.packet)
}

func CreateEncryptionKeyNonce(key, nonce string) *KeyNonce {
	return &KeyNonce{
		keyNonce: C.create_encryption_key_nonce(C.CString(key), C.CString(nonce)),
	}
}

func (keyNonce *KeyNonce) Drop() {
	C.drop_key_nonce(keyNonce.keyNonce)
}

func HashString(str string) string {
	c_str := C.CString(str)
	hash := C.hash_string(c_str)
	defer C.drop_str(hash)
	return C.GoString(hash)
}
