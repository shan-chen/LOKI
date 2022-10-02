use crate::{
    counters,
    logging::LogEvent,
    network_interface::{ConsensusMsg, ConsensusNetworkEvents, ConsensusNetworkSender},
};
use rand::seq::IteratorRandom;

/// Implements the actual networking support for all consensus messaging.
#[derive(Clone)]
pub struct NetworkSender {
    author: Author,
    network_sender: ConsensusNetworkSender,
    // Self sender and self receivers provide a shortcut for sending the messages to itself.
    // (self sending is not supported by the networking API).
    // Note that we do not support self rpc requests as it might cause infinite recursive calls.
    self_sender: channel::Sender<Event<ConsensusMsg>>,
    validators: ValidatorVerifier,
}

impl NetworkSender {
    pub fn new(
        author: Author,
        network_sender: ConsensusNetworkSender,
        self_sender: channel::Sender<Event<ConsensusMsg>>,
        validators: ValidatorVerifier,
    ) -> Self {
        NetworkSender {
            author,
            network_sender,
            self_sender,
            validators,
        }
    }

    /// Tries to retrieve num of blocks backwards starting from id from the given peer: the function
    /// returns a future that is fulfilled with BlockRetrievalResponse.
    pub async fn request_block(
        &mut self,
        retrieval_request: BlockRetrievalRequest,
        from: Author,
        timeout: Duration,
    ) -> anyhow::Result<BlockRetrievalResponse> {
        ensure!(from != self.author, "Retrieve block from self");
        let msg = ConsensusMsg::BlockRetrievalRequest(Box::new(retrieval_request.clone()));
        let response_msg = monitor!(
            "block_retrieval",
            self.network_sender.send_rpc(from, msg, timeout).await?
        );
        let response = match response_msg {
            ConsensusMsg::BlockRetrievalResponse(resp) => *resp,
            _ => return Err(anyhow!("Invalid response to request")),
        };
        response
            .verify(
                retrieval_request.block_id(),
                retrieval_request.num_blocks(),
                &self.validators,
            )
            .map_err(|e| {
                error!(
                    SecurityEvent::InvalidRetrievedBlock,
                    request_block_response = response,
                    error = ?e,
                );
                e
            })?;

        Ok(response)
    }

    /// Tries to send the given msg to all the participants.
    ///
    /// The future is fulfilled as soon as the message put into the mpsc channel to network
    /// internal(to provide back pressure), it does not indicate the message is delivered or sent
    /// out. It does not give indication about when the message is delivered to the recipients,
    /// as well as there is no indication about the network failures.
    pub async fn broadcast(&mut self, msg: ConsensusMsg) {
        // Directly send the message to ourself without going through network.
        let self_msg = Event::Message(self.author, msg.clone());
        if let Err(err) = self.self_sender.send(self_msg).await {
            error!("Error broadcasting to self: {:?}", err);
        }

        // Get the list of validators excluding our own account address. Note the
        // ordering is not important in this case.
        let self_author = self.author;
        let other_validators = self
            .validators
            .get_ordered_account_addresses_iter()
            .filter(|author| author != &self_author);

        // Broadcast message over direct-send to all other validators.
        if let Err(err) = self.network_sender.send_to_many(other_validators, msg) {
            error!(error = ?err, "Error broadcasting message");
        }
    }

    /// added by fcorleone, this is used to send packages to some random nodes for crusader
    pub fn send_to_others(&mut self, msg: ConsensusMsg) {
        info!("This is the crusader fuzzer functions!!");
        // Directly send the message to ourself without going through network.

        // Get the list of validators excluding our own account address. Note the
        // ordering is not important in this case.
        let mut rng = rand::thread_rng();
        // let temp : u8 = rng.gen();
        let self_author = self.author;
        let other_validators = self
            .validators
            .get_ordered_account_addresses_iter().choose_multiple(&mut rng,2).into_iter()
            .filter(|author| author != &self_author);
        info!(
            "Crusader fuzzer randomly send to 2 other validator nodes!!!"
        );
        // Broadcast message over direct-send to all other validators.
        if let Err(err) = self.network_sender.send_to_many(other_validators, msg) {
            error!(error = ?err, "Error broadcasting message");
        }
    }

    /// Sends the vote to the chosen recipients (typically that would be the recipients that
    /// we believe could serve as proposers in the next round). The recipients on the receiving
    /// end are going to be notified about a new vote in the vote queue.
    ///
    /// The future is fulfilled as soon as the message put into the mpsc channel to network
    /// internal(to provide back pressure), it does not indicate the message is delivered or sent
    /// out. It does not give indication about when the message is delivered to the recipients,
    /// as well as there is no indication about the network failures.
    pub async fn send_vote(&self, vote_msg: VoteMsg, recipients: Vec<Author>) {
        let mut network_sender = self.network_sender.clone();
        let mut self_sender = self.self_sender.clone();
        let msg = ConsensusMsg::VoteMsg(Box::new(vote_msg));
        for peer in recipients {
            if self.author == peer {
                let self_msg = Event::Message(self.author, msg.clone());
                if let Err(err) = self_sender.send(self_msg).await {
                    error!(error = ?err, "Error delivering a self vote");
                }
                continue;
            }
            if let Err(e) = network_sender.send_to(peer, msg.clone()) {
                error!(
                    remote_peer = peer,
                    error = ?e, "Failed to send a vote to peer",
                );
            }
        }
    }

    /// Sends the given sync info to the given author.
    /// The future is fulfilled as soon as the message is added to the internal network channel
    /// (does not indicate whether the message is delivered or sent out).
    pub fn send_sync_info(&self, sync_info: SyncInfo, recipient: Author) {
        let msg = ConsensusMsg::SyncInfo(Box::new(sync_info));
        let mut network_sender = self.network_sender.clone();
        if let Err(e) = network_sender.send_to(recipient, msg) {
            warn!(
                remote_peer = recipient,
                error = "Failed to send a sync info msg to peer {:?}",
                "{:?}",
                e
            );
        }
    }
}