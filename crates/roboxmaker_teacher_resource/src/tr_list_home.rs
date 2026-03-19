use log::*;
use uuid::Uuid;
use yew::prelude::*;
use roboxmaker_main::lang;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_utils::funtions::get_creation_date;
use roboxmaker_models::{school_model, lesson_model::{self, lesson_by_group_id}};
use roboxmaker_loaders::placeholders::card_post_placeholder::CardPostPlaceholder;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};
use roboxmaker_types::types::{GroupId, AppRoute, SchoolId, LessonId, MyUserProfile, UserId};


#[derive(Debug, Clone)]
enum LoadLessonFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadLesson {
    Loading,
    Load(LoadLessonFound),
}

#[derive(Debug, Clone, PartialEq)]
pub struct LessonProfile {
    pub title: String,
    pub timestamp: String,
    pub lesson_id: LessonId,
    pub full_name: String,
    pub author_id: Uuid,
    pub pic_path: String,
    pub lesson_type: lesson_by_group_id::RoboxLessonTypeEnum,
}

pub struct LessonListHome {
    link: ComponentLink<Self>,
    props: LessonListHomeProps,
    graphql_task: Option<GraphQLTask>,
    lesson_list_sub: Option<SubscriptionTask>,
    lesson_list: Vec<LessonProfile>,
    list_lessons_state: LoadLesson,

}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LessonListHomeProps {
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
    pub school_id: SchoolId,
    pub filter_lessons: bool,
    pub maybe_author: bool,
}

#[derive(Debug)]
pub enum LessonListHomeMessage {
    AppRoute(AppRoute),
    FetchLessonsByGroupId,
    Lessons(Option<lesson_model::lesson_by_group_id::ResponseData>),
}

impl Component for LessonListHome {
    type Message = LessonListHomeMessage;
    type Properties = LessonListHomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(LessonListHomeMessage::FetchLessonsByGroupId);

        LessonListHome {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            lesson_list_sub: None,
            lesson_list: vec![],
            list_lessons_state: LoadLesson::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            LessonListHomeMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            LessonListHomeMessage::FetchLessonsByGroupId => {
                self.list_lessons_state = LoadLesson::Loading;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let group_id = self.props.group_id;

                    let vars = lesson_model::lesson_by_group_id::Variables {
                        group_id: group_id.0,
                        // limit: 15
                    };

                    let task = lesson_model::LessonByGroupId::subscribe(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                LessonListHomeMessage::Lessons(response)
                            },
                    );
                    self.lesson_list_sub = Some(task);
                }
            }
            LessonListHomeMessage::Lessons(response) => {
                self.lesson_list = response
                    .clone()
                    .and_then(|data| Some(data.lesson_profile))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|lesson_profile| {
                        let title = lesson_profile.title.clone();
                        let lesson_id = lesson_profile.lesson_id;
                        let full_name = lesson_profile.author_profile.clone().and_then(|data| Some(data.full_name)).unwrap_or("".to_string());
                        let pic_path = lesson_profile.author_profile.clone().and_then(|data| data.pic_path).unwrap_or("".to_string());
                        let author_id = lesson_profile.author_profile.clone().and_then(|data| Some(data.user_id)).unwrap_or(Uuid::default());
                        
                        let timestamp = lesson_profile.timestamp;
                        
                        let time_fn = get_creation_date(timestamp);

                        let lesson_type = lesson_profile.lesson_type.clone().unwrap_or(lesson_by_group_id::RoboxLessonTypeEnum::Extra);

                        LessonProfile {
                            title: title,
                            timestamp: time_fn,
                            lesson_id: LessonId(lesson_id),
                            full_name: full_name,
                            pic_path: pic_path,
                            author_id: author_id,
                            lesson_type: lesson_type,
                        }
                    }).collect();
                if !response.clone().and_then(|data| Some(data.lesson_profile)).unwrap_or(vec![]).is_empty() {
                    self.list_lessons_state = LoadLesson::Load(LoadLessonFound::Found);
                } else {
                    self.list_lessons_state = LoadLesson::Load(LoadLessonFound::NotFound);
                }
            }
        }
        should_render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.group_id != props.group_id {
            self.link.send_message(LessonListHomeMessage::FetchLessonsByGroupId);
        }
        
        if self.props != props {
            self.props = props;
            should_render = true;
        } 

        should_render
    }

    fn view(&self) -> Html {
        let group_id = self.props.group_id;
        let school_id = self.props.school_id;

        let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));


        let lesson_staff = self.lesson_list
            .iter()
            // .filter(|lesson| lesson.filter_lessons == true)
            .map(|item| {
            let lesson_id = item.lesson_id;
            let on_lesson_view = self.link.callback(move |_| LessonListHomeMessage::AppRoute(AppRoute::LessonView(school_id, group_id, lesson_id)));
            if item.author_id == user_id.0 || item.lesson_type == lesson_by_group_id::RoboxLessonTypeEnum::ElectronicsLessons || item.lesson_type == lesson_by_group_id::RoboxLessonTypeEnum::Extra {
                html! {
                    <div class="card-post-view-home bg-white d-flex flex-column justify-content-between align-items-center p-5 me-5">
                        <a onclick={&on_lesson_view}>
                            <div class="module-message-post line-clamp-message-post">
                                <span class="text-blue-two text-justify noir-medium is-size-18 lh-22 ">
                                    {&item.title}
                                </span>
                            </div>
                        </a>
                        <a class="w-100" onclick={&on_lesson_view}>
                            <div class="d-flex align-items-center justify-content-between">
                                <img src=item.pic_path.clone() class="img-card-32" />
                                <span class="text-dark noir-light is-size-14 lh-17 text-truncate col-5 mb-0">
                                    {&item.full_name}
                                </span>
                                <div class="ms-2">
                                    <span class="text-brown noir-light is-size-13 lh-22  d-flex align-items-center">
                                        <i class="far fa-clock me-1"></i>
                                        <div class="d-flex flex-wrap">
                                            <span class="text-brown noir-light is-size-13 lh-22 ">{&item.timestamp}</span>
                                        </div>
                                    </span>
                                </div>
                            </div>
                        </a>
                    </div>
                }
            } else {
                html! {}
            }
        }).collect::<Html>();

        let lessons_list = match self.list_lessons_state {
            LoadLesson::Loading => {
                html! {
                    <>
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                    </>
                }
            },
            LoadLesson::Load(LoadLessonFound::Found) => {
                html! {
                    {lesson_staff}
                }
            },
            LoadLesson::Load(LoadLessonFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <p class="is-size-5">{lang::dict("No lessons here.")}</p>
                    </div>
                }
            },
        };
        html! {
            <div class="d-flex flex-row">   
                {lessons_list}
            </div>
        }
    }
}