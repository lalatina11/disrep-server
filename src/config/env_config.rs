use std::{env, fmt::Display};

pub struct EnvConfig;

impl EnvConfig {
    pub fn get<T: Display>(key: T) -> String {
        dotenvy::dotenv().unwrap_or_else(|_| panic!("Error while loading .env"));
        let key = format!("{}", key);
        env::var(&key).unwrap_or_else(|_| panic!("Cannot get {} in .env!", key))
    }
}
