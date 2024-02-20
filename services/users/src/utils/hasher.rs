pub use bcrypt::verify;
use bcrypt::{hash, BcryptResult, DEFAULT_COST};

pub fn hash_password(pwd: &str) -> BcryptResult<String> {
    hash(pwd, DEFAULT_COST)
}
