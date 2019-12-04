extern crate web3;

use web3::futures::Future;
use web3::types::BlockId;
use web3::types::BlockNumber;
use web3::types::U64;
use structopt::StructOpt;
use std::thread; 
use std::time::Duration; 

#[derive(StructOpt)]
struct Arguments {
    start: u32,
    end: u32,
    node: String,
}

fn main() {
	let _args = Arguments::from_args();
    let node = _args.node;
    let (_eloop, http) = web3::transports::Http::new(&node).unwrap();
    let web3 = web3::Web3::new(http);
    let accounts = web3.eth().accounts().wait().unwrap();
    println!("Account is : {:?}", accounts);
    let mut blk = _args.start;
    let mut total = 0 as u32;
    loop
        {
        	thread::sleep(Duration::from_secs(1));
            println!("block number is {}", blk);
            let index = blk.to_string();
            let blk_index = U64::from_dec_str(index.as_str()).unwrap();
            let counts = web3.eth().block_transaction_count(BlockId::Number(BlockNumber::Number(blk_index))).wait().unwrap();

            let txs = match counts {
                Some(value) => value.as_u32(), // This prints "/root", if you run this in Rust playground
                None => 0 as u32,
            };
            total = total + txs;
            println!("total is {}", total);
            if blk > _args.end
            {
                break;
            }
            blk += 1;
        }
}
