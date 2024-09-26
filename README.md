[![crate-name at crates.io](https://img.shields.io/crates/v/rustforce.svg)](https://crates.io/crates/rustforce)
[![crate-name at docs.rs](https://docs.rs/rustforce/badge.svg)](https://docs.rs/rustforce)
![Rust](https://github.com/tzmfreedom/rustforce/workflows/Rust/badge.svg)

## Rustforce

Salesforce Client for Rust

## Usage

```rust
use rustforce::{Client, Error};
use rustforce::response::{QueryResponse, ErrorResponse};
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

    let sfdx_auth_url = "force://PlatformCLI::Aep8613Yf9cciJ8KNj51RDvetkI8Fxf78YRstb1GEUj9PFtlw4V42xT4XKJPi3lMMzYdPd3BymGXoLxsiRjyOS_@orgname--sandboxname.sandbox.my.salesforce.com".to_string();

    let mut client = Client::new();
    client.login_with_sfdx_auth_url(sfdx_auth_url).await?;

    let res: QueryResponse<Account> = client.query("SELECT Id, Name FROM Account WHERE id = '0012K00001drfGYQAY'".to_string()).await?;
    println!("{:?}", res);

    Ok(())
}
```

### Authentication

Username Password Flow
```rust
let mut client = Client::new_with_client_secret(client_id, client_secret);
client.login_with_credential(username, password).await?;
```

SFDX Authentication Url
```rust
let mut client = Client::new();
client.login_with_sfdx_auth_url(sfdx_auth_url).await?;
```

[WIP]Authorization Code Grant

### Refresh Token

```rust
let r = client.refresh("xxxx").await?;
```

### Query Records

```rust
let r: Result<QueryResponse<Account>, Error> = client.query("SELECT Id, Name FROM Account").await?;
```

### Query All Records

```rust
let r: Result<QueryResponse<Account>, Error> = client.query_all("SELECT Id, Name FROM Account").await?;
```

### Find By Id

```rust
let r: Result<Account, Error> = client.find_by_id("Account", "{sf_id}").await?;
```

### Create Record

```rust
let mut params = HashMap::new();
params.insert("Name", "hello rust");
let r = client.create("Account", params).await?;
println!("{:?}", r);
```

### Update Record

```rust
let r = client.update("Account", "{sobject_id}", params).await?;
```

### Upsert Record

```rust
let r = client.upsert("Account", "{external_key_name}", "{external_key", params).await?;
```

### Delete Record

```rust
let r = client.destroy("Account", "{sobject_id}").await?;
```

### Describe Global

```rust
let r = client.describe_global().await?;
```

### Describe SObject

```rust
let r = client.describe("Account").await?;
```

### Versions

```rust
let versions = client.versions().await?;
```

### Search(SOSL)

```rust
let r = client.search("FIND {Rust}").await?;
```
