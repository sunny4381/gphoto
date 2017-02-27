use std;

use hyper;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_native_tls::NativeTlsClient;
use hyper::header::{ContentType, UserAgent, Authorization};

use url::form_urlencoded;

use serde_json;

use error::Error;

pub const AUTH_URL: &'static str = "https://accounts.google.com/o/oauth2/v2/auth";
pub const TOKEN_URL: &'static str = "https://www.googleapis.com/oauth2/v4/token";
pub const INFO_URL: &'static str = "https://www.googleapis.com/oauth2/v1/userinfo";
pub const REDIRECT_URI: &'static str = "urn:ietf:wg:oauth:2.0:oob";
pub const USER_AGENT: &'static str = "rust-oauth-test/0.1";

header! { (GDataVersion, "GData-Version") => [String] }
header! { (Slug, "Slug") => [String] }

#[derive(Debug, RustcDecodable)]
pub struct Token {
    pub access_token: String,
    pub expires_in: u64,
    pub token_type: String,
    pub refresh_token: Option<String>,
}

#[derive(Debug, RustcDecodable)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: String,
    pub email: String,
}

pub fn client() -> Result<Client, Error> {
    let ssl = try!(NativeTlsClient::new().map_err(Error::NativeTlsError));
    return Ok(Client::with_connector(HttpsConnector::new(ssl)));
}

pub fn auth_url(client_id: &str) -> String {
    let auth_params: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", client_id)
        .append_pair("redirect_uri", REDIRECT_URI)
        .append_pair("response_type", "code")
        .append_pair("scope", "profile email http://picasaweb.google.com/data/")
        .finish();
    return format!("{}?{}", AUTH_URL, auth_params);
}

pub fn auth_token(client_id: &str, client_secret: &str, code: &str) -> Result<Token, Error> {
    let token_body: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("code", code)
        .append_pair("client_id", client_id)
        .append_pair("client_secret", client_secret)
        .append_pair("redirect_uri", REDIRECT_URI)
        .append_pair("grant_type", "authorization_code")
        .finish();

    let client = try!(client());
    let req = client.post(TOKEN_URL)
        .body(&token_body)
        .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
        .header(UserAgent(USER_AGENT.to_owned()));
    
    let res = try!(req.send());

    let token_body: serde_json::Value = match res.status {
        hyper::status::StatusCode::Ok => try!(serde_json::from_reader(res)),
        _ => return Err(From::from(std::io::Error::new(std::io::ErrorKind::Other, res.status.to_string())))
    };

    return Ok(
        Token {
            access_token: String::from(token_body["access_token"].as_str().unwrap()),
            expires_in: token_body["expires_in"].as_u64().unwrap(),
            token_type: String::from(token_body["token_type"].as_str().unwrap()),
            refresh_token: Some(String::from(token_body["refresh_token"].as_str().unwrap())),
        });
}

pub fn refresh_token(client_id: &str, client_secret: &str, refresh_token: &str) -> Result<Token, Error> {
    let refresh_body: String = form_urlencoded::Serializer::new(String::new())
        .append_pair("client_id", client_id)
        .append_pair("client_secret", client_secret)
        .append_pair("refresh_token", refresh_token)
        .append_pair("grant_type", "refresh_token")
        .finish();

    let client = try!(client());
    let req = client.post(TOKEN_URL)
        .body(&refresh_body)
        .header(ContentType("application/x-www-form-urlencoded".parse().unwrap()))
        .header(UserAgent(USER_AGENT.to_owned()));
    
    let res = try!(req.send());

    let refresh_body: serde_json::Value = match res.status {
        hyper::status::StatusCode::Ok => try!(serde_json::from_reader(res)),
        _ => return Err(From::from(std::io::Error::new(std::io::ErrorKind::Other, res.status.to_string())))
    };

    return Ok(
        Token {
            access_token: String::from(refresh_body["access_token"].as_str().unwrap()),
            expires_in: refresh_body["expires_in"].as_u64().unwrap(),
            token_type: String::from(refresh_body["token_type"].as_str().unwrap()),
            refresh_token: None,
        });
}

pub fn user_info(access_token: &str) -> Result<UserInfo, Error> {
    let client = try!(client());

    let info_req = client.get(INFO_URL)
        .header(Authorization(format!("Bearer {}", access_token)))
        .header(UserAgent(USER_AGENT.to_owned()));
    
    let info_res = try!(info_req.send());

    let info_body: serde_json::Value = match info_res.status {
        hyper::status::StatusCode::Ok => try!(serde_json::from_reader(info_res)),
        _ => return Err(From::from(std::io::Error::new(std::io::ErrorKind::Other, info_res.status.to_string())))
    };

    return Ok(UserInfo {
        id: String::from(info_body["id"].as_str().unwrap()),
        name: String::from(info_body["name"].as_str().unwrap()),
        given_name: String::from(info_body["given_name"].as_str().unwrap()),
        family_name: String::from(info_body["family_name"].as_str().unwrap()),
        picture: String::from(info_body["picture"].as_str().unwrap()),
        email: String::from(info_body["email"].as_str().unwrap()),
    });
}
