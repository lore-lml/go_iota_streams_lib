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

func testSend(filePath string, psw string) []byte {
	channel := go_streams_lib.NewChannelWriter()
	// info := channel.Open() // OPEN THE CHANNEL BY SENDING ONLY THE ANNOUNCE
	info := channel.OpenAndSave(psw) // OPEN THE CHANNEL BY SENDING THE ANNOUNCE AND THEN A FIRST MESSAGE WITH THE STATE
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

	msgid := channel.SendRawData(packet, keyNonce)
	fmt.Println("Msg Sent:", msgid)

	fmt.Println("Saving channel state...")
	if channel.ExportToFile(filePath, psw) {
		fmt.Println("... Channel state Saved")
	} else {
		fmt.Println("... Error during saving state")
	}

	return channel.ExportToBytes(psw)
}

func testRestoreFromFile(filePath string, psw string) {
	fmt.Println("Restoring state ...")
	channel := go_streams_lib.ImportChannelWriterFromFile(filePath, psw)
	if channel == nil {
		fmt.Println("... Failed to restore")
		return
	}
	fmt.Println("... Channel restored")
	defer channel.Close()
	info := channel.ChannelInfo()
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

	msgid := channel.SendRawData(packet, keyNonce)
	fmt.Println("Msg Sent:", msgid)
}

func testRestoreFromBytes(byteState []byte, psw string) {
	fmt.Println("Restoring state ...")
	channel := go_streams_lib.ImportChannelWriterFromBytes(byteState, psw)
	if channel == nil {
		fmt.Println("... Failed to restore")
		return
	}
	fmt.Println("... Channel restored")
	defer channel.Close()
	info := channel.ChannelInfo()
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

	msgid := channel.SendRawData(packet, keyNonce)
	fmt.Println("Msg Sent:", msgid)
}

func main() {
	filePath := "./ch.state"
	psw := "psw"
	byteState := testSend(filePath, psw)
	testRestoreFromFile(filePath, psw)
	testRestoreFromBytes(byteState, psw)
}
