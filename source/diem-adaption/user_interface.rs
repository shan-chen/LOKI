//! The interface that user needs to extend and implement for LOKI adaption
use crate::loki_message::LokiMessage;
use serde_json::{Map, Value};

/// the adatption interface of diem
/// encode the LOKI message into a stream
pub fn encode(msg_name: String, content: Map<String, Value>) -> Vec<u8> {
    let msg = crate::loki_message::get_structure_from_msg_type(msg_name).unwrap();
    return crate::encode_map_to_buffer::encode(msg, content);
}

/// decode a stream to a LOKI message
pub fn decode(msg_name: String, stream: &[u8]) -> Map<String, Value> {
    let msg = crate::loki_message::get_structure_from_msg_type(msg_name).unwrap();
    return crate::decode_map_to_buffer::decode(msg, stream);
}

async fn recv_packets(
    &mut self,
    peer_id: AccountAddress,
    msg: ConsensusMsg,
) -> anyhow::Result<Option<UnverifiedEvent>> {
    info!("LOKI fuzzer push the last msg !!");
    match msg {
        ConsensusMsg::ProposalMsg(_) => {
            info!("LOKI received Proposol Msg !!!");
            unsafe{
                if LAST_MSG.len() > 10{
                    LAST_MSG.remove(0);
                    LAST_MSG.push(msg.clone());
                }
                else{
                    LAST_MSG.push(msg.clone());
                }
            }

            let event: UnverifiedEvent = msg.into();
            if event.epoch() == self.epoch() {
                return Ok(Some(event));
            } else {
                monitor!(
                    "process_different_epoch_consensus_msg",
                    self.process_different_epoch(event.epoch(), peer_id).await?
                );
            }
        }
        ConsensusMsg::SyncInfo(_)
        | ConsensusMsg::VoteMsg(_)
        | ConsensusMsg::CommitVoteMsg(_)
        | ConsensusMsg::CommitDecisionMsg(_) => {
            unsafe{
                if LAST_MSG.len() > 10{
                    LAST_MSG.remove(0);
                    LAST_MSG.push(msg.clone());
                }
                else{
                    LAST_MSG.push(msg.clone());
                }
            }

            let event: UnverifiedEvent = msg.into();
            if event.epoch() == self.epoch() {
                return Ok(Some(event));
            } else {
                monitor!(
                    "process_different_epoch_consensus_msg",
                    self.process_different_epoch(event.epoch(), peer_id).await?
                );
            }
        }
        ConsensusMsg::EpochChangeProof(proof) => {
            let msg_epoch = proof.epoch()?;
            debug!(
                LogSchema::new(LogEvent::ReceiveEpochChangeProof)
                    .remote_peer(peer_id)
                    .epoch(self.epoch()),
                "Proof from epoch {}", msg_epoch,
            );
            if msg_epoch == self.epoch() {
                monitor!("process_epoch_proof", self.start_new_epoch(*proof).await?);
            } else {
                bail!(
                    "[EpochManager] Unexpected epoch proof from epoch {}, local epoch {}",
                    msg_epoch,
                    self.epoch()
                );
            }
        }
        ConsensusMsg::EpochRetrievalRequest(request) => {
            ensure!(
                request.end_epoch <= self.epoch(),
                "[EpochManager] Received EpochRetrievalRequest beyond what we have locally"
            );
            monitor!(
                "process_epoch_retrieval",
                self.process_epoch_retrieval(*request, peer_id).await?
            );
        }
        _ => {
            bail!("[EpochManager] Unexpected messages: {:?}", msg);
        }
    }
    Ok(None)
}