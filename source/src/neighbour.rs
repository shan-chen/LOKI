//! the neighbour nodes of the loki node

use anyhow::Result;
/// the neighbour nodes of loki node
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Neighbour {
    /// the neighbour's id
    node_id: String,
    /// the state of the neighbour node
    cur_state: String,
}

impl Neighbour {
    /// construct a new neighbour node
    pub fn new(node_id: String, cur_state: String) -> Self {
        Self { node_id, cur_state }
    }

    /// set the node id
    pub fn set_node_id(&mut self, new_id: String) -> Result<bool> {
        self.node_id = new_id;
        Ok(true)
    }

    /// get the node id
    pub fn get_node_id(&self) -> String {
        self.node_id.clone()
    }

    /// get the mut node id
    pub fn get_mut_node_id(&mut self) -> &mut String {
        &mut self.node_id
    }

    /// set current state
    pub fn set_current_state(&mut self, new_state: String) -> Result<bool> {
        self.cur_state = new_state;
        Ok(true)
    }

    /// get current state
    pub fn get_current_state(&self) -> String {
        self.cur_state.clone()
    }

    /// get mutable current state
    pub fn get_mut_current_state(&mut self) -> &mut String {
        &mut self.cur_state
    }
}
