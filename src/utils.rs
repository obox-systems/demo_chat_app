use argon2::{
  password_hash::SaltString, Algorithm, Argon2, Params, PasswordHash, PasswordHasher,
  PasswordVerifier, Version,
};
use rand::{rngs::OsRng, seq::SliceRandom};

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
  let salt = SaltString::generate(&mut OsRng);
  let params = Params::new(48, 1, 1, None).unwrap();
  let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
  Ok(
    argon2
      .hash_password(password.as_bytes(), &salt)?
      .to_string(),
  )
}

fn generate_password(symbols: &[char], length: usize) -> Option<String> {
  if symbols.is_empty() {
    return None;
  }
  let mut rand = rand::thread_rng();
  Some(
    std::iter::repeat_with(|| symbols.choose(&mut rand).unwrap())
      .take(length)
      .collect(),
  )
}

const AVAILABE: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
pub fn new_access_token() -> String {
  generate_password(&AVAILABE.chars().collect::<Vec<char>>()[..], 64).unwrap()
}

pub fn verify_password(password: &str, hash: &str) -> Result<(), argon2::password_hash::Error> {
  let params = Params::new(48, 1, 1, None).unwrap();
  let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
  let parsed_hash = PasswordHash::new(&hash)?;
  argon2.verify_password(password.as_bytes(), &parsed_hash)
}
