package main

import "C"
import (
	"fmt"
	"github.com/lore-lml/go_streams_lib"
)

func main() {
	channel := go_streams_lib.NewChannelWriter()
	var info = channel.Open()
	defer channel.Close()
	fmt.Printf("%s:%s\n", info.ChannelId, info.AnnounceId)

	keyNonce := go_streams_lib.CreateEncryptionKeyNonce("This is a secret key", "This is a secret nonce")
	defer keyNonce.Drop()

	pub := []byte("Public message")
	mask := []byte("Private message")
	packet := go_streams_lib.NewRawPacket(pub, mask)
	defer packet.Drop()

	msgid := channel.SendRawData(packet, keyNonce)
	fmt.Println("Msg Sent:", msgid)
}
