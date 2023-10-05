use super::block;

fn main() {
    println!("Hello, world!");

    let ab = block.Block {
        index: 23,
        hash: String::from("#38884"),
        prev_block_hash: String::from("#38884"),
    };

    print!("Blockchain struc {}", ab.index);
}
