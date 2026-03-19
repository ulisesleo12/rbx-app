use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use crate::robots_card::RobotsCard;
use crate::robot_select::RobotSelect;
use crate::robot_select::RobotSelectOption;
use yew_router::scope_ext::RouterScopeExt;
use yew::{html, Component, Html, Properties};

use roboxmaker_main::lang;
use roboxmaker_models::robot_model;
use roboxmaker_searches::search_robots_group::SearchRobotdGroup;
use roboxmaker_models::robot_model::{get_robot_list, robot_group_add};
use roboxmaker_types::types::{RobotId, GroupId, UserId, AppRoute, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};

#[derive(Debug, Clone, PartialEq)]
pub struct RobotProfile {
    pub name: String,
    pub path: String,
    pub robot_id: RobotId,
    pub enabled: bool,
    pub robot_type: get_robot_list::RoboxRobotTypeEnum,
}

pub struct RobotsList {
    graphql_task: Option<GraphQLTask>,
    robot_sub: Option<SubscriptionTask>,
    robot_delete_task: Option<RequestTask>,
    robot_add_task: Option<RequestTask>,
    show_dropdown_filter: bool,
    filter: RobotFilter,
    robot_list: Vec<RobotProfile>,
    robot_list_view: Vec<RobotProfile>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RobotFilter {
    Alls,
    Enabled,
    Disabled,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RobotsListProperties {
    pub group_id: GroupId,
    pub user_profile: Option<MyUserProfile>,
    pub user_id: Option<UserId>,
    pub class_name: String,
}

#[derive(Debug)]
pub enum RobotsListMessage {
    FetchRobotsByGroupId,
    Robots(Option<robot_model::get_robot_list::ResponseData>),
    AddRobot(RobotId),
    RemoveRobot(RobotId),
    RobotAdded(Option<RobotId>),
    RobotRemoved(Option<RobotId>),
    ShowDropdown,
    ChangeFilter(RobotFilter),
    UpdateRobotIdList(RobotId, bool),
}

impl Component for RobotsList {
    type Message = RobotsListMessage;
    type Properties = RobotsListProperties;

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(RobotsListMessage::FetchRobotsByGroupId);
        
        RobotsList {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            robot_sub: None,
            robot_delete_task: None,
            robot_add_task: None,
            show_dropdown_filter: false,
            filter: RobotFilter::Alls,
            robot_list: vec![],
            robot_list_view: vec![],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            RobotsListMessage::FetchRobotsByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let group_id = ctx.props().group_id;

                    let vars = robot_model::get_robot_list::Variables {
                        group_id: group_id.0,
                    };

                    let task = robot_model::GetRobotList::subscribe(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                RobotsListMessage::Robots(response)
                            },
                    );
                    self.robot_sub = Some(task);
                }
            }
            RobotsListMessage::Robots(response) => {
                info!("RESPONSE-ROBOTS {:?}", response.clone());

                if let Some(class_group) = response.clone().and_then(|data| Some(data.class_group)) {
                
                    for class_robot in class_group.iter() {
                        let main_section_id = class_robot.class_profile.clone().and_then(|class| Some(class.section_id)).unwrap_or(Uuid::default());
                        // let section_id = class_robot.class_profile.clone().and_then(|data| data.class_robot);

                        let robots = class_robot.class_profile.clone().and_then(|data| Some(data.class_robot)).unwrap_or(vec![]);
                        let robot_group = class_robot.robot_groups.clone();

                        let robot_list = robots.iter().map(|item| {

                            // let robot_type = item.robot_profile.robot_group.clone().and_then(|data| data.robot_type).unwrap_or(get_robot_list::RoboxRobotTypeEnum::Different);
                            let robot_type = if main_section_id == item.section_id { true } else { false };
                            RobotProfile { 
                                name: item.robot_profile.name.clone(), 
                                path: item.robot_profile.path.clone(), 
                                robot_id: RobotId(item.robot_profile.robot_id.clone()), 
                                enabled: false,
                                robot_type: get_robot_list::RoboxRobotTypeEnum::Different,
                            }
                        }).collect();

                        self.robot_list = robot_list;

                        for robot_list in self.robot_list.iter_mut() {
                            for item in robot_group.iter() {
                                if robot_list.robot_id.0 == item.robot_id {
                                    robot_list.enabled = item.enabled;
                                    robot_list.robot_type = item.robot_type.clone().unwrap_or(get_robot_list::RoboxRobotTypeEnum::Different);
                                }
                            }
                        }
                    }
                }

                ctx.link().send_message(RobotsListMessage::ChangeFilter(self.filter.clone()))
            }
            RobotsListMessage::AddRobot(robot_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                        
                    let vars = robot_model::robot_group_add::Variables { 
                        group_id: ctx.props().group_id.0,
                        robot_id: robot_id.0,
                        robot_type: robot_group_add::RoboxRobotTypeEnum::Different,
                    };

                    let task = robot_model::RobotGroupAdd::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            let robot_id = if let Some(robot) = response {
                                robot.insert_robot_group_one.and_then(|data| Some(RobotId(data.robot_id)))
                            } else {
                                None
                            };
                            RobotsListMessage::RobotAdded(robot_id)
                        },
                    );
                    self.robot_add_task = Some(task);
                }
            }
            RobotsListMessage::RemoveRobot(robot_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = robot_model::robot_group_delete::Variables { 
                        group_id: ctx.props().group_id.0,
                        robot_id: robot_id.0,
                    };

                    let task = robot_model::RobotGroupDelete::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            let robot_id = if let Some(response) = response {
                                if response.delete_robot_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![]).len() > 0 {
                                    Some(RobotId(response.delete_robot_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![])[0].robot_id))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            RobotsListMessage::RobotRemoved(robot_id)
                        },
                    );
                    self.robot_delete_task = Some(task);
                }
            }
            RobotsListMessage::RobotAdded(robot_id) => {
                let group_id = ctx.props().group_id;

                if let (Some(robot_id), Some(user_id)) = (robot_id, ctx.props().user_id) {

                    ctx.link().navigator().unwrap().push(&AppRoute::Robot { robot_id, group_id, user_id })

                }
            }
            RobotsListMessage::RobotRemoved(robot_id) => {
                info!("Remove Robot {:?}", robot_id);

            }
            RobotsListMessage::ShowDropdown => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            RobotsListMessage::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;

                let lessons_clone = self.robot_list.clone();

                let robots: Vec<RobotProfile> = lessons_clone.iter().filter(|filter| {
                    self.filter == RobotFilter::Alls && {filter.enabled == true || filter.enabled == false} ||
            
                    self.filter == RobotFilter::Enabled && filter.enabled == true ||
    
                    self.filter == RobotFilter::Disabled && filter.enabled == false
                })
                .cloned()
                .collect();

                info!("FILTER {:?} <-----> ROBOTS - VIEW {:?} ", self.filter, robots);

                self.robot_list_view = robots;
            }
            RobotsListMessage::UpdateRobotIdList(robot_id, enabled) => {
                for robot in self.robot_list.iter_mut() {
                    if robot.robot_id == robot_id {
                        robot.enabled = enabled;
                    }
                }
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // info!("{:?} => {:?}", ctx.props(), old_props);
        
        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_alls = ctx.link().callback(|_| RobotsListMessage::ChangeFilter(RobotFilter::Alls));
        let on_enabled = ctx.link().callback(|_| RobotsListMessage::ChangeFilter(RobotFilter::Enabled));
        let on_disabled = ctx.link().callback(|_| RobotsListMessage::ChangeFilter(RobotFilter::Disabled));
        let on_dropdown = ctx.link().callback(|_| RobotsListMessage::ShowDropdown);
        let on_change_list = ctx.link().callback(|(robot_id, enabled)| RobotsListMessage::UpdateRobotIdList(robot_id, enabled));
        let on_robot_delete = ctx.link().callback(|robot_id| RobotsListMessage::RemoveRobot(robot_id));
        let maybe_option_seleted = match self.filter {
            RobotFilter::Alls => "Everyone",
            RobotFilter::Enabled => "Enabled",
            RobotFilter::Disabled => "Disabled",
        };
        let class_dropdown = if self.show_dropdown_filter {
            "btn btn-secondary btn-see-degree dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-see-degree dropdown-toggle d-flex align-items-center justify-content-between"
        };
        let class_dropdown_list = if self.show_dropdown_filter {
            "dropdown-menu dropdown-menu-degree show"
        } else {
            "dropdown-menu dropdown-menu-degree"
        };
        let maybe_dropdown_by_user = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown">
                            <button class={class_dropdown} type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick={on_dropdown}>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(maybe_option_seleted)}</span>
                            </button>
                            <ul class={class_dropdown_list} aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick={on_alls}>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == RobotFilter::Alls {true} else {false}} />
                                        <span class={if self.filter == RobotFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick={on_enabled}>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == RobotFilter::Enabled {true} else {false}} />
                                        <span class={if self.filter == RobotFilter::Enabled {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Enabled")}{"s"}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick={on_disabled}>
                                    <input class="bg-checkbox" type="checkbox" checked={if self.filter == RobotFilter::Disabled {true} else {false}} />
                                    <span class={if self.filter == RobotFilter::Disabled {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Disabled")}{"s"}</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let robot_list = self
            .robot_list_view
            .iter()
            .filter(|data| data.robot_type != get_robot_list::RoboxRobotTypeEnum::Different)
            .map(|item| {
            let robot_profile = item.clone();
            html! {
                <RobotsCard user_profile={ctx.props().user_profile.clone()}
                    user_id={ctx.props().user_id.clone()}
                    robot_id={item.robot_id.clone()}
                    group_id={ctx.props().group_id.clone()}
                    on_robot_delete={Some(on_robot_delete.clone())}
                    on_change_list={on_change_list.clone()}
                    robot_profile={robot_profile} />
            }
        }).collect::<Html>();

        let other_robot_list = self
            .robot_list_view
            .iter()
            .filter(|data| data.robot_type == get_robot_list::RoboxRobotTypeEnum::Different)
            .map(|item| {
            let robot_profile = item.clone();
            html! {
                <RobotsCard user_profile={ctx.props().user_profile.clone()}
                    user_id={ctx.props().user_id.clone()}
                    robot_id={item.robot_id.clone()}
                    group_id={ctx.props().group_id.clone()}
                    on_robot_delete={Some(on_robot_delete.clone())}
                    on_change_list={on_change_list.clone()}
                    robot_profile={robot_profile} />
            }
        }).collect::<Html>();

        let maybe_robot_search = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = ctx.link().callback(|select_option| match select_option {
                    RobotSelectOption::Robot(robot_id) => RobotsListMessage::AddRobot(robot_id),
                });
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <RobotSelect on_select={on_select} 
                            allow_create={true}
                            group_id={ctx.props().group_id}
                            user_profile={ctx.props().user_profile.clone()} />
                    })
                } else {
                    Some(html! {
                        <SearchRobotdGroup group_id={ctx.props().group_id} user_id={None} />
                    })
                }
            })
            .unwrap_or(html! {});
        let group_id = ctx.props().group_id; 

        let navigator = ctx.link().navigator().unwrap();
        let on_direct_meet = Callback::from(move |_| navigator.push(&AppRoute::MeetDirect{group_id}));

        let maybe_meet = {
            html! {
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick={on_direct_meet}>
                       <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
            }
        };
        let maybe_user_profile_pic = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });
            
        let head_section = html! {
            <div class="d-flex flex-wrap align-items-lg-center justify-content-between mb-md-5 mb-lg-6">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {ctx.props().class_name.clone()}
                </h1>
                <div class="d-flex flex-wrap justify-content-between align-items-center col-12 col-xl-5 mb-4 mb-lg-0">
                    {maybe_meet}
                    {maybe_robot_search}
                    {maybe_user_profile_pic}
                </div>
            </div>
        };
        let maybe_dropdown = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                    {lang::dict("Robots")} <span class="ps-1">{"("}{self.robot_list_view.iter().filter(|item | item.robot_type != get_robot_list::RoboxRobotTypeEnum::Different).count()}{")"}</span>
                </span>
                {maybe_dropdown_by_user}
            </div>
        };

        let maybe_option = if self.robot_list_view.iter().filter(|item | item.robot_type != get_robot_list::RoboxRobotTypeEnum::Different).count() > 0 {
            html! {
                <div class="d-flex flex-wrap mb-3">
                    {robot_list}
                </div>
            }
        } else {
            html! {
                <div class="text-center">
                    <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No robots here.")}</span>
                </div>
            }
        };
        let maybe_option_two = if self.robot_list_view.iter().filter(|item | item.robot_type == get_robot_list::RoboxRobotTypeEnum::Different).count() > 0 {
            html! {
                <div class="d-flex flex-wrap pt-5">
                    {other_robot_list}
                </div>
            }
        } else {
            html! {}
        };

        html! { 
            <div class="scroll-y w-100 h-100 p-3 p-md-4 p-lg-7">
                {head_section}
                {maybe_dropdown}
                {maybe_option}
                {   
                    if self.robot_list_view.iter().filter(|item | item.robot_type == get_robot_list::RoboxRobotTypeEnum::Different).count() > 0 {
                        html! {
                            <>
                                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                                    {lang::dict("Other Robots")} <span class="ps-1">{"("}{self.robot_list_view.iter().filter(|item | item.robot_type == get_robot_list::RoboxRobotTypeEnum::Different).count()}{")"}</span>
                                </span>
                                {maybe_option_two}
                            </>
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
         }
    }
}