// use uuid::Uuid;
// use graphql_client::GraphQLQuery;

// use roboxmaker_graphql::{Subscribe, Request};

// type Timestamp = chrono::NaiveDateTime;


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Clone, Debug",
//     normalization = "rust"
// )]
// pub struct FilesByClassesId;
// impl Subscribe for FilesByClassesId {}


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Debug, Clone",
//     normalization = "rust"
// )]
// pub struct FilesGroupAdd;
// impl Request for FilesGroupAdd {}


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Debug, Clone",
//     normalization = "rust"
// )]
// pub struct FilesGroupDelete;
// impl Request for FilesGroupDelete {}


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Debug, Clone",
//     normalization = "rust"
// )]
// pub struct FilesGroupCreate;
// impl Request for FilesGroupCreate {}


// #[derive(GraphQLQuery, Clone)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Debug, Clone",
//     normalization = "rust"
// )]
// pub struct FilesById;
// impl Request for FilesById {}


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Clone, Debug",
//     normalization = "rust"
// )]
// pub struct FilesByAuthorId;
// impl Request for FilesByAuthorId {}


// // #[derive(GraphQLQuery)]
// // #[graphql(
// //     schema_path = "schema.graphql",
// //     query_path = "query.graphql",
// //     response_derives = "Debug",
// //     normalization = "rust"
// // )]
// // pub struct FilesGroupCreateMySpace;
// // impl Request for FilesGroupCreateMySpace {}


// #[derive(GraphQLQuery)]
// #[graphql(
//     schema_path = "schema.graphql",
//     query_path = "query.graphql",
//     response_derives = "Clone, Debug",
//     normalization = "rust"
// )]
// pub struct FilesTitleByClassesId;
// impl Request for FilesTitleByClassesId {}
