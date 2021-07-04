use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use std::collections::HashMap;
use serde::Deserialize;

fn main() {
    println!("Hello, world!");

    let client = reqwest::blocking::Client::new();
    
    let conf: Result<OAuth, envy::Error> = envy::prefixed("FITBIT_").from_env::<OAuth>();
    let c = conf.unwrap();

    let mut params = HashMap::new();
    gen_form_params(&mut params, &c);

    let resp = client.post("https://api.fitbit.com/oauth2/token")
        .headers(gen_headers())
        .form(&params)
        .send();

    match resp {
        Ok(r) => println!("success: {:#?}", r.text()),
        Err(err) => println!("error: {:#?}", err)
    }    
}

fn gen_form_params<'a>(params: &mut HashMap<&'a str, &'a str>, conf: &'a OAuth) {
    params.insert("clientId", &(conf.client_id));
    params.insert("grant_type", "authorization_code");
    params.insert("redirect_uri", "http://localhost");
    params.insert("code", "<code>");
}

fn gen_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/x-www-form-urlencoded"));
    headers.insert(AUTHORIZATION, HeaderValue::from_static("Basic <token>"));
    headers
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
