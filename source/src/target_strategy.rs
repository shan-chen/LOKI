//! decide which targets to choose when sending packets
// random + 1/3 + 1/2 + 2/3

use rand::seq::SliceRandom;

use anyhow::{anyhow, Result};
use rand::Rng;

/// randomly choose some of the connected as the target nodes.
/// the parameter represents all of the connected nodes
pub fn random_target_nodes_strategy(connected_nodes: Vec<String>) -> Result<Vec<String>> {
    let mut rng = rand::thread_rng();
    let res: u32 = rng.gen::<u32>();
    if connected_nodes.is_empty() {
        return Err(anyhow!(
            "The connected nodes are empty! Cannot selecet the target nodes!"
        ));
    }
    let total = connected_nodes.len();
    // this means the number of targets that are chosen
    let random_num = res % (total as u32);
    // randomly choose random_num nodes from all the connected nodes
    let sample: Vec<_> = connected_nodes
        .choose_multiple(&mut rand::thread_rng(), random_num.try_into().unwrap())
        .map(|x| x.clone())
        .collect();
    Ok(sample)
}

/// randomly choose certain number of the connected as the target nodes.
pub fn random_target_nodes_with_num_strategy(
    connected_nodes: Vec<String>,
    num: u32,
) -> Result<Vec<String>> {
    if connected_nodes.is_empty() {
        return Err(anyhow!(
            "The connected nodes are empty! Cannot selecet the target nodes!"
        ));
    }
    if num > (connected_nodes.len().try_into().unwrap()) {
        return Err(anyhow!(
            "Cannot choose targets more than the connnected nodes' length!"
        ));
    }
    // randomly choose random_num nodes from all the connected nodes
    let sample: Vec<_> = connected_nodes
        .choose_multiple(&mut rand::thread_rng(), num.try_into().unwrap())
        .map(|x| x.clone())
        .collect();
    Ok(sample)
}
