use uuid::Uuid;
use graphql_client::GraphQLQuery;
use roboxmaker_graphql::{Subscribe, Request};

type Timestamp = chrono::NaiveDateTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct RobotById;
impl Request for RobotById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct RobotsByName;
impl Request for RobotsByName {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct SearchByRobotGradeByGroupId;
impl Request for SearchByRobotGradeByGroupId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct UpdateRobotGroupEnabled;
impl Request for UpdateRobotGroupEnabled {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct RobotsByGroupId; 
impl Subscribe for RobotsByGroupId {}



// <------------------------ NEW QUERY------------------------> //

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "robot_sub.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct GetRobotList; 
impl Subscribe for GetRobotList {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "robot_mut.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct RobotGroupAdd;
impl Request for RobotGroupAdd {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "robot_mut.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct RobotGroupDelete;
impl Request for RobotGroupDelete {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "robot_mut.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct UpdateRobotType;
impl Request for UpdateRobotType {}