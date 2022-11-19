
use serde::{Deserialize, Serialize};
use serde_json::{Result};
use std::fmt;

#[derive(Serialize, Deserialize)]
#[derive(Debug)]

pub struct Obj{
    pub results:Vec<User>
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct User{
    pub name:Name,
    pub email:String,
}
impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.email)
    }
}
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct Name{
    first:String,
    last:String,
}
impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.first,self.last)
    }
}

pub async fn get_user()->Result<Obj>{
    let res = reqwest::get("https://randomuser.me/api/").await.unwrap();
    let body = res.text().await.unwrap();
    let v = serde_json::from_str(&body);
    return v;
}