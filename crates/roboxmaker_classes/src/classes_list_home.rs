use log::*;
use yew::prelude::*;
use roboxmaker_main::lang;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;
use crate::date_of_classes_list::DateOfClassesList;

use roboxmaker_types::types::{GroupId, AppRoute, SchoolId, MyUserProfile, ClassesProfile};
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

pub struct ClassesListHome {
    classes_list: Vec<ClassesProfile>,
    list_classes_state: LoadClasses,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ClassesListHomeProps {
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub user_profile: Option<MyUserProfile>,
    pub classes_list: Vec<ClassesProfile>,
}

#[derive(Debug)]
pub enum ClassesListHomeMessage {
    FetchClassesByGroupId,
}

impl Component for ClassesListHome {
    type Message = ClassesListHomeMessage;
    type Properties = ClassesListHomeProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(ClassesListHomeMessage::FetchClassesByGroupId);

        ClassesListHome {
            classes_list: vec![],
            list_classes_state: LoadClasses::Loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            ClassesListHomeMessage::FetchClassesByGroupId => {
                self.list_classes_state = LoadClasses::Loading;

                self.classes_list = ctx.props().classes_list.clone();

                if !self.classes_list.is_empty() {
                    self.list_classes_state = LoadClasses::Load(LoadClassesFound::Found);
                } else {
                    self.list_classes_state = LoadClasses::Load(LoadClassesFound::NotFound);
                }
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // info!("{:?} => {:?}", ctx.props(), old_props);

        if ctx.props().classes_list != old_props.classes_list {
            ctx.link().send_message(ClassesListHomeMessage::FetchClassesByGroupId);
        }

        ctx.props() != old_props 
        
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let group_id = ctx.props().group_id;
        let school_id =ctx.props().school_id;
        let classes_card_list = self.classes_list.iter().map(|item| {
            let classes_id = item.classes_id;

            let navigator = ctx.link().navigator().unwrap();
            let on_classes = Callback::from(move |_| navigator.push(&AppRoute::Classes{school_id, group_id, classes_id}));

            html! {
                <div class="card-classes-view bg-white d-flex flex-column justify-content-between p-5 me-5">
                    <a onclick={on_classes.clone()}>
                        <div class="module-message-classes line-clamp-message-classes">
                            <span class="text-blue-two text-justify noir-medium is-size-18 lh-22 ">
                                {&item.topic.clone()}
                            </span>
                        </div>
                    </a>
                    <a onclick={&on_classes}>
                        <div class="d-flex align-items-center justify-content-between w-100 m-0">
                            <DateOfClassesList classes_id={classes_id} />
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