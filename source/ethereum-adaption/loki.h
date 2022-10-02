extern "C" char* get_active_sending_packets();
extern "C" char* get_active_sending_targets();
extern "C" char* recv_packets(int msg_type, char* from_id, const unsigned char* payload,int payload_size);
