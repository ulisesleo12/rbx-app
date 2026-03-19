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
pub struct ClassesById;
impl Request for ClassesById {}


#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ClassNameClasses; 
impl Request for ClassNameClasses {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ActivityProfileAggregate;
impl Request for ActivityProfileAggregate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ClassesByIdUpdate;
impl Request for ClassesByIdUpdate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct ClassesByName;
impl Request for ClassesByName {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct SearchByClassesGradeByGroupId;
impl Request for SearchByClassesGradeByGroupId {}


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Debug",
//     normalization = "rust"
// )]
// pub struct DeleteClassesById;
// impl Request for DeleteClassesById {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct UpdateClassesGroupOptions;
impl Request for UpdateClassesGroupOptions {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct DateActivityClassesById;
impl Request for DateActivityClassesById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct ClassesByGroupId; 
impl Subscribe for ClassesByGroupId {}


// <------------------------ NEW QUERY------------------------> //

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "classes_sub.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct GetClassesList; 
impl Subscribe for GetClassesList {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "classes_mut.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ClassesClassAndGroupAdd;
impl Request for ClassesClassAndGroupAdd {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "classes_mut.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ClassesGroupDelete;
impl Request for ClassesGroupDelete {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "classes_mut.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct ClassesClassAndGroupCreate;
impl Request for ClassesClassAndGroupCreate {}