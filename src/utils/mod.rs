use bcrypt::{hash, verify, DEFAULT_COST};


pub fn hash_password(password: &str) -> String {
    let hash = hash(password, DEFAULT_COST).unwrap();
    hash
}

pub fn verify_password(password: &str, hash_password: &str) -> bool {
    verify(password, hash_password).unwrap()
}
