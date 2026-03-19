use log::*;
use yew::prelude::*;
use serde_derive::{Deserialize};
use code_location::code_location;
use yew::{html, Component, Html, NodeRef};
use yew_router::scope_ext::RouterScopeExt;
use crate::contributions_and_comments::ContributionsAndComments;
use crate::{user_robots::UserRobots, last_robots_card::UserStyle};
// use yew::services::{fetch::{FetchService, FetchTask, Request, Response},
//     reader::{FileData, ReaderService, File, ReaderTask},
// };

use roboxmaker_main::{lang, config};
use roboxmaker_models::{user_model, message_model, group_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request as OtherRequest, RequestTask};
use roboxmaker_types::types::{GroupId, UserId, SchoolId, AppRoute, gen_private_group_id, MyUserProfile};

pub struct UserPage {
    task_load: Option<RequestTask>,
    task_save: Option<RequestTask>,
    // upload_task: Option<FetchTask>,
    edit: UserPageEdit,
    node_full_name: NodeRef,
    meet_private_group_id: Option<GroupId>,
    direct_message_private_group_id: Option<GroupId>,
    // reader_task: Vec<ReaderTask>,
    graphql_task: Option<GraphQLTask>,
    user_update: Option<user_model::user_profile_by_id_update::ResponseData>,
    full_name: String,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct UserPageProperties {
    pub user_id: UserId,
    pub user_profile: Option<MyUserProfile>,
    #[prop_or(None)]
    pub school_id: Option<SchoolId>,
    pub close_modal_callback: Callback<MouseEvent>,
    #[prop_or(None)]
    pub group_id: Option<GroupId>,
    pub full_name: String,
    pub pic_path: String,
    pub staff: bool,
    pub teacher: bool,
    pub school_name: String,
    pub license: String,
}

#[derive(Debug)]
pub enum UserPageEdit {
    None,
    EditProfile,
    SaveProfile,
    SavePicture(String),
    // ChoosePic(Vec<File>),
    // ChangePic(FileData),
    Done,
}

#[derive(Debug)]
pub enum UserPageMessage {
    FetchUserById(UserId),
    User(Option<user_model::user_by_id::ResponseData>),
    UpdateUserResponse(Option<user_model::user_profile_by_id_update::ResponseData>),
    DirectMessageGroup(Option<message_model::direct_message_group_by_group_id::ResponseData>),
    MessageGroupCreate(Option<group_model::group_create_with_members::ResponseData>),
    Edit(UserPageEdit),
}

#[derive(Deserialize, Debug)]
struct UserPageFileUploadResponse {
    url: String,
}

impl Component for UserPage {
    type Message = UserPageMessage;
    type Properties = UserPageProperties;

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(UserPageMessage::FetchUserById(ctx.props().user_id));

        UserPage {
            task_load: None,
            task_save: None,
            // upload_task: None,
            edit: UserPageEdit::None,
            node_full_name: NodeRef::default(),
            meet_private_group_id: None,
            direct_message_private_group_id: None,
            // reader_task: vec![],
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            user_update: None,
            full_name: ctx.props().full_name.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            UserPageMessage::UpdateUserResponse(user_update) => {
                self.user_update = user_update
            }
            UserPageMessage::FetchUserById(_user_id) => {
                should_update = false;
                self.meet_private_group_id = Some(ctx.props().user_id)
                    .zip(ctx.props().user_profile.as_ref())
                    .and_then(|(user_id, user)| {

                    let mut uuids= vec![ &user.user_id.0, &user_id.0];
                    uuids.sort();

                    if user_id.0 == user.user_id.0 {
                        None
                    }
                    else {
                        Some(gen_private_group_id(config::MEET_MD5, uuids))
                    }
                });
                self.direct_message_private_group_id = Some(ctx.props().user_id)
                .zip(ctx.props().user_profile.as_ref())
                .and_then(|(user_id, user)| {

                    let mut uuids= vec![ &user.user_id.0, &user_id.0];
                    uuids.sort();

                    Some(gen_private_group_id(config::DM_MD5, uuids))
                });
                // self.link().send_message(UserPageMessage::User(ctx.props().user_profile.clone()));
            }
            UserPageMessage::User(_user) => {
                // ctx.props().user_profile = user;
                self.edit = UserPageEdit::None;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if self.direct_message_private_group_id.is_some() {
                        let vars = message_model::direct_message_group_by_group_id::Variables {
                            group_id: self.direct_message_private_group_id.unwrap().0
                        };
                        let task = message_model::DirectMessageGroupByGroupId::request(
                            graphql_task, 
                            &ctx, 
                            vars, 
                            |response| {
                                UserPageMessage::DirectMessageGroup(response)
                            }
                        );
                        self.task_load = Some(task);
                    }
                }
            }
            UserPageMessage::DirectMessageGroup(response) =>{
                if response.clone().and_then(|data| data.group_by_pk).is_none() {

                    let user_id = ctx.props().user_id.0;
                    let user_auth_id = ctx.props().user_profile.as_ref().unwrap().user_id.0;
                    let group_id = self.direct_message_private_group_id.unwrap().0;
                    // let school_id = ctx.props().school_id.unwrap().0;
                    if ctx.props().school_id.is_some() {
                        let mut members: Vec<group_model::group_create_with_members::CreateGroupWithMembersGroupMemberInsertInput> = Vec::new();
    
                        members.push(group_model::group_create_with_members::CreateGroupWithMembersGroupMemberInsertInput{group_id: Some(group_id), user_id: Some(user_auth_id)});
                        
                        if user_id != user_auth_id {
                            members.push(group_model::group_create_with_members::CreateGroupWithMembersGroupMemberInsertInput{group_id: Some(group_id), user_id: Some(user_id)});
                        };
                        if let Some(graphql_task) = self.graphql_task.as_mut() {
                            let vars = group_model::group_create_with_members::Variables {
                                group_id,
                                school_id: ctx.props().school_id.unwrap().0,
                                members
                            };
                            let task = group_model::GroupCreateWithMembers::request(
                                graphql_task, 
                                &ctx, 
                                vars, 
                                |response| {
                                    UserPageMessage::MessageGroupCreate(response)
                                }
                            );
                            self.task_load = Some(task);
                        }
                    }

                } 
            }
            UserPageMessage::MessageGroupCreate(_) => {
                // info!("{:?}", response)
            }
            UserPageMessage::Edit(edit) => {
                self.edit = edit;
                match &self.edit {
                    UserPageEdit::Done => {}
                    UserPageEdit::SaveProfile => {
                        let user_id = ctx.props().user_id.0;
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
                                pic_path: ctx.props().pic_path.clone(),
                            };
        
                            let task = user_model::UserProfileByIdUpdate::request(
                                graphql_task,
                                &ctx,
                                vars,
                                |response| {
                                    UserPageMessage::UpdateUserResponse(response)
                                },
                            );
                            self.task_save = Some(task);
                        }
                        self.edit = UserPageEdit::None;
                    }
                    UserPageEdit::SavePicture(pic_path) => {
                        let user_id = ctx.props().user_id;
                        if let Some(graphql_task) = self.graphql_task.as_mut() {

                            let vars = user_model::user_profile_by_id_update::Variables { 
                                user_id: user_id.0,
                                full_name: self.full_name.clone(),
                                pic_path: pic_path.clone(),
                            };
        
                            let task = user_model::UserProfileByIdUpdate::request(
                                graphql_task,
                                &ctx,
                                vars,
                                |response| {
                                    UserPageMessage::UpdateUserResponse(response)
                                },
                            );
                            self.task_save = Some(task);
                        }
                        self.edit = UserPageEdit::None;
                    }
                    // UserPageEdit::ChoosePic(files) => {
                    //     if let Some(file) = files.get(0) {
                    //         info!("{:?}", file);
                    //         let task = { 
                    //             let callback =ctx.link().callback(move |file| {
                    //                 UserPageMessage::Edit(UserPageEdit::ChangePic(file))
                    //             });
                    //             ReaderService::read_file(
                    //                 file.clone(),
                    //                 callback
                    //             )
                    //             .unwrap()
                    //         };
                    //         self.reader_task.push(task);
                    //     }
                    //     self.edit = UserPageEdit::None;
                    // }
                    // UserPageEdit::ChangePic(file) => {
                    //     const BOUNDARY: &'static str = "------------------------ea3bbcf87c101592";

                    //     let image_data = |content: &[u8]| {
                    //         let mut data = Vec::new();
                    //         write!(data, "--{}\r\n", BOUNDARY)?;
                    //         write!(
                    //                 data,
                    //                 "Content-Disposition: form-data; name=\"upload\"; filename=\"{}\"\r\n",
                    //                 file.name)?;
                    //         write!(data, "\r\n")?;
                    //         data.extend_from_slice(content);
                    //         write!(data, "\r\n")?;
                    //         write!(data, "--{}--\r\n", BOUNDARY)?;
                    //         Ok(data)
                    //     };

                    //     let img_bytes = image_data(&file.content[..]);

                    //     let upload_url = format!("{}/upload.php", config::AKER_FILES_URL);

                    //     let req = Request::post(upload_url)
                    //         .header("aker-user-id", ctx.props().user_id.0.to_string())
                    //         .header(
                    //             "Content-Type",
                    //             format!("multipart/form-data; boundary={}", BOUNDARY),
                    //         )
                    //         .body(img_bytes)
                    //         .expect("Failed to build request.");

                    //     let pic_path = ctx.props().pic_path.clone();
                    //     let callback = ctx.link().callback(
                    //         move |res: Response<
                    //             Json<Result<UserPageFileUploadResponse, anyhow::Error>>,
                    //         >| {
                    //             info!("{:?}", res);
                    //             let url = if let (_meta, Json(Ok(file_upload))) = res.into_parts() {
                    //                 Some(file_upload.url)
                    //             } else {
                    //                 None
                    //             };
                    //             UserPageMessage::Edit(UserPageEdit::SavePicture(url.unwrap_or(pic_path.clone())))
                    //         },
                    //     );

                    //     self.upload_task = FetchService::fetch_binary(req, callback).ok();
                    //     self.edit = UserPageEdit::None;
                    // }
                    _ => {}
                }
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
        let user_id = ctx.props().user_id;
        let full_name = self.full_name.clone();
        let pic_path = ctx.props().pic_path.clone();

        let maybe_user_profile_edit  = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item|{
                let on_edit = ctx
                    .link()
                    .callback(move |_| UserPageMessage::Edit(UserPageEdit::EditProfile));
                    if item.user_staff.is_some() || item.user_id.0 == user_id.0 {
                        Some(html! {
                            <a style="color: #A4A5E3; !important height: 24px;" onclick={on_edit}>
                                <span class="icon">
                                    <i class="far fa-edit"></i>
                                </span>
                            </a>
                        })
                    } else {
                        None
                    }
            })
            .unwrap_or(html! {});
        let maybe_user_profile = match self.edit {
            UserPageEdit::None => {

                let maybe_meet = self
                    .meet_private_group_id
                    .as_ref()
                    .zip(ctx.props().user_profile.as_ref())
                    .and_then(|(group_id, user)| {
                        let group_id = group_id.clone();

                        let navigator = ctx.link().navigator().unwrap();
                        let on_direct_meet = Callback::from(move |_| navigator.push(&AppRoute::MeetDirect{group_id}));

                        if user.user_teacher.is_some() || user.user_staff.is_some(){
                            Some(html!(
                                <a onclick={on_direct_meet}>
                                    <img src="/icons/video-3.svg" style="height: 24px;" />
                                </a>
                            ))
                        } else {
                            Some(html!{})
                        }
                    })
                    .unwrap_or_default();
                    // let user_id = ctx.props().user_id.to_string();

                Some(html! {
                    <div class="d-flex flex-column">
                        <div class="text-center">
                            <h1 class="text-primary-blue-dark noir-bold is-size-18 lh-22 py-4">{&full_name.clone()}</h1>
                        </div>
                        // <h1 class="text-primary-blue-dark noir-bold is-size-18 lh-22 my-4 text-center">{&user_id}</h1>
                        <div class="d-flex flex-wrap justify-content-evenly pb-5 mb-2">
                            {maybe_meet}
                            {maybe_user_profile_edit}
                        </div>
                    </div>
                })
            }
            UserPageEdit::EditProfile => {
                let maybe_pic_profile_edit = ctx
                    .props()
                    .user_profile
                    .as_ref()
                    .and_then(|item|{
                        // let on_change = self
                        //     .link
                        //     .callback(move |data| {
                        //         let mut result = Vec::new();
                        //         if let ChangeData::Files(files) = data {
                        //             let files = js_sys::try_iter(&files)
                        //                 .unwrap()
                        //                 .unwrap()
                        //                 .map(|v| File::from(v.unwrap()));
                        //             result.extend(files);
                        //         }
                        //         UserPageMessage::Edit(UserPageEdit::ChoosePic(result))
                        //     });
                            if item.user_staff.is_some() || user_id.0 == item.user_id.0 {
                                Some(html! {
                                    <div class="input-group my-3">
                                        <label class="input-group-text" for="inputGroupFile02">
                                            <i class="fas fa-upload"></i>
                                        </label>
                                        // <input type="file" class="form-control" id="inputGroupFile02" onchange=on_change />
                                        <input type="file" class="form-control" id="inputGroupFile02" />
                                    </div>
                                })
                            } else {
                                None
                            }
                    })
                    .unwrap_or(html! {});

                let on_done = ctx.link().callback(move |_| UserPageMessage::Edit(UserPageEdit::Done));
                let on_save = ctx.link().callback(move |_| UserPageMessage::Edit(UserPageEdit::SaveProfile));
                Some(html! {
                    <>
                        <div class="my-3">{maybe_pic_profile_edit}</div>
                        <input ref={self.node_full_name.clone()} class="input input-style-universal px-3 w-100" type="text" placeholder="Full name" value={full_name.clone()} />
                        <div class="d-flex flex-wrap justify-content-between my-3">
                            <a class="btn btn-outline-purple-on" onclick={on_done}>
                                <i class="fas fa-times"></i>
                            </a>
                            <a class="btn btn-outline-primary-blue-dark" onclick={on_save}>
                                <i class="fas fa-check"></i>
                            </a>
                        </div>
                    </>
                })
            }
            UserPageEdit::SaveProfile => {
                Some(html! {
                    <h1 class="text-primary-blue-dark noir-bold is-size-18 lh-22 my-4 text-center">{&full_name.clone()}</h1>
                })
            }
            _ => {
                Some(html! {
                    <>
                        <h1 class="text-primary-blue-dark noir-bold is-size-18 lh-22 my-4 text-center">{&full_name.clone()}</h1>
                    </>
                })
            }
        }.unwrap_or(html! {});

        let maybe_user_robots = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_teacher.is_some() || item.user_staff.is_some() || (user_id.0 == item.user_id.0){
                    let robots_list = {
                        html! {
                            <UserRobots user_id={ctx.props().user_id} 
                                user_profile={ctx.props().user_profile.clone()}
                                maybe_style={UserStyle::MemberProfile} />
                        }
                    };
                    Some(
                        html!{
                            {robots_list}
                        }
                    )
                } else {
                    Some(html!{})
                }
            })
            .unwrap_or_default();
        
        let maybe_license = if ctx.props().staff || ctx.props().teacher {
            html! {
                <>
                    <span class="text-primary-blue-dark noir-bold is-size-20 lh-24 pb-2">{lang::dict("College")}</span>
                    <div class="mb-4"><span class="text-brown noir-light is-size-18 lh-22">{&ctx.props().school_name}</span></div>
                </>
            }
        } else {
            html! {
                <>
                    <span class="text-primary-blue-dark noir-bold is-size-20 lh-24 pb-2">{lang::dict("College")}</span>
                    <div class="mb-4"><span class="text-brown noir-light is-size-18 lh-22">{&ctx.props().school_name}</span></div>
                    <br/>
                    <span class="text-primary-blue-dark noir-bold is-size-20 lh-24 pb-2">{lang::dict("License")}</span>
                    <div class="mb-4"><span class="text-brown noir-light is-size-18 lh-22">{&ctx.props().license}</span></div>
                </>
            }
        };

        html! {
            <div class="">
                <div class="d-flex justify-content-start position-absolute">
                    <a class="btn bg-white text-gray" onclick={ctx.props().close_modal_callback.clone()}>
                        <i class="fas fa-times"></i>
                    </a>
                </div>
                <div class="d-flex justify-content-center">
                    <img class="img-card-128" src={pic_path.clone()} />
                </div>  
                {maybe_user_profile}
                {maybe_license}
                <h1 class="text-primary-blue-dark noir-bold is-size-20 lh-24 mb-4-2 mb-4">{lang::dict("Initiated Robots")}</h1>
                {maybe_user_robots}
                <div class="mt-4">
                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">{"Colaboraciones y comentarios"}</span>
                </div>
                <div class="mt-4">
                    <ContributionsAndComments user_id={user_id} />
                </div>
            </div>
        }
    }
}