#![allow(non_camel_case_types, non_snake_case)]

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub enum EyeColor {
    brown, blue
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug, Clone)]
pub struct Element {
    _id: String,
    index: usize,
    isActive: bool,
    balance: String,
    picture: String,
    age: u32,
    eyeColor: EyeColor,
    name: HashMap<String, String>,
    company: String,
    email: String,
    phone: String,
    address: String,
    about: String,
    registered: String,
    latitude: String,
    longitude: String,
    tags: Vec<String>,
    greeting: String,
    favoriteFruit: String,
}
