use uuid::Uuid;
use serde::Deserialize;
use graphql_client::GraphQLQuery;
use roboxmaker_graphql::{Subscribe, Request};

type Timestamp = chrono::NaiveDateTime;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct User {
    pub user_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct LessonAuthor {
    pub user_id: Uuid,
    pub full_name: String,
    pub pic_path: String,
    pub user_staff: Option<User>,
    pub user_teacher: Option<User>,
    pub user_student: Option<User>,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct LessonProfile {
    pub lesson_id: Uuid,
    pub author: LessonAuthor,
}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone, PartialEq",
    normalization = "rust"
)]
pub struct LessonById;
impl Subscribe for LessonById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct LessonByIdUpdate;
impl Request for LessonByIdUpdate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct LessonsByName;
impl Request for LessonsByName {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct LessonGroupAdd;
impl Request for LessonGroupAdd {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct LessonGroupDelete;
impl Request for LessonGroupDelete {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct LessonGroupCreate;
impl Request for LessonGroupCreate {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct DeleteLessonById;
impl Request for DeleteLessonById {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Debug, Clone",
    normalization = "rust"
)]
pub struct UpdateLessonGroupOptions;
impl Request for UpdateLessonGroupOptions {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug",
    normalization = "rust"
)]
pub struct SearchByLessonGradeByGroupId;
impl Request for SearchByLessonGradeByGroupId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct LessonsListByGroup; 
impl Subscribe for LessonsListByGroup {}

#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct LessonByGroupId; 
impl Subscribe for LessonByGroupId {}


#[derive(GraphQLQuery)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "query.graphql",
    response_derives = "Clone, Debug, PartialEq",
    normalization = "rust"
)]
pub struct GetLessonType; 
impl Request for GetLessonType {}