use log::*;
use std::vec;
use yew::prelude::*;
use std::time::Duration;
use code_location::code_location;
use roboxmaker_activity::ActivityStyle;
use yew::services::{TimeoutService, Task};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::classes_model;
use roboxmaker_activity::{activity_list::ActivityList};
// use roboxmaker_files::files_list_classes::FilesListClasses;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_loaders::placeholders::card_classes_list::CardClassesListPlaceholder;
use roboxmaker_types::types::{ClassesId, GroupId, AppRoute, SchoolId, MyUserProfile};

pub struct ClassesCard {
    link: ComponentLink<Self>,
    props: ClassesCardProps,
    graphql_task: Option<GraphQLTask>,
    activity_task: Option<RequestTask>,
    activity_profile_aggregate: Option<classes_model::activity_profile_aggregate::ActivityProfileAggregateActivityProfileAggregate>,
    show_child_classes: bool,
    maybe_placeholder: bool,
    job: Option<Box<dyn Task>>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ClassesCardProps {
    pub classes_id: ClassesId,
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub on_classes_delete: Option<Callback<ClassesId>>,
    pub topic: String,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum ClassesCardMessage {
    FetchActivityByClassesById(ClassesId),
    ActivityByClasses(Option<classes_model::activity_profile_aggregate::ResponseData>),
    AppRoute(AppRoute),
    DeleteClasses(ClassesId),
    ShowChildClasses,
    HiddenPlaceholder,
}

impl Component for ClassesCard {
    type Message = ClassesCardMessage;
    type Properties = ClassesCardProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(ClassesCardMessage::FetchActivityByClassesById(props.classes_id));
        let handle = TimeoutService::spawn(
            Duration::from_millis(400),
            link.callback(|_| ClassesCardMessage::HiddenPlaceholder),
        );
        ClassesCard {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            activity_task: None,
            activity_profile_aggregate: None,
            show_child_classes: false,
            maybe_placeholder: true,
            job: Some(Box::new(handle)),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            ClassesCardMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            ClassesCardMessage::FetchActivityByClassesById(classes_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = classes_model::activity_profile_aggregate::Variables {
                        classes_id: classes_id.0 
                    };

                    let task = classes_model::ActivityProfileAggregate::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ClassesCardMessage::ActivityByClasses(response)
                            },
                    );
                    self.activity_task = Some(task);
                }
            }
            ClassesCardMessage::ActivityByClasses(activity_profile_aggregate) => {
                self.activity_profile_aggregate = activity_profile_aggregate.clone().and_then(|data| Some(data.activity_profile_aggregate))
            }
            ClassesCardMessage::DeleteClasses(classes_id) => {
                if let Some(on_classes_delete) = &self.props.on_classes_delete {
                    on_classes_delete.emit(classes_id)
                }
            }
            ClassesCardMessage::ShowChildClasses => {
                self.show_child_classes = !self.show_child_classes;
            }
            ClassesCardMessage::HiddenPlaceholder => {
                self.maybe_placeholder = false;
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.classes_id != props.classes_id {
            self.link.send_message(ClassesCardMessage::FetchActivityByClassesById(self.props.classes_id));
        }

        if self.props != props {
            self.props = props;
            should_render = true;
        }

        should_render
    }

    fn view(&self) -> Html {
        let _none = self.job.as_ref();

        let classes_id = self.props.classes_id;
        let group_id = self.props.group_id;
        let school_id = self.props.school_id;
        let on_classes = self.link.callback(move |_| ClassesCardMessage::AppRoute(AppRoute::Classes(school_id, group_id, classes_id)));
        let maybe_option_action = self
            .props
            .user_profile
            .as_ref()
            .and_then(|auth_user| {
                let on_classes_delete = self
                    .link
                    .callback(move |_| ClassesCardMessage::DeleteClasses(classes_id));
                if auth_user.user_staff.is_some() || auth_user.user_teacher.is_some() {
                    Some(html! {
                        <div class="d-flex flex-wrap">
                            <a class="btn bg-transparent me-3" onclick=&on_classes>
                                <span class="text-primary-blue-dark">
                                    <i class="far fa-edit"></i>
                                </span>
                            </a>
                            <a class="btn bg-transparent me-3" onclick=on_classes_delete>
                                <img src="/icons/trash-3.svg" style="height: 20px;" />
                            </a>
                        </div>
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {});

        let card_classes_view = if self.show_child_classes {
            "classes-card-view-2 d-flex align-items-center justify-content-between"
        } else {
            "classes-card-view bg-white d-flex align-items-center justify-content-between mb-5"
        };
        let card_classes_child = if self.show_child_classes {
            "child-card-classes bg-white mt-2 mb-5"
        } else {
            "d-none"
        };
        let on_child_classes = self.link.callback(|_| ClassesCardMessage::ShowChildClasses);  

        let maybe_classes = if self.maybe_placeholder {
            html! {
                <CardClassesListPlaceholder />
            }
        } else {
            html! {
                <>
                    <div class=card_classes_view>
                        <a onclick=&on_classes>
                            <div class="module-message-universal-2 line-clamp-message-universal">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 ps-5">{&self.props.topic}</span>
                            </div>
                        </a>
                        <div class="d-flex flex-wrap pe-5">
                            {maybe_option_action}
                            <button onclick=on_child_classes class="btn btn-outline-silver text-gray-blue border-0">
                                <i class="fas fa-angle-down"></i>
                            </button>
                        </div>
                    </div>
                    <div class=card_classes_child>
                        <div class="resources-classes px-5 py-4 d-flex flex-column">
                            // <span class="text-secondary-purple noir-bold is-size-14 lh-17">{lang::dict("Resources")}</span>
                            // <FilesListClasses classes_id=self.props.classes_id />
                            <span class="text-secondary-purple noir-bold is-size-14 lh-17 mt-4">{lang::dict("Activities")}</span>
                            <ActivityList on_app_route=self.props.on_app_route.clone()
                                user_profile=self.props.user_profile.clone()
                                user_id=None
                                group_id=self.props.group_id
                                classes_id=self.props.classes_id
                                maybe_style=ActivityStyle::ClassesCard />
                        </div>
                    </div>
                </>
            }
        };
        html! {
            maybe_classes
        }
    }
}