use log::*;
use uuid::Uuid;
use yew::web_sys::Node;
use yew::web_sys::window;
use yew::{prelude::*, web_sys};
use code_location::code_location;
use roboxmaker_types::types::UserId;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_activity::ActivityStyle;
// use roboxmaker_files::files_list::FilesList;
use roboxmaker_models::{school_model, classes_model};
use roboxmaker_activity::{activity_list::ActivityList};
use roboxmaker_searches::search_classes_group::SearchClassesGroup;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{ClassesId, GroupId, SchoolId, AppRoute, ClassGroupFiles, ClassGroupCategory, MyUserProfile};

#[derive(Debug, Clone, PartialEq)]
pub enum ClassesMode {
    Edit,
    Preview
}

pub struct ClassesPage {
    link: ComponentLink<Self>,
    props: ClassesPageProperties,
    graphql_task: Option<GraphQLTask>,
    update_classes_task: Option<RequestTask>,
    // files_by_classes_sub: Option<SubscriptionTask>,
    task_load: Option<RequestTask>,
    task_save: Option<RequestTask>,
    task_class_name: Option<RequestTask>,
    classes: Option<classes_model::classes_by_id::ClassesByIdClassesByPk>,
    class_name_classes: Option<classes_model::class_name_classes::ClassNameClassesClassesGroupByPk>,
    node: Option<Node>,
    topic: String,
    content: String,
    save_status: bool,
    tab_page_mode: ClassesMode,
    files_profile: Vec<ClassGroupFiles>,
    classes_update: Option<classes_model::update_classes_group_options::UpdateClassesGroupOptionsUpdateClassesGroupByPk>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ClassesPageProperties {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub classes_id: ClassesId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum ClassesPageMessage {
    AppRoute(AppRoute),
    FetchClassesById(ClassesId),
    Classes(Option<classes_model::classes_by_id::ResponseData>),
    SaveClasses,
    Topic(String),
    Saved(Option<classes_model::classes_by_id_update::ResponseData>),
    Back,
    TabPageMode(ClassesMode),
    FetchClassesClassName(GroupId, ClassesId),
    ClassName(Option<classes_model::class_name_classes::ResponseData>),
    // FetchFilesByClassesId,
    // Files(Option<files_model::files_by_classes_id::ResponseData>),
    PublisheClasses(Option<classes_model::update_classes_group_options::ResponseData>),
    ClassesUpdate(ClassesId),
}

impl Component for ClassesPage {
    type Message = ClassesPageMessage;
    type Properties = ClassesPageProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // link.send_message(ClassesPageMessage::FetchFilesByClassesId);
        link.send_message(ClassesPageMessage::FetchClassesById(props.classes_id));
        link.send_message(ClassesPageMessage::FetchClassesClassName(props.group_id, props.classes_id));
        ClassesPage {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            update_classes_task: None,
            // files_by_classes_sub: None,
            task_load: None,
            task_save: None,
            task_class_name: None,
            classes: None,
            class_name_classes: None,
            node: None,
            topic: String::from(""),
            content: String::from(""),
            save_status: true,
            tab_page_mode: ClassesMode::Edit,
            files_profile: vec![],
            classes_update: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            ClassesPageMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            ClassesPageMessage::FetchClassesById(classes_id) => {
                should_update = true;
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = classes_model::classes_by_id::Variables {
                        classes_id: classes_id.0 
                    };

                    let task = classes_model::ClassesById::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ClassesPageMessage::Classes(response)
                            },
                    );
                    self.task_load = Some(task);
                }
            }
            ClassesPageMessage::Classes(classes) => {
                should_update = true;
                self.classes = classes.clone().and_then(|data| data.classes_by_pk);
                if let Some(classes) = classes.clone().and_then(|data| data.classes_by_pk) {
                    if let Some(classes_content) = &classes.classes_content {
                        self.content = classes_content.content.clone();
                        let node = web_sys::window()
                            .and_then(|window| window.document())
                            .and_then(|document| document.create_element("div").ok())
                            .and_then(|div| {
                                div.set_class_name("ck-content");
                                div.set_inner_html(&classes_content.content);
                                Some(Node::from(div))
                            });
                        self.node = node;
                        self.topic = classes.classes_profile.clone().and_then(|data| Some(data.topic)).unwrap_or("".to_string())
                    }
                }
            }
            ClassesPageMessage::Topic(topic) => {
                self.topic = topic;
                self.save_status = false;
                should_update = true;
            }
            ClassesPageMessage::SaveClasses => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = classes_model::classes_by_id_update::Variables {
                        classes_id: self.props.classes_id.0,
                        classes_topic: self.topic.clone(),
                        classes_content: self.content.clone(),
                    };

                    let task = classes_model::ClassesByIdUpdate::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ClassesPageMessage::Saved(response)
                            },
                    );
                    self.task_save = Some(task);
                }
                let classes_id = self.props.classes_id;
                self.link.send_message(ClassesPageMessage::ClassesUpdate(classes_id));
                should_update = true;
            }
            ClassesPageMessage::Saved(profile) => {
                if profile.clone().and_then(|data| data.update_classes_content_by_pk).is_some() &&
                    profile.clone().and_then(|data| data.update_classes_profile_by_pk).is_some() {
                    self.save_status = true;
                }
            }
            ClassesPageMessage::Back =>{
                let _ = window().expect("no windows").window().history().unwrap().back();
            }
            ClassesPageMessage::TabPageMode(tab) => self.tab_page_mode = tab,
            ClassesPageMessage::FetchClassesClassName(group_id, classes_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = classes_model::class_name_classes::Variables {
                        group_id: group_id.0,
                        classes_id: classes_id.0
                    };

                    let task = classes_model::ClassNameClasses::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ClassesPageMessage::ClassName(response)
                            },
                    );
                    self.task_class_name = Some(task);
                }
            }
            ClassesPageMessage::ClassName(class_name_classes) => {
                self.class_name_classes = class_name_classes.clone().and_then(|data| data.classes_group_by_pk);
            }
            // ClassesPageMessage::FetchFilesByClassesId => {
            //     if let Some(graphql_task) = self.graphql_task.as_mut() {
            //         let vars = files_model::files_by_classes_id::Variables {
            //             group_id: self.props.group_id.0,
            //             classes_id: self.props.classes_id.0,
            //         };

            //         let task = files_model::FilesByClassesId::subscribe(
            //                 graphql_task,
            //                 &self.link,
            //                 vars,
            //                 |response| {
            //                     ClassesPageMessage::Files(response)
            //                 },
            //         );
            //         self.files_by_classes_sub = Some(task);
            //     }
            // }
            // ClassesPageMessage::Files(files) => {
            //     self.files_profile = files 
            //         .clone()
            //         .and_then(|data| Some(data.files_group))
            //         .unwrap_or(vec![])
            //         .iter()
            //         .map(|files| {
            //             ClassGroupFiles {
            //                 files_id: FilesId(files.files_id),
            //             }
            //         }).collect();
            // }
            ClassesPageMessage::PublisheClasses(response) => {
                self.classes_update = response.clone().and_then(|data| data.update_classes_group_by_pk)
            }
            ClassesPageMessage::ClassesUpdate(classes_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = classes_model::update_classes_group_options::Variables { 
                        classes_id: classes_id.0,
                        group_id: self.props.group_id.0,
                        published: true,
                        archived: false,
                    };

                    let task = classes_model::UpdateClassesGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            ClassesPageMessage::PublisheClasses(response)
                        },
                    );
                    self.update_classes_task = Some(task);
                }
            }
        };
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;
        
        // if self.props.classes_id != props.classes_id {
        //     self.link.send_message(ClassesPageMessage::FetchFilesByClassesId);
        // }
        if self.props != props {
            self.props = props;
            self.link.send_message(ClassesPageMessage::FetchClassesById(self.props.classes_id));
            should_render = true;
        }
        should_render
    }

    fn view(&self) -> Html {
        let group_id = self.props.group_id;
        let class_name_by_classes = self
            .class_name_classes.clone().and_then(|data| Some(data.group).clone())
            .iter()
            .map(|data| {
                let class_groups = data.class_groups.clone();
                let class_profile = class_groups
                    .clone()
                    .iter()
                    .map(|data| data.class_profile.clone().and_then(|data| Some(data.name.clone())).unwrap_or("".to_string())).collect::<Html>();
                html! {
                    {class_profile}
                }
            }).collect::<Html>();

        if let Some(classes) = &self.classes {
            let maybe_classes_title = {
                let on_data = self
                    .link
                    .callback(|data: InputData| ClassesPageMessage::Topic(data.value));
                html! {
                    <input class="input input-style-universal px-3 mb-lg-4 mb-md-0 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={lang::dict("Module Title")} value=self.topic.clone() oninput=on_data />
                }
            };
            let school_id = self.props.school_id;
            let user_id = self.props.user_profile.clone().and_then(|user_by_pk| Some(user_by_pk.user_id)).unwrap_or(UserId(Uuid::default()));

            let on_class_group_classes = self.link.callback(move |_| {
                ClassesPageMessage::AppRoute(AppRoute::SchoolGroupSection(
                    school_id.clone(),
                    group_id.clone(),
                    ClassGroupCategory::Classes,
                ))
            });
            let on_class_group_classes_st = self.link.callback(move |_| {
                ClassesPageMessage::AppRoute(AppRoute::GroupSectionStudent(
                    school_id.clone(),
                    user_id.clone(),
                    ClassGroupCategory::Classes,
                ))
            });
            let go_back_grade = self.props.user_profile.clone()
                .and_then(|user| {
                    if user.user_teacher.is_some() || user.user_staff.is_some() {
                        Some(html! {
                            <a onclick=on_class_group_classes>
                                <span class="icon-text text-gray-blue noir-bold is-size-16 lh-20 d-flex align-items-center pb-5">
                                    <i class="fas fa-arrow-left"></i>
                                    <span class="mx-2">{lang::dict("To Classes")}</span>
                                    {class_name_by_classes.clone()}
                                </span>
                            </a>
                        })
                    } else {
                        Some(html! {
                            <a onclick=on_class_group_classes_st>
                                <span class="icon-text text-gray-blue noir-bold is-size-16 lh-20 d-flex align-items-center pb-5">
                                    <i class="fas fa-arrow-left"></i>
                                    <span class="mx-2">{lang::dict("To Classes")}</span>
                                    {class_name_by_classes.clone()}
                                </span>
                            </a>
                        })
                    }
                }).unwrap_or(html! {});
            let on_edit = self.link.callback(|_| ClassesPageMessage::TabPageMode(ClassesMode::Edit));
            let on_preview = self.link.callback(|_| ClassesPageMessage::TabPageMode(ClassesMode::Preview));
            let tab_class = |flag: bool | match flag {
                true => "nav-link active is-active-tab",
                false => "nav-link is-no-active-tab",
            };
            let _maybe_tabs = html! {
                <ul class="nav nav-tabs mb-5">
                    <li class="nav-item">
                        <a class={tab_class(self.tab_page_mode==ClassesMode::Edit)} onclick=on_edit.clone()>{lang::dict("Resources")}</a>
                    </li>
                    <li class="nav-item">
                        <a class={tab_class(self.tab_page_mode==ClassesMode::Preview)} onclick=on_preview.clone()>{lang::dict("Activities")}</a>
                    </li>
                </ul>
            };

            let maybe_user_profile_pic = self
                .props
                .user_profile
                .as_ref()
                .and_then(|user_profile| Some(user_profile.pic_path.clone()))
                .and_then(|pic_path| {
                    Some(html! {
                        <img class="img-card-72 ms-5" src=pic_path.clone() alt="photo of user" />
                    })
                })
                .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
                });

            let _files = self.files_profile.iter().cloned().collect::<Vec<ClassGroupFiles>>();
            // let files_list = html! {
            //     <FilesList files=files.clone()
            //         on_app_route=self.props.on_app_route.clone()
            //         auth_user=self.props.auth_user.clone()
            //         auth_school=self.props.auth_school.clone()
            //         group_id=self.props.group_id.clone()
            //         classes_id=self.props.classes_id.clone()
            //         school_id=self.props.school_id />
            // };
            
            // let page_mode = match self.tab_page_mode {
            //     ClassesMode::Edit => {
            //         html! {
            //             {files_list}
            //         }
            //     }
            //     ClassesMode::Preview => {
            //         html! {
            //             <ActivityList on_app_route=self.props.on_app_route.clone()
            //                 auth_user=self.props.auth_user.clone()
            //                 user_id=None
            //                 group_id=self.props.group_id
            //                 classes_id=self.props.classes_id
            //                 maybe_style=ActivityStyle::ClassesPage />
            //         }
            //     }
            // };

            let maybe_publish = self
                .props
                .user_profile
                .as_ref()
                .zip(self.classes.as_ref().and_then(|classes| classes.classes_profile.as_ref()))
                .and_then(|(auth_user, classes_profile)| {
                    let on_save = self.link.callback(move |_| ClassesPageMessage::SaveClasses);
                    if auth_user.user_staff.is_some() || auth_user.user_teacher.is_some() || auth_user.user_id.0 == classes_profile.author_id {
                        Some(html! {
                            <>
                                <a class="btn button-saved-classes bg-primary-blue-dark d-flex align-items-center justify-content-center" onclick=on_save>
                                    <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("To Post")}</span>
                                </a>
                            </>
                        })
                    } else {
                        None
                    }
                })
                .unwrap_or(html! {});
            let topic = classes.classes_profile.clone().and_then(|data| Some(data.topic)).unwrap_or("".to_string());
            let maybe_header_classes = self.props.user_profile.as_ref()
                .and_then(|user| {
                    if user.user_staff.is_some() || user.user_teacher.is_some() {
                        Some(html! {
                            <>
                                <h1 class="text-primary-blue-light noir-bold is-size-24 lh-30 mb-0">{lang::dict("New Module")}</h1>
                                <div class="d-flex align-items-center justify-content-between pt-4 pb-6">
                                    <div class="pe-4 w-100">{maybe_classes_title}</div>
                                    {maybe_publish}
                                </div>
                            </>
                        })
                    } else {
                        Some(html! {
                            <>
                                <div class="d-flex align-items-center justify-content-between pt-4 pb-6">
                                    <h1 class="text-primary-blue-dark is-size-24 lh-20 noir-bold mt-2">{ &topic }</h1>
                                </div>
                            </>
                        })
                    }
                }).unwrap_or(html! {});
            html! {
                <>
                    <div class="scroll-y h-100 w-100 p-3 p-md-5 p-lg-7">
                        <div class="d-flex flex-wrap align-items-center justify-content-between">
                            {go_back_grade}
                            <div class="d-flex flex-row align-items-center">
                                <SearchClassesGroup on_app_route=self.props.on_app_route.clone()
                                    group_id=self.props.group_id
                                    school_id=self.props.school_id />
                                {maybe_user_profile_pic}
                            </div>
                        </div>
                        {maybe_header_classes}
                        <span class="noir-bold text-purple-on is-size-18 lh-22">{lang::dict("Activities")}</span>
                        <br/>
                        <br/>
                        // {maybe_tabs}
                        // {page_mode}
                        <ActivityList on_app_route=self.props.on_app_route.clone()
                            user_profile=self.props.user_profile.clone()
                            user_id=None
                            group_id=self.props.group_id
                            classes_id=self.props.classes_id
                            maybe_style=ActivityStyle::ClassesPage />
                    </div>
                </>
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
