use uuid::Uuid;
use graphql_client::GraphQLQuery;
use roboxmaker_graphql::Request;

// type Date = chrono::NaiveDate;
// type Timestamp = chrono::NaiveDateTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "general.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct GroupsBySchoolId;
impl Request for GroupsBySchoolId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "general.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct GetSchools;
impl Request for GetSchools {}