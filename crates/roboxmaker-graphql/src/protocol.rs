use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientPayload {
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "operationName")]
    pub operation_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payload<T = serde_json::Value, E = serde_json::Value> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<E>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    #[serde(rename = "connection_init")]
    ConnectionInit {
        #[serde(skip_serializing_if = "Option::is_none")]
        payload: Option<serde_json::Value>,
    },

    #[serde(rename = "pong")]
    Pong,

    #[serde(rename = "subscribe")]
    Subscribe { id: String, payload: ClientPayload },

    #[serde(rename = "complete")]
    Complete { id: String },

    #[serde(rename = "connection_terminate")]
    ConnectionTerminate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ServerMessage {
    #[serde(rename = "error")]
    ConnectionError { payload: serde_json::Value },

    #[serde(rename = "connection_ack")]
    ConnectionAck,

    #[serde(rename = "ping")]
    Ping,

    #[serde(rename = "next")]
    Next { id: String, payload: Payload },

    #[serde(rename = "error")]
    Error {
        id: String,
        payload: serde_json::Value,
    },

    #[serde(rename = "complete")]
    Complete { id: String },

    #[serde(rename = "ka")]
    ConnectionKeepAlive,
}

impl ServerMessage {
    pub fn id(&self) -> Option<&str> {
        match self {
            ServerMessage::Next { id, .. } => Some(&id),
            ServerMessage::Error { id, .. } => Some(&id),
            ServerMessage::Complete { id } => Some(&id),
            _ => None,
        }
    }
}


#[derive(Debug, Default, Deserialize, Serialize, Clone)]
pub struct Auth {
    pub access_token: String,
    pub expires_in: i64,
    pub refresh_expires_in: i64,
    pub refresh_token: String,
    pub token_type: String,
    pub id_token: String,
    pub session_state: String,
    pub scope: String,
}
