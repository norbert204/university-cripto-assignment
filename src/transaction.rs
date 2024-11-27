use rsa::{pkcs1v15::SigningKey, pkcs8::DecodePrivateKey, signature::SignerMut, RsaPrivateKey};

use crate::helpers::ToHash;

const KEYS_LOCATION: &str = "keys/";

#[derive(Clone)]
pub struct Transaction {
    pub amount: u32,
    pub sender: String,
    pub receiver: String,
    pub signature: String,
}

impl Transaction {
    pub fn new(sender: &str, receiver: &str, amount: u32) -> Self {
        let sender_pub_key_text = std::fs::read_to_string(format!("{}{}.pub", KEYS_LOCATION, sender)).unwrap().trim().to_owned();
        let receiver_pub_key_text = std::fs::read_to_string(format!("{}{}.pub", KEYS_LOCATION, receiver)).unwrap().trim().to_owned();

        let sender_priv_key = RsaPrivateKey::read_pkcs8_pem_file(format!("{}{}", KEYS_LOCATION, sender)).unwrap();

        let to_encrypt = format!("{} {} {}", sender_pub_key_text, receiver_pub_key_text, amount);

        let mut signing_key = SigningKey::<rsa::sha2::Sha256>::new(sender_priv_key);

        let signature = signing_key.sign(to_encrypt.as_bytes());

        Self {
            amount,
            sender: sender_pub_key_text,
            receiver: receiver_pub_key_text,
            signature: signature.to_string(),
        }
    }
}

impl ToHash for Transaction {
    fn hash(&self) -> String {
        let to_hash = format!("{} {} {} {}", self.sender, self.receiver, self.amount, self.signature);

        sha256::digest(to_hash)
    }
}

pub fn get_merkle_root(previous_hash: Option<String>, transactions: &Vec<Transaction>) -> String {
    if transactions.is_empty() {
        return String::new();
    }

    let mut transaction_hashes: Vec<String> = transactions
        .iter()
        .map(|x| x.hash())
        .collect();

    while transaction_hashes.len() > 1 {
        let mut current_hashes = Vec::new();

        for i in (0..transaction_hashes.len()).step_by(2) {
            if i + 1 >= transaction_hashes.len() {
                current_hashes.push(transaction_hashes[i].clone());
                continue;
            }

            let concat = format!("{}{}", transaction_hashes[i], transaction_hashes[i + 1]);

            current_hashes.push(sha256::digest(concat));
        }

        transaction_hashes = current_hashes;
    }

    match previous_hash {
        Some(hash) => sha256::digest(format!("{}{}", transaction_hashes[0], hash)),
        None => transaction_hashes[0].clone(),
    }
}
