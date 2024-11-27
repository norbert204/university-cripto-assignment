use std::io::Write;

mod blockchain;
mod helpers;
mod transaction;

fn generate_html(blockchain: &blockchain::BlockChain) {
    let index_template = std::fs::read_to_string("templates/index.html").unwrap();
    let block_template = std::fs::read_to_string("templates/block.html").unwrap();
    let transaction_template = std::fs::read_to_string("templates/transaction.html").unwrap();

    let blocks = std::iter::zip(blockchain.blocks.clone(), blockchain.block_infos.clone())
        .enumerate()
        .map(|(index, (block, info))| {
            let transactions = info.transactions
                .iter()
                .map(|transaction| transaction_template.clone()
                    .replace("<!--SENDER-->", transaction.sender.as_str())
                    .replace("<!--RECEIVER-->", transaction.receiver.as_str())
                    .replace("<!--AMOUNT-->", transaction.amount.to_string().as_str())
                    .replace("<!--SIGNATURE-->", transaction.signature.as_str()))
                .collect::<Vec<String>>()
                .join("\n");

            block_template.clone()
                .replace("<!--INDEX-->", index.to_string().as_str())
                .replace("<!--NONCE-->", block.nonce.to_string().as_str())
                .replace("<!--MERKLE_ROOT-->", block.merkle_root.as_str())
                .replace("<!--HASH-->", info.hash.as_str())
                .replace("<!--PREVIOUS_HASH-->", block.previous_hash.as_str())
                .replace("<!--TIMESTAMP-->", block.timestamp.to_string().as_str())
                .replace("<!--TRANSACTIONS-->", transactions.as_str())
                .to_owned()
        })
        .collect::<Vec<String>>()
        .join("\n");

    let index = index_template
        .clone()
        .replace("<!--BLOCKS-->", blocks.as_str())
        .to_owned();

    let mut output = std::fs::File::create("blockchain.html").unwrap();
    output.write_all(index.as_bytes()).unwrap();
}

fn main() {
    let mut blockchain = blockchain::BlockChain::default();

    blockchain.new_block(vec![
        transaction::Transaction::new("alice", "bob", 32),
        transaction::Transaction::new("bob", "jason", 16),
    ]);

    blockchain.new_block(vec![
        transaction::Transaction::new("jason", "alice", 2),
        transaction::Transaction::new("bob", "alice", 322),
        transaction::Transaction::new("alice", "jason", 1),
    ]);

    blockchain.new_block(vec![
        transaction::Transaction::new("alice", "bob", 32),
    ]);

    generate_html(&blockchain);
}
