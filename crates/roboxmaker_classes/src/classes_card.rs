use log::*;
use yew::prelude::*;
use std::time::Duration;
use yew::{html, Component, Html};
use gloo_timers::callback::Timeout;
use roboxmaker_activity::ActivityStyle;
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_activity::{activity_list::ActivityList};
// use roboxmaker_files::files_list_classes::FilesListClasses;
use roboxmaker_loaders::placeholders::card_classes_list::CardClassesListPlaceholder;
use roboxmaker_types::types::{ClassesId, GroupId, AppRoute, SchoolId, MyUserProfile};

pub struct ClassesCard {
    show_child_classes: bool,
    maybe_placeholder: bool,
    job: Option<Timeout>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ClassesCardProps {
    pub classes_id: ClassesId,
    pub group_id: GroupId,
    pub user_profile: Option<MyUserProfile>,
    pub on_classes_delete: Option<Callback<ClassesId>>,
    pub topic: String,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum ClassesCardMessage {
    DeleteClasses(ClassesId),
    ShowChildClasses,
    HiddenPlaceholder,
    None,
}

impl Component for ClassesCard {
    type Message = ClassesCardMessage;
    type Properties = ClassesCardProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(ClassesCardMessage::None);

        ClassesCard {
            show_child_classes: false,
            maybe_placeholder: true,
            job: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            ClassesCardMessage::DeleteClasses(classes_id) => {
                if let Some(on_classes_delete) = &ctx.props().on_classes_delete {
                    on_classes_delete.emit(classes_id)
                }
            }
            ClassesCardMessage::ShowChildClasses => {
                self.show_child_classes = !self.show_child_classes;
            }
            ClassesCardMessage::None => {
                let duration = Duration::from_secs(600).as_secs() as u32;
                let link = ctx.link().clone();
                let handle = Timeout::new( duration, move || {
                    link.send_message(ClassesCardMessage::HiddenPlaceholder)
                });
                self.job = Some(handle);
            }
            ClassesCardMessage::HiddenPlaceholder => {
                self.maybe_placeholder = false;
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _none = self.job.as_ref();

        let classes_id = ctx.props().classes_id;
        let group_id = ctx.props().group_id;
        let school_id = ctx.props().school_id;

        let navigator = ctx.link().navigator().unwrap();
        let on_classes = Callback::from(move |_| navigator.push(&AppRoute::Classes{school_id, group_id, classes_id}));

        let maybe_option_action = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|auth_user| {
                let on_classes_delete = ctx
                    .link()
                    .callback(move |_| ClassesCardMessage::DeleteClasses(classes_id));
                if auth_user.user_staff.is_some() || auth_user.user_teacher.is_some() {
                    Some(html! {
                        <div class="d-flex flex-wrap">
                            <a class="btn bg-transparent me-3" onclick={&on_classes}>
                                <span class="text-primary-blue-dark">
                                    <i class="far fa-edit"></i>
                                </span>
                            </a>
                            <a class="btn bg-transparent me-3" onclick={on_classes_delete}>
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
        let on_child_classes = ctx.link().callback(|_| ClassesCardMessage::ShowChildClasses);  

        let maybe_classes = if self.maybe_placeholder {
            html! {
                <CardClassesListPlaceholder />
            }
        } else {
            html! {
                <>
                    <div class={card_classes_view}>
                        <a onclick={&on_classes}>
                            <div class="module-message-universal-2 line-clamp-message-universal">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 ps-5">{&ctx.props().topic}</span>
                            </div>
                        </a>
                        <div class="d-flex flex-wrap pe-5">
                            {maybe_option_action}
                            <button onclick={on_child_classes} class="btn btn-outline-silver text-gray-blue border-0">
                                <i class="fas fa-angle-down"></i>
                            </button>
                        </div>
                    </div>
                    <div class={card_classes_child}>
                        <div class="resources-classes px-5 py-4 d-flex flex-column">
                            // <span class="text-secondary-purple noir-bold is-size-14 lh-17">{lang::dict("Resources")}</span>
                            // <FilesListClasses classes_id=ctx.props().classes_id />
                            <span class="text-secondary-purple noir-bold is-size-14 lh-17 mt-4">{lang::dict("Activities")}</span>
                            <ActivityList user_profile={ctx.props().user_profile.clone()}
                                user_id={None}
                                group_id={ctx.props().group_id}
                                classes_id={ctx.props().classes_id}
                                maybe_style={ActivityStyle::ClassesCard} />
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