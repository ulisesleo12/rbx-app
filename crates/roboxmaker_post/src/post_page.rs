use log::*;
use uuid::Uuid;
use chrono::Local;
use yew::prelude::*;
use std::time::Duration;
use web_sys::{window, Node};
use yew::virtual_dom::VNode;
use code_location::code_location;
use yew::{html, Component, Html};
use gloo_timers::callback::Interval;
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_models::post_model;
use roboxmaker_ckeditor::ckeditor;
use roboxmaker_main::{lang, config};
use roboxmaker_utils::functions::get_value_from_input_event;
use roboxmaker_searches::search_posts_group::SearchPostsGroup;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask, Subscribe, SubscriptionTask};
use roboxmaker_types::types::{PostId, GroupId, AppRoute, SchoolId, ClassGroupCategory, UserId, MyUserProfile};

#[derive(Debug, Clone, PartialEq)]
pub enum PostMode {
    Edit,
    Preview
}

#[derive(Debug)]
pub enum PostPageEdit {
    None,
    Edit,
    Save,
}

pub struct PostPage {
    graphql_task: Option<GraphQLTask>,
    load_task: Option<SubscriptionTask>,
    post_task: Option<RequestTask>,
    task_save: Option<RequestTask>,
    post: Option<post_model::post_by_id::PostByIdPostGroupByPk>,
    node: Option<Node>,
    edit: PostPageEdit,
    preview: PostPageEdit,
    topic: String,
    content: String,
    save_status: bool,
    job: Option<Interval>,
    tab_page_mode: PostMode,
    class_name: String,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PostPageProperties {
    // pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub post_id: PostId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum PostPageMessage {
    // AppRoute(AppRoute),
    StartAutoSave,
    FetchPostById(PostId, GroupId),
    Post(Option<post_model::post_by_id::ResponseData>),
    Edit(PostPageEdit),
    Content(String),
    Topic(String),
    Saved(Option<post_model::post_by_id_update::ResponseData>),
    Back,
    TabPageMode(PostMode),
    SaveDraftToggle(Option<post_model::update_post_group_options::ResponseData>),
    NoPublishedPost(PostId),
    PublishedPost(PostId),
    SavedPost,
}

impl Component for PostPage {
    type Message = PostPageMessage;
    type Properties = PostPageProperties;

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(PostPageMessage::FetchPostById(ctx.props().post_id, ctx.props().group_id));
        
        roboxmaker_utils::functions::school_state();

        PostPage {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            load_task: None,
            post_task: None,
            task_save: None,
            post: None,
            node: None,
            edit: PostPageEdit::Edit,
            preview: PostPageEdit::None,
            topic: String::from(""),
            content: String::from(""),
            save_status: true,
            job: None,
            tab_page_mode: PostMode::Edit,
            class_name: String::from(""),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let mut should_update = true;
        match msg {
            // PostPageMessage::AppRoute(route) => {
            //     ctx.props().on_app_route.emit(route)
            // }
            PostPageMessage::StartAutoSave => {
                let duration = Duration::from_secs(600).as_secs() as u32;
                let link = ctx.link().clone();
                let handle = Interval::new( duration, move || {
                    link.send_message(PostPageMessage::Edit(PostPageEdit::Save))
                });
                self.job = Some(handle);
                should_update = true;
            }
            PostPageMessage::FetchPostById(post_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = post_model::post_by_id::Variables { 
                        group_id: group_id.0,
                        post_id: post_id.0,
                    };

                    let task = post_model::PostById::subscribe(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            PostPageMessage::Post(response)
                        },
                    );
                    self.load_task = Some(task);
                }
            }
            PostPageMessage::Post(response) => {
                self.post = response.clone().and_then(|data| data.post_group_by_pk);

                self.edit = PostPageEdit::None;

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
                }

                let user_id = ctx.props().user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = ctx.props().school_id;
                let group_id = ctx.props().group_id;
                let category = ClassGroupCategory::Posts;
                
                if response.clone().and_then(|data| data.post_group_by_pk).is_none() {
                    if ctx.props().user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        ctx.link().navigator().unwrap().push(&AppRoute::SchoolGroupSection{school_id, group_id, category});
                    } else {
                        
                        ctx.link().navigator().unwrap().push(&AppRoute::GroupSectionStudent{school_id, user_id, category});
                    }
                }
            }
            PostPageMessage::Topic(topic) => {
                self.topic = topic;
                self.save_status = false;
                should_update = true;
            }
            PostPageMessage::Content(content) => {
                self.content = content;
                self.save_status = false;
                should_update = true;
            }
            PostPageMessage::Edit(edit) => {
                self.edit = edit;
                match self.edit {
                    PostPageEdit::Edit => ctx.link().send_message(PostPageMessage::StartAutoSave),
                    PostPageEdit::None => {}
                    PostPageEdit::Save => {
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let vars = post_model::post_by_id_update::Variables { 
                                post_id: ctx.props().post_id.0,
                                post_topic: self.topic.clone(),
                                post_content: self.content.clone(),
                            };
        
                            let task = post_model::PostByIdUpdate::request(
                                graphql_task,
                                &ctx,
                                vars,
                                |response| {
                                    PostPageMessage::Saved(response)
                                },
                            );
                            self.task_save = Some(task);
                        }
                        let post_id = ctx.props().post_id;
                        ctx.link().send_message(PostPageMessage::PublishedPost(post_id));

                        let duration = Duration::from_secs(2).as_secs() as u32;

                        let link = ctx.link().clone();

                        let handle = Interval::new( duration, move || {
                            link.send_message(PostPageMessage::Back)
                        });

                        self.job = Some(handle);
                        should_update = true;
                    }
                }
            }
            PostPageMessage::SavedPost => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = post_model::post_by_id_update::Variables { 
                        post_id: ctx.props().post_id.0,
                        post_topic: self.topic.clone(),
                        post_content: self.content.clone(),
                    };

                    let task = post_model::PostByIdUpdate::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            PostPageMessage::Saved(response)
                        },
                    );
                    self.task_save = Some(task);
                }
            }
            PostPageMessage::Saved(profile) => {
                if profile.is_some() {
                    self.save_status = true;
                }
            }
            PostPageMessage::Back => {
                let _ = window().expect("no windows").window().history().unwrap().back();
            }
            PostPageMessage::TabPageMode(tab) => {
                self.tab_page_mode = tab;
                if self.tab_page_mode == PostMode::Preview {
                    ctx.link().send_message(PostPageMessage::SavedPost);
                }
            }
            PostPageMessage::SaveDraftToggle(response) => {
                response.clone().and_then(|data| data.update_post_group_by_pk).clone().and_then(|data| Some(data.published)).unwrap_or(false);
            }
            PostPageMessage::NoPublishedPost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let maybe_timestamp = Local::now().naive_local();

                    let vars = post_model::update_post_group_options::Variables { 
                        post_id: post_id.0,
                        group_id: ctx.props().group_id.0,
                        published: false,
                        archived: false,
                        maybe_timestamp,
                    };

                    let task = post_model::UpdatePostGroupOptions::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            PostPageMessage::SaveDraftToggle(response)
                        },
                    );
                    self.post_task = Some(task);
                }
                ctx.link().send_message(PostPageMessage::SavedPost);
            }
            PostPageMessage::PublishedPost(post_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let maybe_timestamp = Local::now().naive_local();

                    let vars = post_model::update_post_group_options::Variables { 
                        post_id: post_id.0,
                        group_id: ctx.props().group_id.0,
                        published: true,
                        archived: false,
                        maybe_timestamp,
                    };

                    let task = post_model::UpdatePostGroupOptions::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            PostPageMessage::SaveDraftToggle(response)
                        },
                    );
                    self.post_task = Some(task);
                }
            }
        };
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
        let post_id = ctx.props().post_id;
        let group_id = ctx.props().group_id;
            
        let on_not_published_post = ctx.link().callback(move |_| PostPageMessage::NoPublishedPost(post_id)); 
        let author_post = self
            .post
            .as_ref()
            .and_then(|post | post.post_profile.as_ref())
            .and_then(|post_profile| {
                let author_profile = post_profile.author_profile.as_ref().unwrap();
                let pic_path = author_profile.pic_path.clone().unwrap_or("https://files.roboxmaker.network/uploads/avatar.png".to_owned());
                Some(html! {
                    <div class="d-flex flex-wrap align-items-center justify-content-between pt-5 pb-6 mb-2">
                        <div class="d-flex align-items-center">
                            <img class="img-card-32" src={pic_path} alt="" style="height: 32px; object-fit: cover;" />
                            <span class="text-dark noir-light is-size-18 lh-22 ps-2">{&author_profile.full_name}</span>
                        </div>
                        <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                            <i class="far fa-clock"></i>
                            <span class="ps-2">{&post_profile.timestamp.format("%a %e %b %Y %T").to_string()}</span>
                        </span>
                        <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                            <i class="fas fa-graduation-cap"></i>
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
        if let Some(_post) = &self.post {
            let maybe_edit_options = match self.edit {
                PostPageEdit::Edit => {
                    let on_save = ctx
                        .link()
                        .callback(move |_| PostPageMessage::Edit(PostPageEdit::Save));
                    html! {
                        <button class="btn is-white is-rounded" onclick={on_save}>
                            <span class="icon">
                                <i class="fas fa-cloud-upload-alt"></i>
                            </span>
                        </button>
                    }
                }
                PostPageEdit::Save => {
                    let maybe_post_edit = ctx
                        .props()
                        .user_profile
                        .as_ref()
                        .zip(
                            self.post
                                .as_ref()
                                .and_then(|post| post.post_profile.as_ref()),
                        )
                        .and_then(|(item, post_profile)| {
                            let on_saved = ctx
                                .link()
                                .callback(move |_| PostPageMessage::Edit(PostPageEdit::Save));
    
                            if item.user_staff.is_some() || item.user_teacher.is_some() || item.user_id.0 == post_profile.author_id {
                                Some(html! {
                                    <>
                                        <a class="btn button-saved-eraser d-flex align-items-center justify-content-center" onclick={&on_not_published_post}>
                                            <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Save draft")}</span>
                                        </a>
                                        {status_save.clone()}
                                        <a class="btn button-saved-post d-flex align-items-center justify-content-center" onclick={on_saved}>
                                            <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Save and Publish")}</span>
                                        </a>
                                    </>
                                })
                            } else {
                                None
                            }
                        })
                        .unwrap_or(html! {});
    
                    html! {
                        {maybe_post_edit}
                    }
                }
                PostPageEdit::None => {
                    let maybe_post_edit = ctx
                        .props()
                        .user_profile
                        .as_ref()
                        .zip(
                            self.post
                                .as_ref()
                                .and_then(|post| post.post_profile.as_ref()),
                        )
                        .and_then(|(item, post_profile)| {
                            let on_saved = ctx
                                .link()
                                .callback(move |_| PostPageMessage::Edit(PostPageEdit::Save));
                            if item.user_staff.is_some() || item.user_teacher.is_some() || item.user_id.0 == post_profile.author_id {
                                Some(html! {
                                    <> 
                                        <a class="btn button-saved-eraser d-flex align-items-center justify-content-center" onclick={&on_not_published_post}>
                                            <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Save draft")}</span>
                                        </a>
                                        {status_save.clone()}
                                        <a class="btn button-saved-post d-flex align-items-center justify-content-center" onclick={on_saved}>
                                            <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Save and Publish")}</span>
                                        </a>
                                    </>
                                })
                            } else {
                                None
                            }
                        })
                        .unwrap_or(html! {});

                    html! {
                        {maybe_post_edit}
                    }
                }
            };
            let maybe_post_title = {
                let on_data = ctx
                    .link()
                    .callback(|data: InputEvent| PostPageMessage::Topic(get_value_from_input_event(data)));
                match self.edit {
                    PostPageEdit::Edit => {
                        html! {
                            <input class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={lang::dict("Publication Title")} value={self.topic.clone()} oninput={on_data} />
                        }
                    }
                    PostPageEdit::Save => {
                        html! {
                            <input class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={lang::dict("Publication Title")} value={self.topic.clone()} oninput={on_data} />
                        }
                    }
                    _=> {
                        html! {
                            <input class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={lang::dict("Publication Title")} value={self.topic.clone()} oninput={on_data} />
                        }
                    }
                }
            };
            let maybe_post_content_edit = {
                let on_data = ctx
                    .link()
                    .callback(move |data| PostPageMessage::Content(data));
                let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
                match self.edit {
                    PostPageEdit::Edit => {
                        html! {
                            <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone() }
                                content={self.content.clone()}
                                upload_url={upload_url} 
                                on_data={on_data} />
                        }
                    }
                    PostPageEdit::Save => {
                        html! {
                            <ckeditor::CKEditor 
                                user_profile={ctx.props().user_profile.clone() }
                                content={self.content.clone()}
                                upload_url={upload_url} 
                                on_data={on_data} />
                        }
                    }
                    _ => {
                        html! {
                            <ckeditor::CKEditor 
                                user_profile={ctx.props().user_profile.clone() }
                                content={self.content.clone()}
                                upload_url={upload_url} 
                                on_data={on_data} />
                        }
                    }
                }
            };

            let maybe_post_content_preview = {
                let on_data = ctx
                    .link()
                    .callback(move |data| PostPageMessage::Content(data));
                let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);
                match self.preview {
                    PostPageEdit::Edit => {
                        html! {
                            <ckeditor::CKEditor user_profile={ctx.props().user_profile.clone()} 
                                content={self.content.clone()}
                                upload_url={upload_url} 
                                on_data={on_data} />
                        }
                    }
                    PostPageEdit::Save => {
                        html! {
                            <ckeditor::CKEditor 
                                user_profile={ctx.props().user_profile.clone()} 
                                content={self.content.clone()}
                                upload_url={upload_url} 
                                on_data={on_data} />
                        }
                    }
                    _ => {
                        html! {
                            VNode::VRef(self.node.clone().unwrap())
                        }
                    }
                }
            };
                
            let school_id = ctx.props().school_id;
            let category = ClassGroupCategory::Posts;
            let navigator = ctx.link().navigator().unwrap();

            let go_back_group = Callback::from(move |_| {
                navigator.push(&AppRoute::SchoolGroupSection{
                    school_id,
                    group_id,
                    category,
                })
            });
            let go_back_grade = html! {
                <>
                    <a onclick={go_back_group}>
                        <span class="text-gray-blue noir-bold is-size-16 lh-20 d-flex align-items-center">
                            <i class="fas fa-arrow-left"></i>
                            <span class="mx-2">{lang::dict("To Publications")}</span>
                            {self.class_name.clone()}
                        </span>
                    </a>
                </>
            };
            let on_edit = ctx.link().callback(|_| PostPageMessage::TabPageMode(PostMode::Edit));
            let on_preview = ctx.link().callback(|_| PostPageMessage::TabPageMode(PostMode::Preview));
            let tab_class = |flag: bool | match flag {
                true => "nav-link active is-active-tab",
                false => "nav-link is-no-active-tab",
            };
            let maybe_tabs = html! {
                <ul class="nav nav-tabs mb-5">
                    <li class="nav-item">
                        <a class={tab_class(self.tab_page_mode==PostMode::Edit)} onclick={on_edit.clone()}>{lang::dict("Edit")}</a>
                    </li>
                    <li class="nav-item">
                        <a class={tab_class(self.tab_page_mode==PostMode::Preview)} onclick={on_preview.clone()}>{lang::dict("Preview")}</a>
                    </li>
                </ul>
            };
            let maybe_user_profile_pic = ctx
                .props()
                .user_profile
                .as_ref()
                .and_then(|user_profile| Some(user_profile.pic_path.clone()))
                .and_then(|pic_path| {
                    Some(html! {
                        <img class="img-card-72 ms-4" src={pic_path.clone()} alt="photo of user" />
                    })
                })
                .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
                });

            let page_mode = match self.tab_page_mode {
                PostMode::Edit => {
                    html! {
                        <div style="border: 1px solid #C8C1CD; border-radius: 10px;">
                            {maybe_post_content_edit}
                        </div>
                    }
                }
                PostMode::Preview => {
                    html! {
                        <>
                            <div class="mt-7">
                                <h1 class="text-primary-blue-dark noir-bold is-size-32 lh-38 text-uppercase mt-4">{ &self.topic }</h1>
                                {author_post}
                                <div class="text-dark noir-light is-size-18 lh-22 pb-5">{maybe_post_content_preview}</div>
                            </div>
                        </>
                    }
                }
            };
            html! {
                <div class="w-100 h-100 scroll-y p-3 p-md-5 p-lg-7">
                    <div class="d-flex align-items-center justify-content-between">
                        {go_back_grade}
                        <div class="d-flex align-items-center flex-wrap">
                            <SearchPostsGroup user_profile={ctx.props().user_profile.clone()}
                                group_id={ctx.props().group_id}
                                school_id={ctx.props().school_id} />
                            {maybe_user_profile_pic}
                        </div>
                    </div>
                    <h1 class="text-primary-blue-light noir-bold is-size-24 lh-30 mb-0">{lang::dict("New Post")}</h1>
                    <div class="d-flex flex-wrap align-items-center justify-content-between mt-4 mb-6">
                        {maybe_post_title}
                        {maybe_edit_options}
                    </div>
                    {maybe_tabs}
                    {page_mode}
                </div>
            }
        } else {
            html! {
                <progress class="progress is-small is-primary" max="100"></progress>
            }
        }
    }
}