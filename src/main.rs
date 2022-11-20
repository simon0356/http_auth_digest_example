
extern crate reqwest; // 0.9.18
use digest_auth::AuthContext;

use reqwest::Client;

const USER: &str = "stropee";
const PASSWORD: &str = "password";
/*
Configuration for Apache2 digest auth :
<Directory /var/www/html/digest >
  AuthType Digest
  AuthName "test"
  AuthDigestProvider file
  AuthUserFile "/etc/apache2/conf-available/.htpasswd"
  AuthDigestDomain /
  Require valid-user
</Directory>


Generate .htpasswd :  
$ htdigest -c /etc/apache2/conf-available/.htpasswd REALM USER
 > Password prompt
*/
#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>>  {
    println!("Hello, world!");
    let url = String::from("http://172.31.164.108:80/digest/");
    let resp = reqwest::get(url.clone())
    .await?;
    println!("{:#?}", resp);
    let wwwauth = resp.headers()["www-authenticate"].to_str()?;
    
    // Step 2:  Given the auth header, sign the digest for the real req.
    let parsed_uri = url.parse::<http::Uri>()?;
    let context = AuthContext::new(USER, PASSWORD, parsed_uri.path());
    let mut prompt = digest_auth::parse(wwwauth)?;
    let answer = prompt.respond(&context)?.to_header_string();
    let client =  Client::builder().build()?;
    let resp = client.get(url.clone()).header("Authorization", answer).send().await?;
    println!("{:#?}", resp);

    Ok(())

}