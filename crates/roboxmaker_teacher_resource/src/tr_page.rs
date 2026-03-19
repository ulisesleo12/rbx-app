use log::*;
use uuid::Uuid;
use std::io::Write;
use serde::Deserialize;
use yew::web_sys::{self, window};
use js_sys::wasm_bindgen::JsCast;
use yew::format::{Json, Nothing};
use code_location::code_location;
use yew::{prelude::*, web_sys::File};
use yew::services::{FetchService, ReaderService};
use yew::services::reader::{FileData, ReaderTask};
use yew::services::fetch::{FetchTask, Request, Response, StatusCode};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::{config, lang};
use roboxmaker_graphql::Subscribe;
use roboxmaker_types::types::UserId;
use roboxmaker_graphql::SubscriptionTask;
use roboxmaker_models::{school_model, teacher_resource};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request as OtherRequest, RequestTask};
use roboxmaker_types::types::{GroupId, ResourceId, AppRoute, SchoolId, ClassGroupCategory, MyUserProfile};


#[derive(Debug, Clone, PartialEq)]
pub enum TeacherResourceType {
    ManualAndGuides,
    SoftwareAndTools,
    PresentationAndDidacticMaterial,
    AdditionalResources,
}

#[derive(Deserialize, Debug)]
struct ResourceFileUploadResponse {
    url: String,
}

pub struct TeacherResourcePage {
    link: ComponentLink<Self>,
    props: Properties,
    graphql_task: Option<GraphQLTask>,
    load_task: Option<SubscriptionTask>,
    save_task: Option<RequestTask>,
    delete_task: Option<RequestTask>,
    update_task: Option<RequestTask>,
    get_tr_task: Option<RequestTask>,
    teacher_resource: Option<teacher_resource::teacher_resource_by_id::TeacherResourceByIdTeacherResourceGroupByPk>,
    title: String,
    save_status: bool,
    class_name: String,
    robox_tr_type: Vec<TeacherResourceType>,
    show_dropdown: bool,
    teacher_resource_type: Option<TeacherResourceType>,
    load_spinner: bool,
    on_dropdown_menu: bool,
    modal_delete_resource_by_id: bool,
    reader_task: Vec<ReaderTask>,
    upload_task: Option<FetchTask>,
    file_data: Option<FileData>,
    file_task: Option<FetchTask>,
    resource_list: Vec<String>,
    upload_loading: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Properties {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub resource_id: ResourceId,
    pub group_id: GroupId,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum Message {
    AppRoute(AppRoute),
    FetchResourceById(ResourceId, GroupId),
    RespResource(Option<teacher_resource::teacher_resource_by_id::ResponseData>),
    SaveResource,
    Title(String),
    Saved(Option<teacher_resource::teacher_resource_by_id_update::ResponseData>),
    Back,
    GetTeacherResourceType,
    RespTeacherResourceType(Option<teacher_resource::get_teacher_resource_type::ResponseData>),
    ShowDropdown,
    SelectResourceType(Option<TeacherResourceType>),
    OnDropdownMenu,

    ResourceGroupDelete(ResourceId),
    RespResourceGroupDelete(Option<teacher_resource::teacher_resource_group_delete::ResponseData>),
    DeleteResourceById(ResourceId),
    RespDeleteResourceById(Option<teacher_resource::delete_teacher_resource_by_id::ResponseData>),
    ShowModalDeleteResourceById,

    Publish(ResourceId),
    UnPublish(ResourceId),
    RespPublish(Option<teacher_resource::update_teacher_resource_group_options::ResponseData>),

    ExpectFileFromFiles,
    ChooseFile(Vec<File>),
    FileData(FileData),
    FileUpload,
    FileUploadResp(Option<String>, StatusCode),
    GetResourceFiles,
    ResourceFilesResp(Vec<String>),
    None,
}

impl Component for TeacherResourcePage {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::FetchResourceById(props.resource_id, props.group_id));
        link.send_message(Message::GetTeacherResourceType);
        link.send_message(Message::GetResourceFiles);

        TeacherResourcePage {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            load_task: None,
            save_task: None,
            delete_task: None,
            update_task: None,
            get_tr_task: None,
            teacher_resource: None,
            title: String::from(""),
            save_status: true,
            class_name: String::from(""),
            robox_tr_type: vec![],
            show_dropdown: false,
            teacher_resource_type: None,
            load_spinner: false,
            on_dropdown_menu: false,
            modal_delete_resource_by_id: false,
            reader_task: vec![],
            upload_task: None,
            file_data: None,
            file_task: None,
            resource_list: vec![],
            upload_loading: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            Message::FetchResourceById(resource_id, group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = teacher_resource::teacher_resource_by_id::Variables { 
                        resource_id: resource_id.0,
                        group_id: group_id.0,
                    };

                    let task = teacher_resource::TeacherResourceById::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::RespResource(response)
                        },
                    );
                    self.load_task = Some(task);
                }
            }
            Message::RespResource(response) => {
                self.teacher_resource = response.clone().and_then(|data| data.teacher_resource_group_by_pk);
                if let Some(tr) = &self.teacher_resource {
                    self.class_name = tr.clone().class_profile.and_then(|data| data.class_profile).and_then(|class_profile| Some(class_profile.name)).unwrap_or("".to_string());

                    if let Some(resource_profile) = &tr.teacher_resource_profile {
                        self.title = resource_profile.title.clone();

                        if resource_profile.teacher_resource_type.is_some() {
                            let teacher_resource_type = match resource_profile.teacher_resource_type.clone().unwrap() {
                                teacher_resource::teacher_resource_by_id::RoboxTeacherResourceTypeEnum::ManualAndGuides => TeacherResourceType::ManualAndGuides,
                                teacher_resource::teacher_resource_by_id::RoboxTeacherResourceTypeEnum::SoftwareAndTools => TeacherResourceType::SoftwareAndTools,
                                teacher_resource::teacher_resource_by_id::RoboxTeacherResourceTypeEnum::PresentationAndDidacticMaterial => TeacherResourceType::PresentationAndDidacticMaterial,
                                teacher_resource::teacher_resource_by_id::RoboxTeacherResourceTypeEnum::AdditionalResources => TeacherResourceType::AdditionalResources,
                                teacher_resource::teacher_resource_by_id::RoboxTeacherResourceTypeEnum::Other(_) => TeacherResourceType::AdditionalResources,
                            };

                            self.teacher_resource_type = Some(teacher_resource_type);
                        }
                    }
                }

                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                
                if response.clone().and_then(|data| data.teacher_resource_group_by_pk).is_none() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::Lessons)));
                    } else {
                        
                        self.link.send_message(Message::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::Lessons)));
                    }
                }
            }
            Message::Title(title) => {
                self.title = title;
                self.save_status = false;
                should_update = true;
            }
            Message::SaveResource => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let teacher_resource_type = match self.teacher_resource_type.clone().unwrap_or(TeacherResourceType::AdditionalResources) {
                        TeacherResourceType::ManualAndGuides => teacher_resource::teacher_resource_by_id_update::RoboxTeacherResourceTypeEnum::ManualAndGuides,
                        TeacherResourceType::SoftwareAndTools => teacher_resource::teacher_resource_by_id_update::RoboxTeacherResourceTypeEnum::SoftwareAndTools,
                        TeacherResourceType::PresentationAndDidacticMaterial => teacher_resource::teacher_resource_by_id_update::RoboxTeacherResourceTypeEnum::PresentationAndDidacticMaterial,
                        TeacherResourceType::AdditionalResources => teacher_resource::teacher_resource_by_id_update::RoboxTeacherResourceTypeEnum::AdditionalResources,
                    };

                    let vars = teacher_resource::teacher_resource_by_id_update::Variables { 
                        resource_id: self.props.resource_id.0,
                        resource_title: self.title.clone(),
                        teacher_resource_type: Some(teacher_resource_type)
                    };

                    let task = teacher_resource::TeacherResourceByIdUpdate::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::Saved(response)
                        },
                    );
                    self.save_task = Some(task);
                }
                self.link.send_message(Message::Back);
            }
            Message::Saved(response) => {
                if response.clone().and_then(|data| data.update_teacher_resource_profile_by_pk).is_some() &&
                    response.clone().and_then(|data| data.update_teacher_resource_profile_by_pk).is_some() {
                    self.save_status = true;
                }
            }
            Message::Back => {
                let _ = window().expect("no windows").window().history().unwrap().back();
            }
            Message::GetTeacherResourceType => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = teacher_resource::get_teacher_resource_type::Variables {};
    
                    let task = teacher_resource::GetTeacherResourceType::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| Message::RespTeacherResourceType(response),
                    );
                    self.get_tr_task = Some(task);
                }
            }
            Message::RespTeacherResourceType(resp) => {
                if resp.is_some() {
                    let teacher_resource_type: Vec<TeacherResourceType> = resp.unwrap().robox_teacher_resource_type.iter().map(|item| {
                        match item.teacher_resource_type.clone().as_str() {
                            "ManualAndGuides" => TeacherResourceType::ManualAndGuides,
                            "SoftwareAndTools" => TeacherResourceType::SoftwareAndTools,
                            "PresentationAndDidacticMaterial" => TeacherResourceType::PresentationAndDidacticMaterial,
                            "AdditionalResources" => TeacherResourceType::AdditionalResources,
                            _ => TeacherResourceType::AdditionalResources,
                        }
                    }).collect();

                    self.robox_tr_type = teacher_resource_type;
                }
            }
            Message::ShowDropdown => {
                self.show_dropdown = !self.show_dropdown;
            }
            Message::SelectResourceType(select_resource_type) => {
                self.teacher_resource_type = select_resource_type
            }
            Message::OnDropdownMenu => {
                self.on_dropdown_menu = !self.on_dropdown_menu;
            }

            Message::ResourceGroupDelete(resource_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = teacher_resource::teacher_resource_group_delete::Variables { 
                        group_id: self.props.group_id.0,
                        resource_id: resource_id.0,
                    };

                    let task = teacher_resource::TeacherResourceGroupDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::RespResourceGroupDelete(response)
                        },
                    );
                    self.delete_task = Some(task);
                }
            }
            Message::RespResourceGroupDelete(response) => {
                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                if response.clone().and_then(|data| data.delete_teacher_resource_group).is_some() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::TeacherResources)));
                    } else {
                        
                        self.link.send_message(Message::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::TeacherResources)));
                    }
                    
                    info!("{:?} del", response);
                }
                should_update = true;
            }

            Message::DeleteResourceById(resource_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = teacher_resource::delete_teacher_resource_by_id::Variables { 
                        resource_id: resource_id.0,
                    };

                    let task = teacher_resource::DeleteTeacherResourceById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::RespDeleteResourceById(response)
                        },
                    );
                    self.delete_task = Some(task);
                }
            }
            Message::RespDeleteResourceById(response) => {
                let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                if response.clone().and_then(|data| data.delete_teacher_resources_by_pk).is_some() {
                    if self.props.user_profile.clone().and_then(|item| Some(item.user_staff.is_some() || item.user_teacher.is_some())).unwrap_or(false) {
                        self.link.send_message(Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::TeacherResources)));
                    } else {
                        
                        self.link.send_message(Message::AppRoute(AppRoute::GroupSectionStudent(school_id, user_id, ClassGroupCategory::TeacherResources)));
                    }
                    
                    info!("{:?} del", response);
                }
            }
            Message::ShowModalDeleteResourceById => {
                self.modal_delete_resource_by_id = !self.modal_delete_resource_by_id
            }

            Message::Publish(resource_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = teacher_resource::update_teacher_resource_group_options::Variables { 
                        resource_id: resource_id.0,
                        group_id: self.props.group_id.0,
                        archived: false,
                        send_to_grade: true,
                    };

                    let task = teacher_resource::UpdateTeacherResourceGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::RespPublish(response)
                        },
                    );
                    self.update_task = Some(task);
                    self.load_spinner = true;
                }
            }
            Message::UnPublish(resource_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = teacher_resource::update_teacher_resource_group_options::Variables { 
                        resource_id: resource_id.0,
                        group_id: self.props.group_id.0,
                        archived: false,
                        send_to_grade: false,
                    };

                    let task = teacher_resource::UpdateTeacherResourceGroupOptions::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            Message::RespPublish(response)
                        },
                    );
                    self.update_task = Some(task);
                    self.load_spinner = true;
                }
            }
            Message::RespPublish(response) => {
                if let Some(mut resource) = self.teacher_resource.clone() {
                    resource.send_to_grade = response
                        .clone()
                        .and_then(|data| data.update_teacher_resource_group_by_pk)
                        .clone().and_then(|update_resource_group_by_pk| Some(update_resource_group_by_pk.send_to_grade))
                        .unwrap_or(false);
                }

                if response.is_some() {
                    self.load_spinner = false;
                    should_update = true
                }
            }
            Message::ExpectFileFromFiles => {
                if let Some(element) = web_sys::window()
                    .and_then(|window| window.document())
                    .and_then(|document| document.get_element_by_id("files-robox-resources")) {
                    
                    if let Ok(input) = element.dyn_into::<web_sys::HtmlElement>() {
                        input.click()
                    }
                } else { 
                    info!("input file. Not found") 
                }
            }
            Message::ChooseFile(files) => {
                if let Some(file) = files.get(0) {
                    info!("{:?}", file);
                    let task = { 
                        let callback =self.link.callback(move |file| {
                            Message::FileData(file)
                        });
                        ReaderService::read_file(
                            file.clone(),
                            callback
                        )
                        .unwrap()
                    };
                    self.reader_task.push(task);
                }
            }
            Message::FileData(file) => {
                self.file_data = Some(file)
            }
            Message::FileUpload => {
                self.upload_loading = true;
                if let Some(file) = &self.file_data {

                    const BOUNDARY: &'static str = "------------------------ea3bbcf87c101592";

                    let file_data = |content: &[u8]| {
                        let mut data = Vec::new();
                        write!(data, "--{}\r\n", BOUNDARY)?;
                        write!(
                                data,
                                "Content-Disposition: form-data; name=\"upload\"; filename=\"{}\"\r\n",
                                file.name)?;
                        write!(data, "\r\n")?;
                        data.extend_from_slice(content);
                        write!(data, "\r\n")?;
                        write!(data, "--{}--\r\n", BOUNDARY)?;
                        Ok(data)
                    };

                    let file_bytes: Result<Vec<u8>, anyhow::Error> = file_data(&file.content[..]);

                    let upload_url = format!("{}/upload-res.php", config::AKER_FILES_URL);

                    let req = Request::post(upload_url)
                        .header("robox-resource-id", self.props.resource_id.0.to_string())
                        .header(
                            "Content-Type",
                            format!("multipart/form-data; boundary={}", BOUNDARY),
                        )
                        .body(file_bytes)
                        .expect("Failed to build request.");

                    info!("RequestPost {:?}", req);

                    let callback = self.link.callback(
                        move |res: Response<
                            Json<Result<ResourceFileUploadResponse, anyhow::Error>>,
                        >| {

                            let status_code = res.status();
                            let url = if let (_meta, Json(Ok(file_upload))) = res.into_parts() {
                                Some(file_upload.url)
                            } else {
                                None
                            };
                            Message::FileUploadResp(url , status_code)
                        },
                    );

                    self.upload_task = FetchService::fetch_binary(req, callback).ok();
                }
            }
            Message::FileUploadResp(url, status_code) => {
                if status_code == StatusCode::OK {
                    self.upload_loading = false;
                }
                if url.is_some() {
                    self.file_data = None;
                    self.link.send_message(Message::GetResourceFiles);
                }
            }
            Message::GetResourceFiles => {
                let url = format!("https://files.roboxmaker.com/file-res.php?file={}*", self.props.resource_id.0);

                let get_request = Request::get(url)
                    .header("robox-resource-id", self.props.resource_id.0.to_string())
                    .body(Nothing)
                    .expect("Unable to build request!");
    
                let get_callback = self.link.callback(move |response: Response<Json<Result<Vec<String>, anyhow::Error>>>| {
                    let (_meta, Json(files)) = response.into_parts();

                    if let Ok(files) = files {
                        Message::ResourceFilesResp(files)
                    } else {
                        Message::ResourceFilesResp(vec![])
                    }
                });
        
                let task = FetchService::fetch(get_request, get_callback);
                self.file_task = task.ok();
            }
            Message::ResourceFilesResp(files) => {
                self.resource_list = files
            }
            Message::None => {}
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);

        self.props != props
    }

    fn view(&self) -> Html {
        let on_dropdown = self.link.callback(|_| Message::ShowDropdown);
        
        if let Some(resource) = &self.teacher_resource {
            let resource_clone = resource.clone();
            let resource_id = resource_clone.clone().teacher_resource_profile.and_then(|data| Some(data.resource_id)).unwrap_or(Uuid::default());

            let action_menu = self
                .props
                .user_profile
                .as_ref()
                .and_then(|item| {

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
                    // let maybe_no_send = if resource_clone.send_to_grade {
                    //     html! {
                    //         <li>
                    //             <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={ self.link.callback(move |_| Message::UnPublish(ResourceId(resource_id))) }>
                    //                 <i class="fas fa-upload me-2"></i>
                    //                 <span>{lang::dict("Do Not Post")}</span>
                    //             </a>
                    //         </li>
                    //     }
                    // } else {
                    //     html! {}
                    // };
    
                    // let maybe_send = if resource_clone.send_to_grade {
                    //     html! {}
                    // } else {
                    //     html! {
                    //         <li>
                    //             <a class="dropdown-item drop-hover-filter text-gray-purple-two" onclick={ self.link.callback(move |_| Message::Publish(ResourceId(resource_id))) }>
                    //                 <i class="fas fa-upload me-2"></i>
                    //                 <span>{lang::dict("To Post")}</span>
                    //             </a>
                    //         </li>
                    //     }
                    // };
    
                    // let spinner = if self.load_spinner {
                    //     html! {
                    //         <div class="text-center text-gray-purple-two">
                    //             <div class="spinner-border" role="status">
                    //             </div>
                    //         </div>
                    //     }
                    // } else {
                    //     html! {}
                    // };
    
                    if item.user_staff.is_some() || item.user_teacher.is_some() {
                        Some(html! {
                            <div class="dropdown">
                                <a class=maybe_menu onclick={ self.link.callback( move |_| Message::OnDropdownMenu) } role="button" id="dropdownMenuLink" data-bs-toggle="dropdown" aria-expanded="false">
                                    <i class="fas fa-ellipsis-v"></i>
                                </a>
                                <ul class=maybe_item aria-labelledby="dropdownMenuLink" style="top: 40px; right: 0px;">
                                    // {
                                    //     if self.load_spinner {
                                    //         { spinner }
                                    //     } else {
                                    //         html! {
                                    //             <>
                                    //                 {maybe_no_send}
                                    //                 {maybe_send}
                                    //             </>
                                    //         }
                                    //     }
                                    // }
                                    <li class="border-top">
                                        <a class="dropdown-item drop-hover-filter text-gray-purple-two mt-2" onclick={ self.link.callback( move |_| Message::ResourceGroupDelete(ResourceId(resource_id))) }>
                                            <i class="fas fa-lock me-2"></i>
                                            <span>{lang::dict("Disguise")}</span>
                                        </a>
                                        <a class="dropdown-item drop-hover-filter-del text-red-delete mt-2" onclick={ self.link.callback(move |_| Message::ShowModalDeleteResourceById) }>
                                            <i class="fas fa-trash me-2"></i>
                                            <span>{lang::dict("Remove")}</span>
                                        </a>
                                    </li>
                                </ul>
                            </div>
                        })
                    } else {
                        None
                    }
                })
                .unwrap_or(html! {});
            let author_resource = resource_clone
                .clone()
                .teacher_resource_profile
                .and_then(|resource_profile| {
                    let author_profile = resource_profile.author_profile.as_ref().unwrap();
                    let pic_path = author_profile.pic_path.clone().unwrap_or("https://files.roboxmaker.network/uploads/avatar.png".to_owned());
    
                    Some(html! {
                        <div class="d-flex flex-wrap align-items-center justify-content-between pb-6">
                            <div class="d-flex align-items-center">
                                <img class="img-card-32" src=pic_path />
                                <span class="text-dark noir-light is-size-18 lh-22 ps-2">{ &author_profile.full_name }</span>
                            </div>
                            <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                                <span class="icon">
                                    <i class="far fa-clock"></i>
                                </span>
                                <span class="ps-2">{ &resource_profile.timestamp.format("%a %b %e %T %Y").to_string() }</span>
                            </span>
                            <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                                <span class="icon">
                                    <i class="fas fa-graduation-cap"></i>
                                </span>
                                <span class="ps-2">{ self.class_name.clone() }</span>
                            </span>
    
                            { action_menu }
                        </div>
                    })
                })
                .unwrap_or(html! {});

            let resource_title = self
                .props
                .user_profile
                .as_ref()
                .and_then(|item| {
                let on_data = self
                    .link
                    .callback(|data: InputData| Message::Title(data.value));
                if item.user_staff.is_some() {
                    Some(html! {
                        <input class="input input-style-universal px-3 mb-4 mb-md-4 mb-lg-0 mb-xl-0 col-sm-12 col-md-12 col-lg-6" type="text" placeholder={"Título del Recurso"} value=self.title.clone() oninput=on_data />
                    })
                } else {
                    Some(html! {
                        <h13 class="text-primary-blue-light noir-bold is-size-16 lh-30 mb-0">{ self.title.clone() }</h13>
                    })
                }
            }).unwrap_or(html! {});

            let school_id = self.props.school_id;
            let group_id = self.props.group_id;
            let go_back_group = self.link.callback(move |_| Message::AppRoute(AppRoute::SchoolGroupSection(school_id, group_id, ClassGroupCategory::TeacherResources)));

            let user_profile_picture = self
                .props
                .user_profile
                .as_ref()
                .and_then(|item| Some(item.pic_path.clone()))
                .and_then(|pic_path| {
                    Some(html! {
                        <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
                    })
                })
                .unwrap_or(html! { <img class="img-card-72" src="/static/avatar.png"/> });

            let status_save = if self.save_status {
                html! {
                    <span class="text-success mx-4">{lang::dict("Saved")}</span>
                }
            } else {
                html! {
                    <span class="text-danger mx-4">{lang::dict("Unsaved")}</span>
                }
            };
            let save_resource_btn = self
                .props
                .user_profile
                .as_ref()
                .zip(
                    resource_clone
                        .clone()
                        .teacher_resource_profile
                )
                .and_then(|(item, resource_profile)| {
                    let on_save = self.link.callback( |_| Message::SaveResource);
                    // if item.user_staff.is_some() || item.user_teacher.is_some() || item.user_id.0 == resource_profile.author_id {
                    if item.user_staff.is_some() || item.user_id.0 == resource_profile.author_id {
                        Some(html! {
                            <>
                                { status_save }
                                <a class="button button-saved-lesson bg-primary-blue-dark d-flex align-items-center justify-content-center" onclick=on_save>
                                    <span class="text-white noir-bold is-size-16 lh-20">{lang::dict("Save")}</span>
                                </a>
                            </>
                        })
                    } else {
                        None
                    }
                })
                .unwrap_or(html! {});

            let selected_resource = match self.teacher_resource_type.is_some() {
                true => match self.teacher_resource_type.clone().unwrap() {
                    TeacherResourceType::ManualAndGuides => "Manuales y guías",
                    TeacherResourceType::SoftwareAndTools => "Software y herramientas",
                    TeacherResourceType::PresentationAndDidacticMaterial => "Presentaciones y material didáctico",
                    TeacherResourceType::AdditionalResources => "Recursos complementarios",
                },
                false => "",
            };

            let class_dropdown = if self.show_dropdown {
                "btn btn-secondary btn-see-degree dropdown-toggle show d-flex align-items-center justify-content-between"
            } else {
                "btn btn-secondary btn-see-degree dropdown-toggle d-flex align-items-center justify-content-between"
            };
            let class_dropdown_list = if self.show_dropdown {
                "dropdown-menu dropdown-menu-degree show"
            } else {
                "dropdown-menu dropdown-menu-degree"
            };

            let tr_types = self.robox_tr_type.iter().map(move |item| {
                let resource_type = item.clone();
                let tr_type = match resource_type {
                    TeacherResourceType::ManualAndGuides => "Manuales y guías",
                    TeacherResourceType::SoftwareAndTools => "Software y herramientas",
                    TeacherResourceType::PresentationAndDidacticMaterial => "Presentaciones y material didáctico",
                    TeacherResourceType::AdditionalResources => "Recursos complementarios",
                };
        
                let is_checked = Some(resource_type.clone()) == self.teacher_resource_type;
                let on_select_resource_type = self.link.callback(move |_| Message::SelectResourceType(Some(resource_type.clone())));
                html! {
                    <li class="d-flex flex-nowrap">
                        <a class="dropdown-item d-flex flex-nowrap align-items-center mt-1 pe-0" onclick={ on_select_resource_type }>
                            <input class="bg-checkbox" type="checkbox" checked={is_checked} />
                            <span class={ if is_checked {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"} } style="white-space: normal !important;">{ tr_type }</span>
                        </a>
                    </li>
                }
            }).collect::<Html>();

            let show_dropdown_by_user = self
                .props
                .user_profile
                .as_ref()
                .and_then(|user|{
                    if user.user_staff.is_some() || user.user_teacher.is_some() {
                        Some(html! {
                            <div class="dropdown me-5">
                                <button class=class_dropdown type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" style="width: 275px !important;" onclick={ on_dropdown }>
                                    <img src="/icons/filter.svg" style="height: 22px;" />
                                    <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22" style="white-space: normal !important;">{lang::dict(selected_resource)}</span>
                                </button>
                                <ul class=class_dropdown_list aria-labelledby="dropdownMenuButton2" style="width: 275px !important;">
                                    { tr_types }
                                </ul>
                            </div>
                        })
                    } else {
                        Some(html! {})
                    }
                })
                .unwrap_or(html! {});

            let class_delete_resource_by_id = if self.modal_delete_resource_by_id {
                "modal fade show"
            } else {
                "modal fade"
            };
    
            let style_delete_resource_by_id = if self.modal_delete_resource_by_id {
                "display: block;"
            } else {
                "display: none;"
            };
    
            let modal_delete_resource_by_id = if self.modal_delete_resource_by_id {
                html! {
                    <div class={ class_delete_resource_by_id } style={ style_delete_resource_by_id } id="staticBackdrop" data-bs-backdrop="static" data-bs-keyboard="false" tabindex="-1" aria-labelledby="staticBackdropLabel" aria-hidden="true">
                        <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header d-flex justify-content-center">
                                <h1 class="modal-title noir-bold fs-5" id="staticBackdropLabel">{"Borrar recurso"}</h1>
                            </div>
                            <div class="modal-body text-center">
                                <span class="text-primary-blue-dark noir-medium is-size-16 lh-22">{"Para borrar el recurso por completo, presione "}
                                    <span class="noir-bold">{"confirmar"}</span></span>
                            </div>
                            <div class="modal-footer">
                                <button type="button" class="btn btn-outline-purple-on noir-medium" onclick={ self.link.callback(move |_| Message::ShowModalDeleteResourceById) } data-bs-dismiss="modal">{"Cancelar"}</button>
                                <button type="button" class="btn btn-outline-primary-blue-dark noir-medium" onclick={ self.link.callback(move |_| Message::DeleteResourceById(ResourceId(resource_id))) }>{"Confirmar"}</button>
                            </div>
                        </div>
                        </div>
                    </div>
                }
            } else {
                html! {}
            };

            let on_change = self
                .link
                .callback(move |data| {
                    let mut result = Vec::new();
                    if let ChangeData::Files(files) = data {
                        let files = js_sys::try_iter(&files)
                            .unwrap()
                            .unwrap()
                            .map(|v| File::from(v.unwrap()));
                        result.extend(files);
                    }
                    Message::ChooseFile(result)
                });
            
            let on_upload = self.link.callback(move |_| Message::FileUpload);
            let on_select_file = self.link.callback(move |_| Message::ExpectFileFromFiles);

            let file_size = self.file_data.as_ref().map(|file| {
                let size_in_mb = file.content.len() as f64 / (1024.0 * 1024.0);
                format!("{:.2} MB", size_in_mb)
            }).unwrap_or_else(|| "".to_string());

            let file_name_display = self.file_data.clone().and_then(|file| {
                Some(html! {
                    <span class="text-dark noir-light is-size-18 lh-22 ps-2">{ &file.name }{" - "}{ file_size }</span>
                })
            }).unwrap_or(html! {});

            let resource_list = self.resource_list.iter().map(|item| {
                let replace_resource_id = format!("{}-", self.props.resource_id.0.to_string());
                let resource_name = item.replace(&replace_resource_id, "");

                html! {
                    <div>
                        <span class="text-primary-blue-dark noir-bold is-size-16 lh-20 me-6 mt-4">{ resource_name }</span>
                        <a href={format!("https://files.roboxmaker.com/resources/{}", item)} download={ item.clone() } target="_blank">
                        // <a href={format!("https://files.roboxmaker.com/resources/{}", item)} download={ item.clone() }>
                            <img src="/icons/download.svg" style="height: 22px;" />
                        </a>
                    </div>
                }
            }).collect::<Html>();

            let spinner = if self.upload_loading {
                html! {
                    <div class="text-center text-gray-purple-two">
                        <div class="spinner-border" role="status">
                        </div>
                    </div>
                }
            } else {
                html! {}
            };
            html! {
                <>
                    <div class="w-100 h-100 scroll-y p-3 p-md-5 p-lg-7">
                        <div class="d-flex flex-wrap align-items-center justify-content-between">
                            <a onclick=go_back_group class="mb-2">
                                <span class="text-gray-blue noir-bold is-size-16 lh-20 d-flex align-items-center">
                                    <i class="fas fa-arrow-left"></i>
                                    <span class="mx-2">{"Ir a recursos"}</span>
                                    { self.class_name.clone() }
                                </span>
                            </a>
                            <div class="d-flex flex-row align-items-center">
                                // <div class="mx-5">
                                // <SearchLessonGroup on_app_route=self.props.on_app_route.clone()
                                //     group_id=self.props.group_id
                                //     resource_id=self.props.resource_id 
                                //     school_id=self.props.school_id />
                                // </div>
                                { user_profile_picture }
                            </div>
                        </div>
                        <h1 class="text-primary-blue-light noir-bold is-size-24 lh-30 mb-0">{"Título del recurso"}</h1>
                        <div class="d-flex flex-wrap align-items-center justify-content-between pt-4 pb-6">
                            { resource_title }
                            { show_dropdown_by_user }
                            { save_resource_btn }
                        </div>
                        <h1 class="text-primary-blue-dark noir-bold is-size-32 lh-38 text-uppercase pb-3">{&self.title}</h1>
                        { author_resource }

                        {
                            if self.resource_list.is_empty() {
                                html! {
                                    <div class="d-flex align-items-end">
                                        <a class="btn btn-create-card bg-secondary-purple d-flex align-items-center justify-content-center" onclick={ on_select_file }>
                                            <span class="text-white noir-bold is-size-16 lh-20 text-center">{lang::dict("Select File")}</span>
                                        </a>
                                        { file_name_display }
                                        <button class="btn btn-create-card bg-primary-blue-dark text-white noir-bold is-size-16 lh-20 ms-6" type="button" onclick=on_upload disabled=self.file_data.is_none() style="height: 50px;">
                                            {lang::dict("Upload File...")}
                                        </button>
                                        { spinner }
                                    </div>
                                }
                            } else {
                                html! {}
                            }
                        }

                        <div class="d-flex flex-column pt-6">
                            { resource_list }
                        </div>
                    </div>
                    { modal_delete_resource_by_id }

                    <input id="files-robox-resources" type="file" class="d-none" accept=".pdf,.ppt,.zip,.png,.jpeg" onchange=on_change />
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