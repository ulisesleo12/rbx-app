use super::protocol::*;
use roboxmaker_main::config::AKER_API_WSS_URL;
use code_location::CodeLocation;
use graphql_client::{GraphQLQuery, QueryBody};
use instant::Instant;
use gloo_storage::{LocalStorage, Storage};
use log::*;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::{
    rc::{Rc, Weak},
    sync::{Arc, Mutex},
};
use uuid::Uuid;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CloseEvent, ErrorEvent, MessageEvent, WebSocket};
use yew::{services::Task, Callback, Component, ComponentLink};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum Headers {
    #[serde(rename = "headers")]
    Auth {
        #[serde(rename = "Authorization")]
        #[serde(skip_serializing_if = "Option::is_none")]
        authorization: Option<String>,
    },
}

#[derive(Debug, Serialize)]
enum SubscriptionAction {
    Request,
    Subscribe,
}

#[derive(Debug, Serialize)]
struct SubscriptionInfo {
    operation: String,
    uuid: Uuid,
    message: ClientMessage,
    #[serde(skip_serializing)]
    callback: Callback<String>,
    action: SubscriptionAction,
}

struct GraphQLState {
    caller: CodeLocation,
    is_ready: bool,
    subs: HashMap<Uuid, Weak<SubscriptionInfo>>,
}

impl Drop for GraphQLState {
    fn drop(&mut self) {
        info!("GraphQLState: Dropped: {}", self.caller.file)
    }
}

struct WebSocketState {
    caller: CodeLocation,
    ws: Option<WebSocket>,
    ws_onmessage: Option<Closure<dyn FnMut(MessageEvent)>>,
    ws_onerror: Option<Closure<dyn FnMut(ErrorEvent)>>,
    ws_onclose: Option<Closure<dyn FnMut(CloseEvent)>>,
    ws_onopen: Option<Closure<dyn FnMut(JsValue)>>,
}

impl Drop for WebSocketState {
    fn drop(&mut self) {
        info!("WebSocketState: Dropped: {}", self.caller.file);
        if let Some(ws) = self.ws.as_mut() {
            ws.set_onmessage(None);
            ws.set_onerror(None);
            ws.set_onclose(None);
            ws.set_onopen(None);
            let _ = ws.close();
        }
    }
}

struct PingState {
    caller: CodeLocation,
    #[allow(dead_code)]
    on_keep_alive: Option<Closure<dyn FnMut(JsValue)>>,
    keep_alive_id: i32,
    last_ping: Instant,
}

impl Drop for PingState {
    fn drop(&mut self) {
        info!("PingState: Dropped: {}", self.caller.file);
        web_sys::window()
            .expect("Missing Window")
            .clear_interval_with_handle(self.keep_alive_id);
    }
}

pub struct GraphQLTask {
    caller: CodeLocation,
    is_active: bool,
    ws_state: Arc<Mutex<WebSocketState>>,
    graphql_state: Arc<Mutex<GraphQLState>>,
    ping_state: Arc<Mutex<PingState>>,
}

impl Task for GraphQLTask {
    fn is_active(&self) -> bool {
        self.is_active
    }
}

impl Drop for GraphQLTask {
    fn drop(&mut self) {
        info!("GraphQLTask: Dropped: {}", self.caller.file);
        if let Ok(ping_state) = self.ping_state.lock().as_mut() {
            ping_state.on_keep_alive = None;
            web_sys::window()
                .expect("Missing Window")
                .clear_interval_with_handle(ping_state.keep_alive_id);
        }
    }
}

impl GraphQLTask {
    fn add_subscription(&self, sub_info: &Rc<SubscriptionInfo>) {
        let uuid = sub_info.uuid.clone();
        let weak_sub = Rc::downgrade(&sub_info);
        if let Ok(graphql_state) = self.graphql_state.lock().as_mut() {
            graphql_state.subs.insert(uuid, weak_sub);
        }
    }

    fn setup(
        ws_state: Arc<Mutex<WebSocketState>>,
        graphql_state: Arc<Mutex<GraphQLState>>,
        ping_state: Arc<Mutex<PingState>>,
    ) {
        if let Ok(ws_state) = ws_state.lock().as_mut() {
            if let Some(ws) = &ws_state.ws {
                let _ = ws.close();
            }
        }

        let ws = WebSocket::new_with_str(&AKER_API_WSS_URL, "graphql-transport-ws").ok();

        if let Some(ws) = ws {
            if let Ok(ws_state) = ws_state.lock().as_mut() {
                ws_state.ws = Some(ws.clone());
            }

            // OnMessage
            let cloned_ws = ws.clone();
            let cloned_graphql_state = graphql_state.clone();
            let ws_onmessage: Closure<dyn FnMut(MessageEvent)> =
                Closure::wrap(Box::new(move |e: MessageEvent| {
                    if let Ok(abuf) = e.data().dyn_into::<js_sys::ArrayBuffer>() {
                        warn!("GraphQLTask: OnMessage, received arraybuffer: {:?}", abuf);
                    } else if let Ok(blob) = e.data().dyn_into::<web_sys::Blob>() {
                        warn!("GraphQLTask: OnMessage, received blob: {:?}", blob);
                    } else if let Ok(txt) = e.data().dyn_into::<js_sys::JsString>() {
                        let msg = serde_json::from_str::<ServerMessage>(
                            txt.as_string().unwrap_or_default().as_str(),
                        );
                        match msg {
                            Ok(msg) => match msg {
                                ServerMessage::ConnectionAck => {
                                    info!("GraphQLTask: ConnectionAck [Handled]");
                                    if let Ok(graphql_state) = cloned_graphql_state.lock().as_mut()
                                    {
                                        graphql_state.is_ready = true;
                                    }
                                    if let Ok(graphql_state) = cloned_graphql_state.lock() {
                                        for (_, sub_info) in graphql_state.subs.iter() {
                                            if let Some(sub_info) = sub_info.upgrade() {
                                                GraphQLTask::subscribe(&cloned_ws, &sub_info);
                                            }
                                        }
                                    }
                                }
                                ServerMessage::Ping => {
                                    debug!("GraphQLTask: Ping [Handled]");
                                    if let Ok(ping_state) = ping_state.lock().as_mut() {
                                        let now = Instant::now();
                                        if now - ping_state.last_ping
                                            > core::time::Duration::from_secs(1)
                                        {
                                            let payload = ClientMessage::Pong {};
                                            let payload =
                                                serde_json::to_string(&payload).unwrap_or_default();
                                            let res = cloned_ws.send_with_str(&payload);
                                            if let Err(err) = res {
                                                error!("GraphQLTask: {:?}", err);
                                            }
                                        }
                                        ping_state.last_ping = now;
                                    }
                                    // TODO: Pong is happening too fast, disabling for now
                                    // let payload = ClientMessage::Pong {};
                                    // let payload =
                                    //     serde_json::to_string(&payload).unwrap_or_default();
                                    // let res = cloned_ws.send_with_str(&payload);
                                    // if let Err(err) = res {
                                    //     error!("GraphQLTask: {:?}", err);
                                    // }
                                }
                                ServerMessage::Next { id, payload } => {
                                    if let Ok(id) = Uuid::parse_str(&*id) {
                                        let mut callback = None;
                                        if let Ok(graphql_state) = cloned_graphql_state.lock() {
                                            if let Some(sub_info) = graphql_state.subs.get(&id) {
                                                if let Some(sub_info) = sub_info.upgrade() {
                                                    info!(
                                                        "GraphQLTask: Next [Handled] {} {:?}",
                                                        sub_info.operation, id
                                                    );
                                                    info!(
                                                        "{}",
                                                        serde_json::to_string(&payload)
                                                            .unwrap_or_default()
                                                    );
                                                    callback = Some(sub_info.callback.clone());
                                                }
                                            }
                                        }
                                        if let Some(callback) = callback {
                                            if let Some(data) = payload.data {
                                                callback.emit(data.to_string());
                                            }
                                        }
                                    }
                                }
                                ServerMessage::Complete { id } => {
                                    if let Ok(id) = Uuid::parse_str(&*id) {
                                        if let Ok(graphql_state) =
                                            cloned_graphql_state.lock().as_mut()
                                        {
                                            if let Some(sub_info) = graphql_state.subs.get(&id) {
                                                if let Some(sub_info) = sub_info.upgrade() {
                                                    info!(
                                                        "GraphQLTask: Complete [Handled] {} {:?}",
                                                        sub_info.operation, id
                                                    );
                                                }
                                            }
                                            graphql_state.subs.remove(&id);
                                        }
                                    }
                                }
                                _ => {
                                    warn!("GraphQLTask: {:?} [NOT Handled]", msg);
                                }
                            },
                            Err(err) => {
                                warn!("GraphQLTask: {:?}", err)
                            }
                        };
                    } else {
                        warn!("GraphQLTask: OnMessage, received Unknown: {:?}", e.data());
                    }
                }) as Box<dyn FnMut(MessageEvent)>);
            ws.set_onmessage(Some(ws_onmessage.as_ref().unchecked_ref()));
            if let Ok(ws_state) = ws_state.lock().as_mut() {
                ws_state.ws_onmessage = Some(ws_onmessage);
            }

            // OnError
            let ws_onerror: Closure<dyn FnMut(ErrorEvent)> =
                Closure::wrap(Box::new(move |e: ErrorEvent| {
                    error!("GraphQLTask: OnError: {:?}", e);
                }) as Box<dyn FnMut(ErrorEvent)>);
            ws.set_onerror(Some(ws_onerror.as_ref().unchecked_ref()));
            if let Ok(ws_state) = ws_state.lock().as_mut() {
                ws_state.ws_onerror = Some(ws_onerror);
            }

            // OnClose
            let cloned_graphql_state = graphql_state.clone();
            let ws_onclose: Closure<dyn FnMut(CloseEvent)> =
                Closure::wrap(Box::new(move |e: CloseEvent| {
                    info!("GraphQLTask: OnClose: {:?}", e);
                    if let Ok(graphql_state) = cloned_graphql_state.lock().as_mut() {
                        graphql_state.is_ready = false;
                    }
                }) as Box<dyn FnMut(CloseEvent)>);
            ws.set_onclose(Some(ws_onclose.as_ref().unchecked_ref()));
            if let Ok(ws_state) = ws_state.lock().as_mut() {
                ws_state.ws_onclose = Some(ws_onclose);
            }

            // OnOpen
            let cloned_ws = ws.clone();
            let ws_onopen: Closure<dyn FnMut(JsValue)> = Closure::wrap(Box::new(move |_| {
                info!("GraphQLTask: OnOpen");
                let auth: Option<Auth> = LocalStorage::get("app.aker.auth").ok();

                let access_token: Option<String> = auth.and_then(|data| Some("Bearer ".to_owned() + &data.access_token));

                let msg = Headers::Auth {
                    authorization: access_token,
                };
                let headers = serde_json::to_value(&msg).ok();
                let payload = ClientMessage::ConnectionInit { payload: headers };
                let payload = serde_json::to_string(&payload).unwrap_or_default();
                let res = cloned_ws.send_with_str(&payload);
                if res.is_ok() {
                    info!("{}", payload);
                } else {
                    error!("{}", payload);
                }
            })
                as Box<dyn FnMut(JsValue)>);
            ws.set_onopen(Some(ws_onopen.as_ref().unchecked_ref()));
            if let Ok(ws_state) = ws_state.lock().as_mut() {
                ws_state.ws_onopen = Some(ws_onopen);
            };
        }
    }

    fn subscribe(ws: &WebSocket, sub_info: &SubscriptionInfo) {
        let msg_str = serde_json::to_string(&sub_info.message);
        match msg_str {
            Ok(msg_str) => {
                info!(
                    "GraphQLTask: {:?}: {} {:?}",
                    sub_info.action, sub_info.operation, sub_info.uuid
                );
                info!("{}", serde_json::to_string(sub_info).unwrap_or_default());
                let res = ws.send_with_str(&msg_str);
                if let Err(err) = res {
                    error!("GraphQLTask: {:?}", err);
                }
            }
            Err(err) => {
                error!("GraphQLTask: {:?}", err);
            }
        };
    }
}

pub struct SubscriptionTask {
    sub_info: Rc<SubscriptionInfo>,
    ws_state: Arc<Mutex<WebSocketState>>,
    graphql_state: Arc<Mutex<GraphQLState>>,
}

impl Task for SubscriptionTask {
    fn is_active(&self) -> bool {
        true
    }
}

impl Drop for SubscriptionTask {
    fn drop(&mut self) {
        let id = self.sub_info.uuid.to_string();
        let msg = ClientMessage::Complete { id: id.clone() };
        let msg = serde_json::to_string(&msg).unwrap();

        info!(
            "SubscriptionTask: Dropped: {:?} {:?}",
            self.sub_info.operation, self.sub_info.uuid
        );
        info!(
            "{}",
            serde_json::to_string(self.sub_info.as_ref()).unwrap_or_default()
        );

        if let Ok(graphql_state) = self.graphql_state.lock().as_mut() {
            graphql_state.subs.remove(&self.sub_info.uuid);
        }

        if let Ok(ws_state) = self.ws_state.lock() {
            match &ws_state.ws {
                Some(ws) => {
                    let res = ws.send_with_str(&msg);
                    if let Err(err) = res {
                        error!("SubscriptionTask: {:?}", err);
                    }
                }
                None => {
                    warn!("SubscriptionTask: Invalid WebSocket");
                }
            }
        }
    }
}

pub trait Subscribe {
    fn subscribe<C, M, F>(
        graphql_task: &GraphQLTask,
        link: &ComponentLink<C>,
        vars: Self::Variables,
        on_response: F,
    ) -> SubscriptionTask
    where
        Self: GraphQLQuery,
        C: Component,
        M: Into<C::Message>,
        F: Fn(Option<Self::ResponseData>) -> M + 'static,
    {
        let uuid = Uuid::new_v4();
        let vars_value = serde_json::to_value(&vars).ok();
        let query: QueryBody<Self::Variables> = Self::build_query(vars);
        let callback: Callback<String> = link.callback(move |json: String| {
            let data: Result<Self::ResponseData, serde_json::Error> = serde_json::from_str(&json);
            on_response(data.ok())
        });

        
        let payload = ClientMessage::Subscribe {
            id: uuid.to_string(),
            payload: ClientPayload {
                variables: vars_value,
                query: query.query.to_string(),
                operation_name: Some(query.operation_name.to_string()),
            },
        };

        let sub_info = SubscriptionInfo {
            operation: query.operation_name.to_string(),
            uuid,
            message: payload,
            callback,
            action: SubscriptionAction::Subscribe,
        };
        let sub_info = Rc::new(sub_info);
        graphql_task.add_subscription(&sub_info);

        if let Ok(graphl_state) = graphql_task.graphql_state.lock() {
            if graphl_state.is_ready {
                if let Ok(ws_state) = &graphql_task.ws_state.lock() {
                    if let Some(ws) = &ws_state.ws {
                        GraphQLTask::subscribe(ws, &sub_info);
                    }
                }
            }
        }

        SubscriptionTask {
            sub_info,
            ws_state: graphql_task.ws_state.clone(),
            graphql_state: graphql_task.graphql_state.clone(),
        }
    }
}

pub struct RequestTask {
    sub_info: Rc<SubscriptionInfo>,
    ws_state: Arc<Mutex<WebSocketState>>,
    graphql_state: Arc<Mutex<GraphQLState>>,
}

impl Task for RequestTask {
    fn is_active(&self) -> bool {
        true
    }
}

impl Drop for RequestTask {
    fn drop(&mut self) {
        let id = self.sub_info.uuid.to_string();
        let msg = ClientMessage::Complete { id: id.clone() };
        let msg = serde_json::to_string(&msg).unwrap();

        info!(
            "RequestTask: Dropped: {:?} {:?}",
            self.sub_info.operation, self.sub_info.uuid
        );
        info!(
            "{}",
            serde_json::to_string(self.sub_info.as_ref()).unwrap_or_default()
        );

        if let Ok(graphql_state) = self.graphql_state.lock().as_mut() {
            graphql_state.subs.remove(&self.sub_info.uuid);
        }

        if let Ok(ws_state) = self.ws_state.lock() {
            match &ws_state.ws {
                Some(ws) => {
                    let res = ws.send_with_str(&msg);
                    if let Err(err) = res {
                        error!("RequestTask: {:?}", err);
                    }
                }
                None => {
                    warn!("RequestTask: Invalid WebSocket");
                }
            }
        } else {
            warn!("RequestTask: Invalid WebSocket");
        }
    }
}

pub trait Request {
    fn request<C, M, F>(
        graphql_task: &GraphQLTask,
        link: &ComponentLink<C>,
        vars: Self::Variables,
        on_response: F,
    ) -> RequestTask
    where
        Self: GraphQLQuery,
        C: Component,
        M: Into<C::Message>,
        F: Fn(Option<Self::ResponseData>) -> M + 'static,
    {
        let uuid = Uuid::new_v4();
        let vars_value = serde_json::to_value(&vars).ok();
        let query: QueryBody<Self::Variables> = Self::build_query(vars);
        let callback: Callback<String> = link.callback(move |json: String| {
            let data: Result<Self::ResponseData, serde_json::Error> = serde_json::from_str(&json);
            on_response(data.ok())
        });

        let payload = ClientMessage::Subscribe {
            id: uuid.to_string(),
            payload: ClientPayload {
                variables: vars_value,
                query: query.query.to_owned(),
                operation_name: Some(query.operation_name.to_string()),
            },
        };

        let sub_info = SubscriptionInfo {
            operation: query.operation_name.to_string(),
            uuid,
            message: payload,
            callback,
            action: SubscriptionAction::Request,
        };
        let sub_info = Rc::new(sub_info);
        graphql_task.add_subscription(&sub_info);

        if let Ok(graphl_state) = graphql_task.graphql_state.lock() {
            if graphl_state.is_ready {
                if let Ok(ws_state) = &graphql_task.ws_state.lock() {
                    if let Some(ws) = &ws_state.ws {
                        GraphQLTask::subscribe(ws, &sub_info);
                    }
                }
            }
        }

        RequestTask {
            sub_info,
            ws_state: graphql_task.ws_state.clone(),
            graphql_state: graphql_task.graphql_state.clone(),
        }
    }
}

pub struct GraphQLService {}

impl GraphQLService {
    pub fn connect(caller: &CodeLocation) -> GraphQLTask {
        info!("GraphQLService: Connect: {}", caller.file);

        let ws_state = Arc::new(Mutex::new(WebSocketState {
            caller: caller.clone(),
            ws: None,
            ws_onmessage: None,
            ws_onerror: None,
            ws_onclose: None,
            ws_onopen: None,
        }));

        let graphql_state = Arc::new(Mutex::new(GraphQLState {
            caller: caller.clone(),
            is_ready: false,
            subs: HashMap::new(),
        }));

        let ping_state = Arc::new(Mutex::new(PingState {
            caller: caller.clone(),
            on_keep_alive: None,
            keep_alive_id: 0,
            last_ping: Instant::now(),
        }));

        let on_keep_alive: Closure<dyn FnMut(JsValue)> = {
            let ws_state = ws_state.clone();
            let graphql_state = graphql_state.clone();
            let ping_state = ping_state.clone();

            Closure::wrap(Box::new(move |_| {
                debug!("GraphQLTask: KeepAlive");

                let ping_duration = {
                    let mut last_ping = Instant::now();
                    if let Ok(ping_state) = ping_state.lock() {
                        last_ping = ping_state.last_ping;
                    }
                    last_ping.elapsed()
                };

                // if let Ok(ws_state_two) = ws_state.lock() {
                //     if let Some(ws) = ws_state_two.ws.clone() { 
                //         let state = ws.ready_state();

                //         if state == 0 {
                //             info!("WS state: CONNECTING")
                //         }
                //         if state == 1 {
                //             info!("WS state: OPEN")
                //         }
                //         if state == 2 {
                //             warn!("WS state: CLOSING")
                //         }
                //         if state == 3 {
                //             error!("WS state: CLOSED");
                //         }
                //     }
                // }

                // if let Ok(graphql_state) = graphql_state.lock() {
                //     let graphql_st =  format!("{:p}", &graphql_state);
                //     info!("Subs {:?}  state: {}", graphql_state.subs, graphql_st);
                // }

                if ping_duration > core::time::Duration::from_secs(10) {
                    warn!(
                        "GraphQLTask: KeepAlive [DEAD] ping: {}",
                        ping_duration.as_millis()
                    );
                    GraphQLTask::setup(ws_state.clone(), graphql_state.clone(), ping_state.clone());
                } else {
                    debug!(
                        "GraphQLTask: KeepAlive [ALIVE] ping: {}",
                        ping_duration.as_millis()
                    );
                }
            }) as Box<dyn FnMut(JsValue)>)
        };

        let keep_alive_id = web_sys::window()
        .expect("Missing Window")
        .set_interval_with_callback_and_timeout_and_arguments(
            on_keep_alive.as_ref().unchecked_ref(),
            3500,
            &js_sys::Array::new(),
        )
        .unwrap();
        
        if let Ok(ping_state) = ping_state.lock().as_mut() {
            ping_state.on_keep_alive = Some(on_keep_alive);
            ping_state.keep_alive_id = keep_alive_id;
        };

        let graphl_task = GraphQLTask {
            caller: caller.clone(),
            is_active: true,
            ws_state: ws_state.clone(),
            graphql_state: graphql_state.clone(),
            ping_state: ping_state.clone(),
        };

        GraphQLTask::setup(ws_state, graphql_state, ping_state);

        graphl_task
    }
}
