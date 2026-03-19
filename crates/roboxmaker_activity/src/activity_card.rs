use log::*;
use std::vec;
use uuid::Uuid;
use chrono::NaiveDate;
use crate::ActivityStyle;
use code_location::code_location;
use crate::activity_list::ActivityProfile;
use yew::{prelude::*, web_sys::{Node, self}, virtual_dom::VNode};
use crate::v_calendar_activity::ActivityCardVCalendar;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
use roboxmaker_models::activity_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{AppRoute, ActivityId, ClassesId, GroupId, MyUserProfile};

pub struct ActivityCard {
    link: ComponentLink<Self>,
    props: ActivityCardProps,
    graphql_task: Option<GraphQLTask>,
    save_task: Option<RequestTask>,
    delete_task: Option<RequestTask>,
    content: String,
    edit: bool,
    node: Option<Node>,
    score: i64,
    title: String,
    date: String,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ActivityCardProps {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub activity_profile: Option<ActivityProfile>,
    pub classes_id: ClassesId,
    pub group_id: GroupId,
    pub maybe_style: ActivityStyle,
}


#[derive(Debug)]
pub enum ActivityCardMessage {
    CreateActivity,
    ContentSaved(Option<activity_model::activity_classes_group_create::ResponseData>),
    DeleteActivity(ActivityId),
    ActivityDeleted(
        Option<activity_model::delete_activity_by_id::ResponseData>,
    ),
    EditActivity(ActivityId),
    ContentActivity,
    OnContent(String),
    Title(String),
    OnScore(String),
    SaveActivity(ActivityId),
    UpdateActivity(Option<activity_model::update_activity_content_by_id::ResponseData>),
    CancelEditActivity(ActivityId),
    OnDateActivity(String),
}

impl Component for ActivityCard {
    type Message = ActivityCardMessage;
    type Properties = ActivityCardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(ActivityCardMessage::ContentActivity);
        ActivityCard {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            save_task: None,
            delete_task: None,
            content: String::from(""),
            edit: false,
            node: None,
            title: String::from(""),
            score: 0,
            date: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            ActivityCardMessage::ContentSaved(_) => {
                self.content = String::default();
            }
            ActivityCardMessage::CreateActivity => {
                let content = self.content.clone();
                let local = chrono::Local::now().naive_local();

                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = activity_model::activity_classes_group_create::Variables {
                        content,
                        group_id: self.props.group_id.0,
                        classes_id: self.props.classes_id.0,
                        title: String::from(lang::dict("New Activity")),
                        timestamp: local,
                    };

                    let task = activity_model::ActivityClassesGroupCreate::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ActivityCardMessage::ContentSaved(response)
                            },
                    );
                    self.save_task = Some(task);
                }        
            } 
            ActivityCardMessage::DeleteActivity(activity_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = activity_model::delete_activity_by_id::Variables {
                        activity_id: activity_id.0,
                    };

                    let task = activity_model::DeleteActivityById::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ActivityCardMessage::ActivityDeleted(response)
                            },
                    );
                    self.delete_task = Some(task);
                }
            }
            ActivityCardMessage::ActivityDeleted(_) => {}
            ActivityCardMessage::EditActivity(_activity_id) => {
                self.edit = true;
            }
            ActivityCardMessage::ContentActivity => {
                self.title = self.props.activity_profile.clone().and_then(|data| Some(data.title)).unwrap_or("".to_string());
                self.score = self.props.activity_profile.clone().and_then(|data| Some(data.score)).unwrap_or(0);
                self.date = self.props.activity_profile.clone().and_then(|data| Some(format! ("{}", data.deliver))).unwrap_or("01/01/2022".to_string());
                self.content = self.props.activity_profile.clone().and_then(|data| Some(data.content)).unwrap_or("".to_string());
                // let content = self.props.activity_profile.clone().and_then(|data| Some(data.content)).unwrap_or("".to_string());
                self.node = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.create_element("div").ok())
                    .and_then(|div| {
                        div.set_class_name("ck-content");
                        div.set_inner_html(&self.content);
                        Some(Node::from(div))
                    });
                // self.content = content.clone();
                // info!("DDD {:?}", content.clone());
                // info!("DDD {:?}", self.content);
            }
            ActivityCardMessage::OnContent(content) => {
                self.content = content;
                // content = self.props.activity_profile.clone().and_then(|data| Some(data.content)).unwrap_or("".to_string());
            }
            ActivityCardMessage::Title(title) => {
                self.title = title;
            }
            ActivityCardMessage::OnScore(score) => {
                match score.parse::<i64>() {
                    Ok(v) => self.score = v,
                    Err(e) => {
                        self.score = 0;
                        info!("{:?}", e);
                    }
                }
            }
            ActivityCardMessage::OnDateActivity(date) => {
                self.date = date.clone();
                info!("YYY {:?}", date.clone());
                info!("YYY {:?}", self.date);
            }
            ActivityCardMessage::SaveActivity(activity_id) => {
                self.edit = false;
                let date = self.date.clone();
                let today = chrono::Local::now().date_naive();
                let deliver = NaiveDate::parse_from_str(&date,"%Y-%m-%d").unwrap_or(today);

                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = activity_model::update_activity_content_by_id::Variables {
                        activity_id: activity_id.0,
                        content: self.content.clone(),
                        title: self.title.clone(),
                        score: self.score.clone(),
                        deliver: deliver.clone(),
                    };

                    let task = activity_model::UpdateActivityContentById::request(
                            graphql_task,
                            &self.link,
                            vars,
                            move |response| {
                                ActivityCardMessage::UpdateActivity(response)
                            },
                    );
                    self.save_task = Some(task);
                }
            }
            ActivityCardMessage::UpdateActivity(_) => {}
            ActivityCardMessage::CancelEditActivity(_activity_id) => {
                self.edit = false;
                // self.node = None;
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = true;

        if self.props != props {
            self.props = props;
            self.link.send_message(ActivityCardMessage::ContentActivity);
            should_render = true;
        }

        should_render
    }

    fn view(&self) -> Html {
        let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
        let on_callback_date = self.link.callback(move |date| ActivityCardMessage::OnDateActivity(date));

        if let Some(activity_profile) = &self.props.activity_profile {
            if self.edit {
                let activity_id = activity_profile.activity_id;
                let on_data = self
                    .link
                    .callback(move |data| ActivityCardMessage::OnContent(data));
                let on_save = self
                    .link
                    .callback(move |_| ActivityCardMessage::SaveActivity(activity_id));
                let on_cancel = self
                    .link
                    .callback(move |_| ActivityCardMessage::CancelEditActivity(activity_id));
                let maybe_node = if let Some(node) = &self.node {
                    VNode::VRef(node.clone())
                } else {
                    html! {
                        <span>
                            <i class="fas fa-spinner fa-pulse"></i>
                        </span>
                    }
                };
                let on_title = self.link.callback(|data: InputData| ActivityCardMessage::Title(data.value));
                let on_score = self.link.callback(|data: InputData| ActivityCardMessage::OnScore(data.value));
                let maybe_option_edit = self
                    .props
                    .user_profile
                    .as_ref()
                    .and_then(|user|{
                        if user.user_staff.is_some() || user.user_teacher.is_some()  {
                            Some(html! {
                                <>
                                    <div class="pb-5 mt-5">
                                        <input class="input input-style-universal px-4 w-100" type="text" placeholder={lang::dict("Activity Title")} value=self.title.clone() oninput=on_title />
                                    </div>
                                    <div class="d-flex justify-content-between mb-5">
                                        <div class="d-flex flex-row">
                                            <div class="d-flex flex-column">
                                                <span class="text-secondary-purple noir-bold is-size-16 lh-20 mb-2">{lang::dict("Delivery Date")}</span>
                                                // <input class="input input-style-universal form-date-input px-3" type="date" placeholder="Text input" oninput=on_date />
                                                <ActivityCardVCalendar on_callback_date=on_callback_date
                                                    activity_profile=self.props.activity_profile.clone() />
                                            </div>
                                            <div class="pe-5 me-2"></div>
                                            <div class="d-flex flex-column">
                                                <span class="text-secondary-purple noir-bold is-size-16 lh-20 mb-2">{lang::dict("Punctuation")}</span>
                                                <input class="input input-style-universal px-2" type="number" min="0" max="10" value=format!("{}", self.score.clone()) placeholder="0" oninput=on_score />
                                            </div>
                                        </div>
                                        <div class="d-flex is-align-items-flex-end justify-content-end">
                                            <button class="button btn-cancel-activity" onclick=&on_cancel>
                                                <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Cancel")}</span>
                                            </button>
                                            <div class="pe-4"></div>
                                            <button class="button btn-save-activity bg-primary-blue-dark" onclick=on_save>
                                                <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Modify Activity")}</span>
                                            </button>
                                        </div>
                                    </div>
                                    <div class="container-editor-activity mb-6">
                                        <ckeditor::CKEditor user_profile=self.props.user_profile.clone()
                                            content=self.content.clone()
                                            upload_url=upload_url.clone()
                                            on_data=on_data.clone() />
                                    </div>
                                </>
                            })
                        } else {
                            Some(html! {
                                <div class="pt-4 border-bottom border-top my-4">                                               
                                     <div class="d-flex flex-wrap align-items-center justify-content-between">
                                        <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 col-12 col-sm-12 col-md-12 col-lg-4">
                                            {&activity_profile.title}
                                        </span>
                                        <span class="text-gray-purple-two noir-light is-size-14 lh-17-2">{&activity_profile.score}{" pts"}</span>
                                        <span class="text-gray-purple-two noir-light is-size-14 lh-17-2">{"Hasta "}{&activity_profile.deliver}</span>
                                        <span class="text-gray-purple-two noir-light is-size-14 lh-17-2">{&activity_profile.timestamp}</span>
                                        <button class="btn btn-outline-danger" onclick=&on_cancel>
                                            <i class="fas fa-times"></i>
                                        </button>
                                    </div>
                                    <div class="text-dark noir-light is-size-18 lh-22 mt-4">
                                        {maybe_node}
                                    </div>
                                </div>
                            })
                        }
                    })
                    .unwrap_or(html! {});
                html! {
                    maybe_option_edit
                }
            } else {
                let maybe_edit_activity = self
                    .props
                    .user_profile
                    .as_ref()
                    .and_then(|auth_user| {
                        let activity_id = activity_profile.activity_id;
                        let author_id = activity_profile.user_id;
                        let on_activity_edit = self.link.callback(move |_| ActivityCardMessage::EditActivity(activity_id));
                        let on_activity_delete = self.link.callback(move |_| ActivityCardMessage::DeleteActivity(activity_id));
                        if auth_user.user_id.0 == author_id.0 && auth_user.user_staff.is_some() || auth_user.user_teacher.is_some() {
                            Some(html! {
                                <div class="d-flex flex-wrap justify-content-end col order-lg-2 mt-3 mt-lg-0">
                                    <button class="btn btn-transparent me-4" onclick=on_activity_edit>
                                        <span class="icon is-medium" style="color: #A4A5E3">
                                            <i class="far fa-edit fas fa-lg"></i>
                                        </span>
                                    </button>
                                    <button class="btn btn-outline-danger" onclick=on_activity_delete>
                                        <span class="is-size-14">
                                            <i class="far fa-trash-alt"></i>
                                        </span>
                                    </button>
                                </div>
                            })
                        } else {
                            None
                        }
                    })
                    .unwrap_or(html! {});

                let activity_div_key = self
                    .props
                    .activity_profile
                    .as_ref()
                    .and_then(|activity_profile| Some(activity_profile.activity_id))
                    .unwrap_or(ActivityId(Uuid::default()));
                let activity_id = activity_profile.activity_id;
                let on_activity_edit = self.link.callback(move |_| ActivityCardMessage::EditActivity(activity_id));
                let maybe_model = {
                    match self.props.maybe_style {
                        ActivityStyle::ClassesPage => {
                            html! {
                                <div key=activity_div_key.to_string()>
                                    <div class="card-activity-view bg-white d-flex flex-wrap align-items-center justify-content-between p-4 mt-5">
                                        <a class="col-12 col-sm-12 col-md-12 col-lg-6 order-0" onclick=on_activity_edit>
                                            <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                                                <img src="/icons/clipboard-2.svg" style="width: 22px;" />
                                                <span class="ps-2">{&activity_profile.title}</span>
                                            </span>
                                        </a>
                                        <div class="d-flex flex-wrap justify-content-between col-12 col-sm-12 col-md-12 col-lg-4 order-lg-1 mt-3 mt-lg-0">
                                            <span class="text-gray-purple-two noir-light is-size-14 lh-17-2">{&activity_profile.score}{" pts"}</span>
                                            <span class="text-gray-purple-two noir-light is-size-14 lh-17-2">{lang::dict("Until ")}{&activity_profile.deliver}</span>
                                            <span class="text-gray-purple-two noir-light is-size-14 lh-17-2">{&activity_profile.timestamp}</span>
                                        </div>
                                        {maybe_edit_activity}
                                    </div>
                                </div>
                            }
                        }
                        ActivityStyle::ClassesCard => {
                            html! {}
                        }
                    }
                };
                html! {
                    {maybe_model}
                }
            }
        } else if let Some(user) = &self.props.user_profile {
            let on_send = self
                .link
                .callback(move |_| ActivityCardMessage::CreateActivity);
            let maybe_option = if user.user_staff.is_some() || user.user_teacher.is_some() {
                Some(html! {
                    <button class="btn btn-create-activity bg-primary-blue-dark" onclick=&on_send>
                        <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Create Activity")}</span>
                    </button>
                })
            } else {
                None
            }.unwrap_or(html! {});

            let maybe_button = {
                match self.props.maybe_style {
                    ActivityStyle::ClassesPage => {
                        html! {
                            <div class="d-flex flex-column">
                                {maybe_option}
                                <span class="text-primary-blue-light noir-bold is-size-14 lh-18 py-5 mt-2">{lang::dict("Activities list")}</span>
                            </div>
                        }
                    }
                    ActivityStyle::ClassesCard => {
                        html! {}
                    }
                }
            };
            html! {
                <>
                    {maybe_button}
                </>
            }
        } else {
            html! {}
        }
    }
}