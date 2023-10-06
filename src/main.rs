use blockchainlib::block::Block;

fn main() {
    println!("Hello, world!");

    let ab = Block {
        index: 23,
        hash: String::from("#38884"),
        prev_block_hash: String::from("#38884"),
        timestamp: todo!(),
        nonce: todo!(),
        payload: todo!(),
    };

    print!("Blockchain struc {}", ab.index);
}
