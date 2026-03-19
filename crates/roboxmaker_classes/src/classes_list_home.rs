use log::*;
use yew::prelude::*;
use roboxmaker_main::lang;
use code_location::code_location;
use crate::date_of_classes_list::DateOfClassesList;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_utils::funtions::get_creation_date;
use roboxmaker_models::{school_model, classes_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};
use roboxmaker_types::types::{GroupId, ClassesId, AppRoute, SchoolId, MyUserProfile};
use roboxmaker_loaders::placeholders::card_classes_placeholder::CardClassesPlaceholder;

#[derive(Debug, Clone)]
enum LoadClassesFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadClasses {
    Loading,
    Load(LoadClassesFound),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassesProfile {
    pub topic: String,
    pub timestamp: String,
    pub classes_id: ClassesId,
}

pub struct ClassesListHome {
    link: ComponentLink<Self>,
    props: ClassesListHomeProps,
    graphql_task: Option<GraphQLTask>,
    classes_list_sub: Option<SubscriptionTask>,
    classes_list: Vec<ClassesProfile>,
    list_classes_state: LoadClasses,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ClassesListHomeProps {
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
}

#[derive(Debug)]
pub enum ClassesListHomeMessage {
    AppRoute(AppRoute),
    FetchClassesByGroupId,
    Classes(Option<classes_model::classes_by_group_id::ResponseData>),
}

impl Component for ClassesListHome {
    type Message = ClassesListHomeMessage;
    type Properties = ClassesListHomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(ClassesListHomeMessage::FetchClassesByGroupId);
        ClassesListHome {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            classes_list_sub: None,
            classes_list: vec![],
            list_classes_state: LoadClasses::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            ClassesListHomeMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            ClassesListHomeMessage::FetchClassesByGroupId => {
                self.list_classes_state = LoadClasses::Loading;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let group_id = self.props.group_id;

                    let vars = classes_model::classes_by_group_id::Variables {
                        group_id: group_id.0,
                        limit: 10,
                    };

                    let task = classes_model::ClassesByGroupId::subscribe(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ClassesListHomeMessage::Classes(response)
                            },
                    );
                    self.classes_list_sub = Some(task);
                }
            }
            ClassesListHomeMessage::Classes(response) => {
                self.classes_list = response
                    .clone()
                    .and_then(|data| Some(data.classes_profile))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|classes_profile| {
                        let topic = classes_profile.topic.clone();
                        let classes_id = classes_profile.classes_id;
                        
                        let timestamp = classes_profile.timestamp;

                        let time_fn = get_creation_date(timestamp);

                        ClassesProfile {
                            topic: topic,
                            timestamp: time_fn,
                            classes_id: ClassesId(classes_id),
                        }
                    }).collect();
                if !response.clone().and_then(|data| Some(data.classes_profile)).unwrap_or(vec![]).is_empty() {
                    self.list_classes_state = LoadClasses::Load(LoadClassesFound::Found);
                } else {
                    self.list_classes_state = LoadClasses::Load(LoadClassesFound::NotFound);
                }
            },
        }
        should_render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.group_id != props.group_id {
            self.link.send_message(ClassesListHomeMessage::FetchClassesByGroupId);
        }

        if self.props != props {
            self.props = props;
            should_render = true;
        }
        
        should_render
    }

    fn view(&self) -> Html {
        let group_id = self.props.group_id;
        let school_id =self.props.school_id;
        let classes_card_list = self.classes_list.iter().map(|item| {
            let classes_id = item.classes_id;
            let on_classes = self.link.callback(move |_| ClassesListHomeMessage::AppRoute(AppRoute::Classes(school_id, group_id, classes_id)));
            html! {
                <div class="card-classes-view bg-white d-flex flex-column justify-content-between p-5 me-5">
                    <a onclick=on_classes.clone()>
                        <div class="module-message-classes line-clamp-message-classes">
                            <span class="text-blue-two text-justify noir-medium is-size-18 lh-22 ">
                                {&item.topic.clone()}
                            </span>
                        </div>
                    </a>
                    <a onclick=&on_classes>
                        <div class="d-flex align-items-center justify-content-between w-100 m-0">
                            <DateOfClassesList classes_id=classes_id />
                            <span class="text-brown noir-light is-size-13 lh-22  d-flex align-items-center">
                                <i class="far fa-clock me-1"></i>
                                <div class="d-flex flex-wrap">
                                    <span class="text-brown noir-light is-size-13 lh-22 ">{&item.timestamp}</span>
                                </div>
                            </span>
                        </div>
                    </a>
                </div>
            }
        }).collect::<Html>();
        let classes_list = match self.list_classes_state {
            LoadClasses::Loading => {
                html! {
                    <>
                        <CardClassesPlaceholder />
                        <CardClassesPlaceholder />
                        <CardClassesPlaceholder />
                        <CardClassesPlaceholder />
                    </>
                }
            },
            LoadClasses::Load(LoadClassesFound::Found) => {
                html! {
                    {classes_card_list}
                }
            },
            LoadClasses::Load(LoadClassesFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <p class="is-size-5">{lang::dict("No classes here.")}</p>
                    </div>
                }
            },
        };
        html! {
            <div class="d-flex flex-row">   
                {classes_list}
            </div>
        }
    }
}