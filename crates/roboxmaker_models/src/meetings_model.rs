use uuid::Uuid;
use graphql_client::GraphQLQuery;
use roboxmaker_graphql::{Subscribe, Request};

type Timestamp = chrono::NaiveDateTime;
type Date = chrono::NaiveDate;
type Time = chrono::NaiveTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug",
    normalization = "rust"
)]
pub struct MeetingsGroupCreate;
impl Request for MeetingsGroupCreate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct ListSchoolsMeets; 
impl Request for ListSchoolsMeets {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct SearchMeetingsAllSchools;
impl Request for SearchMeetingsAllSchools {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ClassGroupsBySchoolIdMeetigns;
impl Request for ClassGroupsBySchoolIdMeetigns {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct MeetingsByGroupId; 
impl Subscribe for MeetingsByGroupId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct ListScheduledMeetingsBySchoolId;
impl Subscribe for ListScheduledMeetingsBySchoolId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct DeleteMeetById;
impl Request for DeleteMeetById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct ListLastMeetingsBySchoolId;
impl Request for ListLastMeetingsBySchoolId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ClassNameAndMeetTitle;
impl Request for ClassNameAndMeetTitle {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct SearchMeetingsWhenCreate;
impl Request for SearchMeetingsWhenCreate {}