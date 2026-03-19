use log::*;
use regex::Regex;
use std::io::Write;
use yew::prelude::*;
use yew::format::Json;
use serde::Deserialize;
use code_location::code_location;
use crate::list_schools::SchoolProfile;
use yew::services::{
    fetch::{FetchService, FetchTask, Request, Response},
    reader::{FileData, ReaderService, File, ReaderTask},
};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_models::school_model;
use roboxmaker_main::{config, lang};
use roboxmaker_types::types::MyUserProfile;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request as OtherTask, RequestTask};


pub struct SchoolPage {
    link: ComponentLink<Self>,
    props: Properties,
    graphql_task: Option<GraphQLTask>,
    reader_task: Vec<ReaderTask>,
    task_save: Option<RequestTask>,
    upload_task: Option<FetchTask>,
    edit: SchoolPageEdit,
    save_status: bool,
    url_photo: String,
    school_logo: String,
    school_name: String,
    school_motto: String,
    school_address: String,
    school_telephone: String,
    school_biography: String,
    school_mission: String,
    school_vision: String,
    school_we_are: String,
    validate_telephone: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Properties {
    pub user_profile: Option<MyUserProfile>,
    pub school_profile: SchoolProfile,
    pub close_school_profile: Callback<MouseEvent>,
}

#[derive(Debug)]
pub enum SchoolPageEdit {
    Hidden,
    None,
    EditProfile,
    SaveProfile,
    ChoosePic(Vec<File>),
    SavePicture(String),
    ChangePic(FileData),
}
#[derive(Debug)]
pub enum Message {
    // FetchShoolById(SchoolId),
    // School(Option<school_model::school_by_id::ResponseData>),
    SchoolUpdate(Option<school_model::school_profile_by_id_update::ResponseData>),
    Edit(SchoolPageEdit),
    UpdateSchoolName(String),
    UpdateSchoolMotto(String),
    UpdateSchoolAddress(String),
    UpdateSchoolTelephone(String),
    UpdateSchoolBiography(String),
    UpdateSchoolMission(String),
    UpdateSchoolVision(String),
    UpdateSchoolWeAre(String),
}

#[derive(Deserialize, Debug)]
struct UserPageFileUploadResponse {
    url: String,
}

impl Component for SchoolPage {
    type Message = Message;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        // link.send_message(Message::FetchShoolById(roboxmaker_types::types::SchoolId(props.school_profile.school_id)));
        // info!("DATASCHOOL {:?}", props.school_profile);

        let school_logo = props.school_profile.logo.clone();
        let school_name = props.school_profile.name.clone();
        let school_motto = props.school_profile.motto.clone();
        let school_address = props.school_profile.address.clone();
        let school_telephone = props.school_profile.telephone.clone();
        let school_biography = props.school_profile.web_site.clone();
        let school_mission = props.school_profile.mission.clone();
        let school_vision = props.school_profile.vision.clone();
        let school_we_are = props.school_profile.we_are.clone();
        SchoolPage {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task_save: None,
            reader_task: vec![],
            upload_task: None,
            edit: SchoolPageEdit::None,
            save_status: true,
            url_photo: format!("{}/uploads/school.png", config::AKER_FILES_URL),
            school_logo,
            school_name,
            school_motto,
            school_address,
            school_telephone,
            school_biography,
            school_mission,
            school_vision,
            school_we_are,
            validate_telephone: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            Message::UpdateSchoolName(school_name) => {
                self.save_status = false;
                self.school_name = school_name;
            },
            Message::UpdateSchoolMotto(school_motto) => {
                self.save_status = false;
                self.school_motto = school_motto
            },
            Message::UpdateSchoolAddress(school_address) => {
                self.save_status = false;
                self.school_address = school_address
            },
            Message::UpdateSchoolTelephone(school_telephone) => {
                let telephone_regex = Regex::new(r"^\d{4}-\d{4}$").unwrap();
                if telephone_regex.is_match(&school_telephone) {
                    self.validate_telephone = true;
                } else {
                    self.validate_telephone = false;
                }
                self.save_status = false;
                self.school_telephone = school_telephone
            }
            Message::UpdateSchoolBiography(school_biography) => {
                self.save_status = false;
                self.school_biography = school_biography
            }
            Message::UpdateSchoolMission(school_mission) => {
                self.save_status = false;
                self.school_mission = school_mission
            },
            Message::UpdateSchoolVision(school_vision) => {
                self.save_status = false;
                self.school_vision = school_vision
            },
            Message::UpdateSchoolWeAre(school_we_are) => {
                self.save_status = false;
                self.school_we_are = school_we_are
            },
            // Message::FetchShoolById(school_id) => {
            //     if let Some(graphql_task) = self.graphql_task.as_mut() {
            //         let vars = school_model::school_by_id::Variables {
            //             school_id: school_id.0,
            //         };
            //         let task = school_model::SchoolById::request(
            //             graphql_task,
            //             &self.link, 
            //             vars, 
            //             |response| {
            //                 Message::School(response)
            //             },
            //         );
            //         self.school_task = Some(task);
            //     }
            // }
            // Message::School(school) => {
            //     school
            //         .as_ref()
            //         .and_then(|school| school.school_by_pk.as_ref())
            //         .and_then(|school_by_pk| school_by_pk.school_profile.as_ref())
            //         .and_then(|school_profile| school_profile.logo.as_ref())
            //         .and_then(|logo| {
            //             self.school_logo = logo.clone();
            //             Some(())
            //         });

            //     self.school = school.clone().and_then(|data| data.school_by_pk);
            //     if school.clone().and_then(|data| data.school_by_pk).is_some() {
            //         self.edit = SchoolPageEdit::None;
            //         self.save_status = true;
            //         let school_profile = school.clone().and_then(|data| data.school_by_pk).unwrap().school_profile.unwrap();
            //         self.school_name = school_profile.name.clone();
            //         self.school_motto = school_profile.motto.clone().unwrap_or("".to_string());
            //         self.school_address = school_profile.address.clone().unwrap_or("".to_string());
            //         self.school_telephone =
            //             school_profile.telephone.clone().unwrap_or("".to_string());
            //         self.school_biography =
            //             school_profile.biography.clone().unwrap_or("".to_string());
            //         self.school_mission = school_profile.mission.clone().unwrap_or("".to_string());
            //         self.school_vision = school_profile.vision.clone().unwrap_or("".to_string());
            //         self.school_we_are = school_profile.we_are.clone().unwrap_or("".to_string());
            //     }
            // }
            Message::SchoolUpdate(response) => {
                info!("DATASCHOOL {:?}", response);
            }
            Message::Edit(edit) => {
                self.edit = edit;
                match &self.edit {
                    SchoolPageEdit::Hidden => {}
                    SchoolPageEdit::EditProfile => {}
                    SchoolPageEdit::SaveProfile => {
                        let school_id = self.props.school_profile.school_id;
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let vars = school_model::school_profile_by_id_update::Variables {
                                school_id,
                                logo: self.school_logo.clone(),
                                name: self.school_name.clone(),
                                motto: self.school_motto.clone(),
                                address: self.school_address.clone(),
                                telephone: self.school_telephone.clone(),
                                mission: self.school_mission.clone(),
                                vision: self.school_vision.clone(),
                                we_are: self.school_we_are.clone(),
                                biography: self.school_biography.clone(),
                            };
                            let task = school_model::SchoolProfileByIdUpdate::request(
                                graphql_task,
                                &self.link, 
                                vars, 
                                |response| Message::SchoolUpdate(response),
                            );
                            self.task_save = Some(task);
                            self.edit = SchoolPageEdit::None;
                        }
                    }
                    SchoolPageEdit::SavePicture(pic_patch) => {
                        let school_id = self.props.school_profile.school_id;
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let vars = school_model::school_profile_by_id_update::Variables {
                                school_id,
                                logo: pic_patch.clone(),
                                name: self.school_name.clone(),
                                motto: self.school_motto.clone(),
                                address: self.school_address.clone(),
                                telephone: self.school_telephone.clone(),
                                mission: self.school_mission.clone(),
                                vision: self.school_vision.clone(),
                                we_are: self.school_we_are.clone(),
                                biography: self.school_biography.clone(),
                            };
                            let task = school_model::SchoolProfileByIdUpdate::request(
                                graphql_task,
                                &self.link, 
                                vars, 
                                |response| Message::SchoolUpdate(response),
                            );
                            self.task_save = Some(task);
                        }
                    }
                    SchoolPageEdit::None => {}
                    SchoolPageEdit::ChoosePic(files) => {
                        if let Some(file) = files.get(0) {
                            info!("{:?}", file);
                            let task = { 
                                let callback =self.link.callback(move |file| {
                                    Message::Edit(SchoolPageEdit::ChangePic(file))
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
                    SchoolPageEdit::ChangePic(file) => {
                        let user_id = self
                            .props
                            .user_profile
                            .as_ref()
                            .and_then(|auth_user| Some(auth_user.user_id.to_string()));

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
                            .header("aker-user-id", user_id.unwrap_or_default())
                            .header(
                                "Content-Type",
                                format!("multipart/form-data; boundary={}", BOUNDARY),
                            )
                            .body(img_bytes)
                            .expect("Failed to build request.");

                        let pic_path = self.url_photo.clone();
                        let callback = self.link.callback(
                            move |res: Response<
                                Json<Result<UserPageFileUploadResponse, anyhow::Error>>,
                            >| {
                                info!("{:?}", res);
                                let url = if let (_meta, Json(Ok(file_upload))) = res.into_parts() {
                                    Some(file_upload.url)
                                } else {
                                    None
                                };
                                Message::Edit(SchoolPageEdit::SavePicture(
                                    url.unwrap_or(pic_path.clone()),
                                ))
                            },
                        );
                        self.upload_task = FetchService::fetch_binary(req, callback).ok();
                    }
                }
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            // info!("DATASCHOOL {:?}", self.props.school_profile);
             self.school_logo = self.props.school_profile.logo.clone();
             self.school_name = self.props.school_profile.name.clone();
             self.school_motto = self.props.school_profile.motto.clone();
             self.school_address = self.props.school_profile.address.clone();
             self.school_telephone = self.props.school_profile.telephone.clone();
             self.school_biography = self.props.school_profile.web_site.clone();
             self.school_mission = self.props.school_profile.mission.clone();
             self.school_vision = self.props.school_profile.vision.clone();
             self.school_we_are = self.props.school_profile.we_are.clone();
            should_render = true;
        }
        should_render
    }

    fn view(&self) -> Html {
        let on_update_school_name = self.link.callback(|e: InputData| Message::UpdateSchoolName(e.value));
        let on_update_school_motto = self.link.callback(|e: InputData| Message::UpdateSchoolMotto(e.value));
        let on_update_school_address = self.link.callback(|e: InputData| Message::UpdateSchoolAddress(e.value));
        let on_update_school_telephone = self.link.callback(|e: InputData| Message::UpdateSchoolTelephone(e.value));
        let on_update_school_biography = self.link.callback(|e: InputData| Message::UpdateSchoolBiography(e.value));
        let on_update_school_mission = self.link.callback(|e: InputData| Message::UpdateSchoolMission(e.value));
        let on_update_school_vision = self.link.callback(|e: InputData| Message::UpdateSchoolVision(e.value));
        let on_update_school_we_are = self.link.callback(|e: InputData| Message::UpdateSchoolWeAre(e.value));
        let on_change = self.link.callback(move |data: ChangeData| {
                let mut result = Vec::new();
                if let ChangeData::Files(files) = data {
                    let files = js_sys::try_iter(&files)
                        .unwrap()
                        .unwrap()
                        .map(|v| File::from(v.unwrap()));
                    result.extend(files);
                }
                Message::Edit(SchoolPageEdit::ChoosePic(result))
        });

        let change_picture_school = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                Some(
                    html! {
                        <div class="input-group my-3">
                            <label class="input-group-text" for="inputGroupFile02">
                                <i class="fas fa-upload"></i>
                            </label>
                            <input type="file" class="form-control" id="inputGroupFile02" onchange=on_change />
                        </div>
                    }
                )
            } else {
                None
            }
        }).unwrap_or(html!{});

        let school_logo = {
            html! {
                <img class="img-card-72" src={self.props.school_profile.logo.clone()} />
            }
        };

        let on_edit = self.link.callback(move |_| Message::Edit(SchoolPageEdit::EditProfile));
        let on_none = self.link.callback(move |_| Message::Edit(SchoolPageEdit::None));
        let on_save = self.link.callback(move |_| Message::Edit(SchoolPageEdit::SaveProfile));
        let status_save = if self.save_status {
            html! {
                <>{lang::dict("Saved")}</>
            }
        } else {
            html! {
                <>{lang::dict("Unsaved")}</>
            }
        };

        let buttons = html! {
            <div class="d-flex flex-wrap justify-content-between my-3">
                <a class="btn btn-outline-purple-on" onclick=on_none>
                    <i class="fas fa-times"></i>
                </a>
                {status_save}
                <a class="btn btn-outline-primary-blue-dark" onclick=on_save>
                    <i class="fas fa-check"></i>
                </a>
            </div>

        };

        let edit_button = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <div class="level-right">
                            <div class="level-item">
                                <a style="color: #A4A5E3;" onclick=on_edit>
                                    <span class="icon">
                                        <i class="far fa-edit "></i>
                                    </span>
                                </a>
                            </div>
                        </div>
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {});
            
        let maybe_error_telephone = if !self.validate_telephone && !self.school_telephone.is_empty() {
            html! {
                <span class="text-danger is-size-14">{"Este número no es válido, solo números"}</span>
            }
        } else {
            html! {}
        };

        let maybe_view = match self.edit {
            SchoolPageEdit::Hidden => {
                html! {}
            }
            SchoolPageEdit::None => {
                html! {
                    <>
                        // <a class="btn bg-white text-gray p-1" style="width: 35px; height: 35px;" onclick=self.props.close_school_profile.clone()>
                        //     <i class="fas fa-times"></i>
                        // </a>
                        <a class="btn btn-outline-purple-on" onclick={self.props.close_school_profile.clone()}>
                            <i class="fas fa-times"></i>
                        </a>
                        <div class="d-flex align-items-center justify-content-between mt-2">
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-30">{"Información"}</span>
                            {edit_button}
                        </div>
                        <div class="d-flex justify-content-center" style="margin-top: 30px;">{school_logo}</div>
                        <div class="d-flex flex-column">
                            <div class="text-center pb-5 pt-2 mt-1"><span class="text-secondary-purple noir-bold is-size-18 lh-22">{&self.props.school_profile.name}</span></div>
                            <span class="text-primary-blue-dark noir-bold is-size-20 lh-25 text-center">{self.props.school_profile.motto.clone()}</span>
                            <span class="text-purple-gray noir-regular is-size-14 lh-18 text-center mt-1 mb-7">{"Lema del colegio"}</span>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("Address: ")}</span>
                            <span class="text-brown noir-regular is-size-18 lh-22 mb-5">{self.props.school_profile.address.clone()}</span>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("Telephone: ")}</span>
                            <span class="text-brown noir-regular is-size-18 lh-22 mb-5">{self.props.school_profile.telephone.clone()}</span>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("About us?")}</span>
                            <span class="text-brown noir-regular is-size-18 lh-22 mb-5">{self.props.school_profile.we_are.clone()}</span>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("Mission:")}</span>
                            <span class="text-brown noir-regular is-size-18 lh-22 mb-5">{self.props.school_profile.mission.clone()}</span>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("Vision:")}</span>
                            <span class="text-brown noir-regular is-size-18 lh-22 mb-5">{self.props.school_profile.vision.clone()}</span>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("Website:")}</span>
                            <span class="text-brown noir-regular is-size-18 lh-22 mb-5">{self.props.school_profile.web_site.clone()}</span>
                        </div>
                    </>
                }
            }
            SchoolPageEdit::EditProfile => {
                html! {
                    <>
                        <div class="d-flex justify-content-center" style="margin-top: 40px;">{school_logo}</div>
                        <div class="d-flex justify-content-center py-4">{change_picture_school}</div>
                        {buttons}
                        <div class="field">
                            <label for="" class="label text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("School Name: ")}</label>
                                <input type="text" value=self.school_name.clone() class="input px-3 input-style-universal w-100"
                                    oninput=on_update_school_name required=true />
                        </div>
                        <div class="field">
                            <label for="" class="label text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("School Motto: ")}</label>
                            <input type="text" value=self.school_motto.clone() class="input px-3 input-style-universal w-100"
                                oninput=on_update_school_motto required=true />
                        </div>
                        <div class="field">
                            <label for="" class="label text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("School Address: ")}</label>
                            <input type="text" value=self.school_address.clone() class="input px-3 input-style-universal w-100"
                                oninput=on_update_school_address required=true />
                        </div>
                        <div class="field">
                            <label class="label text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("School Telephone: ")}</label>
                            <div class="control">
                                <input type="tel" value=self.school_telephone.clone() class="input px-3 input-style-universal w-100" oninput=on_update_school_telephone required=true />
                            </div>
                            {maybe_error_telephone}
                        </div>
                        <div class="field">
                            <label for="" class="label text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("About us: ")}</label>
                            <textarea class="textarea px-3 py-2 input-style-universal w-100" oninput=on_update_school_we_are
                                value=self.school_we_are.clone()></textarea>
                        </div>
                        <div class="field">
                            <label for="" class="label text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("School Vision: ")}</label>
                            <textarea class="textarea px-3 py-2 input-style-universal w-100" oninput=on_update_school_vision
                                value=self.school_vision.clone()></textarea>
                        </div>
                        <div class="field">
                            <label for="" class="label text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("School Mission: ")}</label>
                            <textarea class="textarea px-3 py-2 input-style-universal w-100" placeholder="" oninput=on_update_school_mission
                                value=self.school_mission.clone()></textarea>
                        </div>
                        <div class="field">
                            <label for="" class="label text-primary-blue-dark noir-bold is-size-14 lh-18 mb-1 ">{lang::dict("College website: ")}</label>
                            <textarea class="textarea px-3 py-2 input-style-universal w-100" placeholder=""
                                oninput=on_update_school_biography value=self.school_biography.clone()></textarea>
                        </div>
                    </>
                }
            }
            SchoolPageEdit::SaveProfile => {
                html! {<progress class="progress is-small is-primary" max="100"></progress>}
            }
            _ => {
                html! {}
            }
        };
        html! {
            <>
                <div class="scrool-container-scholl-profile">{maybe_view.clone()}</div>
            </>
        }
        // if (school) = &self.props.school_profile {
        //     let maybe_school_profile = if let Some(school_profile) = &school.school_profile {


        //     } else {
        //         html! {}
        //     };
        //     html! {
        //         <>
        //             <div class="scrool-container-scholl-profile">{maybe_school_profile.clone()}</div>
        //         </>
        //     }
        // } else {
        //     html! {
        //         <progress class="progress is-small is-primary" max="100"></progress>
        //     }
        // }
    }
}