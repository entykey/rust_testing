//use std::io::Write;
use rand::RngCore;
use sha2::{Sha256, Digest}; // Sha512's complexity is higher
use base64::{Engine as _, engine::general_purpose};

// https://docs.rs/sha2/latest/sha2/

fn main() {
    let main_time: std::time::Instant = std::time::Instant::now();


    let password = "myPassword123";
    println!("Original password: {}", password);    // i/o reduces execution time !
    
    let hashed_password = hash_password_with_sha256_with_salt(&password);
    println!("Hashed Password: {}", hashed_password);   // i/o reduces execution time !
    
    let entered_password = "myPassword123";
    let is_password_valid = verify_password_with_sha256_with_salt(&entered_password, &hashed_password);
    println!("Password Valid: {}", is_password_valid);


    // end of main
    let duration: std::time::Duration = main_time.elapsed();
    let elapsed_ms: f64 = duration.as_secs_f64() * 1000.0;
    println!("\nExecution time: {:?} ({:?} ms)", duration, elapsed_ms);

}

#[test]
fn test_hash_password_and_verify() {
    let password = "myPassword123";
    let hashed_password = hash_password_with_sha256_with_salt(password.clone());
    
    assert!(verify_password_with_sha256_with_salt(password, &hashed_password));
    assert!(!verify_password_with_sha256_with_salt("wrongPassword", &hashed_password));
}

#[test]
fn test_hash_password_and_verify1() {
    let password = "myPassword@123";
    let hashed_password = hash_password_with_sha256_with_salt(password.clone());
    
    let is_password_valid = verify_password_with_sha256_with_salt("myPassword@123", &hashed_password);
    assert_eq!(true, is_password_valid);
}



fn hash_password_with_sha256_with_salt(password: &str) -> String {
    let salt = generate_salt();
    let salted_password_bytes = generate_salted_password(password, &salt);

    let mut sha256 = Sha256::new();     // Create new hasher instance.
    sha256.update(&salted_password_bytes);

    let hash_bytes = sha256.finalize();     // Retrieve result and consume hasher instance.
    let salted_hash_bytes = generate_salted_hash(&salt, &hash_bytes);

    println!("SHA256 Salt generated: {}", general_purpose::STANDARD.encode(&salt));
    general_purpose::STANDARD.encode(&salted_hash_bytes)
}

fn generate_salt() -> Vec<u8> {
    let mut salt = vec![0u8; 16];
    rand::thread_rng().fill_bytes(&mut salt);
    salt
}

fn generate_salted_password(password: &str, salt: &[u8]) -> Vec<u8> {
    let password_bytes = password.as_bytes().to_vec();
    let mut salted_password_bytes = Vec::with_capacity(password_bytes.len() + salt.len());

    salted_password_bytes.extend_from_slice(&password_bytes);   // Copies elements from vec T's position to the end of the vector.
    salted_password_bytes.extend_from_slice(salt);      // eg. let mut vec = vec![0, 1, 2, 3, 4];  vec.extend_from_within(2..);  -> [0, 1, 2, 3, 4, 2, 3, 4]

    salted_password_bytes
}

fn generate_salted_hash(salt: &[u8], hash_bytes: &[u8]) -> Vec<u8> {
    let mut salted_hash_bytes = Vec::with_capacity(salt.len() + hash_bytes.len());

    salted_hash_bytes.extend_from_slice(salt);
    salted_hash_bytes.extend_from_slice(hash_bytes);

    salted_hash_bytes
}

fn verify_password_with_sha256_with_salt(password: &str, hashed_password: &str) -> bool {
    let salted_hash_bytes =  general_purpose::STANDARD.decode(hashed_password).unwrap();
    let salt = &salted_hash_bytes[..16];

    let salted_password_bytes = generate_salted_password(password, salt);

    let mut sha256 = Sha256::new();
    sha256.update(&salted_password_bytes);

    let hash_to_check = sha256.finalize();
    let salted_hash_to_check_bytes = generate_salted_hash(salt, &hash_to_check);

    salted_hash_bytes == salted_hash_to_check_bytes
}