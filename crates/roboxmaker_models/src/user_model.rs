use uuid::Uuid;
use graphql_client::GraphQLQuery;
// use yew::services::fetch::{FetchService, FetchTask, Response};
// use reqwest::{self, header::HeaderMap, StatusCode};

use roboxmaker_graphql::{Subscribe, Request};

type Timestamp = chrono::NaiveDateTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "test.graphql",
    response_derives = "Clone, Debug, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct UserById;
impl Subscribe for UserById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct UsersByFullName;
impl Request for UsersByFullName {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct UserProfileByIdUpdate;
impl Request for UserProfileByIdUpdate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct MyContributionsAndComments;
impl Request for MyContributionsAndComments {}


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Clone, Debug",
//     normalization = "rust"
// )]
// pub struct RobotIdsByNames;

// type RobotIdsByNamesFetchResponse = Response<
//     Json<Result<graphql_client::Response<robot_ids_by_names::ResponseData>, anyhow::Error>>,
// >;

// pub fn fetch_robot_ids_by_names<C, F, M>(
//     vars: robot_ids_by_names::Variables,
//     response: F,
// ) -> Result<FetchTask, anyhow::Error>
// where
//     C: Component,
//     M: Into<C::Message>,
//     F: Fn(Vec<robot_ids_by_names::RobotIdsByNamesRobot>) -> M + 'static,
// {
//     let query = RobotIdsByNames::build_query(vars);
//     let post_request = auth::query_req(Json(&query));
//     let post_callback = link.callback(move |callback: RobotIdsByNamesFetchResponse| {
//         if let (_meta, Json(Ok(gq_response))) = callback.into_parts() {
//             if let Some(_errors) = gq_response.errors {
//                 response(vec![])
//             } else {
//                 let data: robot_ids_by_names::ResponseData =
//                     gq_response.data.expect("missing response data");
//                 let robots: Vec<robot_ids_by_names::RobotIdsByNamesRobot> = data.robot;
//                 response(robots)
//             }
//         } else {
//             response(vec![])
//         }
//     });
//     post_request
//         .and_then(|req| Some(FetchService::fetch(req, post_callback)))
//         .unwrap_or(Err(anyhow::Error::msg("Unable to build request")))
// }




// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Clone, Debug",
//     normalization = "rust"
// )]
// pub struct RobotIdsByNames;

// pub(crate) type UserRobotsFetchResponse = Response<Json<Result<Vec<String>, anyhow::Error>>>;

// pub async fn robots_by_user(
//     query: robot_ids_by_names::Variables,
//     user_id: String,
// ) -> Result<Option<robot_ids_by_names::ResponseData>, anyhow::Error> {
//     let url = "https://files.roboxmaker.com/file.php?file=*.bot.json";

//     let mut headers = HeaderMap::new();
//         headers.insert("Content-Type", "application/json".parse().unwrap());
//         headers.insert("aker-user-id", user_id.parse().unwrap());

//     let request_body = RobotIdsByNames::build_query(query);
//     let client = reqwest::Client::new();

//     let response = client
//         .get(url)
//         .headers(headers)
//         .json(&request_body)
//         .send()
//         .await?;
    
//     info!("{:?}", response);

//     let response_data = match response.status() {
//         StatusCode::OK => {
//             let response_body: Response<robot_ids_by_names::ResponseData> =
//                 response.json().await?;
//             info!("{:?}", response_body);
//             response_body.data
//         }
//         _ => None,
//     };

//     Ok(response_data)
// }
// pub async fn robots_by_user(
//     query: robot_ids_by_names::Variables,
//     user_id: String,
// ) -> Result<Option<robot_ids_by_names::ResponseData>, anyhow::Error> {
//     let url = "https://files.roboxmaker.com/file.php?file=*.bot.json";

//     let mut headers = HeaderMap::new();
//     headers.insert("Content-Type", "application/json".parse().unwrap());
//     headers.insert("x-hasura-role", "guest".parse().unwrap());
//     headers.insert("aker-user-id", user_id.parse().unwrap());

//     let request_body = RobotIdsByNames::build_query(query);
//     let client = reqwest::Client::new();

//     let response = client
//         .post(url)
//         .headers(headers)
//         .json(&request_body)
//         .send()
//         .await?;
    
//     info!("{:?}", response);

//     let response_data = match response.status() {
//         StatusCode::OK => {
//             let response_body: Response<robot_ids_by_names::ResponseData> =
//                 response.json().await?;
//             info!("{:?}", response_body);
//             response_body.data
//         }
//         _ => None,
//     };

//     Ok(response_data)
// }

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct MembersByGroupId; 
impl Subscribe for MembersByGroupId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct UsersListByGroup;
impl Subscribe for UsersListByGroup {}

