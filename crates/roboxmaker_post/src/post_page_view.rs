use chrono::Local;
use log::*;
use roboxmaker_quizzes::quizresponder;
use uuid::Uuid;
use std::vec;
use yew::prelude::*;
use web_sys::window;
use yew::services::Task;
use std::time::Duration;
use gloo_storage::Storage;
use yew::virtual_dom::VNode;
use yew::web_sys::{Node, self};
use code_location::code_location;
use yew::services::interval::IntervalService;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
// use roboxmaker_message::message_list_post::MessageListPost;
use roboxmaker_models::{school_model, post_model};
use roboxmaker_searches::search_posts_group::SearchPostsGroup;
use roboxmaker_message::{message_list::MessageList, MessageGroupCategory};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask, Subscribe, SubscriptionTask};
use roboxmaker_types::types::{PostId, SchoolId, GroupId, AppRoute, ClassGroupPost, ClassGroupCategory, UserId, MyUserProfile};

#[derive(Debug)]
pub enum PostPageViewEdit {
    None,
    Edit,
    Done,
    Save,
}
pub struct PostPageView {
    link: ComponentLink<Self>,
    props: PostPageViewProperties,
    graphql_task: Option<GraphQLTask>,
    load_task: Option<SubscriptionTask>,
    post_task: Option<RequestTask>,
    task_save: Option<RequestTask>,
    task_delete: Option<RequestTask>,
    post: Option<post_model::post_by_id::PostByIdPostGroupByPk>,
    node: Option<Node>,
    edit: PostPageViewEdit,
    topic: String,
    content: String,
    save_status: bool,
    job: Option<Box<dyn Task>>,
    on_dropdown_menu: bool,
    class_name: String,
    del_post_entirely_modal: bool,
    maybe_load_spinner: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PostPageViewProperties {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub post_id: PostId,
    pub group_id: GroupId,
    pub posts: Option<ClassGroupPost>,
    pub on_list_change: Option<Callback<()>>,
    pub saved_sidebar_state: bool,
    pub school_id: SchoolId,
    pub pic_path: String,
}

#[derive(Debug)]
pub enum PostPageViewMessage {
    AppRoute(AppRoute),
    StartAutoSave,
    StopAutoSave,
    FetchPostById(PostId, GroupId),
    Post(Option<post_model::post_by_id::ResponseData>),
    DeletePostById(PostId),
    PostDeleted(Option<post_model::post_group_delete::ResponseData>),
    Edit(PostPageViewEdit),
    Content(String),
    Topic(String),
    Saved(Option<post_model::post_by_id_update::ResponseData>),
    Back,
    SaveDraftToggle(Option<post_model::update_post_group_options::ResponseData>),
    ArchivedToggle(Option<post_model::update_post_group_options::ResponseData>),
    ArchivedPost(PostId),
    PublishedPost(PostId),
    NoPublishedPost(PostId),
    OnDropdownMenu,
    ChangeSidebarState,
    DeletePostEntirely(PostId),
    PostDeletedEnt(Option<post_model::delete_post_by_id::ResponseData>),
    OnDeletePostEntirely,
}

impl Component for PostPageView {
    type Message = PostPageViewMessage;
    type Properties = PostPageViewProperties;

    fn create(mut props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(PostPageViewMessage::FetchPostById(props.post_id, props.group_id));
        props.saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };

        PostPageView {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            load_task: None,
            post_task: None,
            task_save: None,
            task_delete: None,
            post: None,
            node: None,
            edit: PostPageViewEdit::None,
            topic: String::from(""),
            content: String::from(""),
            save_status: true,
            job: None,
            on_dropdown_menu: false,
            class_name: String::from(""),
            del_post_entirely_modal: false,
            maybe_load_spinner: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            PostPageViewMessage::Content(_) => trace!("{:?}", msg),
            _ => info!("{:?}", msg),
        }
        let mut should_update = true;
        match msg {
            PostPageViewMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
                should_update = true;
            }
            PostPageViewMessage::StartAutoSave => {
                let handle = IntervalService::spawn(
                    Duration::from_secs(600),
                    self.link
                        .callback(|_| PostPageViewMessage::Edit(PostPageViewEdit::Save)),
                );
                self.job = Some(Box::new(handle));
                should_update = true;
            }
            PostPageViewMessage::StopAutoSave => {
                self.job = None;
            }
            PostPageViewMessage::FetchPostById(post_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = post_model::post_by_id::Variables { 
                        post_id: post_id.0,
                        group_id: group_id.0,
                    };

                    let task = post_model::PostById::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            PostPageViewMessage::Post(response)
                        },
                    );
                    self.load_task = Some(task);
                }
            }
            PostPageViewMessage::Post(response) => {
                self.post = response.clone().and_then(|data| data.post_group_by_pk);

                self.edit = PostPageViewEdit::None;
                if let Some(post) = &self.post {

                    self.class_name = post.clone().class_profile.and_then(|data| data.class_profile).and_then(|class_profile| Some(class_profile.name)).unwrap_or("".to_string());

                    if let Some(post_profile) = &post.post_profile {
                        self.topic = post_profile.topic.clone();
                    }

                    if let Some(post_content) = &post.post_content {
                        self.content = post_content.content.clone();
                        let node = web_sys::window()
                            .and_then(|window| window.document())
                            .and_then(|document| document.create_element("div").ok())
                            .and_then(|div| {
                                div.set_class_name("ck-content");
                                div.set_inner_html(&post_content.content);
                                Some(Node::from(div))
                            });
                        self.node = node;
                    }
                    if response.is_some() {
                        self.maybe_load_spinner = false;
                    }
                }

                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                
                if response.clone().and_then(|data| data.post_group_by_pk).is_none() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(PostPageViewMessage::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Posts)));
                    } else {
                        
                        self.link.send_message(PostPageViewMessage::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::Posts)));
                    }
                }
            }
            PostPageViewMessage::DeletePostById(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = post_model::post_group_delete::Variables { 
                        group_id: self.props.group_id.0,
                        post_id: post_id.0,
                    };

                    let task = post_model::PostGroupDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            PostPageViewMessage::PostDeleted(response)
                        },
                    );
                    self.task_delete = Some(task);
                }
            }
            PostPageViewMessage::PostDeleted(lesson_deleted) => {
                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                if lesson_deleted.clone().and_then(|data| data.delete_post_group).is_some() {

                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(PostPageViewMessage::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Posts)));
                    } else {
                        
                        self.link.send_message(PostPageViewMessage::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::Posts)));
                    }

                    info!("{:?} del", lesson_deleted);
                }
                should_update = true;
            }
            PostPageViewMessage::Topic(topic) => {
                self.topic = topic;
                self.save_status = false;
                should_update = true;
            }
            PostPageViewMessage::Content(content) => {
                self.content = content;
                self.save_status = false;
                should_update = true;
            }
            PostPageViewMessage::Edit(edit) => {
                self.edit = edit;
                match self.edit {
                    PostPageViewEdit::Edit => self.link.send_message(PostPageViewMessage::StartAutoSave),
                    PostPageViewEdit::None => {
                        self.save_status = true;
                        self.link.send_message(PostPageViewMessage::StopAutoSave)
                    }
                    PostPageViewEdit::Done => {
                        self.save_status = true;
                        self.link.send_message(PostPageViewMessage::StopAutoSave)
                    }
                    PostPageViewEdit::Save => {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let vars = post_model::post_by_id_update::Variables { 
                                post_id: self.props.post_id.0,
                                post_topic: self.topic.clone(),
                                post_content: self.content.clone(),
                            };
        
                            let task = post_model::PostByIdUpdate::request(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    PostPageViewMessage::Saved(response)
                                },
                            );
                            self.task_save = Some(task);
                        }
                        self.edit = PostPageViewEdit::None;
                        should_update = true;
                    }
                }
            }
            PostPageViewMessage::Saved(profile) => {
                self.link
                    .send_message(PostPageViewMessage::Edit(PostPageViewEdit::Edit));
                if profile.is_some() {
                    self.save_status = true;
                }
            }
            PostPageViewMessage::Back => {
                let _ = window().expect("no windows").window().history().unwrap().back();
            }
            PostPageViewMessage::SaveDraftToggle(response) => {
                if let Some(mut posts) = self.props.posts.clone() {
                    posts.published = response.clone().and_then(|data| data.update_post_group_by_pk).clone().and_then(|data| Some(data.published)).unwrap_or(false);
                } 
            }
            PostPageViewMessage::ArchivedToggle(response) => {
                if let Some(mut posts) = self.props.posts.clone() {
                    posts.archived = response.clone().and_then(|data| data.update_post_group_by_pk).clone().and_then(|data| Some(data.archived)).unwrap_or(false);
                }
            }
            PostPageViewMessage::ArchivedPost(post_id) => {
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
                            PostPageViewMessage::ArchivedToggle(response)
                        },
                    );
                    self.post_task = Some(task);
                }
            }
            PostPageViewMessage::PublishedPost(post_id) => {
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
                            PostPageViewMessage::SaveDraftToggle(response)
                        },
                    );
                    self.post_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            PostPageViewMessage::NoPublishedPost(post_id) => {
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
                            PostPageViewMessage::SaveDraftToggle(response)
                        },
                    );
                    self.post_task = Some(task);
                    self.maybe_load_spinner = true;
                }
            }
            PostPageViewMessage::OnDropdownMenu => {
                self.on_dropdown_menu = !self.on_dropdown_menu;
            }
            PostPageViewMessage::ChangeSidebarState => {
                if let Some(element) = gloo_utils::document().get_element_by_id("show-sidebar-right") {
                    if self.props.saved_sidebar_state {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", false);
                        self.props.saved_sidebar_state = false;
                        let _ = element.set_attribute("class", "fa-angle-double-left fa-w-14 fa-2x");
                    } else {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", true);
                        self.props.saved_sidebar_state = true;
                        let _ = element.set_attribute("class", "fa fa-angle-double-right fa-w-14 fa-2x");
                    }
                }
            }
            PostPageViewMessage::DeletePostEntirely(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = post_model::delete_post_by_id::Variables { 
                        post_id: post_id.0,
                    };

                    let task = post_model::DeletePostById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            PostPageViewMessage::PostDeletedEnt(response)
                        },
                    );
                    self.task_delete = Some(task);
                }
            }
            PostPageViewMessage::PostDeletedEnt(response) => {
                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                if response.clone().and_then(|data| data.delete_post_by_pk).is_some() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(PostPageViewMessage::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Posts)));
                    } else {
                        self.link.send_message(PostPageViewMessage::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::Posts)));
                    }
                    
                    info!("{:?} del", response);
                }
            }
            PostPageViewMessage::OnDeletePostEntirely => {
                self.del_post_entirely_modal = !self.del_post_entirely_modal
            }
        };
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            should_render = true;
        }
        should_render
    }

    fn view(&self) -> Html {
        let on_show_del_post = self.link.callback(move |_| PostPageViewMessage::OnDeletePostEntirely);

        let on_show_sidebar = self.link.callback(move |_| PostPageViewMessage::ChangeSidebarState);
        
        let btn_sidebar_show = if self.props.saved_sidebar_state {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick=&on_show_sidebar>
                    <i class="fas fa-angle-double-right fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        } else {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick=&on_show_sidebar>
                    <i class="fas fa-angle-double-left fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        };
        let post_id = self.props.post_id;
        let group_id = self.props.group_id;
        let on_archived_post = self.link.callback(move |_| PostPageViewMessage::ArchivedPost(post_id));        
        let on_published_post = self.link.callback(move |_| PostPageViewMessage::PublishedPost(post_id)); 
        let on_no_published_post = self.link.callback(move |_| PostPageViewMessage::NoPublishedPost(post_id)); 

        let on_del_post_entirely = self.link.callback(move |_| PostPageViewMessage::DeletePostEntirely(post_id));


        let author_post = self
            .post
            .as_ref()
            .and_then(|post | post.post_profile.as_ref())
            .and_then(|post_profile| {
                let author_profile = post_profile.author_profile.as_ref().unwrap();
                let pic_path = author_profile.pic_path.clone().unwrap_or("https://files.roboxmaker.network/uploads/avatar.png".to_owned());
                Some(html! {
                    <div class="d-flex flex-wrap align-items-center justify-content-between mb-6">
                        <div class="d-flex align-items-center">
                            <img class="img-card-32" src=pic_path alt="" />
                            <span class="text-dark noir-light is-size-18 lh-22 ps-2">{&author_profile.full_name}</span>
                        </div>
                        <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                            <span class="icon">
                                <i class="far fa-clock"></i>
                            </span>
                            <span class="ps-2">{&post_profile.timestamp.format("%a %e %b %Y %T").to_string()}</span>
                        </span>
                        <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                            <span class="icon">
                                <i class="fas fa-graduation-cap"></i>
                            </span>
                            <span class="ps-2">{self.class_name.clone()}</span>
                        </span>
                    </div>
                })
            })
            .unwrap_or(html! {});
        let status_save = if self.save_status {
            html! {
                <span class="text-success mx-4">{lang::dict("Saved")}</span>
            }
        } else {
            html! {
                <span class="text-danger mx-4">{lang::dict("Unsaved")}</span>
            }
        };
        if let Some(post) = &self.post {
            let maybe_edit_options = match self.edit {
                PostPageViewEdit::Edit => {
                    let on_done = self
                        .link
                        .callback(move |_| PostPageViewMessage::Edit(PostPageViewEdit::Done));
                    let on_save = self
                        .link
                        .callback(move |_| PostPageViewMessage::Edit(PostPageViewEdit::Save));
                    let on_data = self
                        .link
                        .callback(|data: InputData| PostPageViewMessage::Topic(data.value));
                    html! {
                        <>
                            <input class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={lang::dict("Publication Title")} value=self.topic.clone() oninput=on_data />
                            <a class="btn button-cancel-lesson px-4 mx-3 d-flex align-items-center justify-content-center" onclick=on_done>
                                <span class="text-white">
                                    <i class="fas fa-times fas fa-lg"></i>
                                </span>
                            </a>
                            {status_save}
                            <a class="btn button-save-lesson px-4 mx-3 d-flex align-items-center justify-content-center" onclick=on_save>
                                <span class="text-white">
                                    <i class="fas fa-cloud-upload-alt fas fa-lg"></i>
                                </span>
                            </a>
                        </>
                    }
                }
                PostPageViewEdit::Done => {
                    html! {
                        <nav class="level">
                            <div class="level-left">
                                <div class="level-item">
                                    <button class="button is-white is-rounded">
                                        <span class="icon">
                                            <i class="fas fa-spinner fa-pulse"></i>
                                        </span>
                                    </button>
                                </div>
                            </div>
                        </nav>
                    }
                }
                PostPageViewEdit::Save => {
                    html! {
                        <nav class="level">
                            <div class="level-left">
                                <div class="level-item">
                                </div>
                            </div>
                            <div class="level-item">
                                <progress class="progress is-small is-primary" max="100"></progress>
                            </div>
                            <div class="level-right">
                                <div class="level-item">
                                    <button class="button is-white is-rounded">
                                        <span class="icon">
                                            <i class="fas fa-spinner fa-pulse"></i>
                                        </span>
                                    </button>
                                </div>
                            </div>
                        </nav>
                    }
                }
                PostPageViewEdit::None => {
                    let maybe_post_edit = self
                        .props
                        .user_profile
                        .as_ref()
                        .zip(
                            self.post
                                .as_ref()
                                .and_then(|post| post.post_profile.as_ref()),
                        )
                        .and_then(|(item, post_profile)| {
                            let post_id = post_profile.post_id;
                            let on_edit = self
                                .link
                                .callback(move |_| PostPageViewMessage::Edit(PostPageViewEdit::Edit));
                            let on_delete = self
                                .link
                                .callback( move |_| PostPageViewMessage::DeletePostById(PostId(post_id)));

                            let on_dropdown = self
                                .link
                                .callback( move |_| PostPageViewMessage::OnDropdownMenu);
                            let maybe_menu = if self.on_dropdown_menu {
                                "btn btn-outline-secondary dropdown-toggle menu-hidden-toggle show border-0"
                            } else {
                                "btn btn-outline-secondary dropdown-toggle menu-hidden-toggle border-0"
                            };

                            let maybe_item = if self.on_dropdown_menu {
                                "dropdown-menu show"
                            } else {
                                "dropdown-menu"
                            };

                            let maybe_no_publish = if post.published {
                                html! {
                                    <li>   
                                        <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={&on_no_published_post}>
                                            <i class="fas fa-upload me-2"></i>
                                            <span>{lang::dict("Do Not Post")}</span>
                                        </a>
                                    </li>
                                }
                            } else {
                                html! {}
                            };
                            let maybe_publish = if post.published {
                                html! {}
                            } else {
                                html! {
                                <li>   
                                    <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={&on_published_post}>
                                        <i class="fas fa-upload me-2"></i>
                                        <span>{lang::dict("To Post")}</span>
                                    </a>
                                </li>
                                }
                            };

                            let spinner = if self.maybe_load_spinner {
                                html! {
                                    <div class="text-center text-gray-purple-two">
                                        <div class="spinner-border" role="status">
                                        </div>
                                    </div>
                                }
                            } else {
                                html! {}
                            };

                            let maybe_delete_post = if item.user_staff.is_some() {
                                html! {
                                    <li class="border-top">
                                        <a class="dropdown-item drop-hover-filter text-gray-purple-two mt-2" onclick={&on_delete}>
                                            <i class="fas fa-lock me-2"></i>
                                            <span>{lang::dict("Disguise")}</span>
                                        </a>
                                        <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={&on_show_del_post}>
                                            <i class="fas fa-trash me-2"></i>
                                            <span>{lang::dict("Remove")}</span>
                                        </a>
                                    </li>
                                }
                            } else if item.user_teacher.is_some() && item.user_id.0 == post_profile.author_id {
                                html! {
                                    <li class="border-top">
                                        <a class="dropdown-item drop-hover-filter text-gray-purple-two mt-2" onclick={&on_delete}>
                                            <i class="fas fa-lock me-2"></i>
                                            <span>{lang::dict("Disguise")}</span>
                                        </a>
                                        <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={&on_show_del_post}>
                                            <i class="fas fa-trash me-2"></i>
                                            <span>{lang::dict("Remove")}</span>
                                        </a>
                                    </li>
                                }
                            } else {
                                html! {}
                            };

                            if item.user_staff.is_some() || item.user_teacher.is_some() {
                                Some(html! {
                                    <>
                                        <div class="dropdown">
                                            <a class=maybe_menu onclick=on_dropdown role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                                <i class="fas fa-ellipsis-v"></i>
                                            </a>
                                            <ul class=maybe_item aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
                                                <li>   
                                                    <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick=on_edit>
                                                        <i class="fas fa-edit me-2"></i>
                                                        <span>{lang::dict("Edit")}</span>
                                                    </a>
                                                </li>
                                                // <li>   
                                                //     <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={&on_published_post}>
                                                //         <i class="fas fa-upload me-2"></i>
                                                //         <span>{lang::dict("To Post")}</span>
                                                //     </a>
                                                // </li>
                                                {
                                                    if self.maybe_load_spinner {
                                                        {spinner}
                                                    } else {
                                                        html! {
                                                            <>
                                                                {maybe_no_publish}
                                                                {maybe_publish}
                                                            </>
                                                        }
                                                    }
                                                }
                                                <li>   
                                                    <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick=on_archived_post>
                                                        <i class="fas fa-archive me-2"></i>
                                                        <span>{lang::dict("File")}</span>
                                                    </a>
                                                </li>
                                                {maybe_delete_post}
                                                // <li class="border-top">
                                                //     <a class="dropdown-item drop-hover-filter text-gray-purple-two mt-2" onclick={&on_delete}>
                                                //         <i class="fas fa-lock me-2"></i>
                                                //         <span>{lang::dict("Disguise")}</span>
                                                //     </a>
                                                //     <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={&on_show_del_post}>
                                                //         <i class="fas fa-trash me-2"></i>
                                                //         <span>{lang::dict("Remove")}</span>
                                                //     </a>
                                                // </li>
                                            </ul>
                                        </div>
                                    </>
                                })
                            } else {
                                None
                            }
                        })
                        .unwrap_or(html! {});

                    html! {
                        <>
                            <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">{ &self.topic }</h1>
                            {maybe_post_edit}
                        </>
                    }
                }
            };

            let maybe_post_content = {
                let on_data = self
                    .link
                    .callback(move |data| PostPageViewMessage::Content(data));
                let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
                match self.edit {
                    PostPageViewEdit::Edit => {
                        html! {
                            <div class="mb-6" style="border: 1px solid #C8C1CD; border-radius: 10px;">
                                <ckeditor::CKEditor user_profile=self.props.user_profile.clone()
                                    content=self.content.clone()
                                    upload_url=upload_url on_data=on_data />
                            </div>
                        }
                    }
                    _ => {
                        html! {
                            <span class="text-dark noir-light is-size-18 lh-22">
                                {VNode::VRef(self.node.clone().unwrap())}
                            </span>
                        }
                    }
                }
            };

            let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

            let school_id = self.props.school_id;
            let on_class_group_posts = self.link.callback(move |_| {
                PostPageViewMessage::AppRoute(AppRoute::SchoolGroupSection(
                    school_id.clone(),
                    group_id.clone(),
                    ClassGroupCategory::Posts,
                ))
            });
            let on_class_group_posts_st = self.link.callback(move |_| {
                PostPageViewMessage::AppRoute(AppRoute::GroupSectionStudent(
                    school_id.clone(),
                    user_id.clone(),
                    ClassGroupCategory::Posts,
                ))
            });

            let go_back_grade = self.props.user_profile.clone()
            .and_then(|item| {
                if item.user_teacher.is_some() || item.user_staff.is_some() {
                    Some(html! {
                        <a onclick=on_class_group_posts>
                            <span class="icon-text text-gray-strong noir-medium is-size-16 lh-19 d-flex align-items-center">
                                <span class="icon">
                                    <i class="fas fa-arrow-left"></i>
                                </span>
                                <span class="mx-2">{lang::dict("To Publications")}</span>
                                {self.class_name.clone()}
                            </span>
                        </a>
                    })
                } else {
                    Some(html! {
                        <a onclick=on_class_group_posts_st>
                            <span class="icon-text text-gray-strong noir-medium is-size-16 lh-19 d-flex align-items-center">
                                <span class="icon">
                                    <i class="fas fa-arrow-left"></i>
                                </span>
                                <span class="mx-2">{lang::dict("To Publications")}</span>
                                {self.class_name.clone()}
                            </span>
                        </a>
                    })
                }
            }).unwrap_or(html! {});
            let pic_path = self.props.pic_path.clone();

            let class_right_sidebar = if self.props.saved_sidebar_state {
                "bg-silver col col-sm-3 col-md-3 col-lg-5 col-xl-4 col-xxl-3 d-none d-sm-none d-md-none d-lg-block p-5"
            } else {
                "d-none"
            };

            let class_sidebar_mobile = if self.props.saved_sidebar_state {
                "offcanvas offcanvas-end show bg-silver d-block d-sm-block d-md-block d-lg-none d-xl-none d-xxl-none"
            } else {
                "offcanvas offcanvas-end"
            };
            
            let style_sidebar_mobile = if self.props.saved_sidebar_state {
                "visibility: visible;"
            } else {
                "display: none;"
            };


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
    
            let modal_del_lesson_entirely = if self.del_post_entirely_modal {
                html! {
                    <div class={class_del_show} style={style_del_display} id="staticBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
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
                                <button type="button" class="btn btn-outline-purple-on noir-medium" onclick={&on_show_del_post} data-bs-dismiss="modal">{"Cancelar"}</button>
                                <button type="button" class="btn btn-outline-primary-blue-dark noir-medium" onclick={&on_del_post_entirely}>{"Confirmar"}</button>
                            </div>
                        </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            };

            if let Some(quiz_json) = self.post.as_ref().and_then(|data| data.post_profile.as_ref()).and_then(|post_profile| post_profile.maybe_quiz.as_ref()) {
                let quiz = quizresponder::get_quiz(quiz_json.clone());
                html! {
                    <quizresponder::QuizResponder quiz=quiz />
                }
            } else {
                html! {
                    <>
                        <div class="w-100 h-100 d-flex flex-row justify-content-between scroll-y scroll-x-hidden">
                            <div class="w-100 ps-3 pt-3 ps-md-5 pt-md-5 ps-lg-7 pt-lg-7">
                                <div class="d-flex flex-wrap alig-items-center justify-content-between mb-6">
                                    {go_back_grade}
                                    {btn_sidebar_show}
                                </div>
                                <div class="d-flex flex-wrap align-items-center justify-content-between mb-4">
                                    {maybe_edit_options}
                                </div>
                                {author_post}
                                {maybe_post_content}
                            </div>
                        </div>
                        <div class=class_right_sidebar>
                            <div class="d-flex align-items-center justify-content-between pb-5">
                                <SearchPostsGroup on_app_route=self.props.on_app_route.clone()
                                    user_profile=self.props.user_profile.clone()
                                    group_id=self.props.group_id
                                    school_id=self.props.school_id />
                                <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
                            </div>
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Discussions")}</span>
                            <div class="section-right-post pt-3 scroll-messages-y mh-80">
                                <MessageList on_app_route=self.props.on_app_route.clone() 
                                    user_profile=self.props.user_profile.clone() 
                                    user_id=None
                                    group_category=MessageGroupCategory::Posts(self.props.group_id, 
                                    self.props.post_id) />
                                // <MessageListPost on_app_route=self.props.on_app_route.clone() 
                                //     auth_user=self.props.auth_user.clone() 
                                //     user_id=None
                                //     group_id=self.props.group_id
                                //     post_id=self.props.post_id />
                            </div>
                        </div>
                        <div class=class_sidebar_mobile data-bs-scroll="true" data-bs-backdrop="false" tabindex="-1" id="offcanvasScrolling" aria-labelledby="offcanvasScrollingLabel" aria-modal="true" role="dialog" style=style_sidebar_mobile>
                            <div class="offcanvas-header d-flex justify-content-end">
                                <button type="button" class="btn btn-outline-danger" data-bs-dismiss="offcanvas" onclick=&on_show_sidebar>
                                    <i class="fas fa-times"></i>
                                </button>
                            </div>
                            <div class="offcanvas-body pt-0">
                                <div class="d-flex align-items-center justify-content-between pb-5">
                                    <SearchPostsGroup on_app_route=self.props.on_app_route.clone()
                                        user_profile=self.props.user_profile.clone()
                                        group_id=self.props.group_id
                                        school_id=self.props.school_id />
                                    <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
                                </div>
                                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Discussions")}</span>
                                <div class="section-right-post pt-3 scroll-messages-y mh-80">
                                    // <MessageList on_app_route=self.props.on_app_route.clone() 
                                    //     auth_user=self.props.auth_user.clone() 
                                    //     user_id=None
                                    //     group_category=MessageGroupCategory::Posts(self.props.group_id, 
                                    //     self.props.post_id) />
                                    // <MessageListPost on_app_route=self.props.on_app_route.clone() 
                                    //     auth_user=self.props.auth_user.clone() 
                                    //     user_id=None
                                    //     group_id=self.props.group_id
                                    //     post_id=self.props.post_id />
                                </div>
                            </div>
                        </div>
                        {modal_del_lesson_entirely}
                    </>
                }
            }
        } else {
            html! {
                <div class="progress w-100">
                    <div class="progress-bar" role="progressbar" style="width: 100%;" aria-valuenow="100" aria-valuemin="0" aria-valuemax="100">{"100%"}</div>
                </div>
            }
        }
    }
}