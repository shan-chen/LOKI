#include "bcos-crypto/signature/key/KeyFactoryImpl.h"
#include "bcos-pbft/pbft/engine/PBFTEngine.h"

extern std::shared_ptr<bcos::front::FrontServiceInterface> FRONTSERVICE;

extern "C" {
void send_packet(std::string target_id, unsigned char* _data){
    auto keyFactory = std::make_shared<bcos::crypto::KeyFactoryImpl>();
    unsigned char *u_target_id = new unsigned char[target_id.length()+1];
    strcpy( (char*) u_target_id, target_id.c_str());
    auto node_id = keyFactory->createKey(bcos::bytesConstRef((bcos::byte*)u_target_id, target_id.length()));
    auto bcosNodeIDs = std::make_shared<std::vector<bcos::crypto::NodeIDPtr>>();
    bcosNodeIDs->reserve(1);
    bcosNodeIDs->push_back(node_id);
    auto data = bcos::bytesConstRef((bcos::byte*)_data, strlen((char*)_data));
    // send only the PBFT messages by indicating the module id with 1000
    FRONTSERVICE->asyncSendMessageByNodeIDs(bcos::protocol::ModuleID::PBFT,*bcosNodeIDs,data);
}

void test_call_cpp(char* par){
    std::cout<<par<<std::endl;
}
}