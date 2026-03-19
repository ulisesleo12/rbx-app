use log::*;
use yew::prelude::*;
use yew::{html, Component, Html};
use gloo_timers::callback::Timeout;

use roboxmaker_main::lang;
use roboxmaker_types::types::{UserId, MyUserProfile};
use roboxmaker_loaders::placeholders::card_member_list::CardMemberListPlaceholder;

pub struct UserCard {
    maybe_placeholder: bool,
    job: Option<Timeout>,
    on_dropdown_menu: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct UserCardProperties {
    pub user_id: UserId,
    #[prop_or(None)]
    pub user_profile: Option<MyUserProfile>,
    pub on_user_delete: Option<Callback<UserId>>,
    pub saved_sidebar_state: bool,
    pub full_name: String,
    pub pic_path: String,
    pub user_staff: bool,
    pub user_teacher: bool,
    pub user_student: bool,
    pub view_profile: bool,
    pub on_dropdown_menu: bool,
    pub onclick: Callback<MouseEvent>,
}

#[derive(Debug)]
pub enum UserCardMessage {
    DeleteUser(UserId),
    OnDropdownMenu,
    HiddenPlaceholder,
    None,
}

impl Component for UserCard {
    type Message = UserCardMessage;
    type Properties = UserCardProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(UserCardMessage::None);
        UserCard {
            maybe_placeholder: true,
            job: None,
            on_dropdown_menu: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            UserCardMessage::DeleteUser(user_id) => {
                if let Some(on_user_delete) = &ctx.props().on_user_delete {
                    on_user_delete.emit(user_id)
                }
            }
            UserCardMessage::OnDropdownMenu => {
                self.on_dropdown_menu = !self.on_dropdown_menu;
            }
            UserCardMessage::HiddenPlaceholder => {
                self.maybe_placeholder = false;
            }
            UserCardMessage::None => {
                let link = ctx.link().clone();
                self.job = Some(Timeout::new(400, move || {
                    link.send_message(UserCardMessage::HiddenPlaceholder)
                }));
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;
        
        if ctx.props() != old_props {
            should_render = true;
        }
        
        should_render
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let _none = self.job.as_ref();

        let user_id = ctx.props().user_id;
        let full_name = ctx.props().full_name.clone();
        let pic_path = ctx.props().pic_path.clone();
        let on_user_delete = ctx.link().callback(move |_| UserCardMessage::DeleteUser(user_id));

        let maybe_user_type = if ctx.props().user_staff {
            html! {
                <span class="text-gray-purple-two noir-regular is-size-14 lh-17 ps-2 pt-2">{lang::dict("Staff")}</span>
            }
        } else if ctx.props().user_teacher {
            html! {
                <span class="text-gray-purple-two noir-regular is-size-14 lh-17 ps-2 pt-2">{lang::dict("Teacher")}</span>
            }
        } else {
            html! {
                <span class="text-gray-purple-two noir-regular is-size-14 lh-17 ps-2 pt-2">{lang::dict("Student")}</span>
            }
        };

        let on_dropdown = ctx.link().callback( move |_| UserCardMessage::OnDropdownMenu);
        let maybe_menu = if self.on_dropdown_menu {
            "btn btn-outline-gray-purple-two dropdown-toggle menu-hidden-toggle border-0 show"
        } else {
            "btn btn-outline-gray-purple-two dropdown-toggle menu-hidden-toggle border-0"
        };
        let maybe_item = if self.on_dropdown_menu {
                "dropdown-menu show"
            } else {
                "dropdown-menu"
        };
        let menu_dropdown_user = html! {
            <div class="dropdown">
                <a class={maybe_menu} onclick={on_dropdown} role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                    <i class="fas fa-ellipsis-v"></i>
                </a>
                <ul class={maybe_item} aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
                    <li class="my-1">   
                        <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={&ctx.props().onclick.clone()}>
                            <i class="fas fa-edit fas fa-lg me-2 ms-1"></i>
                            <span>{lang::dict("Edit")}</span>
                        </a>
                    </li>
                    <li class="mt-2 mb-1">   
                        <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={&on_user_delete}>
                            <i class="fas fa-lock fas fa-lg me-2 ms-1"></i>
                            <span>{lang::dict("Disguise")}</span>
                        </a>
                    </li>
                    // <li class="border-top">
                    //     <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={&on_user_delete}>
                    //         <img class="me-2" src="/icons/trash.svg" style="height: 22px;" />
                    //         <span>{lang::dict("Remove")}</span>
                    //     </a>
                    // </li>
                </ul>
            </div>
        };

        let class_member = if ctx.props().view_profile {
            "card-member-view-degree-2 bg-white d-flex align-items-center justify-content-between mb-3 mb-lg-5 me-2 me-lg-5"
        } else {
            "card-member-view-degree bg-white d-flex align-items-center justify-content-between mb-3 mb-lg-5 me-2 me-lg-5"
        };
        
        let maybe_users = if self.maybe_placeholder {
            html! {
                <CardMemberListPlaceholder />
            }
        } else {
            html! {
                <div class={class_member}>
                    <div class="d-flex align-items-center ps-4 pe-2">
                        <a onclick={ctx.props().onclick.clone()}>
                            <img class="img-card-64" src={pic_path.clone()} />
                        </a>
                        <div class="d-flex flex-column">
                            <div class="d-flex align-items-center justify-content-between">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 text-truncate ps-2 ps-2" style="width: 160px;">{&full_name}</span>
                                {menu_dropdown_user.clone()}
                            </div>
                            {maybe_user_type.clone()}
                        </div>
                    </div>
                </div>
            }
        };
        html! {
            maybe_users
        }
    }
}