use uuid::Uuid;
use serde::Deserialize;
use graphql_client::GraphQLQuery;
use roboxmaker_graphql::{Subscribe, Request};

type Timestamp = chrono::NaiveDateTime;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct User {
    pub user_id: Uuid,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct MessagesByLessonGroup;
impl Subscribe for MessagesByLessonGroup {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct MessagesByPostGroup;
impl Subscribe for MessagesByPostGroup {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct MessagesByRobotGroup;
impl Subscribe for MessagesByRobotGroup {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]

pub struct MessageContentById;
impl Request for MessageContentById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct MessageLessonGroupCreate;
impl Request for MessageLessonGroupCreate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct MessagePostGroupCreate;
impl Request for MessagePostGroupCreate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct MessageRobotGroupCreate;
impl Request for MessageRobotGroupCreate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct DirectMessageGroupCreate;
impl Request for DirectMessageGroupCreate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct DeleteMessageById;
impl Request for DeleteMessageById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct UpdateMessageContentById;
impl Request for UpdateMessageContentById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct MessagesByDirectMessageGroup;
impl Subscribe for MessagesByDirectMessageGroup {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct DirectMessageGroupByGroupId;
impl Request for DirectMessageGroupByGroupId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct ContributionFilesByAuthorId;
impl Request for ContributionFilesByAuthorId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct MyMessagesWithReplies;
impl Request for MyMessagesWithReplies {}