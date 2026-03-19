use log::*;
use yew::prelude::*;
// use yew::format::{Json, Nothing};
use yew::{html, Component, Html};
use crate::last_robots_card::UserStyle;
use crate::last_robots_list::RobotListByUser;
// use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use roboxmaker_main::lang;
// use roboxmaker_models::user_model;
use roboxmaker_types::types::{RobotId, UserId, MyUserProfile};

pub struct UserRobots {
    robots: Vec<RobotId>,
    // task_load: Option<FetchTask>,
    // list_robots: Vec<user_model::robot_ids_by_names::RobotIdsByNamesRobot>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct UserRobotsProperties {
    pub user_id: UserId,
    pub user_profile: Option<MyUserProfile>,
    #[prop_or(None)]
    pub on_list_change: Option<Callback<()>>,
    pub maybe_style: UserStyle,
}

#[derive(Debug)]
pub enum UserRobotsMessage {
    RobotNames(Vec<String>),
    // RobotIds(Vec<user_model::robot_ids_by_names::RobotIdsByNamesRobot>),
}

// pub(crate) type UserRobotsFetchResponse = Response<Json<Result<Vec<String>, anyhow::Error>>>;

impl Component for UserRobots {
    type Message = UserRobotsMessage;
    type Properties = UserRobotsProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        // let get_request = Request::get("https://files.roboxmaker.com/file.php?file=*.bot.json")
        //     .header("aker-user-id", props.user_id.0.to_string())
        //     .body(Nothing)
        //     .expect("Unable to build request!");

        // let get_callback = link.callback(move |response: UserRobotsFetchResponse| {
        //     let (_meta, Json(files)) = response.into_parts();
        //     if let Ok(files) = files {
        //         UserRobotsMessage::RobotNames(files)
        //     } else {
        //         UserRobotsMessage::RobotNames(vec![])
        //     }
        // });

        // let task = FetchService::fetch(get_request, get_callback);

        UserRobots {
            robots: vec![],
            // task_load: task.ok(),
            // list_robots: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            UserRobotsMessage::RobotNames(files) => {
                info!("ROBOTS: {:?}", files);
                let reg = regex::Regex::new(r"(^\d+-)(\w*)(\b.bot.json\b$)").unwrap();
                let robots: Vec<String> = files
                    .iter()
                    .filter_map(|file| {
                        let captures = reg.captures(file);
                        captures
                            .and_then(|capture| capture.get(2))
                            .and_then(|m| Some(m.as_str().to_string()))
                    })
                    .collect();
                info!("ROBOT NAMES: {:?}", robots);
                // let task = user_model::fetch_robot_ids_by_names(
                //     &self.link,
                //     user_model::robot_ids_by_names::Variables {
                //         robot_names: robots,
                //     },
                //     |response: Vec<user_model::robot_ids_by_names::RobotIdsByNamesRobot>| {
                //         UserRobotsMessage::RobotIds(response)
                //     },
                // );
                // self.task_load = task.ok();
            }
            // UserRobotsMessage::RobotIds(ids) => {
                // self.list_robots = ids.clone();
                // let ids: Vec<user_model::robot_ids_by_names::RobotIdsByNamesRobot> = ids.clone();
            //     self.robots = ids
            //         .iter()
            //         .map(|robot| RobotId(robot.id))
            //         .collect::<Vec<RobotId>>();
            // }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;
        
        if ctx.props() != old_props {
            should_render = true;
        }
        
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let maybe_no_robots = if self.robots.len() == 0 {
            html! { 
                <div>
                    <p>{lang::dict("Go build some robots!")}</p>
                </div> 
            }
        } else {
            html! {}
        };
        html! {
            <>
                // {list_robots}
                <RobotListByUser robots={self.robots.clone()} 
                    allow_edit=true 
                    // group_id=Some(group_id)
                    group_id={None}
                    user_profile={ctx.props().user_profile.clone()} 
                    user_id={Some(ctx.props().user_id.clone())}
                    maybe_style={ctx.props().maybe_style.clone()} />
                {maybe_no_robots}
            </>
        }
    }
}