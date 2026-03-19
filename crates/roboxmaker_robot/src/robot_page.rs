use log::*;
use uuid::Uuid;
use gloo_storage::Storage;
use code_location::code_location;
use yew::{prelude::*, virtual_dom::VNode};
use yew::{html, Component, Html, Properties};

use roboxmaker_models::robot_model;
use roboxmaker_main::{lang, config};
use roboxmaker_searches::search_robots_group::SearchRobotdGroup;
use roboxmaker_message::{message_list::MessageList, MessageGroupCategory};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{RobotId, UserId, GroupId, AppRoute, MyUserProfile};
use yew_router::scope_ext::RouterScopeExt;

#[derive(Debug, Clone, PartialEq)]
pub struct RobotProfile {
    pub name: String,
    pub path: String,
    pub robot_id: RobotId,
}

pub struct RobotPage {
    graphql_task: Option<GraphQLTask>,
    robot_task: Option<RequestTask>,
    robot_profile: Option<RobotProfile>,
    robot_view: VNode,
    saved_sidebar_state: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct RobotPageProperties {
    pub user_profile: Option<MyUserProfile>,
    pub user_id: Option<UserId>,
    pub robot_id: RobotId,
    pub group_id: GroupId,
}

#[derive(Debug)]
pub enum RobotPageMessage {
    // AppRoute(AppRoute),
    FetchRobotById(RobotId),
    Robot(Option<robot_model::robot_by_id::ResponseData>),
    ChangeSidebarState,
}

impl Component for RobotPage {
    type Message = RobotPageMessage;
    type Properties = RobotPageProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(RobotPageMessage::FetchRobotById(ctx.props().robot_id));
        
        let saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };

        roboxmaker_utils::functions::school_state();

        RobotPage {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            robot_task: None,
            robot_profile: None,
            robot_view: html! {},
            saved_sidebar_state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            // RobotPageMessage::AppRoute(route) => {
            //     if let Some(on_app_route) = &ctx.props().on_app_route {
            //         on_app_route.emit(route)
            //     }
            // }
            RobotPageMessage::FetchRobotById(robot_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = robot_model::robot_by_id::Variables {
                        robot_id: robot_id.0,
                    };

                    let task = robot_model::RobotById::request(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                RobotPageMessage::Robot(response)
                            },
                    );
                    self.robot_task = Some(task);
                }
            }
            RobotPageMessage::Robot(response) => {
                self.robot_profile = Some(response
                    .clone()
                    .and_then(|data| data.robot_by_pk)
                    .and_then(|robot_pk| {
                        Some(RobotProfile {
                            name: robot_pk.robot_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string()),
                            path: robot_pk.robot_profile.clone().and_then(|data| Some(data.path)).unwrap_or("".to_string()),
                            robot_id: RobotId(robot_pk.id),
                        })
                    }).unwrap());

                self.robot_view = self.robot_profile.clone().and_then(|data| {
                    let robot_path = format!(
                        "{}/#kiosk=1&model={}/robots/{}.glb",
                        config::AKER_ROBOT_URL,
                        config::AKER_FILES_URL,
                        data.path
                    );
                    Some(html! {
                        <iframe class="has-ratio m-0" src={robot_path}
                            style="height: 100%; width: 100%; border: 0px; padding: 0px; border-radius: 10px;"
                            allow="fullscreen"></iframe>
                    })
                }).unwrap_or(html! {});
            }
            RobotPageMessage::ChangeSidebarState => {
                if let Some(element) = gloo_utils::document().get_element_by_id("show-sidebar-right") {
                    if self.saved_sidebar_state {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", false);
                        self.saved_sidebar_state = false;
                        let _ = element.set_attribute("class", "fa-angle-double-left fa-w-14 fa-2x");
                    } else {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", true);
                        self.saved_sidebar_state = true;
                        let _ = element.set_attribute("class", "fa fa-angle-double-right fa-w-14 fa-2x");
                    }
                }
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if ctx.props().robot_id != old_props.robot_id {
            self.robot_view = html! {};
            should_render = true;
        }

        if ctx.props() != old_props {
            ctx.link().send_message(RobotPageMessage::FetchRobotById(ctx.props().robot_id));
            should_render = true;
        }
        
        if ctx.props() != old_props {
            should_render = true;
        }
        
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_show_sidebar = ctx.link().callback(move |_| RobotPageMessage::ChangeSidebarState);
        let btn_sidebar_show = if self.saved_sidebar_state {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-right fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        } else {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-left fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        };
        let auth_user_id = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|data| Some(data.user_id.to_string()));

        let user_id = ctx
            .props()
            .user_id
            .as_ref()
            .and_then(|user_id| Some(user_id.0.to_string()));

        let user_uuid = if auth_user_id != user_id
            && user_id != Some("00000000-0000-0000-0000-000000000000".to_string())
        {
            user_id
        } else {
            auth_user_id
        };
        let user_id = ctx.props().user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));
        let navigator = ctx.link().navigator().unwrap();

        let go_back_group = Callback::from(move |_| navigator.push(&AppRoute::MySpace{user_id}));
        let go_back_my_space = html! {
            <a onclick={go_back_group}>
                <span class="text-gray-purple noir-medium is-size-16 lh-19 d-flex align-items-center">
                    <i class="fas fa-arrow-left me-1"></i>
                    <span>{lang::dict("To My Space/Robots")}</span>
                </span>
            </a>
        };
        let maybe_robot_viewer = self.robot_profile.clone().and_then(|data| {
            Some(html! {
                <>
                    <div class="d-flex align-items-center justify-content-between">
                        {go_back_my_space}
                        {btn_sidebar_show}
                    </div>
                    <div class="d-flex flex-column w-100 mb-4">
                        <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-3 mt-4">{&data.name}</h1>
                        <div class="d-flex justify-content-between text-primary-blue-dark noir-normal is-size-14 lh-17 my-3">
                            <span>{lang::dict("Progress")}</span>
                            <span>{"25%"}</span>
                        </div>
                        <progress class="progress is-small" style="width: 100%;" value="20" max="100"></progress>
                    </div>
                    <div class="box-robot-view d-block d-sm-block d-md-block d-lg-none vh-60">
                        { self.robot_view.clone() }
                    </div>
                </>
            })
        }).unwrap_or(html! {});
        let maybe_robot_editor = self.robot_profile.clone().and_then(|data| {
            let robot_url = format!(
                "{}/?robot={}&user={}",
                config::AKER_EDITOR_URL,
                data.path,
                user_uuid.unwrap_or_default()
            );
            Some(html! {
                <div class="box-robot-editor d-none d-sm-none d-md-none d-lg-block vh-85 position-relative">
                    <div class="h-100">
                        <iframe class="has-ratio m-0" src={robot_url} style="height: 100%; width: 100%; border: 0px; padding: 0px; border-radius: 10px;"></iframe>
                        <div class="box-robot-view-2 position-absolute bottom-20 end-2">
                            { self.robot_view.clone() }
                        </div>
                    </div>
                </div>
            })
        }).unwrap_or(html! {});

        let class_right_sidebar = if self.saved_sidebar_state {
            "bg-silver col col-sm-3 col-md-3 col-lg-5 col-xl-4 col-xxl-3 d-none d-sm-none d-md-none d-lg-block p-5"
        } else {
            "d-none"
        };
        let class_sidebar_mobile = if self.saved_sidebar_state {
            "offcanvas offcanvas-end show bg-silver d-block d-sm-block d-md-block d-lg-none d-xl-none d-xxl-none"
        } else {
            "offcanvas offcanvas-end"
        };
        let style_sidebar_mobile = if self.saved_sidebar_state {
            "visibility: visible;"
        } else {
            "display: none;"
        };
        let pic_path = ctx.props().user_profile.clone().and_then(|d|Some(d.pic_path)).unwrap_or_default();
        html! {
            <>
                <div class="w-100 h-100 d-flex flex-row justify-content-between">
                    <div class="w-100 scroll-y pt-3 ps-3 pt-md-4 ps-md-4 pt-lg-7 ps-lg-7">
                        <div class="d-flex flex-column pb-4">
                            {maybe_robot_viewer}
                            {maybe_robot_editor}
                        </div>
                    </div>
                    <div class={class_right_sidebar}>
                        <div class="d-flex align-items-center justify-content-between">
                            <SearchRobotdGroup group_id={ctx.props().group_id}
                                user_id={None} />
                            <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                        </div>
                        <hr class="hr-section" />
                        <span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Discussions")}</span>
                        <div class="section-right-post mt-3 scroll-messages-y mh-80">
                            <MessageList
                                user_profile={ctx.props().user_profile.clone()} user_id={None}
                                group_category={MessageGroupCategory::Robots(ctx.props().group_id, ctx.props().robot_id)} />
                        </div>
                    </div>
                    <div class={class_sidebar_mobile} data-bs-scroll="true" data-bs-backdrop="false" tabindex="-1" id="offcanvasScrolling" aria-labelledby="offcanvasScrollingLabel" aria-modal="true" role="dialog" style={style_sidebar_mobile}>
                        <div class="offcanvas-header d-flex justify-content-end">
                            <button type="button" class="btn btn-outline-danger" data-bs-dismiss="offcanvas" onclick={&on_show_sidebar}>
                                <i class="fas fa-times"></i>
                            </button>
                        </div>
                        <div class="offcanvas-body pt-0">
                            <div class="d-flex align-items-center justify-content-between">
                                <SearchRobotdGroup group_id={ctx.props().group_id}
                                    user_id={None} />
                                <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                            </div>
                            <hr class="hr-section" />
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Discussions")}</span>
                            <div class="section-right-post mt-3 scroll-messages-y mh-80">
                                <MessageList
                                    user_profile={ctx.props().user_profile.clone()} user_id={None}
                                    group_category={MessageGroupCategory::Robots(ctx.props().group_id, ctx.props().robot_id)} />
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}