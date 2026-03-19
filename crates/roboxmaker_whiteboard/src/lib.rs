pub mod whiteboard_view;

// use std::{fmt, str::FromStr};
// use uuid::Uuid;

// #[derive(Debug, Clone, Copy, PartialEq)]
// pub struct WhiteboardId(pub Uuid);

// impl FromStr for WhiteboardId {
//     type Err = uuid::Error;

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         Ok(WhiteboardId(Uuid::parse_str(s)?))
//     }
// }

// impl fmt::Display for WhiteboardId {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "{}", self.0)
//     }
// }