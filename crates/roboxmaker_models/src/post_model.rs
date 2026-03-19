use uuid::Uuid;
use graphql_client::GraphQLQuery;
use roboxmaker_graphql::{Subscribe, Request};

type Timestamp = chrono::NaiveDateTime;

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct PostById;
impl Subscribe for PostById {}

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct InteractionsByGroupIdByPostId;
impl Subscribe for InteractionsByGroupIdByPostId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct PostByIdUpdate;
impl Request for PostByIdUpdate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct PostsByName;
impl Request for PostsByName {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct SearchByPostGradeByGroupId;
impl Request for SearchByPostGradeByGroupId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct PostGroupAdd;
impl Request for PostGroupAdd {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct UpdatePostGroupOptions;
impl Request for UpdatePostGroupOptions {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct PostByGroupId; 
impl Subscribe for PostByGroupId {}




// <------------------------ NEW QUERY------------------------> //


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "post_sub.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct GetPostList;
impl Subscribe for GetPostList {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "post_mut.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct PostClassAndGroupCreate;
impl Request for PostClassAndGroupCreate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "post_mut.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct PostGroupDelete;
impl Request for PostGroupDelete {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "post_mut.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct DeletePost;
impl Request for DeletePost {}