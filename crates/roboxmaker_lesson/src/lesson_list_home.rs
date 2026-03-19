use log::*;
use uuid::Uuid;
use yew::prelude::*;
use roboxmaker_main::lang;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_loaders::placeholders::card_post_placeholder::CardPostPlaceholder;
use roboxmaker_types::types::{GroupId, AppRoute, SchoolId, MyUserProfile, UserId, LessonProfile};


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

pub struct LessonListHome {
    lesson_list: Vec<LessonProfile>,
    list_lessons_state: LoadLesson,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct LessonListHomeProps {
    pub group_id: GroupId,
    pub user_profile: Option<MyUserProfile>,
    pub school_id: SchoolId,
    pub filter_lessons: bool,
    pub maybe_author: bool,
    pub lesson_list: Vec<LessonProfile>,
}

#[derive(Debug)]
pub enum LessonListHomeMessage {
    FetchLessonsByGroupId,
}

impl Component for LessonListHome {
    type Message = LessonListHomeMessage;
    type Properties = LessonListHomeProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(LessonListHomeMessage::FetchLessonsByGroupId);

        LessonListHome {
            lesson_list: vec![],
            list_lessons_state: LoadLesson::Loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            LessonListHomeMessage::FetchLessonsByGroupId => {
                self.list_lessons_state = LoadLesson::Loading;
                
                self.lesson_list = ctx.props().lesson_list.clone();

                if !self.lesson_list.is_empty() {
                    self.list_lessons_state = LoadLesson::Load(LoadLessonFound::Found);
                } else {
                    self.list_lessons_state = LoadLesson::Load(LoadLessonFound::NotFound);
                }
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // info!("{:?} => {:?}", ctx.props(), old_props);

        if ctx.props().lesson_list != old_props.lesson_list {
            ctx.link().send_message(LessonListHomeMessage::FetchLessonsByGroupId);
        }
        
        ctx.props() !=  old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let group_id = ctx.props().group_id;
        let school_id = ctx.props().school_id;

        let user_id = ctx.props().user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));


        let lesson_staff = self.lesson_list
            .iter()
            .map(|item| {
            let lesson_id = item.lesson_id;

            let navigator = ctx.link().navigator().unwrap();
            let on_lesson_view = Callback::from(move |_| navigator.push(&AppRoute::LessonView{school_id, group_id, lesson_id}));

            if item.author_id == user_id.0 || (item.lesson_type == String::from("ELECTRONICSLESSONS") || item.lesson_type == String::from("EXTRA")) {
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
                                <img src={item.pic_path.clone()} class="img-card-32" />
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