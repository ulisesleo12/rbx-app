use log::*;
use yew::prelude::*;
use yew::format::{Json, Nothing};
use crate::last_robots_card::UserStyle;
use crate::last_robots_list::RobotListByUser;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use yew::services::fetch::{FetchService, FetchTask, Request, Response};

use roboxmaker_main::lang;
use roboxmaker_models::user_model;
use roboxmaker_types::types::{RobotId, UserId, AppRoute, MyUserProfile};

pub struct UserRobots {
    link: ComponentLink<Self>,
    props: UserRobotsProperties,
    robots: Vec<RobotId>,
    task_load: Option<FetchTask>,
    list_robots: Vec<user_model::robot_ids_by_names::RobotIdsByNamesRobot>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct UserRobotsProperties {
    pub user_id: UserId,
    pub user_profile: Option<MyUserProfile>,
    pub on_app_route: Callback<AppRoute>,
    pub on_list_change: Option<Callback<()>>,
    pub maybe_style: UserStyle,
}

#[derive(Debug)]
pub enum UserRobotsMessage {
    RobotNames(Vec<String>),
    RobotIds(Vec<user_model::robot_ids_by_names::RobotIdsByNamesRobot>),
}

pub(crate) type UserRobotsFetchResponse = Response<Json<Result<Vec<String>, anyhow::Error>>>;

impl Component for UserRobots {
    type Message = UserRobotsMessage;
    type Properties = UserRobotsProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let get_request = Request::get("https://files.roboxmaker.com/file.php?file=*.bot.json")
            .header("aker-user-id", props.user_id.0.to_string())
            .body(Nothing)
            .expect("Unable to build request!");

        let get_callback = link.callback(move |response: UserRobotsFetchResponse| {
            let (_meta, Json(files)) = response.into_parts();
            if let Ok(files) = files {
                UserRobotsMessage::RobotNames(files)
            } else {
                UserRobotsMessage::RobotNames(vec![])
            }
        });

        let task = FetchService::fetch(get_request, get_callback);

        UserRobots {
            link,
            robots: vec![],
            props,
            task_load: task.ok(),
            list_robots: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_render = true;
        match msg {
            UserRobotsMessage::RobotNames(files) => {
                should_render = false;
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
                let task = user_model::fetch_robot_ids_by_names(
                    &self.link,
                    user_model::robot_ids_by_names::Variables {
                        robot_names: robots,
                    },
                    |response: Vec<user_model::robot_ids_by_names::RobotIdsByNamesRobot>| {
                        UserRobotsMessage::RobotIds(response)
                    },
                );
                self.task_load = task.ok();
            }
            UserRobotsMessage::RobotIds(ids) => {
                self.list_robots = ids.clone();
                let ids: Vec<user_model::robot_ids_by_names::RobotIdsByNamesRobot> = ids.clone();
                self.robots = ids
                    .iter()
                    .map(|robot| RobotId(robot.id))
                    .collect::<Vec<RobotId>>();
            }
        }
        should_render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        if self.props != props {
            self.props = props;
        }
        false
    }

    fn view(&self) -> Html {
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
                <RobotListByUser robots=self.robots.clone() 
                    allow_edit=true 
                    // group_id=Some(group_id)
                    group_id=None
                    user_profile=self.props.user_profile.clone() 
                    user_id=Some(self.props.user_id.clone()) 
                    on_app_route=self.props.on_app_route.clone()
                    on_list_change=None
                    maybe_style=self.props.maybe_style.clone() />
                {maybe_no_robots}
            </>
        }
    }
}