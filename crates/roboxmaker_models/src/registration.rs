use log::info;
use uuid::Uuid;
use graphql_client::*;
use reqwest::{self, header::HeaderMap, StatusCode};

use roboxmaker_main::config;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct SchoolIdByLicense;

pub async fn school_id_license(
    query: school_id_by_license::Variables,
    license: String,
) -> Result<Option<school_id_by_license::ResponseData>, anyhow::Error> {
    let url = config::AKER_API_URL;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-hasura-role", "guest".parse().unwrap());
    headers.insert("x-hasura-license", license.parse().unwrap());

    let request_body = SchoolIdByLicense::build_query(query);
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;
    
    info!("{:?}", response);

    let response_data = match response.status() {
        StatusCode::OK => {
            let response_body: Response<school_id_by_license::ResponseData> =
                response.json().await?;
            info!("{:?}", response_body);
            response_body.data
        }
        _ => None,
    };

    Ok(response_data)
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct CheckLicenseInformation;

pub async fn check_school_by_license(
    query: check_license_information::Variables,
    license: String,
) -> Result<Option<check_license_information::ResponseData>, anyhow::Error> {
    let url = config::AKER_API_URL;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-hasura-role", "guest".parse().unwrap());
    headers.insert("x-hasura-license", license.parse().unwrap());

    let request_body = CheckLicenseInformation::build_query(query);
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;
    
    info!("{:?}", response);

    let response_data = match response.status() {
        StatusCode::OK => {
            let response_body: Response<check_license_information::ResponseData> =
                response.json().await?;
            info!("{:?}", response_body);
            response_body.data
        }
        _ => None,
    };
    Ok(response_data)
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct GroupIdByClassName;

pub async fn class_name_group(
    query: group_id_by_class_name::Variables,
) -> Result<Option<group_id_by_class_name::ResponseData>, anyhow::Error> {
    let url = config::AKER_API_URL;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-hasura-role", "guest".parse().unwrap());

    let request_body = GroupIdByClassName::build_query(query);
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;
    
    info!("{:?}", response);

    let response_data = match response.status() {
        StatusCode::OK => {
            let response_body: Response<group_id_by_class_name::ResponseData> =
                response.json().await?;
            info!("{:?}", response_body);
            response_body.data
        }
        _ => None,
    };

    Ok(response_data)
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct InventoryGroupIdBySchoolId;

pub async fn inventory_group_id(
    query: inventory_group_id_by_school_id::Variables,
) -> Result<Option<inventory_group_id_by_school_id::ResponseData>, anyhow::Error> {
    let url = config::AKER_API_URL;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-hasura-role", "guest".parse().unwrap());

    let request_body = InventoryGroupIdBySchoolId::build_query(query);
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;
    
    info!("{:?}", response);

    let response_data = match response.status() {
        StatusCode::OK => {
            let response_body: Response<inventory_group_id_by_school_id::ResponseData> =
                response.json().await?;
            info!("{:?}", response_body);
            response_body.data
        }
        _ => None,
    };

    Ok(response_data)
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct NewUserCreate;

pub async fn new_user_add(
    query: new_user_create::Variables,
    license: String,
) -> Result<Option<new_user_create::ResponseData>, anyhow::Error> {
    let url = config::AKER_API_URL;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-hasura-role", "guest".parse().unwrap());
    headers.insert("x-hasura-license", license.parse().unwrap());

    let request_body = NewUserCreate::build_query(query);
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;
    
    info!("{:?}", response);

    let response_data = match response.status() {
        StatusCode::OK => {
            let response_body: Response<new_user_create::ResponseData> =
                response.json().await?;
            info!("{:?}", response_body);
            response_body.data
        }
        _ => None,
    };

    Ok(response_data)
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct CredentialsResetAction;

pub async fn credential_reset(
    query: credentials_reset_action::Variables,
) -> Result<Option<credentials_reset_action::ResponseData>, anyhow::Error> {
    let url = config::AKER_API_URL;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-hasura-role", "guest".parse().unwrap());

    let request_body = CredentialsResetAction::build_query(query);
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;
    
    info!("{:?}", response);

    let response_data = match response.status() {
        StatusCode::OK => {
            let response_body: Response<credentials_reset_action::ResponseData> =
                response.json().await?;
            info!("{:?}", response_body);
            response_body.data
        }
        _ => None,
    };

    Ok(response_data)
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct VerifyUserExist;

pub async fn user_exist(
    query: verify_user_exist::Variables,
) -> Result<Option<verify_user_exist::ResponseData>, anyhow::Error> {
    let url = config::AKER_API_URL;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-hasura-role", "guest".parse().unwrap());

    let request_body = VerifyUserExist::build_query(query);
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;
    
    info!("{:?}", response);

    let response_data = match response.status() {
        StatusCode::OK => {
            let response_body: Response<verify_user_exist::ResponseData> =
                response.json().await?;
            info!("{:?}", response_body);
            response_body.data
        }
        _ => None,
    };

    Ok(response_data)
}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct UserEmailIsVerified;

pub async fn email_is_verified(
    query: user_email_is_verified::Variables,
) -> Result<Option<user_email_is_verified::ResponseData>, anyhow::Error> {
    let url = config::AKER_API_URL;

    let mut headers = HeaderMap::new();
    headers.insert("Content-Type", "application/json".parse().unwrap());
    headers.insert("x-hasura-role", "guest".parse().unwrap());

    let request_body = UserEmailIsVerified::build_query(query);
    let client = reqwest::Client::new();

    let response = client
        .post(url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await?;
    
    info!("{:?}", response);

    let response_data = match response.status() {
        StatusCode::OK => {
            let response_body: Response<user_email_is_verified::ResponseData> =
                response.json().await?;
            info!("{:?}", response_body);
            response_body.data
        }
        _ => None,
    };

    Ok(response_data)
}
