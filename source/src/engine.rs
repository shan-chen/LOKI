//! LOKI fuzzing engine

use crate::global_definition::*;
use crate::hash_functions::*;
use crate::loki_message::*;
use crate::message_pool::MessagePool;
use crate::neighbour::Neighbour;
use crate::state_model::{State, StateEdge, StateModel};
use crate::target_strategy::*;
use crate::user_interface::*;
use antlr_rust as _;
use anyhow::{anyhow, Context, Result};
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs;
use std::path::PathBuf;
use std::vec;
use std::{thread, time};

use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::input_stream::InputStream;
use antlr_rust::token_factory::CommonTokenFactory;
use antlr_rust::tree::Visitable;
use loki_spec::loki_spec::loki_speclexer::*;
use loki_spec::loki_spec::loki_specparser::*;
use loki_spec::loki_spec::spec_visitor::*;

/// the interval of active package sending, the unit is second
const SENDING_INTERVAL: u64 = 5;

/// the total number of target selection strategies
const TARGET_SELECTION_STRATEGIES_NUMBER: u32 = 4;

/// LOKI fuzz engine struct
pub struct Engine {
    /// the message pool of the engine
    message_pool: MessagePool,
    /// connected nodes of loki node
    connnected_nodes: Vec<Neighbour>,
    /// state model of the tested protocol
    state_model: StateModel,
    /// current state of LOKI node
    cur_state: String,
}

/// config the hash functions, this should be done right after the fuzzer starts
pub fn config_hash_func() {
    add_hash_function("keccak256".to_string(), keccak256);
    add_hash_function("sha256".to_string(), sha3_256);
    add_hash_function("sm3".to_string(), sm3);
}

impl Engine {
    /// create a fuzz engine
    pub fn new(
        message_pool: MessagePool,
        connnected_nodes: Vec<Neighbour>,
        state_model: StateModel,
        cur_state: String,
    ) -> Self {
        Self {
            message_pool,
            connnected_nodes,
            state_model,
            cur_state,
        }
    }

    /// create a fuzz engine without any field
    pub fn new_with_empty_fields() -> Self {
        let message_pool = MessagePool::new(Vec::new());
        let connnected_nodes = Vec::new();
        let state_model = StateModel::new(Vec::new());
        let cur_state = "".to_string();
        Self {
            message_pool,
            connnected_nodes,
            state_model,
            cur_state,
        }
    }

    /// passive sending fuzz packets without initial seeds
    /// this function returns the next message that the fuzzer need to send with LokiMessage type
    /// it will first update the neighbours and statemodels according to the received message
    pub fn passive_sending(&mut self, rec_message: LokiMessage) -> LokiMessage {
        // first, update the neighbours
        match self.update_neighbours_with_rec_msg(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the neighbours with received message!")
            }
        };

        // judge sender's current state, if it needs to transfer, choose the proper state
        match self.update_sender_node_state(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the sender node's state")
            }
        };

        // check whethere the received message leads to a new edge
        match self.check_and_update_new_edges(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the sender node's state")
            }
        };

        // Then, we need to decide which State to transfer, and what type of messages we need to send
        let next_msg_type = match self.get_next_message_type_and_update_state() {
            Ok(msg_type) => msg_type,
            Err(_) => {
                panic!("Failed to get next message type")
            }
        };

        // add the received message into the message pool
        self.get_mut_message_pool()
            .unwrap()
            .add_msg_to_pool(rec_message)
            .unwrap();

        // get the latest message of the next message type from the message pool
        let mut org_msg = match self
            .get_message_pool()
            .unwrap()
            .find_latest_message_with_type(next_msg_type.clone())
        {
            Some(msg) => msg,
            None => {
                // no fit messages found in the message pool, generate a new message
                LokiMessage::generate(next_msg_type.clone())
            }
        };

        // mutate the message and return
        org_msg.mutate();
        org_msg
    }

    /// passive sending fuzz packets with initial seeds
    pub fn passive_sending_with_seed(
        &mut self,
        based_seed: &mut LokiMessage,
        rec_message: LokiMessage,
    ) -> LokiMessage {
        // first, update the neighbours
        match self.update_neighbours_with_rec_msg(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the neighbours with received message!")
            }
        };

        // judge sender's current state, if it needs to transfer, choose the proper state
        match self.update_sender_node_state(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the sender node's state")
            }
        };

        // check whethere the received message leads to a new edge
        match self.check_and_update_new_edges(&rec_message) {
            Ok(_) => {}
            Err(_) => {
                panic!("Failed to update the sender node's state")
            }
        };

        // add the received message into the message pool
        self.get_mut_message_pool()
            .unwrap()
            .add_msg_to_pool(rec_message)
            .unwrap();

        // update the current state of loki
        let next_msg_type = based_seed.get_structure().get_name();

        match self
            .get_mut_state_model()
            .find_state_with_msg_type(next_msg_type.clone())
        {
            Ok(_) => {
                self.set_cur_state(next_msg_type).unwrap();
            }
            Err(_) => {
                // This indicates that a new state need to be added into the state model
                let new_state = State::new(next_msg_type.clone());
                self.get_mut_state_model().add_new_state(new_state).unwrap();
                self.set_cur_state(next_msg_type).unwrap();
            }
        };

        // mutate the message and return
        based_seed.mutate();
        based_seed.clone()
    }

    /// active sending fuzz packets
    /// this need to be called in a new thread
    pub fn active_sending(&mut self) {
        // infinite loop, never stop unless LOKI is down
        loop {
            // first get the sending targets for this round
            let mut send_node_ids: Vec<String> = vec![];
            let mut rng = rand::thread_rng();
            let temp: u32 = rng.gen::<u32>();
            let connected_node_ids = self
                .connnected_nodes
                .clone()
                .into_iter()
                .map(|n| n.get_node_id())
                .collect::<Vec<_>>();
            let total_nodes = connected_node_ids.len() as u32;
            // if the random number indicates that we should choose the random trarget selection strategy
            if temp % TARGET_SELECTION_STRATEGIES_NUMBER == 0 {
                match random_target_nodes_strategy(connected_node_ids) {
                    Ok(ids) => {
                        send_node_ids = ids;
                    }
                    Err(_) => {
                        panic!("Connected node is empty! Check your network settings!");
                    }
                }
            }
            // if the random number indicates that we should choose the random trarget selection strategy and select 1/3 nodes
            else if temp % TARGET_SELECTION_STRATEGIES_NUMBER == 1 {
                match random_target_nodes_with_num_strategy(connected_node_ids, total_nodes / 3) {
                    Ok(ids) => {
                        send_node_ids = ids;
                    }
                    Err(_) => {
                        panic!("Connected node is empty! Check your network settings!");
                    }
                }
            }
            // if the random number indicates that we should choose the random trarget selection strategy and select 2/3 nodes
            else if temp % TARGET_SELECTION_STRATEGIES_NUMBER == 2 {
                match random_target_nodes_with_num_strategy(connected_node_ids, 2 * total_nodes / 3)
                {
                    Ok(ids) => {
                        send_node_ids = ids;
                    }
                    Err(_) => {
                        panic!("Connected node is empty! Check your network settings!");
                    }
                }
            }
            // if the random number indicates that we should choose the random trarget selection strategy and select 1/2 nodes
            else if temp % TARGET_SELECTION_STRATEGIES_NUMBER == 3 {
                match random_target_nodes_with_num_strategy(connected_node_ids, total_nodes / 2) {
                    Ok(ids) => {
                        send_node_ids = ids;
                    }
                    Err(_) => {
                        panic!("Connected node is empty! Check your network settings!");
                    }
                }
            }

            // secondly, generate the message for each target and send the package out
            // here we have 4 choices:
            // 1) same messages for each target, randomly choose
            // 2) same messages for each target, according to LOKI's state
            // 3) various messages for each target, randomly choose
            // 4) various messages for each target, according to the target's state
            let temp_type: u32 = rng.gen::<u32>();
            if temp_type % 4 == 0 {
                // 1) randomly choose a message type
                let chosen_msg_type = get_message_types()
                    .choose_multiple(&mut rand::thread_rng(), 1)
                    .map(|x| x.to_string().clone())
                    .collect::<Vec<String>>()[0]
                    .clone();
                // get a random value of such type
                let chosen_msg = match self
                    .message_pool
                    .find_latest_message_with_type(chosen_msg_type.clone())
                {
                    Some(msg) => msg,
                    None => {
                        // if there is no such type msg in the pool, generate one
                        LokiMessage::generate(chosen_msg_type.clone())
                    }
                };
                // send the message
                for target in send_node_ids {
                    send_packets(target, chosen_msg.clone());
                }
            } else if temp_type % 4 == 1 {
                // 2) choose a type according to LOKI's state
                let current_state = match self
                    .state_model
                    .find_state_with_msg_type(self.cur_state.clone())
                {
                    Ok(state) => state,
                    Err(_) => {
                        panic!(
                            "cannot find the message with type: {:?}",
                            self.cur_state.clone()
                        );
                    }
                };
                let mut next_states: Vec<String> = current_state
                    .get_cur_edges()
                    .iter()
                    .map(|e| e.get_to_state())
                    .collect();
                next_states.push(self.cur_state.clone());
                let chosen_msg_type = next_states
                    .choose_multiple(&mut rand::thread_rng(), 1)
                    .map(|x| x.clone())
                    .collect::<Vec<String>>()[0]
                    .clone();
                // get a random value of such type
                let chosen_msg = match self
                    .message_pool
                    .find_latest_message_with_type(chosen_msg_type.clone())
                {
                    Some(msg) => msg,
                    None => {
                        // if there is no such type msg in the pool, generate one
                        LokiMessage::generate(chosen_msg_type.clone())
                    }
                };
                // send the message
                for target in send_node_ids {
                    send_packets(target, chosen_msg.clone());
                }
            } else if temp_type % 4 == 2 {
                // 3) various random messages for each target
                for target in send_node_ids {
                    let chosen_msg_type = get_message_types()
                        .choose_multiple(&mut rand::thread_rng(), 1)
                        .map(|x| x.to_string().clone())
                        .collect::<Vec<String>>()[0]
                        .clone();
                    // get a random value of such type
                    let chosen_msg = match self
                        .message_pool
                        .find_latest_message_with_type(chosen_msg_type.clone())
                    {
                        Some(msg) => msg,
                        None => {
                            // if there is no such type msg in the pool, generate one
                            LokiMessage::generate(chosen_msg_type.clone())
                        }
                    };
                    send_packets(target, chosen_msg.clone());
                }
            } else if temp_type % 4 == 3 {
                // 4) various messages for each target according to the target's state
                for target in send_node_ids {
                    let neighbour = match self.get_neighbour_by_id(target.clone()) {
                        Ok(n) => n,
                        Err(_) => {
                            panic!("Try to send to neighbour node not connected with LOKI, the id is {:?}!",target);
                        }
                    };
                    let neighbour_current_state = match self
                        .state_model
                        .find_state_with_msg_type(neighbour.get_current_state().clone())
                    {
                        Ok(state) => state,
                        Err(_) => {
                            panic!(
                                "cannot find the message with type: {:?}",
                                neighbour.get_current_state().clone()
                            );
                        }
                    };
                    let mut next_states: Vec<String> = neighbour_current_state
                        .get_cur_edges()
                        .iter()
                        .map(|e| e.get_to_state())
                        .collect();
                    next_states.push(neighbour.get_current_state().clone());
                    let chosen_msg_type = next_states
                        .choose_multiple(&mut rand::thread_rng(), 1)
                        .map(|x| x.clone())
                        .collect::<Vec<String>>()[0]
                        .clone();
                    // get a random value of such type
                    let chosen_msg = match self
                        .message_pool
                        .find_latest_message_with_type(chosen_msg_type.clone())
                    {
                        Some(msg) => msg,
                        None => {
                            // if there is no such type msg in the pool, generate one
                            LokiMessage::generate(chosen_msg_type.clone())
                        }
                    };
                    send_packets(target, chosen_msg.clone());
                }
            }

            // sleep for the interval time
            let sleep_time = time::Duration::from_secs(SENDING_INTERVAL);
            thread::sleep(sleep_time);
        }
    }

    /// set the addresses of each node
    pub fn set_node_addr(&mut self, _nodes: Vec<Neighbour>) -> Result<bool> {
        self.connnected_nodes = _nodes;
        Ok(true)
    }

    /// get the connnected nodes
    pub fn get_connected_nodes(&self) -> Result<Vec<Neighbour>> {
        Ok(self.connnected_nodes.clone())
    }

    /// get the mutable connected nodes
    pub fn get_mut_connected_nodes(&mut self) -> Result<&mut Vec<Neighbour>> {
        Ok(&mut self.connnected_nodes)
    }

    /// update current state
    pub fn set_cur_state(&mut self, new_state: String) -> Result<bool> {
        self.cur_state = new_state;
        Ok(true)
    }

    /// get current state
    pub fn get_cur_state(&self) -> String {
        self.cur_state.clone()
    }

    /// get mut current state
    pub fn get_cur_mut_state(&mut self) -> &mut String {
        &mut self.cur_state
    }

    /// init the fuzz engine, the main controller of the LOKI
    /// First, the engine resisters the global variables
    /// Then, the engine inits the DSL parser
    /// After that, the engine constructs the state model
    /// Then, constructs the engine and starts the fuzz
    pub fn start_fuzz_engine() -> Self {
        // set the global variables
        // === HERE TO MODIFY ===
        crate::loki_type::set_timestamp_length(16);
        crate::loki_type::set_current_language("Cpp".to_string());
        crate::loki_type::set_current_private_key("private.key".to_string());
        // === === === === == ===
        println!(
            "LOKI has startted!!! Current blockchain's language is {:?}.",
            crate::loki_type::get_current_language()
        );
        // get the spec file location
        let mut config_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        // === HERE TO MODIFY ===
        config_path.push("testcase");
        config_path.push("simple.spec");
        // === === === === == ===
        println!(
            "Loading the spec file, its location is {:?}...",
            config_path
        );
        let input_str: String = fs::read_to_string(config_path).unwrap();
        let input = InputStream::new(&*input_str);
        let tf = CommonTokenFactory::default();
        let lexer = Loki_specLexer::new_with_token_factory(input, &tf);
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = Loki_specParser::new(token_source);
        let result = parser.document();
        // Check whether the loading is successful
        let result_ctx = result.expect("Error: load spec file unsuccessfully!!!!");
        println!("Load successfully! Now set the message list!!!");
        let mut visitor = Specvisitor::default();
        result_ctx.accept(&mut visitor);
        let msgs_list = visitor.get_spec_messages().clone();
        set_message_list(msgs_list);
        // Then LOKI sets the initial state model
        let state_model = StateModel::initialize_the_state_model();
        println!("LOKI has set up the initial state model!!!");
        // Then LOKI initializes the neighbour nodes info
        println!("Loading the neighbour nodes!!!");
        let cur_state = state_model.clone().get_states()[0].get_msg_type();
        // // === HERE TO MODIFY ===
        // const NEIGHBOUR_NUM:u32 = 3;
        // let mut neighbours = Vec::<Neighbour>::new();
        // for i in (0..NEIGHBOUR_NUM){
        //     let nei = Neighbour::new(String::from(i),cur_state.cloen())
        // }
        //  // === === === === == ===

        // Then LOKI constructs the engine, the message pool is empty and the neighbour nodes are empty too
        let loki_engine = Engine::new(MessagePool::new(vec![]), vec![], state_model, cur_state);
        println!("LOKI engined constructed!!!");
        // println!("LOKI engined constructed!!! Now we start the active fuzzing thread!!!");
        // thread::spawn(|| {
        //     loki_engine.active_sending();
        // });
        // println!("OK the active fuzzing thread has started!!! All is well!!! Enjoy LOKI's tricks!!!");
        loki_engine
    }

    /// start the active fuzzing in a new thread
    pub fn start_active_fuzzing(&'static mut self) {
        thread::spawn(|| {
            self.active_sending();
        });
    }

    // some protected functions that can only be called in the engine object
    /// set the message_pool
    #[allow(dead_code)]
    pub fn set_message_pool(&mut self, new_pool: MessagePool) -> Result<bool> {
        self.message_pool = new_pool;
        Ok(true)
    }

    /// get the message_pool
    #[allow(dead_code)]
    pub fn get_message_pool(&self) -> Result<MessagePool> {
        Ok(self.message_pool.clone())
    }

    /// get the mutable message_pool
    #[allow(dead_code)]
    pub fn get_mut_message_pool(&mut self) -> Result<&mut MessagePool> {
        Ok(&mut self.message_pool)
    }

    /// set the state model
    #[allow(dead_code)]
    pub fn set_state_model(&mut self, new_model: StateModel) -> Result<bool> {
        self.state_model = new_model;
        Ok(true)
    }

    /// get the state model
    pub fn get_state_model(&self) -> &StateModel {
        &self.state_model
    }

    /// get the mutable state model
    pub fn get_mut_state_model(&mut self) -> &mut StateModel {
        &mut self.state_model
    }

    /// get the mutable neighbour node by id
    pub fn get_mut_neighbour_by_id(&mut self, node_id: String) -> Result<&mut Neighbour> {
        for n in &mut self.connnected_nodes {
            if n.get_node_id() == node_id {
                return Ok(n);
            }
        }
        return Err(anyhow!(
            "Try to get a neighbour node not connected with LOKI!"
        ));
    }

    /// get the neighbour node by id
    pub fn get_neighbour_by_id(&self, node_id: String) -> Result<&Neighbour> {
        for n in &self.connnected_nodes {
            if n.get_node_id() == node_id {
                return Ok(n);
            }
        }
        return Err(anyhow!(
            "Try to get a neighbour node not connected with LOKI!"
        ));
    }

    /// update the connected nodes according to the received messages
    pub fn update_neighbours_with_rec_msg(&mut self, rec_message: &LokiMessage) -> Result<bool> {
        let from_node = match rec_message.get_from_node() {
            Ok(node) => node,
            Err(e) => {
                return Err(e).with_context(|| format!("Received a Message without From Node"));
            }
        };
        // judge whether the from node is included in the connected_nodes
        let node_ids: Vec<_> = self
            .connnected_nodes
            .iter()
            .map(|n| n.get_node_id())
            .collect();
        // if not, add the current node into the neighbours
        if !node_ids.contains(&from_node) {
            self.connnected_nodes.push(Neighbour::new(
                from_node.clone(),
                rec_message.get_structure().get_name(),
            ))
        }
        Ok(true)
    }

    /// update the sender nodes state
    pub fn update_sender_node_state(&mut self, rec_message: &LokiMessage) -> Result<bool> {
        let from_node = match rec_message.get_from_node() {
            Ok(node) => node,
            Err(e) => {
                return Err(e).with_context(|| format!("Received a Message without From Node"));
            }
        };
        let new_type = rec_message.get_structure().get_name();

        // After that, update the sender's state, first we find the sender node
        let sender_node: &mut Neighbour = self
            .connnected_nodes
            .iter_mut()
            .find(|n| n.get_node_id() == from_node)
            .unwrap();

        // update the sender's state
        sender_node.set_current_state(new_type.clone()).unwrap();
        return Ok(true);
    }

    /// check whether we should add new edges to the state model when received some msgs
    pub fn check_and_update_new_edges(&mut self, rec_message: &LokiMessage) -> Result<bool> {
        let new_type = rec_message.get_structure().get_name();
        let cur_type = self.get_cur_state();
        // update the edges if there is no edge before, the initial weight is 1.0
        let cur_state = match self
            .get_mut_state_model()
            .find_mut_state_with_msg_type(cur_type)
        {
            Ok(state) => state,
            Err(e) => return Err(e),
        };
        let e = cur_state
            .get_cur_edges()
            .iter()
            .find(|e| e.get_to_state() == new_type);
        if e.is_none() {
            let new_edge = StateEdge::new(new_type.clone(), 1.0);
            cur_state.add_cur_edges(new_edge).unwrap();
        }
        Ok(true)
    }

    /// get the next message type according to the transfer weight
    pub fn get_next_message_type_and_update_state(&mut self) -> Result<String> {
        let cur_state_type = self.get_cur_state();
        let cur_state = match self
            .get_state_model()
            .find_state_with_msg_type(cur_state_type)
        {
            Ok(state) => state,
            Err(e) => return Err(e),
        };
        let weigths: Vec<f32> = cur_state
            .get_cur_edges()
            .iter()
            .map(|e| e.get_weight())
            .collect();
        // get the total weights
        let total_weights: f32 = weigths.iter().sum();
        let mut to_state = String::new();
        let mut rng = rand::thread_rng();
        let mut min_range: u32 = 0;
        let mut max_range: u32 = 0;
        let temp: u32 = rng.gen_range(0..101);
        for edge in cur_state.get_cur_edges().iter() {
            max_range += ((edge.get_weight() / total_weights) as u32) * 100;
            if min_range <= temp && max_range >= temp {
                to_state = edge.get_to_state();
                break;
            }
            min_range = max_range;
        }
        // if the to state is empty, something is wrong
        if to_state.is_empty() {
            return Err(anyhow!(
                "can't find any state to transfer, some states may have no edges in the statemodel"
            ));
        } else {
            self.set_cur_state(to_state).unwrap();
        }
        // get the type of next message
        Ok(self.get_cur_state())
    }
}
