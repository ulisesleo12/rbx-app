use uuid::Uuid;
use graphql_client::GraphQLQuery;

use roboxmaker_graphql::{Subscribe, Request};

type Float8 = i64;
type Timestamp = chrono::NaiveDateTime;
type Date = chrono::NaiveDate;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct DeleteActivityById;
impl Request for DeleteActivityById {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct ActivityClassesGroupCreate;
impl Request for ActivityClassesGroupCreate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]

pub struct ActivityContentById;
impl Request for ActivityContentById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct ActivityByClassesGroup;
impl Subscribe for ActivityByClassesGroup {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct UpdateActivityContentById;
impl Request for UpdateActivityContentById {}
