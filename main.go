package main

import "C"
import (
	"encoding/json"
	"fmt"
	"github.com/lore-lml/go_streams_lib"
)

type Message struct {
	DeviceId    string  `json:"device_id"`
	OperatorId  string  `json:"operator_id"`
	Temperature float32 `json:"temperature"`
}

func main() {
	channel := go_streams_lib.NewChannelWriter()
	var info = channel.Open()
	defer channel.Close()
	fmt.Printf("%s:%s\n", info.ChannelId, info.AnnounceId)

	keyNonce := go_streams_lib.CreateEncryptionKeyNonce("This is a secret key", "This is a secret nonce")
	defer keyNonce.Drop()
	m1 := &Message{
		DeviceId:    "Device1",
		OperatorId:  "Operator1",
		Temperature: 12.3,
	}
	m2 := &Message{
		DeviceId:    "Device1",
		OperatorId:  "Operator1",
		Temperature: 12.3,
	}

	pub, _ := json.Marshal(m1)
	mask, _ := json.Marshal(m2)
	packet := go_streams_lib.NewRawPacket(pub, mask)
	defer packet.Drop()

	msgid := channel.SendRawData(packet, nil)
	fmt.Println("Msg Sent:", msgid)
}
