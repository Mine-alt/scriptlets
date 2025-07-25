use std::fs::File;
use std::io::Write;
use std::path::Path;
use rpassword::read_password;

const MASTER_PASSWORD: &str = "Saksham@2009"; // replace with your actual master password
const STORE_PASSWORD: &str = "Saksham@2009";
const KEY_ALIAS: &str = "Release";
const KEY_PASSWORD: &str = "Saksham@2009";

const JKS_BYTES: &[u8] = include_bytes!("../assets/Release.jks"); // place your file in `assets/`

fn main() {
    println!("Enter master password:");
    let input = read_password().unwrap();

    if input != MASTER_PASSWORD {
        println!("❌ Incorrect password. Aborting.");
        return;
    }

    // 1. Write keystore.properties
    let mut props = File::create("keystore.properties").expect("Failed to create properties file");
    writeln!(props, "storePassword={}", STORE_PASSWORD).unwrap();
    writeln!(props, "keyAlias={}", KEY_ALIAS).unwrap();
    writeln!(props, "keyPassword={}", KEY_PASSWORD).unwrap();

    // 2. Write the keystore.jks
    let jks_path = Path::new("keystore.jks");
    let mut jks_file = File::create(jks_path).expect("Failed to create keystore.jks");
    jks_file.write_all(JKS_BYTES).expect("Failed to write JKS data");

    println!("✅ keystore.properties and keystore.jks created successfully.");
}
