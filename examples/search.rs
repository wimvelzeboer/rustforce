use rustforce::{Client, Error};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client_id = env::var("SFDC_CLIENT_ID").unwrap();
    let client_secret = env::var("SFDC_CLIENT_SECRET").unwrap();
    let username = env::var("SFDC_USERNAME").unwrap();
    let password = env::var("SFDC_PASSWORD").unwrap();

    let mut client = Client::new_with_client_secret(Some(client_id), Some(client_secret));
    client.login_with_credential(username, password).await?;

    let res = client.search("FIND {rust}").await?;
    println!("{:?}", res);

    Ok(())
}
