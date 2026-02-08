use std::collections::HashMap;
use std::fmt::format;
use std::sync::{Arc,Mutex, MutexGuard};
use std::thread;
use std::time::Duration;
use sha2::{Sha256, Digest};
use chrono::Utc;



#[derive(Debug,Clone)]
struct Account{
    address: String,
    balance: u64,
    nonce: u64,
}

#[derive(Debug,Clone)]
struct Block{
    index: u64,
    timestamp: i64,
    data: String,
    prev_hash: String,
    hash: String,
    nonce: u64,
}

fn calculate_hash(index: u64, timestamp: i64, data: &str, prev_hash: &str, nonce: u64) -> String {
    let mut hasher = Sha256::new();
    let input = format!("{}{}{}{}{}", index, timestamp, data, prev_hash, nonce);
    hasher.update(input);
    let result = hasher.finalize();
    hex::encode(result)
}

struct Blockchain{
    chain: Mutex<Vec<Block>>,
    state: Mutex<HashMap<String, Account>>,
}

impl Blockchain{
    //constructor
    fn new()->Self{
        let genesis_block = Block{
            index: 0,
            timestamp: Utc::now().timestamp(),
            data: String::from("Genesis Block"),
            prev_hash: String::from("0"),
            hash: String::from("0000000000000000"),
            nonce: 0,
        };
        
        let mut initial_state = HashMap::new();      
        initial_state.insert(
            String::from("0xAdmin"),
            Account{
                address: String::from("0xAdmin"),
                balance: 1_000_000,
                nonce:0,
            },
        );

        Blockchain{
            chain: Mutex::new(vec![genesis_block]),
            state: Mutex::new(initial_state),
        }
    }

    //view
    fn print_chain(&self){
        let chain_guard = self.chain.lock().unwrap();
        for block in chain_guard.iter(){
            println!(
                "Block #{} [Hash: {}...] [Data: {}]",
                block.index,
                &block.hash[0..8],
                block.data
            );
        }
    }

    //state check
    fn get_balance(&self, address: &str)->u64{
        let state_guard = self.state.lock().unwrap();
        if let Some(account) = state_guard.get(address){
            account.balance
        } else{
            0
        }
    }   

    fn mine_block(&self, data:String){
        let(index, prev_hash) = {
            let chain_guard = self.chain.lock().unwrap();
            let last_block = chain_guard.last().unwrap();
            (last_block.index+1,last_block.hash.clone())
        }; //lock releases here

        println!("Mining Block #{}...", index);
        let mut nonce = 0;
        let mut hash;
        let timestamp = Utc::now().timestamp();

        loop{
            hash = calculate_hash(index, timestamp, &data, &prev_hash, nonce);
            //hash must end in 5 (easy difficulty)

            if hash.ends_with('0'){
                break;
            }
            nonce+=1;
        }

        let new_block = Block{
            index,
            timestamp,
            data,
            prev_hash,
            hash,
            nonce,
        };
        {
            let mut chain_guard = self.chain.lock().unwrap();
            chain_guard.push(new_block);
        }
        println!("Block #{} added to chain!", index);   
    }

    fn transfer(&self, from:&str, to:&str,  amount:u64)->bool{
        let mut state_guard = self.state.lock().unwrap();

        //clone the balance to avoid borrowing issues inside the struct
        let sender_balance = if let Some(account) = state_guard.get(from){
            account.balance
        } else{
            return false;
        };

        if sender_balance>= amount{
            //update sender
            if let Some(account)= state_guard.get_mut(from){
                account.balance-=amount;
                account.nonce+=1;
            }

            let receiver = state_guard
                .entry(to.to_string())
                .or_insert(
                    Account{
                        address: to.to_string(),
                        balance: 0,
                        nonce: 0,
                    }
                );
            receiver.balance+=amount;
            true
        } else {
            false
        }

    }


}

fn simulation(bc: Arc<Blockchain>){
    thread::sleep(Duration::from_secs(1));
    println!("Processing transaction...");

    let success = bc.transfer("0xAdmin", "0xUser1", 500);
    if success{
        println!("Transaction Success!");
    } else{
        println!("Transaction Failed!");
    }
}

fn main() {
    let eth_node = Arc::new(Blockchain::new());
    let eth_node_clone = Arc::clone(&eth_node);

    //user
    let t1 = thread::spawn(move||{
        simulation(eth_node_clone);
    });

    //miner
    let eth_node_miner = Arc::clone(&eth_node);
    let t2 = thread::spawn(move||{
        for i in 0..3{
            eth_node_miner.mine_block(format!("Tx Data Batch {}",i));
            thread::sleep(Duration::from_secs(2));
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();

    //final print
    eth_node.print_chain();

    println!("User1 Balance: {}", eth_node.get_balance("0xUser1"));
}


