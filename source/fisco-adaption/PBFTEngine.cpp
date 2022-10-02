extern "C" bytesConstRef get_active_sending_packets();
extern "C" bytesConstRef get_active_sending_targets();
extern "C" char* recv_packets(int msg_type, char* from_id, const unsigned char* payload,int payload_size);

int recv_packects_num = 0;

void start_active_fuzzing(PBFTConfig::Ptr _config) {
    while (true){
        auto msg = get_active_sending_packets();
        auto target = get_active_sending_targets();
        auto keyFactory = std::make_shared<bcos::crypto::KeyFactoryImpl>();
        auto target_node = keyFactory->createKey(target);
        _config->frontService()->asyncSendMessageByNodeID(ModuleID::PBFT, target_node, msg, 0, nullptr);
        std::this_thread::sleep_for(std::chrono::milliseconds(5000));
    }
}

void PBFTEngine::onReceivePBFTMessage(Error::Ptr _error, NodeIDPtr _fromNode, bytesConstRef _data,
    SendResponseCallback _sendResponseCallback)
{
    try
    {
        if (_error != nullptr)
        {
            return;
        }
        // the node is not the consensusNode
        if (!m_config->isConsensusNode())
        {
            PBFT_LOG(TRACE) << LOG_DESC(
                "onReceivePBFTMessage: reject the message "
                "for the node is not the consensus "
                "node");
            return;
        }
        // decode the message and push the message into the queue
        auto pbftMsg = m_config->codec()->decode(_data);
        pbftMsg->setFrom(_fromNode);
        // the committed proposal request message
        if (pbftMsg->packetType() == PacketType::CommittedProposalRequest)
        {
            auto self = std::weak_ptr<PBFTEngine>(shared_from_this());
            m_worker->enqueue([self, pbftMsg, _sendResponseCallback]() {
                try
                {
                    auto pbftEngine = self.lock();
                    if (!pbftEngine)
                    {
                        return;
                    }
                    pbftEngine->onReceiveCommittedProposalRequest(pbftMsg, _sendResponseCallback);
                }
                catch (std::exception const& e)
                {
                    PBFT_LOG(WARNING) << LOG_DESC("onReceiveCommittedProposalRequest exception")
                                      << LOG_KV("error", boost::diagnostic_information(e));
                }
            });
            return;
        }
        // the precommitted proposals request message
        if (pbftMsg->packetType() == PacketType::PreparedProposalRequest)
        {
            auto self = std::weak_ptr<PBFTEngine>(shared_from_this());
            m_worker->enqueue([self, pbftMsg, _sendResponseCallback]() {
                try
                {
                    auto pbftEngine = self.lock();
                    if (!pbftEngine)
                    {
                        return;
                    }
                    pbftEngine->onReceivePrecommitRequest(pbftMsg, _sendResponseCallback);
                }
                catch (std::exception const& e)
                {
                    PBFT_LOG(WARNING) << LOG_DESC("onReceivePrecommitRequest exception")
                                      << LOG_KV("error", boost::diagnostic_information(e));
                }
            });
            return;
        }
        m_msgQueue->push(pbftMsg);
        m_signalled.notify_all();
        // start the passive fuzzing logic of loki
        // here we send the pbftMsg whose type is a PBFTBaseMessage to the loki
        
        auto payload = pbftMsg->encode_basic();
        PBFT_LOG(WARNING) << LOG_DESC("the packet type is: ")
                          << LOG_KV("type", pbftMsg->packetType());
        std::cout<<_fromNode->hex()<<std::endl;
        char *cstr_fromNode = new char[_fromNode->hex().length() + 1];
        strcpy(cstr_fromNode, _fromNode->hex().c_str());
        auto loki_encoded_data = recv_packets((int)pbftMsg->packetType(),cstr_fromNode,_data.data(), _data.size());
        
        m_config->frontService()->asyncSendBroadcastMessage(
        bcos::protocol::NodeType::CONSENSUS_NODE, ModuleID::PBFT, bytesConstRef(loki_encoded_data));
    }
    catch (std::exception const& _e)
    {
        PBFT_LOG(WARNING) << LOG_DESC("onReceivePBFTMessage exception")
                          << LOG_KV("fromNode", _fromNode->hex())
                          << LOG_KV("Idx", m_config->nodeIndex())
                          << LOG_KV("nodeId", m_config->nodeID()->hex())
                          << LOG_KV("error", boost::diagnostic_information(_e));
    }
}