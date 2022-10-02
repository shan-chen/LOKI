package geth-adaption
/*
#cgo LDFLAGS: -L./lib -lloki
#include <stdlib.h>
#include "./lib/loki.h"
*/
import "C"
import (
	"fmt"
	"unsafe"
    "time"
)
/** Omit the original geth code */

func start_active_fuzzing() {
    while (true){
        msg_bytes := c.get_active_sending_packets()
        target := get_active_sending_targets()

        var destination uint64  = convertTargetAddress(target)
        var msg raftpb.Message  = messageFactory.createRaftFromBytes(msg_bytes)

        send_packet(destination, msg)
        time.Sleep(time.Second)
    }
}

// receive interface adaption
func handleMessage(backend Backend, peer *Peer) error {
	// Read the next message from the remote peer, and ensure it's fully consumed
	msg, err := peer.rw.ReadMsg()

	if err != nil {
		return err
	}
	if msg.Size > maxMessageSize {
		return fmt.Errorf("%w: %v > %v", errMsgTooLarge, msg.Size, maxMessageSize)
	}
	defer msg.Discard()

	var handlers = eth66
	//if peer.Version() >= ETH67 { // Left in as a sample when new protocol is added
	//	handlers = eth67
	//}

	// Track the amount of time it takes to serve the request and run the handler
	if metrics.Enabled {
		h := fmt.Sprintf("%s/%s/%d/%#02x", p2p.HandleHistName, ProtocolName, peer.Version(), msg.Code)
		defer func(start time.Time) {
			sampler := func() metrics.Sample {
				return metrics.ResettingSample(
					metrics.NewExpDecaySample(1028, 0.015),
				)
			}
			metrics.GetOrRegisterHistogramLazy(h, nil, sampler).Update(time.Since(start).Microseconds())
		}(time.Now())
	}
	
	loki_encoded_data := C.recv_packets(msg.Code,msg.Receiver, msg.Payload, msg.Size);

	if handler := handlers[loki_encoded_data]; handler != nil {
		return handler(backend, loki_encoded_data, peer)
	}
	return fmt.Errorf("%w: %v", errInvalidMsgCode, loki_encoded_data)
}
