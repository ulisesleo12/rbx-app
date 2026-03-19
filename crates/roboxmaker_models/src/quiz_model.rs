// use serde::Serialize;
use uuid::Uuid;
use graphql_client::GraphQLQuery;
use roboxmaker_graphql::{Request, Subscribe};

type Float8 = i64;
type Timestamp = chrono::NaiveDateTime;
type Date = chrono::NaiveDate;
type Time = chrono::NaiveTime;

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct NewQuiz; 
impl Request for NewQuiz {}


#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct UpdateInsetSection; 
impl Request for UpdateInsetSection {}


#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct UpsetQuestionAnswerOptions; 
impl Request for UpsetQuestionAnswerOptions {}


#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct UpdateQuiz; 
impl Request for UpdateQuiz {}


#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct DeleteQuestionSectionById; 
impl Request for DeleteQuestionSectionById {}

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct DeleteQuestionById; 
impl Request for DeleteQuestionById {}

#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct DeleteOptionById; 
impl Request for DeleteOptionById {}


#[derive(GraphQLQuery, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct UpsertQuizResponse; 
impl Request for UpsertQuizResponse {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct SubmitAnswers; 
impl Request for SubmitAnswers {}

#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct SubmitSingleAnswer; 
impl Request for SubmitSingleAnswer {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct SubmitMultiAnswer; 
impl Request for SubmitMultiAnswer {}

#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct DeleteQuizById; 
impl Request for DeleteQuizById {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct QuizzesListByGroup; 
impl Subscribe for QuizzesListByGroup {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct QuizGroupDelete; 
impl Request for QuizGroupDelete {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct QuizToGroupAdd; 
impl Request for QuizToGroupAdd {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct UpdateQuizGroupOptions; 
impl Request for UpdateQuizGroupOptions {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct QuizById; 
impl Subscribe for QuizById {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct QuizByName; 
impl Request for QuizByName {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct QuizzesByGroupId; 
impl Subscribe for QuizzesByGroupId {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct GetQuizzes; 
impl Request for GetQuizzes {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct UsersByGroupId; 
impl Request for UsersByGroupId {}


// #[derive(GraphQLQuery, Debug, Clone)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "quiz.graphql",
//     response_derives = "Debug, Clone, Default",
//     normalization = "rust"
// )]
// pub struct QuizWithUserAnswersByGroup; 
// impl Request for QuizWithUserAnswersByGroup {}


#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct QuizWithUserAnswersByUser; 
impl Subscribe for QuizWithUserAnswersByUser {}

#[derive(GraphQLQuery, Debug, Clone)]
#[graphql(
    schema_path = "schema.graphql",
    query_path = "quiz.graphql",
    response_derives = "Debug, Clone, Default",
    normalization = "rust"
)]
pub struct UpdateUserAnswer; 
impl Request for UpdateUserAnswer {}