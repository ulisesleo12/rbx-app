use log::*;
use uuid::Uuid;
use chrono::Local;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::post_model;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{PostId, GroupId, AppRoute, SchoolId, MyUserProfile, PageMode};

pub struct PostCard {
    link: ComponentLink<Self>,
    props: PostCardProperties,
    graphql_task: Option<GraphQLTask>,
    update_post_task: Option<RequestTask>,
    // task_messages: Option<SubscriptionTask>,
    // interactions_messages: Option<post_model::interactions_by_group_id_by_post_id::InteractionsByGroupIdByPostIdMessageGroupAggregate>,
    del_post_entirely_modal: bool,
    maybe_load_spinner: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PostCardProperties {
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub on_post_delete: Option<Callback<PostId>>,
    pub on_post_delete_entirely: Callback<PostId>,
    pub on_change_list: Callback<(PostId, bool, bool)>,
    pub topic: String,
    pub timestamp: String,
    pub timestamp_published: String,
    pub post_id: PostId,
    pub author_id: Uuid,
    pub author_pic_path: String,
    pub author_full_name: String,
    pub shares: i64,
    pub archived: bool,
    pub published: bool,
    pub on_dropdown_menu: bool,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum PostCardMessage {
    AppRoute(AppRoute),
    // FetchMessagesByPost(GroupId, PostId),
    // Interactiones(Option<post_model::interactions_by_group_id_by_post_id::ResponseData>),
    DeletePost(PostId),
    DeletePostEntirely(PostId),
    SaveDraftToggle(Option<post_model::update_post_group_options::ResponseData>),
    ArchivedToggle(Option<post_model::update_post_group_options::ResponseData>),
    ArchivedPost(PostId),
    PublishedPost(PostId),
    NoPublishedPost(PostId),
    OnDropdownMenu,
    OnDeletePostEntirely,
}

impl Component for PostCard {
    type Message = PostCardMessage;
    type Properties = PostCardProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // link.send_message(PostCardMessage::FetchMessagesByPost(props.group_id, props.post_id));
        PostCard {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            update_post_task: None,
            // task_messages: None,
            // interactions_messages: None,
            maybe_load_spinner: false,
            del_post_entirely_modal: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            PostCardMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            // PostCardMessage::FetchMessagesByPost(group_id, post_id) => {
            //     if let Some(graphql_task) = self.graphql_task.as_mut() {
            //         let vars = post_model::interactions_by_group_id_by_post_id::Variables { 
            //             post_id: post_id.0,
            //             group_id: group_id.0,
            //         };

            //         let task = post_model::InteractionsByGroupIdByPostId::subscribe(
            //             graphql_task,
            //             &self.link,
            //             vars,
            //             |response| {
            //                 PostCardMessage::Interactiones(response)
            //             },
            //         );
            //         self.task_messages = Some(task);
            //     }
            // }
            // PostCardMessage::Interactiones(interactions_messages) => {
            //     self.interactions_messages = interactions_messages.clone().and_then(|data| Some(data.message_group_aggregate));
            // }
            PostCardMessage::DeletePost(post_id) => {
                if let Some(on_post_delete) = &self.props.on_post_delete {
                    on_post_delete.emit(post_id);
                }
            }
            PostCardMessage::DeletePostEntirely(post_id) => {
                self.props.on_post_delete_entirely.emit(post_id);
                
                self.del_post_entirely_modal = false;
            }
            PostCardMessage::OnDeletePostEntirely => {
                self.del_post_entirely_modal = !self.del_post_entirely_modal;
            }
            PostCardMessage::SaveDraftToggle(response) => {
                let post_id = self.props.post_id;
                if response.clone().and_then(|data| data.update_post_group_by_pk).clone().and_then(|data| Some(data.published)).is_some() {

                    self.props.published = response.clone().and_then(|data| data.update_post_group_by_pk).clone().and_then(|data| Some(data.published)).unwrap_or(false);
                    self.props.archived = response.clone().and_then(|data| data.update_post_group_by_pk).clone().and_then(|data| Some(data.archived)).unwrap_or(false);
                    self.props.on_change_list.emit((post_id, self.props.published, self.props.archived));

                    self.maybe_load_spinner = false;
                }
            }
            PostCardMessage::ArchivedToggle(response) => {
                let post_id = self.props.post_id;
                if response.clone().and_then(|data| data.update_post_group_by_pk).clone().and_then(|data| Some(data.archived)).is_some() {

                    self.props.published = response.clone().and_then(|data| data.update_post_group_by_pk).clone().and_then(|data| Some(data.published)).unwrap_or(false);
                    self.props.archived = response.clone().and_then(|data| data.update_post_group_by_pk).clone().and_then(|data| Some(data.archived)).unwrap_or(false);
                    self.props.on_change_list.emit((post_id, self.props.published, self.props.archived));

                    self.maybe_load_spinner = false;
                }
            }
            PostCardMessage::ArchivedPost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let maybe_timestamp = Local::now().naive_local();

                    let vars = post_model::update_post_group_options::Variables { 
                        post_id: post_id.0,
                        group_id: self.props.group_id.0,
                        published: false,
                        archived: true,
                        maybe_timestamp,
                    };

                    let task = post_model::UpdatePostGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            PostCardMessage::ArchivedToggle(response)
                        },
                    );
                    self.update_post_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            PostCardMessage::PublishedPost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let maybe_timestamp = Local::now().naive_local();

                    let vars = post_model::update_post_group_options::Variables { 
                        post_id: post_id.0,
                        group_id: self.props.group_id.0,
                        published: true,
                        archived: false,
                        maybe_timestamp,
                    };

                    let task = post_model::UpdatePostGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            PostCardMessage::SaveDraftToggle(response)
                        },
                    );
                    self.update_post_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            PostCardMessage::NoPublishedPost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let maybe_timestamp = Local::now().naive_local();

                    let vars = post_model::update_post_group_options::Variables { 
                        post_id: post_id.0,
                        group_id: self.props.group_id.0,
                        published: false,
                        archived: false,
                        maybe_timestamp,
                    };

                    let task = post_model::UpdatePostGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            PostCardMessage::SaveDraftToggle(response)
                        },
                    );
                    self.update_post_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            PostCardMessage::OnDropdownMenu => {
                self.props.on_dropdown_menu = !self.props.on_dropdown_menu;
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        // if self.props.post_id != props.post_id {
            // }
            
        if self.props != props {
            self.props = props;
            // self.link.send_message(PostCardMessage::FetchMessagesByPost(self.props.group_id, self.props.post_id));
            should_render = true;
        } 

        should_render
    }

    fn view(&self) -> Html {
        let post_id = self.props.post_id;
        let group_id = self.props.group_id;
        let author_id = self.props.author_id;
        let topic = self.props.topic.clone();
        let timestamp = self.props.timestamp.clone();
        let timestamp_published = self.props.timestamp_published.clone();
        let author_pic_path = self.props.author_pic_path.clone();
        let author_full_name = self.props.author_full_name.clone();

        let on_archived_post = self.link.callback(move |_| PostCardMessage::ArchivedPost(post_id));        
        let on_published_post = self.link.callback(move |_| PostCardMessage::PublishedPost(post_id)); 
        let on_not_published_post = self.link.callback(move |_| PostCardMessage::NoPublishedPost(post_id));
        
        let on_show_del_post = self.link.callback(move |_| PostCardMessage::OnDeletePostEntirely);
        let on_del_post_entirely = self.link.callback(move |_| PostCardMessage::DeletePostEntirely(post_id));
        
        // let count_messages = self 
        //     .interactions_messages
        //     .as_ref()
        //     .and_then(|data| data.aggregate.as_ref())
        //     .and_then(|aggregate | {
        //         let count = aggregate.count;
        //         Some(html! {
        //             <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 flex-fill">
        //                 <img class="me-1" src="/icons/comments.svg" style="height: 18px;" />
        //                 <span class="ps-2">{format!("{:?}", count)}{lang::dict(" shares")}</span>
        //             </span>
        //         })
        //     })
        //     .unwrap_or(html! {});
        let school_id = self.props.school_id;
        let on_post = self
            .link
            .callback(move |_| PostCardMessage::AppRoute(AppRoute::Post(school_id, group_id, post_id, PageMode::Edit)));
        let on_post_view = self
            .link
            .callback(move |_| PostCardMessage::AppRoute(AppRoute::Post(school_id, group_id, post_id, PageMode::View)));

        let maybe_post_delete = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_post_delete = self
                    .link
                    .callback(move |_| PostCardMessage::DeletePost(post_id));
                if item.user_staff.is_some() {
                    Some(html! {
                        <li class="border-top">
                            <a class="dropdown-item drop-hover-filter text-purple-gray my-2" onclick={ &on_post_delete }>
                                <i class="fas fa-lock fas fa-lg me-2 ms-1"></i>
                                <span>{ lang::dict("Disguise") }</span>
                            </a>
                            <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={ &on_show_del_post }>
                                <img class="me-2" src="/icons/trash.svg" style="height: 22px;" />
                                <span>{ lang::dict("Remove") }</span>
                            </a>
                        </li>
                    })
                } else if item.user_teacher.is_some() && item.user_id.0 == author_id {
                    Some(html! {
                        <li class="border-top">
                            <a class="dropdown-item drop-hover-filter text-purple-gray my-2" onclick={ &on_post_delete }>
                                <i class="fas fa-lock fas fa-lg me-2 ms-1"></i>
                                <span>{ lang::dict("Disguise") }</span>
                            </a>
                            <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={ &on_show_del_post }>
                                <img class="me-2" src="/icons/trash.svg" style="height: 22px;" />
                                <span>{ lang::dict("Remove") }</span>
                            </a>
                        </li>
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {});       
        let maybe_published_draft = if self.props.published {
            html! {}
        } else {
            html! {
                <div class="saved-draft-container d-flex align-items-center justify-content-center ms-2">
                    <span class="text-white noir-bold is-size-12 lh-14">{ lang::dict("Draft Copy") }</span>
                </div>
            }
        };

        let maybe_option_icon_text = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    let no_maybe_published_icon_text = if self.props.published {
                        html! {
                            <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 justify-content-lg-end flex-fill">
                                <i class="far fa-eye me-1"></i>
                                <span>{ lang::dict("Published") }</span>
                            </span>
                        }
                    } else {
                        html! {
                            // <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 justify-content-lg-end flex-fill">
                            //     <i class="far fa-eye-slash me-1"></i>
                            //     <span>{lang::dict("Not published")}</span>
                            // </span>
                        }
                    };
                    let maybe_published_icon_text = if self.props.published {
                        html! {
                            // <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 justify-content-lg-end flex-fill">
                            //     <i class="far fa-eye me-1"></i>
                            //     <span>{lang::dict("Published")}</span>
                            // </span>
                        }
                    } else {
                        html! {
                            <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 justify-content-lg-end flex-fill">
                                <i class="far fa-eye-slash me-1"></i>
                                <span>{lang::dict("Not published")}</span>
                            </span>
                        }
                    };
                    Some(html! {
                        <>
                            { no_maybe_published_icon_text }
                            { maybe_published_icon_text }
                        </>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let spinner = if self.maybe_load_spinner {
            html! {
                <div class="text-center text-purple-gray">
                    <div class="spinner-border" role="status">
                        // <span class="visually-hidden">Loading...</span>
                    </div>
                </div>
            }
        } else {
            html! {}
        };

        let published_option_btn = if self.props.published {
            html! {
                <li class="my-1">   
                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick=on_not_published_post>
                        <img class="me-2" src="/icons/upload.svg" style="height: 25px;" />
                        <span>{lang::dict("Do Not Post")}</span>
                    </a>
                </li>
            }
        } else {
            html! {                        
                <li class="my-1">   
                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick=on_published_post>
                        <img class="me-2" src="/icons/upload.svg" style="height: 25px;" />
                        <span>{lang::dict("To Post")}</span>
                    </a>
                </li>
            }
        };

        
        let dropdown_menu = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                let on_dropdown = self
                    .link
                    .callback( move |_| PostCardMessage::OnDropdownMenu);
                let maybe_menu = if self.props.on_dropdown_menu {
                    "btn btn-outline-purple-gray dropdown-toggle menu-hidden-toggle border-0 show"
                } else {
                    "btn btn-outline-purple-gray dropdown-toggle menu-hidden-toggle border-0"
                };
                let maybe_item = if self.props.on_dropdown_menu {
                    "dropdown-menu show"
                } else {
                    "dropdown-menu"
                };
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown">
                            <a class={ maybe_menu } onclick={ on_dropdown } role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                <i class="fas fa-ellipsis-v"></i>
                            </a>
                            <ul class=maybe_item aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
                                <li class="my-1">   
                                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={ &on_post }>
                                        <i class="fas fa-edit fas fa-lg me-2 ms-1"></i>
                                        <span>{lang::dict("Edit")}</span>
                                    </a>
                                </li>
                                {
                                    if self.maybe_load_spinner {
                                        { spinner }
                                    } else {
                                        { published_option_btn }
                                    }
                                }
                                <li class="my-1">   
                                    <a class="dropdown-item drop-hover-filter text-purple-gray" onclick={ on_archived_post }>
                                        <img class="me-2" src="/icons/archive.svg" style="height: 25px;" />
                                        <span>{ lang::dict("File") }</span>
                                    </a>
                                </li>
                                { maybe_post_delete }
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let class_del_show = if self.del_post_entirely_modal {
            "modal fade show"
        } else {
            "modal fade"
        };

        let style_del_display = if self.del_post_entirely_modal {
            "display: block;"
        } else {
            "display: none;"
        };

        let modal_del_post_entirely = if self.del_post_entirely_modal {
            html! {
                <div class={ class_del_show } style={ style_del_display } id="staticBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                    <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header d-flex justify-content-center">
                            <h1 class="modal-title noir-bold fs-5" id="staticBackdropLabel">{"Borrar Publicación"}</h1>
                        </div>
                        <div class="modal-body text-center">
                            <span class="text-primary-blue-dark noir-medium is-size-16 lh-22">{"Para borrar la publicacón por completo, presione "}
                                <span class="noir-bold">{"confirmar"}</span></span>
                        </div>
                        <div class="modal-footer">
                            <button type="button" class="btn btn-outline-purple-on noir-medium" onclick={ &on_show_del_post } data-bs-dismiss="modal">{"Cancelar"}</button>
                            <button type="button" class="btn btn-outline-primary-blue-dark noir-medium" onclick={ &on_del_post_entirely }>{"Confirmar"}</button>
                        </div>
                    </div>
                    </div>
                </div>
            }
        } else {
            html! {}
        };

        let maybe_datetime = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 flex-fill">
                            <i class="far fa-clock me-1"></i>
                            <span>{ &timestamp }</span>
                        </span>
                    })
                } else {
                    Some(html! {
                        <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 flex-fill">
                            <i class="far fa-clock me-1"></i>
                            <span>{ &timestamp_published }</span>
                        </span>
                    })
                }
            })
            .unwrap_or(html! {});

        html! {
            <>
                <div class="card-post-view bg-white d-flex flex-column justify-content-between p-4 mb-4 w-100" key={ post_id.to_string() }>
                    <div class="d-flex align-items-center justify-content-between">
                        <a onclick={ &on_post_view }>
                            <div class="d-flex flex-wrap">
                                <div class="module-message-universal-2 line-clamp-message-universal">
                                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">{ &topic }</span>
                                </div>
                                { maybe_published_draft }
                            </div>
                        </a>
                        { dropdown_menu }
                    </div>
                    <a class="d-flex flex-wrap align-items-center justify-content-between" onclick={ &on_post_view }>
                        <div class="d-flex flex-row align-items-center justify-content-start col-6 col-sm-6 col-md-2 col-lg-3">
                            // <img class="img-card-32" src={ author_pic_path.clone() } />
                            // <span class="text-dark noir-light is-size-14 lh-17 ms-1">{ &author_full_name }</span>
                            <img class="img-card-32" src={ author_pic_path.clone() } />
                            <span class="text-dark noir-light is-size-14 lh-17 ms-1">{ &author_full_name }</span>
                        </div>
                        // <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 flex-fill">
                        //     <i class="far fa-clock me-1"></i>
                        //     <span>{&timestamp}</span>
                        // </span>
                        { maybe_datetime }
                        <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 flex-fill">
                            <img class="me-1" src="/icons/comments.svg" style="height: 18px;" />
                            <span class="ps-2">{ self.props.shares.to_string()}{lang::dict(" shares") }</span>
                        </span>
                        // <span class="d-flex align-items-center text-purple-gray noir-light is-size-14 lh-17 flex-fill">
                        //     <i class="far fa-file-alt me-1"></i>
                        //     <span>{"0 archivos adjuntos"}</span>
                        // </span>
                        { maybe_option_icon_text }
                    </a>
                </div>
                { modal_del_post_entirely }
            </>
        }
    }
}