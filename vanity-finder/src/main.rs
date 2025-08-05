use solana_sdk::signer::{keypair::Keypair, Signer};

fn main() {
    let prefix = "spek";
    let mut attempts = 0;

    loop {
        let keypair = Keypair::new();
        let publickey = keypair.pubkey().to_string();


        println!("Checked {} keys...", attempts);
        
        attempts += 1;
        if publickey.starts_with(prefix) {
            //found key
            println!("\n🎉 Found a match after {} attempts!", attempts);
            println!("   => Wallet Address (Public Key): {}", publickey);
            println!("   => Private Key (Base58): {}", keypair.to_base58_string());
            println!("\nIMPORTANT: Save this private key somewhere safe!");
            break;
        }

    }
}
