use crate::loki_fuzzer;

static INIT: Once = Once::new();
static mut LAST_MSG: Vec<ConsensusMsg> = Vec::new();
static mut AUTHOR: Author = Author::ZERO;

pub async fn start(
    mut self,
    mut round_timeout_sender_rx: channel::Receiver<Round>,
    mut network_receivers: NetworkReceivers,
) {
    // initial start of the processor
    self.expect_new_epoch().await;
    loop {
        let result = monitor!(
            "main_loop",
            select! {
                msg = network_receivers.consensus_messages.select_next_some() => {
                    let (peer, msg) = (msg.0, msg.1);
                    info!("loki select message from the network");
                    monitor!("process_message", self.process_message(peer, msg).await.with_context(|| format!("from peer: {}", peer)))
                }
                block_retrieval = network_receivers.block_retrieval.select_next_some() => {
                    info!("loki select block_retrieval from the network");
                    monitor!("process_block_retrieval", self.process_block_retrieval(block_retrieval).await)
                }
                round = round_timeout_sender_rx.select_next_some() => {
                    info!("loki select round from the network");
                    monitor!("process_local_timeout", self.process_local_timeout(round).await)
                }
            }
        );
        let round_state = if let RoundProcessor::Normal(p) = self.processor_mut() {
            Some(p.round_state())
        } else {
            None
        };
        match result {
            Ok(_) => trace!(RoundStateLogSchema::new(round_state)),
            Err(e) => {
                counters::ERROR_COUNT.inc();
                error!(error = ?e, kind = error_kind(&e), RoundStateLogSchema::new(round_state));
            }
        }

        // Continually capture the time of consensus process to ensure that clock skew between
        // validators is reasonable and to find any unusual (possibly byzantine) clock behavior.
        counters::OP_COUNTERS
            .gauge("time_since_epoch_ms")
            .set(duration_since_epoch().as_millis() as i64);
    }
}