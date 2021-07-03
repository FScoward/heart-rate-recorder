use serde::Deserialize;

fn main() {
    println!("Hello, world!");
    match envy::prefixed("FITBIT_").from_env::<OAuth>() {
        Ok(oauth) => println!("{:#?}", oauth),
        Err(error) => panic!("{:#?}", error),
    }
}

#[derive(Deserialize, Debug)]
struct OAuth {
    client_id: String,
    client_secret: String,
}
