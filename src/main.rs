use ethane::rpc;
// use ethane::rpc::eth_get_balance;
use ethane::types::{Bytes, Call, H160, H256};
use ethane::types::{PrivateKey, TransactionRequest, U256};
use ethane::Connector;
use std::str;
use std::str::FromStr;
use tiny_keccak::{Hasher, Keccak};

fn main() {
    call_contract_example();
    transaction_example();
    sign_example();
}

pub fn sign_example() {
    let node_endpoint = "http://127.0.0.1:8545";
    let mut client = Connector::http(node_endpoint, None).unwrap();

    const SENDER_SECRET: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    const RECEIVER_ADDRESS: &str = "0x70997970c51812dc3a010c7d01b50e0d17dc79c8";

    let transaction = TransactionRequest {
        from: H160::from_str(SENDER_SECRET).unwrap(),
        to: Some(H160::from_str(RECEIVER_ADDRESS).unwrap()),
        value: Some(U256::from_str("100000000000000").unwrap()),
        ..Default::default()
    };

    let result = client.call(rpc::eth_sign_transaction(transaction)).unwrap();
    println!("{:?}", result);
}

pub fn transaction_example() {
    let node_endpoint = "http://127.0.0.1:8545";
    let mut client = Connector::http(node_endpoint, None).unwrap();

    const SENDER_SECRET: &str = "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266";
    const RECEIVER_ADDRESS: &str = "0x70997970c51812dc3a010c7d01b50e0d17dc79c8";

    let transaction = TransactionRequest {
        from: H160::from_str(SENDER_SECRET).unwrap(),
        to: Some(H160::from_str(RECEIVER_ADDRESS).unwrap()),
        value: Some(U256::from_str("100000000000000").unwrap()),
        ..Default::default()
    };

    // send transaction
    let res = client
        .call(rpc::eth_send_transaction(transaction.clone()))
        .unwrap();
    // wait for transaction
    loop {
        let transaction = client.call(rpc::eth_get_transaction_by_hash(res)).unwrap();
        if transaction.block_hash.is_some() {
            break;
        }
    }
    let receipt = client.call(rpc::eth_get_transaction_receipt(res)).unwrap();
    println!("{:?}", receipt.unwrap());
    println!("{:?}", res);
}

pub fn call_contract_example() {
    // Start up connector
    let node_endpoint = "http://127.0.0.1:8545";
    let mut client = Connector::http(node_endpoint, None).unwrap();

    let res = client.call(rpc::eth_accounts()).unwrap();
    println!("{:?}", res);

    match client.call(rpc::eth_get_balance(res[0], None)) {
        Ok(res1) => println!("{:?}", res1),
        Err(err1) => println!("{:?}", err1),
    }

    match client.call(rpc::web3_client_version()) {
        Ok(res1) => println!("{:?}", res1),
        Err(err1) => println!("{:?}", err1),
    }

    const ADDR_STR: &str = "0x3Aa5ebB10DC797CAC828524e59A333d0A371443c";
    let contract_address: H160 = H160::from_str(ADDR_STR).unwrap();

    let out = keccak(b"solution()");

    let call = Call {
        to: contract_address,
        data: Some(Bytes::from_slice(&out[..4])),
        ..Default::default()
    };
    let res = client.call(rpc::eth_call(call, None)).unwrap();
    println!("{:?}", res);
}

pub fn keccak(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    hasher.update(input);
    let mut out = [0u8; 32];
    hasher.finalize(&mut out);
    out
}
