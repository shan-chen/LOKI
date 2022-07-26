# Example: Adapt LOKI to A New System

## Steps

**Step 1.** Download LOKI.

**Step 2.** Write 4 interfaces related to specific structures in system under test (SUT).

**Step 3.** Start SUT.

**Step 4.** Start fuzzing engine by call LOKI initialization function.

**Step 5.** Finish fuzzing whenever you want!

## 4 Interfaces:

1. wrap(msg):

   * Input Param: Specific Message Structure of SUT

   * Output Param: General Schema of LOKI
   * Usage: Convert specific message structure object to general schema object

   ***Code Example:***

   ``` c++
   loki_schema wrap(net_message msg) {
       loki_schema loki_msg;
       Json::Value features;
       features["name"] = msg->name;
       features["type"] = msg->type;
       // other fields
       ...
       loki_msg = features.toStyledString();
       return loki_msg;
   }
   ```

2. unwrap(schema):

   * Input Param: General Schema of LOKI

   * Output Param: Specific Message Structure of SUT
   * Usage: Convert general schema object to specific message structure object

   ***Code Example:***

   ``` c++
   net_message unwrap(loki_schema loki_msg) {
       net_message msg;
       Json::Reader reader;
       Json::Value features;
       if (reader.parse(convert_to_json_string(msg).c_str(), features) ) {
           msg->name = features["name"];
           msg->type = features["type"];
           // other fields
           ...
       }
       return msg;
   }
   ```

3. send_packet(msg):

   * Input Param: (Mutated) Message Packet

   * Output Param: None
   * Usage: LOKI will use this function to send mutated message packets to neighbor nodes

   ***Code Example:***

   ``` c++
   void send_packet( const net_message& msg ) {
       verify_strand_in_this_thread( strand, __func__, __LINE__ );
       go_away_reason close_after_send = no_reason;
       if (std::holds_alternative<go_away_message>(msg)) {
           close_after_send = std::get<go_away_message>(msg).reason;
       }
   
       buffer_factory buff_factory;
       auto send_buffer = buff_factory.get_send_buffer( msg );
       enqueue_buffer( send_buffer, close_after_send );
   }
   ```

4. receive_packet(msg):

   * Input Param: Message Packet

   * Output Param: None
   * Usage: LOKI will use this function to obtain the received packets and store them into the seed pool

   ***Code Example:***

   ``` c++
   void receive_packet( const net_message& msg ) {
       uint32_t message_length;
       auto index = msg.read_index();
       msg.peek(&message_length, sizeof(message_length), index);
       try {
           latest_msg_time = get_time();
   
           // if next message is a block we already have, exit early
           auto peek_ds = pending_message_buffer.create_peek_datastream();
           unsigned_int which{};
           fc::raw::unpack( peek_ds, which );
           if( which == signed_block_which || which == signed_block_v0_which ) {
               process_next_block_message( message_length );
   
           } else if( which == trx_message_v1_which || which == packed_transaction_v0_which ) {
               process_next_trx_message( message_length );
   
           } else {
               auto ds = pending_message_buffer.create_datastream();
               net_message msg;
               fc::raw::unpack( ds, msg );
               msg_handler m( shared_from_this() );
               std::visit( m, msg );
           }
   
       } catch( const fc::exception& e ) {
           fc_elog( logger, "Exception in handling message from ${p}: ${s}",
                   ("p", peer_name())("s", e.to_detail_string()) );
           close();
       }
   }
   ```

## Start Fuzzing Engine

Register the callback function and call LOKI to start.

***Code Example:***

``` C++
bool succ = init_fuzzer( send_func=void (*send_packet)(net_message&), recev_func=void (*receive_packet)(net_message&) );
```
