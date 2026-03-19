use uuid::Uuid;
use graphql_client::GraphQLQuery;

use roboxmaker_graphql::{Request};

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct GroupMemberAdd;
impl Request for GroupMemberAdd {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct GroupMemberDelete;
impl Request for GroupMemberDelete {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct GroupCreateWithMembers;
impl Request for GroupCreateWithMembers {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct ClassGroupByUserId; 
impl Request for ClassGroupByUserId {}