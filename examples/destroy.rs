use rustforce::{Client, Error};
use serde::Deserialize;
use std::env;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Account {
    #[serde(rename = "attributes")]
    attributes: Attribute,
    id: String,
    name: String,
}

#[derive(Deserialize, Debug)]
struct Attribute {
    url: String,
    #[serde(rename = "type")]
    sobject_type: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client_id = env::var("SFDC_CLIENT_ID").unwrap();
    let client_secret = env::var("SFDC_CLIENT_SECRET").unwrap();
    let username = env::var("SFDC_USERNAME").unwrap();
    let password = env::var("SFDC_PASSWORD").unwrap();

    let mut client = Client::new_with_client_secret(Some(client_id), Some(client_secret));
    client.login_with_credential(username, password).await?;

    let res = client.destroy("Account", "0011t00001FfDoaAAF").await?;
    println!("{:?}", res);

    Ok(())
}
