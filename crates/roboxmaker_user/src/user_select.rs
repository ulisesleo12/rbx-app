use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use roboxmaker_types::types::GroupId;
use yew::{html, Component, Html, NodeRef};

use roboxmaker_main::lang;
use roboxmaker_models::user_model;
use roboxmaker_types::types::{UserId, MyUserProfile};
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

#[derive(Debug, Clone)]
enum LoadSearchFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadSearch {
    Static,
    Load(LoadSearchFound),
}

#[derive(Debug)]
pub enum UserSelectOption {
    User(UserId),
}
pub struct UserSelect {
    graphql_task: Option<GraphQLTask>,
    task: Option<RequestTask>,
    users: Vec<user_model::users_by_full_name::UsersByFullNameUser>,
    search_node: NodeRef,
    show_create: bool,
    maybe_section_search: bool,
    list_search_state: LoadSearch,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct UserSelectProperties {
    pub on_select: Callback<UserSelectOption>,
    #[prop_or(None)]
    pub user_profile: Option<MyUserProfile>,
    #[prop_or(None)]
    pub user_id: Option<UserId>,
    pub group_id: GroupId,
}

#[derive(Debug)]
pub enum UserSelectMessage {
    FetchUsersByUserName(String),
    Users(Option<user_model::users_by_full_name::ResponseData>),
    SelectUser(UserSelectOption),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for UserSelect {
    type Message = UserSelectMessage;
    type Properties = UserSelectProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        UserSelect {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task: None,
            users: vec![],
            search_node: NodeRef::default(),
            show_create: false,
            maybe_section_search: false,
            list_search_state: LoadSearch::Static,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_render = true;
        match msg {
            UserSelectMessage::FetchUsersByUserName(search) => {
                should_render = false;
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = user_model::users_by_full_name::Variables { 
                        search: format!("%{}%" ,search)
                    };

                    let task = user_model::UsersByFullName::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            UserSelectMessage::Users(response)
                        },
                    );
                    self.task = Some(task);
                }
            }
            UserSelectMessage::Users(users) => {
                self.users = users.clone().and_then(|data| Some(data.user)).unwrap_or(vec![]).clone();

                if !users.clone().and_then(|data| Some(data.user)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
            }
            UserSelectMessage::SelectUser(select_option) => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.show_create = false;
                self.maybe_section_search = false;
                self.users = vec![];
                ctx.props().on_select.emit(select_option);
            }
            UserSelectMessage::OnFocus => {
                self.show_create = true;
                self.maybe_section_search = true;
            }
            UserSelectMessage::OnBlur => {
                self.show_create = false;
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.maybe_section_search = false;
                self.list_search_state = LoadSearch::Static;
            }
            UserSelectMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.users = vec![];
                self.list_search_state = LoadSearch::Static;
            }
        }
        should_render
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
        let on_focus = ctx.link().callback(move |_| UserSelectMessage::OnFocus);
        let on_blur = ctx.link().callback(move |_| UserSelectMessage::OnBlur);
        let on_hidden_modal = ctx.link().callback(move |_| UserSelectMessage::HiddenModal);

        let on_search = ctx.link().callback(|search: InputEvent| UserSelectMessage::FetchUsersByUserName(get_value_from_input_event(search)));

        let users = self
            .users
            .iter()
            .map(|user| {
                let user_id = UserId(user.id);
                let on_select = ctx
                    .link()
                    .callback(move |_| UserSelectMessage::SelectUser(UserSelectOption::User(user_id)));
                let full_name = user.user_profile.clone().unwrap().full_name;
                let uuid = user.user_profile.clone().and_then(|data| data.group_member).clone().and_then(|group| Some(group.group_id)).unwrap_or(Uuid::default());
                let _maybe_option = if GroupId(uuid) == GroupId(Uuid::default()) {
                    html! {
                        // <div class="m-4">
                        //     <div class="card vh-15">
                        //         <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                        //             <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                        //                 {&full_name}
                        //             </span>
                        //         </div>
                        //         <div class="card-body border-top d-flex px-5 py-2">
                        //             <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown=on_select>
                        //                 <span>
                        //                     {lang::dict("Add")}
                        //                 </span>
                        //             </a>
                        //         </div>
                        //     </div>
                        // </div>
                    }
                } else {
                    html! {
                        // <div class="m-4">
                        //     <div class="card vh-15">
                        //         <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                        //             <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                        //                 {&full_name}
                        //             </span>
                        //         </div>
                        //         <div class="card-body border-top d-flex px-5 py-2">
                        //             <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown=on_select>
                        //                 <span>
                        //                     {lang::dict("Add")}
                        //                 </span>
                        //             </a>
                        //         </div>
                        //     </div>
                        // </div>
                    }
                };
                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&full_name}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={on_select}>
                                    <span>
                                        {lang::dict("Add")}
                                    </span>
                                </a>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();
        let maybe_message_response = match self.list_search_state {
            LoadSearch::Static => {
                html! {
                    <div class="text-center">
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="ps-2">{lang::dict("Members")}</span></span>
                    </div>
                }
            },
            LoadSearch::Load(LoadSearchFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ users }</div>
                }
            },
            LoadSearch::Load(LoadSearchFound::NotFound) => {
                html! {
                    <div class="d-flex justify-content-center">
                        <span class="text-danger is-size-20 lh-20">{"No se encontró en "}{lang::dict("Members")}</span>
                    </div>
                }
            },
        };
        let class_search_modal = if self.maybe_section_search {
            "modal fade show"
        } else {
            "modal fade"
        };
        let class_search_scroll = if self.maybe_section_search {
            "display: block;"
        } else {
            "display: none;"
        };
        html! {
            <>
                <a class="button btn-create-card bg-primary-blue-dark d-flex align-items-center justify-content-center me-5" onclick={&on_hidden_modal}>
                    <span class="text-white noir-bold is-size-16 lh-20 d-flex align-items-center">
                        <i class="fas fa-plus me-2"></i>
                        <span>{lang::dict("New User")}</span>
                    </span>
                </a>
                <div class={class_search_modal} id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style={class_search_scroll} aria-modal="true" role="dialog">
                    <div class="modal-dialog modal-dialog-scrollable modal-xl">
                        <div class="modal-content">
                            <div class="modal-header">
                                <div class="input-group">
                                    <span class="input-group-text text-primary-blue-dark input-group-search">
                                        <i class="fas fa-search"></i>
                                    </span>
                                    <input type="text" class="form-control input-style-class" ref={self.search_node.clone()}
                                        oninput={on_search.clone()} onfocus={on_focus.clone()} onblur={on_blur.clone()} placeholder={lang::dict("Search")} />
                                </div>
                                <a class="btn bg-purple-on ms-5" onclick={&on_hidden_modal}>
                                    <span class="text-white">
                                        <i class="fas fa-times"></i>
                                    </span>
                                </a>
                            </div>
                            <div class="modal-body vh-100">
                                {maybe_message_response}
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}