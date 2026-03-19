use uuid::Uuid;
use log::info;
use yew::format::Json;
use serde::{Deserialize, Serialize};
use yew::{Component, ComponentLink};
use base64::{engine::general_purpose::{self}, Engine};
use yew::services::{fetch::{FetchService, FetchTask, Request, Response}, storage::{Area, StorageService}};

use roboxmaker_main::config;

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Auth {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
    pub id_token: String,
    pub session_state: String,
    pub scope: String,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Claims {
    #[serde(rename = "x-hasura-allowed-roles", default)]
    pub allowed_roles: Vec<String>,
    #[serde(rename = "x-hasura-default-role", default)]
    pub default_role: String,
    #[serde(rename = "x-hasura-school-id", default)]
    pub school_id: Uuid,
    #[serde(rename = "x-hasura-user-id", default)]
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Default, PartialEq, Deserialize, Serialize)]
pub struct Token {
    #[serde(rename = "https://hasura.io/jwt/claims", default)]
    pub claims: Claims,
    pub name: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
}

#[derive(Debug, Default, Serialize)]
pub struct AccessTokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub username: String,
    pub password: String,
    pub scope: String,
}

#[derive(Debug, Default, Serialize)]
pub struct RefreshTokenRequest {
    pub grant_type: String,
    pub client_id: String,
    pub refresh_token: String,
    pub scope: String,
}

pub enum TokenRequest {
    AccessToken(AccessTokenRequest),
    RefreshToken(RefreshTokenRequest),
}

fn struct_to_json(req: TokenRequest) -> Result<String, serde_urlencoded::ser::Error> {
    match req {
        TokenRequest::AccessToken(req) => serde_urlencoded::to_string(req),
        TokenRequest::RefreshToken(req) => serde_urlencoded::to_string(req),
    }
}
// fn struct_to_json(req: TokenRequest) -> Result<String, serde_json::Error> {
//     match req {
//         TokenRequest::AccessToken(req) => serde_json::to_string(&req),
//         TokenRequest::RefreshToken(req) => serde_json::to_string(&req),
//     }
// }

fn token_req(req: TokenRequest) -> Request<Result<String, anyhow::Error>> {
    let enc_body = Ok(struct_to_json(req).expect("bad urlencoded token request"));
    let url = format!(
        "{}/auth/realms/aker/protocol/openid-connect/token",
        config::AKER_AUTH_URL
    );
    let req = Request::post(url)
        .header("Content-Type", format!("application/x-www-form-urlencoded"))
        .body(enc_body)
        .expect("Failed to build request.");
    req
}

type AuthFetchResponse = Response<Json<Result<Auth, anyhow::Error>>>;

pub fn fetch_token<C, F, M>(
    link: &ComponentLink<C>,
    vars: TokenRequest,
    response: F,
) -> Result<FetchTask, anyhow::Error>
where
    C: Component,
    M: Into<C::Message>,
    F: Fn(Option<(Auth, Token)>) -> M + 'static,
{
    let post_request = token_req(vars);
    let post_callback = link.callback(move |callback: AuthFetchResponse| {
        let (_meta, Json(data)) = callback.into_parts();
        info!("token {:?}", data);
        // info!("MYTOKENDATA {:?}", data);
        let token: Option<Token> = data
            .as_ref()
            .ok()
            .map(|auth| {
                auth.access_token.split(".").collect::<Vec<_>>()
            })
            .unwrap_or_default()
            .iter()
            .skip(1)
            // .map(|b| base64::decode(b))
            .map(|b| Engine::decode(&general_purpose::STANDARD_NO_PAD, b))
            .filter_map(Result::ok)
            .map(|b| String::from_utf8(b))
            .filter_map(Result::ok)
            .map(|token| serde_json::from_str::<Token>(&token))
            .filter_map(Result::ok)
            .last();
        response(data.ok().zip(token))
    });
    FetchService::fetch(post_request, post_callback)
}


pub const AKER_TOKEN_KEY: &'static str = "app.aker.token";
pub const AKER_AUTH_KEY: &'static str = "app.aker.auth";

pub fn query_req<T>(query: Json<T>) -> Option<Request<Json<T>>> {
    let url = config::AKER_API_URL;
    StorageService::new(Area::Local)
        .ok()
        .and_then(|storage| {
            storage
                .restore::<Json<anyhow::Result<Auth>>>(AKER_AUTH_KEY)
                .0
                .ok()
        })
        .and_then(|auth| {
            Some(
                Request::post(url)
                    .header("Content-Type", "application/json")
                    .header("Authorization", format!("Bearer {}", auth.access_token))
                    .body(query)
                    .expect("Failed to build request."),
            )
        })
}


// type UserRobotsFetchResponse = Response<Json<Result<Vec<String>, anyhow::Error>>>;

// pub fn robots_vec(
//     link: ComponentLink<self>,
//     user_id: String,
// ) -> Result<FetchTask, anyhow::Error>{

//     let get_request = Request::get("https://files.roboxmaker.com/file.php?file=*.bot.json")
//         .header("aker-user-id", user_id)
//         .body(Nothing)
//         .expect("Unable to build request!");

//     let get_callback = link.callback(move |response: UserRobotsFetchResponse| {
//         let (_meta, Json(files)) = response.into_parts();
//         if let Ok(files) = files {
//             files
//         } else {
//             vec![]
//         }
//     });
//     FetchService::fetch(get_request, get_callback)
// }
