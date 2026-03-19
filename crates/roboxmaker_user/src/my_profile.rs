use log::*;
use std::io::Write;
use yew::format::Json;
use serde_derive::Deserialize;
use yew::{prelude::*, web_sys};
use code_location::code_location;
use yew::services::{fetch::{FetchService, FetchTask, Request, Response},
    reader::{FileData, ReaderService, File, ReaderTask},
};
use yew::{html, ChangeData, Component, ComponentLink, Html, NodeRef, ShouldRender};

use roboxmaker_main::{lang, config};
use roboxmaker_models::{user_model, school_model};
use roboxmaker_types::types::{UserId, AppRoute, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, RequestTask};

pub struct MyProfilePage {
    link: ComponentLink<Self>,
    props: MyProfilePageProperties,
    graphql_task: Option<GraphQLTask>,
    task_save: Option<RequestTask>,
    upload_task: Option<FetchTask>,
    edit: MyProfilePageEdit,
    node_full_name: NodeRef,
    full_name: String,
    pic_path: String,
    reader_task: Vec<ReaderTask>,
    user: Option<user_model::user_profile_by_id_update::ResponseData>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MyProfilePageProperties {
    pub user_id: UserId,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_app_route: Callback<AppRoute>,
    pub show_user: bool,
    pub close_modal_callback: Callback<bool>,
    pub on_user_profile: Option<Callback<UserId>>,
}

#[derive(Debug)]
pub enum MyProfilePageEdit {
    None,
    EditProfile,
    SaveProfile,
    SavePicture(String),
    ChoosePic(Vec<File>),
    ChangePic(FileData),
    Done,
}

#[derive(Debug)]
pub enum MyProfilePageMessage {
    UpdateUserResponse(Option<user_model::user_profile_by_id_update::ResponseData>),
    Edit(MyProfilePageEdit),
    OnUserShow(bool),
}

#[derive(Deserialize, Debug)]
struct MyProfilePageFileUploadResponse {
    url: String,
}

impl Component for MyProfilePage {
    type Message = MyProfilePageMessage;
    type Properties = MyProfilePageProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let full_name = props.user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.full_name.clone())).unwrap_or("Anonymous Guest".to_string());
        let pic_path = props.user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone())).unwrap_or("https://files.roboxmaker.com/uploads/avatar.png".to_string());

        MyProfilePage {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task_save: None,
            upload_task: None,
            edit: MyProfilePageEdit::EditProfile,
            node_full_name: NodeRef::default(),
            full_name,
            pic_path,
            reader_task: vec![],
            user: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MyProfilePageMessage::UpdateUserResponse(user) => { 
                self.user = user
            }
            MyProfilePageMessage::Edit(edit) => {
                self.edit = edit;
                match &self.edit {
                    MyProfilePageEdit::Done => {
                        self.edit = MyProfilePageEdit::None; 
                    }  
                    MyProfilePageEdit::SaveProfile => {
                        let user_id = self.props.user_id.0;
                        self.node_full_name
                            .cast::<web_sys::HtmlInputElement>()
                            .and_then(|input| {
                                self.full_name = input.value();
                                Some(())
                            });

                        if let Some(graphql_task) = self.graphql_task.as_mut() {

                            let vars = user_model::user_profile_by_id_update::Variables { 
                                user_id,
                                full_name: self.full_name.clone(),
                                pic_path: self.pic_path.clone(),
                            };
        
                            let task = <user_model::UserProfileByIdUpdate as roboxmaker_graphql::Request>::request(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    MyProfilePageMessage::UpdateUserResponse(response)
                                },
                            );
                            self.task_save = Some(task);
                        }
                        self.edit = MyProfilePageEdit::None; 
                    }
                    MyProfilePageEdit::SavePicture(pic_path) => {
                        let user_id = self.props.user_id.0;
                        if let Some(graphql_task) = self.graphql_task.as_mut() {

                            let vars = user_model::user_profile_by_id_update::Variables { 
                                user_id,
                                full_name: self.full_name.clone(),
                                pic_path: pic_path.clone(),
                            };
        
                            let task = <user_model::UserProfileByIdUpdate as roboxmaker_graphql::Request>::request(
                                graphql_task,
                                &self.link,
                                vars,
                                |response| {
                                    MyProfilePageMessage::UpdateUserResponse(response)
                                },
                            );
                            self.task_save = Some(task);
                        }
                        let user_id = self.props.user_id;

                        if let Some(on_user_profile) = &self.props.on_user_profile {
                            on_user_profile.emit(user_id)
                        }
                        self.edit = MyProfilePageEdit::None; 
                    }
                    MyProfilePageEdit::ChoosePic(files) => {
                        if let Some(file) = files.get(0) {
                            info!("{:?}", file);
                            let task = { 
                                let callback =self.link.callback(move |file| {
                                    MyProfilePageMessage::Edit(MyProfilePageEdit::ChangePic(file))
                                });
                                ReaderService::read_file(
                                    file.clone(),
                                    callback
                                )
                                .unwrap()
                            };
                            self.reader_task.push(task);
                        }
                        self.edit = MyProfilePageEdit::None; 
                    }
                    MyProfilePageEdit::ChangePic(file) => {
                        const BOUNDARY: &'static str = "------------------------ea3bbcf87c101592";

                        let image_data = |content: &[u8]| {
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

                        let img_bytes = image_data(&file.content[..]);

                        let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);

                        let req = Request::post(upload_url)
                            .header("aker-user-id", self.props.user_id.0.to_string())
                            .header(
                                "Content-Type",
                                format!("multipart/form-data; boundary={}", BOUNDARY),
                            )
                            .body(img_bytes)
                            .expect("Failed to build request.");

                        let pic_path = self.pic_path.clone();
                        let callback = self.link.callback(
                            move |res: Response<
                                Json<Result<MyProfilePageFileUploadResponse, anyhow::Error>>,
                            >| {
                                info!("{:?}", res);
                                let url = if let (_meta, Json(Ok(file_upload))) = res.into_parts() {
                                    Some(file_upload.url)
                                } else {
                                    None
                                };
                                MyProfilePageMessage::Edit(MyProfilePageEdit::SavePicture(url.unwrap_or(pic_path.clone())))
                            },
                        );
                        self.edit = MyProfilePageEdit::None; 
                        self.upload_task = FetchService::fetch_binary(req, callback).ok();
                    }
                    _ => {}
                }
            }
            MyProfilePageMessage::OnUserShow(show) => self.props.close_modal_callback.emit(show),
        }
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
        let on_close = self.link.callback(|_| MyProfilePageMessage::OnUserShow(false));

        let maybe_user_pic = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user_profile| match self.edit {
                MyProfilePageEdit::None => {
                    let picture = user_profile.pic_path.clone();
                        Some(html! {
                            <>
                                <div class="d-flex justify-content-center">
                                    <img class="img-card-128" src=picture />
                                </div>
                            </>
                        })
                }
                _ => Some(html! {
                    <div class="d-flex justify-content-center">
                        <img class="img-card-128" src=user_profile.pic_path.clone() />
                    </div>       
                }),
            })
            .unwrap_or(html! {});

        let maybe_user_profile_home = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user_profile| match self.edit {
                MyProfilePageEdit::None => {
                    let on_edit = self.link.callback(move |_| MyProfilePageMessage::Edit(MyProfilePageEdit::EditProfile));
                    Some(html! {
                        <>
                            <hr class="hr-section" />
                            <div class="d-flex flex-wrap align-items-center justify-content-between">
                                <span class="title is-4 blue-dark mb-0 lh-30">{lang::dict("My account")}</span>
                                <div class="d-flex flex-wrap align-items-center justify-content-end">
                                    <a style="color: #A4A5E3;" onclick=on_edit>
                                        <i class="far fa-edit fas fa-lg"></i>
                                    </a>
                                    <a class="ms-4" style="color: #A4A5E3;" onclick=on_close.clone()>
                                        <i class="fas fa-times fas fa-lg"></i>
                                    </a>
                                </div>
                            </div>
                            {maybe_user_pic.clone()}
                            <h1 class="text-primary-blue-light noir-bold is-size-18 lh-22  text-center pt-2">{&user_profile.full_name}</h1>
                        </>
                    })
                }
                MyProfilePageEdit::EditProfile => {
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
                            MyProfilePageMessage::Edit(MyProfilePageEdit::ChoosePic(result))
                        });
                    let on_done = self.link.callback(move |_| MyProfilePageMessage::Edit(MyProfilePageEdit::Done));
                    let on_save = self.link.callback(move |_| MyProfilePageMessage::Edit(MyProfilePageEdit::SaveProfile));
                    let change_full_name = self.props.user_profile.as_ref()
                        .and_then(|item| {
                            if item.user_staff.is_some() || item.user_teacher.is_some() {
                                Some(html! {
                                    <input ref=self.node_full_name.clone() class="input input-style-universal px-3 w-100" type="text" placeholder="Full name" value=user_profile.full_name.clone() />
                                })
                            } else {
                                Some(html! {})
                            }
                        }).unwrap_or(html! {});

                    let option_name_student = self.props.user_profile.as_ref()
                        .and_then(|item| {
                            if item.user_student.is_some() {
                                Some(html! {
                                    <h1 class="text-primary-blue-light noir-bold is-size-18 lh-22  text-center pt-2">{&user_profile.full_name}</h1>
                                })
                            } else {
                                Some(html! {})
                            }
                        }).unwrap_or(html! {});

                    Some(html! {
                        <>
                            <div class="my-3">
                                {maybe_user_pic.clone()}
                                <div class="input-group my-3">
                                    <label class="input-group-text" for="inputGroupFile02">
                                        <i class="fas fa-upload"></i>
                                    </label>
                                    <input type="file" class="form-control" id="inputGroupFile02" onchange=on_change />
                                </div>
                            </div>
                            // <input ref=self.node_full_name.clone() class="input input-style-universal px-3 w-100" type="text" placeholder="Full name" value=user_profile.full_name.clone() />
                            {change_full_name}
                            <div class="d-flex flex-wrap justify-content-between my-3">
                                <a class="btn btn-outline-purple-on" onclick=on_done>
                                    <i class="fas fa-times"></i>
                                </a>
                                <a class="btn btn-outline-primary-blue-dark" onclick=on_save>
                                    <i class="fas fa-check"></i>
                                </a>
                            </div>
                            {option_name_student}
                        </>
                    })
                }
                MyProfilePageEdit::SaveProfile => {
                    Some(html! {
                        <>
                            {maybe_user_pic.clone()}
                            <h1 class="text-primary-blue-light noir-bold is-size-18 lh-22  text-center pt-2">{&user_profile.full_name}</h1>
                        </>
                    })
                }
                _ => {
                    Some(html! {
                        <>
                            {maybe_user_pic.clone()}
                            <h1 class="text-primary-blue-light noir-bold is-size-18 lh-22  text-center pt-2">{&user_profile.full_name}</h1>
                        </>
                    })
                }
            })
            .unwrap_or(html! {
                <progress class="progress is-small is-primary" max="100"></progress>
            });

        // let maybe_auth_school = self.props.auth_school.as_ref().and_then(|auth_school| auth_school.school_profile.as_ref())
        //     .and_then(|school_profile| {
        //         Some(html! {
        //             <span>{&school_profile.name}</span>
        //         })
        //     })
        //     .unwrap_or(html! {});

        let maybe_email = self.props.user_profile.as_ref()
            .and_then(|user_profile| {
            let email = user_profile.email.clone();
            Some(html! {
                <span class="text-brown noir-light is-size-18 lh-22">{email}</span>
            })
        }).unwrap_or(html! {});

        info!("MAYBEDATA {:?}", self.props.user_profile);

        // let my_profile_data = self
        //     .props
        //     .user_profile
        //     .as_ref()
        //     .and_then(|data| data.user_by_pk.as_ref())
        //     .and_then(|user| user.license.as_ref())
        //     .and_then(|license| {
        //         Some(html! {
        //             <>
        //                 <div class="d-flex justify-content-center mt-2">
        //                     <span class="text-purple-gray text-center noir-light is-size-14 lh-17">{lang::dict("Staff")}</span>
        //                 </div>
        //                 // <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("License")}</span>
        //                 // <div class="mb-5"><span class="text-brown noir-light is-size-18 lh-22">{&license.license}</span></div>
        //                 <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("College")}<span>{"(s)"}</span></span>
        //                 <div class="mb-5"><span class="text-brown noir-light is-size-18 lh-22">{maybe_auth_school}</span></div>
        //                 <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("Grade")}<span>{"(s)"}</span></span>
        //                 <div class="mb-5"><span class="text-brown noir-light is-size-18 lh-22">{"Todos los Grados"}</span></div>
        //                 <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("Email")}</span>
        //                 {maybe_email}
        //             </>
        //         })
        //     })
        //     .unwrap_or(html! {});

        let maybe_option_user = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                if item.user_staff.is_some() {
                    Some(html! {
                        <>
                            {maybe_user_profile_home}
                            // {my_profile_data}
                            <div class="d-flex justify-content-center mt-2">
                                <span class="text-purple-gray text-center noir-light is-size-14 lh-17">{lang::dict("Staff")}</span>
                            </div>
                            // <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("License")}</span>
                            // <div class="mb-5"><span class="text-brown noir-light is-size-18 lh-22">{&license.license}</span></div>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("College")}<span>{"(s)"}</span></span>
                            // <div class="mb-5"><span class="text-brown noir-light is-size-18 lh-22">{maybe_auth_school}</span></div>
                            <div class="mb-5"><span class="text-brown noir-light is-size-18 lh-22">{"Todos los Colegios"}</span></div>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("Grade")}<span>{"(s)"}</span></span>
                            <div class="mb-5"><span class="text-brown noir-light is-size-18 lh-22">{"Todos los Grados"}</span></div>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("Email")}</span>
                            {maybe_email}
                        </>
                    })
                } else {
                    Some(html! {
                        {maybe_user_profile_home}
                    })
                }
            })
            .unwrap_or(html! {});
        html! {
            {maybe_option_user}
        }
    }
}