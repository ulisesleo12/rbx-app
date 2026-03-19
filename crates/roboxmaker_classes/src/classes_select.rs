use log::*;
use std::vec;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_models::classes_model;
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{ClassesId, GroupId, AppRoute, SchoolId, MyUserProfile};


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
pub enum ClassesSelectOption {
    Classes(ClassesId),
}
pub struct ClassesSelect {
    graphql_task: Option<GraphQLTask>,
    classes_task: Option<RequestTask>,
    classes: Vec<classes_model::classes_by_name::ClassesByNameClasses>,
    search_node: NodeRef,
    show_create: bool,
    maybe_section_search: bool,
    list_search_state: LoadSearch,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ClassesSelectProperties {
    pub on_select: Callback<ClassesSelectOption>,
    pub user_profile: Option<MyUserProfile>,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum ClassesSelectMessage {
    FetchClassesByClassesName(String),
    Classes(Option<classes_model::classes_by_name::ResponseData>),
    SelectClasses(ClassesSelectOption),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for ClassesSelect {
    type Message = ClassesSelectMessage;
    type Properties = ClassesSelectProperties;

    fn create(_ctx: &Context<Self>) -> Self {

        ClassesSelect {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            classes_task: None,
            classes: vec![],
            search_node: NodeRef::default(),
            show_create: false,
            maybe_section_search: false,
            list_search_state: LoadSearch::Static,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            ClassesSelectMessage::FetchClassesByClassesName(search) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = classes_model::classes_by_name::Variables {
                        search: format!("%{}%", search)
                    };

                    let task = classes_model::ClassesByName::request(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                ClassesSelectMessage::Classes(response)
                            },
                    );
                    self.classes_task = Some(task);
                }
            }
            ClassesSelectMessage::Classes(classes) => {
                self.classes = classes.clone().and_then(|data| Some(data.classes)).unwrap_or(vec![]);

                if !classes.clone().and_then(|data| Some(data.classes)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
            }
            ClassesSelectMessage::SelectClasses(select_option) => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                self.show_create = false;
                self.maybe_section_search = false;
                self.classes = vec![];
                ctx.props().on_select.emit(select_option);
            }
            ClassesSelectMessage::OnFocus => {
                self.show_create = true;
                self.maybe_section_search = true;
            }
            ClassesSelectMessage::OnBlur => {
                self.show_create = false;
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                // self.classes = vec![];
                self.list_search_state = LoadSearch::Static;
            }
            ClassesSelectMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.classes = vec![];
                self.list_search_state = LoadSearch::Static;
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_focus = ctx.link().callback(move |_| ClassesSelectMessage::OnFocus);
        let on_blur = ctx.link().callback(move |_| ClassesSelectMessage::OnBlur);
        let on_hidden_modal = ctx.link().callback(move |_| ClassesSelectMessage::HiddenModal);

        let on_search = ctx.link().callback(|search: InputEvent| ClassesSelectMessage::FetchClassesByClassesName(get_value_from_input_event(search)));
        
        let group_id = ctx.props().group_id;
        let classes = self
            .classes
            .iter()
            .map(|classes| {
                let classes_id = ClassesId(classes.id);
                let on_select = ctx.link().callback(move |_| {
                    ClassesSelectMessage::SelectClasses(ClassesSelectOption::Classes(classes_id))
                });
                let school_id = ctx.props().school_id;

                let navigator = ctx.link().navigator().unwrap();
                let on_classes = Callback::from(move |_| navigator.push(&AppRoute::Classes{school_id, group_id, classes_id}));

                let topic = classes.classes_profile.clone().unwrap().topic;
                let group_uuid = classes.classes_profile.clone().and_then(|data| data.classes_group.clone().and_then(|data| Some(data.group_id))).unwrap_or(Uuid::default());
                let maybe_option = if ctx.props().group_id == GroupId(group_uuid) {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={on_classes}>
                            <span>
                                {lang::dict("View")}
                            </span>
                        </a>
                    }
                } else {
                    html! {
                        <a class="btn btn-outline-secondary btn-sm mx-auto" onmousedown={on_select}>
                            <span>
                                {lang::dict("Add")}
                            </span>
                        </a>
                    }
                };
                html! {
                    <div class="m-4">
                        <div class="card card-search-u vh-15">
                            <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                    {&topic}
                                </span>
                            </div>
                            <div class="card-body border-top d-flex px-5 py-2">
                                {maybe_option}
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();
        let _maybe_option_user = ctx.props().user_profile.as_ref().and_then(|user| {
            if user.user_staff.is_some() || user.user_teacher.is_some() {
                Some(html! {
                    <span class="title is-6 text-white text-center">{"Todas las Clases"}</span>
                })
            } else {
                Some(html! {
                    <span class="title is-6 text-white text-center">{"Clases del grupo"}</span>
                })
            }
        }).unwrap_or(html! {});
        let maybe_message_response = match self.list_search_state {
            LoadSearch::Static => {
                html! {
                    <div class="text-center">
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="pl-2">{lang::dict("Classes")}</span></span>
                    </div>
                }
            },
            LoadSearch::Load(LoadSearchFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ classes }</div>
                }
            },
            LoadSearch::Load(LoadSearchFound::NotFound) => {
                html! {
                    <div class="d-flex justify-content-center">
                        <span class="text-danger is-size-20 lh-20">{"No se encontró en "}{lang::dict("Classes")}</span>
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
                <a class="button-search-univeral mt-3" onclick={&on_hidden_modal}>
                    <span class="icon-text-search-universal">
                        <span>{lang::dict("Search")}</span>
                        <span class="icon">
                            <i class="fas fa-search"></i>
                        </span>
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
