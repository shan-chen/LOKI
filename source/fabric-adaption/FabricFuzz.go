package fabric-adaption
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
/** Omit the original fabric code */

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
func (s *Service) handleMessage(stream StepStream, addr string, exp *certificateExpirationCheck) error {
	request, err := stream.Recv()
	if err == io.EOF {
		return err
	}
	if err != nil {
		s.Logger.Warningf("Stream read from %s failed: %v", addr, err)
		return err
	}

	exp.checkExpiration(time.Now(), extractChannel(request))

	if s.StepLogger.IsEnabledFor(zap.DebugLevel) {
		nodeName := commonNameFromContext(stream.Context())
		s.StepLogger.Debugf("Received message from %s(%s): %v", nodeName, addr, requestAsString(request))
	}

	if submitReq := request.GetSubmitRequest(); submitReq != nil {
		nodeName := commonNameFromContext(stream.Context())
		s.Logger.Debugf("Received message from %s(%s): %v", nodeName, addr, requestAsString(request))

		s.Logger.Errorf("submitRequest is: ", base64.StdEncoding.EncodeToString(data))
		return s.handleSubmit(submitReq, stream, addr)
	}

    Len := unsafe.Sizeof(*submitReq)
	bytes := &Slice{
		addr: uintptr(unsafe.Pointer(submitReq)),
		cap:  int(Len),
		len:  int(Len),
	}
	data := *(*[]byte)(unsafe.Pointer(bytes))

    loki_encoded_data := C.recv_packets(msgType(),uintptr(unsafe.Pointer(submitReq)),data, int(Len));
	return s.Dispatcher.DispatchConsensus(loki_encoded_data, request.GetConsensusRequest())
}