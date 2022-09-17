use std::fmt::Debug;

use reqwest::IntoUrl;
use serde::{de::DeserializeOwned};
pub mod models;

pub fn get<T, U>(uri: U) -> Result<T, Box<dyn std::error::Error>>
where 
    T: DeserializeOwned + Debug,
    U: IntoUrl
 {
    let resp = reqwest::blocking::get(uri)?
        .json::<T>()?;
         Ok(resp)
}
