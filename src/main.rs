use serde::Deserialize;
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let uri = gen_get_code();
    println!("{:?}", uri);




}

fn gen_get_code() -> Result<String, envy::Error>  {
    return envy::prefixed("FITBIT_").from_env::<OAuth>()
        .map(|oauth_config| 
            format!("https://www.fitbit.com/oauth2/authorize?response_type=code&client_id={}&redirect_uri=http%3A%2F%2Flocalhost&scope=activity%20heartrate%20location%20nutrition%20profile%20settings%20sleep%20social%20weight&expires_in=604800", oauth_config.client_id)
        );
}

#[derive(Deserialize, Debug)]
struct OAuth {
    client_id: String,
    client_secret: String,
}
