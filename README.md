# LOKI

LOKI is a fuzzing framework for blockchain consensus protocols. 

# Code Structure

LOKI's source code can be found in the `source` directory. The 4 directorys ended with 'adaption' contain the adaption code to various blockchain systems. The `src` directory contains the main logic of LOKI framework. 

- `state_model.rs` shows how LOKI constructs and updates the state model during the fuzz process.

- `message_pool.rs` , `target_strategy.rs` and `mutator.rs   ` contain the code of the message guider, which records messages into the message pool, mutates the chosen message and sends it to the targets.
- `engine.rs` contains the main logic of the fuzzing process, that is to generate seeds for each round and send them out to the targets.

# Quickstart

To use LOKI to test a new protocol, one can compile LOKI as a dynamc library by:

```
cargo build
```

This may automatically install some dependencies listed in `Cargo.toml`

In addition, one needs to refer to the [FFI documentation](https://doc.rust-lang.org/nomicon/ffi.html) in Rust and write corresponding interfaces. The following code gives an example to write a Rust FFI for C++.

```c++
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
```

This code snippet leverages a `C` style interface to implement the packet sending function. Other examples for FFI in Go can be found in `fabric-adaption` or `ethereum-adaption`. To adapt to a new consensus protocol, the developers of the corresponding blockchain systems also need to implement 4 interfaces, please refer to `adaption.md`.

We prepared the binary code of Fabric, Ethereum, FISCO-BCOS and Diem with LOKI. The following instructions illustrate how to use them. 



## LOKI for fabric

### prerequisites
Setup fabric network environment, can be found in https://hyperledger-fabric.readthedocs.io/en/release/prereqs.html.

### load LOKI-fabric image
```bash
cd fabric
docker import - smartbft/fabric-orderer:latest < LOKI-fabric.tar
```

### setup LOKI testnet & start fuzzing

```bash
cd fabric/testnet
./byfn generate
./byfn up
./byfn down
```


## LOKI for go-ethereum
### prerequisites
Setup geth network environment, can be found in https://priyalwalpita.medium.com/setting-up-the-go-ethereum-geth-environment-in-ubuntu-linux-67c09706b42.

### setup LOKI testnet & start fuzzing

```bash
cd geth/testnet
```
#### setup bootnode
```bash
../bin/bootnode -genkey bootnode.key
../bin/bootnode -nodekey bootnode.key
```
#### initialize node
```bash
for i in 1 2 3 4 5; do ../bin/geth --datadir ./node$i account new; done 
```
#### run loki-geth node

##### node1
```bash
../build/bin/geth --identity "ETH-node1" --datadir "node1" --port "30303" --maxpeers 10 --networkid 10086  --syncmode "full" --bootnodes "enode://5e49cd079bf47d4485867d4fb06a89f211b21be822a05cc8be6ce72b624aa94f4ac65e063fc4d0e62fe2342290bf5c880f6888534ae1df045f67186718d3c3f6@127.0.0.1:0?discport=30301" --mine --miner.etherbase 0xd192415624a039b24ad571f96cb438de9f0556a7 --miner.threads 1 --http --http.port 8545 console
```

##### node2
```bash
../build/bin/geth --identity "ETH-node2" --datadir "node2" --port "30304" --maxpeers 10 --networkid 10086  --syncmode "full" --bootnodes "enode://5e49cd079bf47d4485867d4fb06a89f211b21be822a05cc8be6ce72b624aa94f4ac65e063fc4d0e62fe2342290bf5c880f6888534ae1df045f67186718d3c3f6@127.0.0.1:0?discport=30301" --mine --miner.etherbase 0xd192415624a039b24ad571f96cb438de9f0556a7 --miner.threads 1 console
```

##### node3
```bash
../build/bin/geth --identity "ETH-node3" --datadir "node3" --port "30305" --maxpeers 10 --networkid 10086  --syncmode "full" --bootnodes "enode://5e49cd079bf47d4485867d4fb06a89f211b21be822a05cc8be6ce72b624aa94f4ac65e063fc4d0e62fe2342290bf5c880f6888534ae1df045f67186718d3c3f6@127.0.0.1:0?discport=30301" --mine --miner.etherbase 0xd192415624a039b24ad571f96cb438de9f0556a7 --miner.threads 1 console
```

##### node4
```bash
../build/bin/geth --identity "ETH-node4" --datadir "node4" --port "30306" --maxpeers 10 --networkid 10086  --syncmode "full" --bootnodes "enode://5e49cd079bf47d4485867d4fb06a89f211b21be822a05cc8be6ce72b624aa94f4ac65e063fc4d0e62fe2342290bf5c880f6888534ae1df045f67186718d3c3f6@127.0.0.1:0?discport=30301" --mine --miner.etherbase 0xd192415624a039b24ad571f96cb438de9f0556a7 --miner.threads 1 console
```

##### node5
```bash
../build/bin/geth --identity "ETH-node5" --datadir "node5" --port "30307" --maxpeers 10 --networkid 10086  --syncmode "full" --bootnodes "enode://5e49cd079bf47d4485867d4fb06a89f211b21be822a05cc8be6ce72b624aa94f4ac65e063fc4d0e62fe2342290bf5c880f6888534ae1df045f67186718d3c3f6@127.0.0.1:0?discport=30301" --mine --miner.etherbase 0xd192415624a039b24ad571f96cb438de9f0556a7 --miner.threads 1 console
```

#### ...

#### node_n

## LOKI for fisco

### prerequisites
Setup fisco network environment, can be found in https://fisco-bcos-documentation.readthedocs.io/en/latest/docs/installation.html

### setup LOKI testnet & start fuzzing

```bash
cd fisco/testnet
bash build_chain.sh -l 127.0.0.1:4 -p 30300,20200,8545
```
This will setup a local 4-node consortium chain of fisco-bcos, change 1 of the node to LOKI node:
```bash
# Change the start.sh and stop.sh of node0 to:
fisco_bcos=LOKI/fisco/bin/fisco-bcos
```
Then use the start-all shell to start the chain nodes:
```bash
cd nodes
./start_all.sh
```

## LOKI for Diem
### prerequisites
Just need to setup rust environment.

### setup LOKI testnet & start fuzzing
Use the following command line to start a swarm of 4 nodes with 1 LOKI node and start fuzzing.
```bash
bin/diem-swarm --fuzzer-node bin/diem-node --diem-node YOUR_PATH_TO_NORMAL_DIEM_NODE/diem-node -n 4 -t 1 -c ./tmp
```


# Troubleshooting
Create an issue for questions and bug reports.

# Contribution
We welcome your contributions to LOKI! We aim to create an open-source project that is contributed by the open-source community. For general discussions about development, please refer to the issues. 

# License
[Apache-2.0 License](https://github.com/BlockFuzz/LOKI/blob/main/LICENSE)

