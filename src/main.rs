#[macro_use]
extern crate serde_derive;

mod blockchain_info;
mod blockchain_status;
mod blockchain_address;
mod blockchain_transaction;

use std::{io, thread, time};

use blockchain_address::BlockchainAddress;
use blockchain_status::BlockchainStatus;
use blockchain_transaction::BlockchainTransaction;
use blockchain_info::{blockchain_address_request, blockchain_status_request, blockchain_transaction_request};

fn main() {
    let b_s: BlockchainStatus = blockchain_status_request();
    println!("Chain -> {}, Coin -> {}", &b_s.backend.chain, &b_s.blockbook.coin);

    let b_a: BlockchainAddress = blockchain_address_request("1NDxDDSHVHvSv48vd27NNHkXHYZjDdVLss");
    println!("Address -> {}", &b_a.address);

    let sleep_time = time::Duration::from_millis(1000);
    thread::sleep(sleep_time);

    println!("You have a total of {} transactions!", &b_a.txids.len());

    println!("Do you want to query these transactions? (y/n)");
    let mut command = String::new();
    io::stdin().read_line(&mut command);

    if command.trim().eq("y") {
        println!("Looking up the following transactions...");
        // println!("{:#?}", &b_a.txids);                            Beautifully prints vectors

        let mut balance: i32 = 0;

        for txnid in &b_a.txids {
            let mut total_vin = 0;
            let mut total_vout = 0;

            let b_t: BlockchainTransaction = blockchain_transaction_request(&txnid);

            let match_address = String::from("1NDxDDSHVHvSv48vd27NNHkXHYZjDdVLss");

            for tx in &b_t.vin {
                if tx.addresses.contains(&match_address) {
                    total_vin += tx.value.parse::<i32>().unwrap();
                }
            }
            for tx in &b_t.vout {
                if tx.addresses.contains(&match_address) {
                    total_vout += tx.value.parse::<i32>().unwrap();
                }
            }
            balance += total_vout - total_vin;
            println!("TX ID -> {}, BALANCE -> {}", &b_t.txid, &balance);
        };
        println!("Total Balance -> {}", &balance);
        println!("Total Balance(BTC) -> {}", balance as f32 * 0.00000001);
    }
}
