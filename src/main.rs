use serde_json::{Result};
use gvk::users::get_user;

#[tokio::main]
async fn main() -> Result<()> {
    let v= get_user().await?;

    for user in v.results.iter(){
        println!("{}",user.email);
        println!("{}",user.name);
    }
    Ok(())
}



