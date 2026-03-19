use uuid::Uuid;
use graphql_client::GraphQLQuery;
use roboxmaker_graphql::{Request, Subscribe};

type Date = chrono::NaiveDate;
type Timestamp = chrono::NaiveDateTime;
type Time = chrono::NaiveTime;

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct GradesByName;
impl Request for GradesByName {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct SearchByUniversalGradeByGroupId;
impl Request for SearchByUniversalGradeByGroupId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct ListOfGradesOfSchoolById;
impl Request for ListOfGradesOfSchoolById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct ListOfGradesOfUserById;
impl Request for ListOfGradesOfUserById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct GroupsBySchoolIdListClass;
impl Request for GroupsBySchoolIdListClass {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct NameOfDegreeById;
impl Request for NameOfDegreeById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct DegreeContentById;
impl Request for DegreeContentById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ContentDegreeByUserId;
impl Request for ContentDegreeByUserId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct DeleteClassGroupById;
impl Request for DeleteClassGroupById {}


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Debug, Clone, PartialEq, Serialize",
//     normalization = "rust"
// )]
// pub struct AddUserTeacher;
// impl Request for AddUserTeacher {}


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Debug, Clone, PartialEq, Serialize",
//     normalization = "rust"
// )]
// pub struct UserStaffAdd;
// impl Request for UserStaffAdd {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "test.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct HomeDataByGroupId;
impl Subscribe for HomeDataByGroupId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq, Serialize",
    normalization = "rust"
)]
pub struct GetClassGroupByGroupId;
impl Subscribe for GetClassGroupByGroupId {}
