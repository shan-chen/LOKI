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
)
// send interface adaption
func (n *node) send_packet(destination uint64, msg raftpb.Message){
    n.unreachableLock.RLock()
	defer n.unreachableLock.RUnlock()

	status := raft.SnapshotFinish
	msgBytes := protoutil.MarshalOrPanic(&msg)
	err := n.rpc.SendConsensus(destination, &orderer.ConsensusRequest{Channel: n.chainID, Payload: msgBytes})
	if err != nil {
		n.ReportUnreachable(destination)
		n.logSendFailure(destination, err)

		status = raft.SnapshotFailure
	} else if _, ok := n.unreachable[destination]; ok {
		n.logger.Infof("Successfully sent StepRequest to %d after failed attempt(s)", msg.To)
		delete(n.unreachable, msg.To)
	}

	if msg.Type == raftpb.MsgSnap {
		n.ReportSnapshot(msg.To, status)
	}

}
// go build -buildmode=c-shared -o ./plugins/libfabric.so fabric*.go