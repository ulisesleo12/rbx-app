use log::*;
// use uuid::Uuid;
use gloo_storage::Storage;
use code_location::code_location;
use yew::{html, Component, Html, Properties, NodeRef, Context};

use roboxmaker_main::{config, lang};
// use roboxmaker_files::user_files::UserFiles;
use roboxmaker_utils::functions::school_profile_data;
use roboxmaker_models::{group_model, message_model};
use roboxmaker_user::{user_robots::UserRobots, last_robots_card::UserStyle};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{SchoolId, UserId, GroupId, gen_private_group_id, MyUserProfile};
use roboxmaker_message::{MessageGroupCategory, message_list::MessageList, user_messages::MessagesByUserId};

pub struct MySpaceView {
    graphql_task: Option<GraphQLTask>,
    direct_message_task: Option<RequestTask>,
    meet_private_group_id: Option<GroupId>,
    direct_message_private_group_id: Option<GroupId>,
    more_files: bool,
    udpload_files: bool,
    search_node: NodeRef,
    maybe_section_search: bool,
    saved_sidebar_state: bool,
    school_id: Option<SchoolId>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MySpaceProperties {
    pub user_id: UserId,
    pub user_profile: Option<MyUserProfile>
}

#[derive(Debug)]
pub enum MySpaceMessage {
    FetchUserById,
    User,
    DirectMessageGroup(Option<message_model::direct_message_group_by_group_id::ResponseData>),
    MessageGroupCreate(Option<group_model::group_create_with_members::ResponseData>),
    UploadFiles,
    ShowMoreFiles,
    OnFocus,
    OnBlur,
    HiddenModal,
    ChangeSidebarState,
}

impl Component for MySpaceView {
    type Message = MySpaceMessage;
    type Properties = MySpaceProperties;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(MySpaceMessage::FetchUserById);

        let saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };

        let school_profile = school_profile_data();

        let school_id = school_profile.clone().and_then(|data| Some(data.school_id));

        roboxmaker_utils::functions::myspace_state();


        MySpaceView {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            direct_message_task: None,
            meet_private_group_id: None,
            direct_message_private_group_id: None,
            more_files: false,
            udpload_files: false,
            search_node: NodeRef::default(),
            maybe_section_search: false,
            saved_sidebar_state,
            school_id,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MySpaceMessage::FetchUserById => {
                self.meet_private_group_id = Some(ctx.props().user_id)
                .zip(ctx.props().user_profile.as_ref())
                .and_then(|(user_id, item)| {

                    let mut uuids= vec![ &item.user_id.0, &user_id.0];
                    uuids.sort();

                    if user_id.0 == item.user_id.0 {
                        None
                    }
                    else {
                        Some(gen_private_group_id(config::MEET_MD5, uuids))
                    }
                });
                self.direct_message_private_group_id = Some(ctx.props().user_id)
                .zip(ctx.props().user_profile.as_ref())
                .and_then(|(user_id, item)| {

                    let mut uuids= vec![ &item.user_id.0, &user_id.0];
                    uuids.sort();

                    Some(gen_private_group_id(config::DM_MD5, uuids))
                });
                ctx.link().send_message(MySpaceMessage::User);
            }
            MySpaceMessage::User => {
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
                                MySpaceMessage::DirectMessageGroup(response)
                            }
                        );
                        self.direct_message_task = Some(task);
                    }
                }
            }
            MySpaceMessage::DirectMessageGroup(response) =>{
                if response.clone().and_then(|data| data.group_by_pk).is_none() {

                    let user_id = ctx.props().user_id.0;
                    let user_auth_id = ctx.props().user_profile.as_ref().and_then(|data| Some(data.user_id)).unwrap().0;
                    let group_id = self.direct_message_private_group_id.unwrap().0;
                    let school_id = self.school_id.unwrap().0;

                    let mut members: Vec<group_model::group_create_with_members::CreateGroupWithMembersGroupMemberInsertInput> = Vec::new();

                    members.push(group_model::group_create_with_members::CreateGroupWithMembersGroupMemberInsertInput{group_id: Some(group_id), user_id: Some(user_auth_id)});
                    
                    if user_id != user_auth_id {
                        members.push(group_model::group_create_with_members::CreateGroupWithMembersGroupMemberInsertInput{group_id: Some(group_id), user_id: Some(user_id)});
                    };

                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = group_model::group_create_with_members::Variables {
                            group_id,
                            school_id,
                            members
                        };
                        let task = group_model::GroupCreateWithMembers::request(
                            graphql_task, 
                            &ctx, 
                            vars, 
                            |response| {
                                MySpaceMessage::MessageGroupCreate(response)
                            }
                        );
                        self.direct_message_task = Some(task);
                    }
                } 
            }
            MySpaceMessage::MessageGroupCreate(_) => {
                // info!("{:?}", response)
            }
            MySpaceMessage::ShowMoreFiles => {
                self.more_files = !self.more_files
            }
            MySpaceMessage::UploadFiles => {
                self.udpload_files = !self.udpload_files
            }
            MySpaceMessage::OnFocus => {
                self.maybe_section_search = true;
            }
            MySpaceMessage::OnBlur => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
                // self.users = vec![];
            }
            MySpaceMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search
            }
            MySpaceMessage::ChangeSidebarState => {
                if let Some(element) = gloo_utils::document().get_element_by_id("show-sidebar-right") {
                    if self.saved_sidebar_state {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", false);
                        self.saved_sidebar_state = false;
                        let _ = element.set_attribute("class", "fa-angle-double-left fa-w-14 fa-2x");
                    } else {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", true);
                        self.saved_sidebar_state = true;
                        let _ = element.set_attribute("class", "fa fa-angle-double-right fa-w-14 fa-2x");
                    }
                }
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let on_focus = ctx.link().callback(move |_| MySpaceMessage::OnFocus);
        // let on_blur = ctx.link().callback(move |_| MySpaceMessage::OnBlur);
        // let on_hidden_modal = ctx.link().callback(move |_| MySpaceMessage::HiddenModal);
        // let _on_more_files = ctx.link().callback(|_| MySpaceMessage::ShowMoreFiles);
        // let _on_show_upload_files = ctx.link().callback(|_| MySpaceMessage::UploadFiles);
        // let on_search = ctx.link().callback(|search: InputData| {
        //     info!("search: {:?}", search);
        //     let search = if search.value.len() > 0 {
        //         format!("%{}%", search.value)
        //     } else {
        //         search.value
        //     };
        //     MySpaceMessage::FetchUsersByUserName(search)
        // });
        let on_show_sidebar = ctx.link().callback(move |_| MySpaceMessage::ChangeSidebarState);
        let btn_sidebar_show = if self.saved_sidebar_state {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-right fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        } else {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-left fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        };
        let _class_upload = if self.udpload_files {
            "modal is-active"
        } else {
            "modal"
        };

        // let _maybe_modal = html! {
        //     <div class=class_upload>
        //         <div class="modal-background"></div>
        //         <div class="modal-card">
        //             <p class="modal-card-title text-center">{"Sube tus Archivos"}</p>
        //             <button class="delete" aria-label="close" onclick=&on_show_upload_files></button>
        //         </div>
        //     </div>
        // };

        let maybe_messages = if self.direct_message_private_group_id.is_some(){
            html! {
                <MessageList user_profile={ctx.props().user_profile.clone()} 
                    user_id={Some(ctx.props().user_id)}
                    group_category={MessageGroupCategory::DirectMessages(self.direct_message_private_group_id.unwrap())} />
            }
        } else {
            html!{}
        };

        let maybe_user_robots = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_teacher.is_some() || item.user_staff.is_some() || (ctx.props().user_id.0 == item.user_id.0){
                    let robots_list = {
                            html! {
                                <UserRobots user_id={ctx.props().user_id.clone()} 
                                    user_profile={ctx.props().user_profile.clone()}
                                    maybe_style={UserStyle::MySpace} />
                            }
                        };
                    Some(
                        html!{
                        <>
                            {robots_list}
                        </>
                        }
                    )
                } else {
                    Some(html!{})
                }
            })
            .unwrap_or_default();

        let maybe_user_profile_pic = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });

        let maybe_user_profile_name = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item| {
                Some(html! {
                    {&item.full_name}
                })
            })
            .unwrap_or(html! {});
        // let private_group_id =  self.direct_message_private_group_id.clone().and_then(|data| Some(data.0)).unwrap_or(Uuid::default()).to_string();
        // let user_id_str = ctx.props().user_id.to_string();
        let welcome_user = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between">
                <div class="d-flex flex-column">
                    <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">{lang::dict("Hello, ")}
                        {maybe_user_profile_name}
                    </h1>
                    // <h1 class="is-size-18 text-gray">
                    //     {private_group_id}
                    // </h1>
                    // <h1 class="is-size-18 text-gray">
                    //     {user_id_str}
                    // </h1>
                    <span class="text-gray-purple-two noir-bold is-size-18 lh-22  pt-2">{"¡Puedes hacer lo que te apetezca en tu espacio!"}</span>
                </div>
                {btn_sidebar_show}
            </div>
        };
        let _your_files = html! {
            <>
                <div class="d-flex align-items-center justify-content-between pt-6 pb-5">
                    <span class="text-primary-blue-dark noir-bold is-size-20 lh-24">{lang::dict("Your Files")}</span>
                    // <a class="button-files" onclick=&on_show_upload_files>
                    <a class="bg-primary-blue-dark text-white noir-bold is-size-16 lh-20 text-center button-files">
                        <span>{lang::dict("Upload File...")}</span>
                    </a>
                </div>
            </>
        };
        let _files_class = if !self.more_files {
            "section-messages-files-user"
        } else {
            ""
        };
        let class_search_modal = if self.maybe_section_search {
            "modal is-active p-5"
        } else {
            "modal"
        };
        let class_right_sidebar = if self.saved_sidebar_state {
            "bg-silver col col-sm-3 col-md-3 col-lg-5 col-xl-4 col-xxl-3 d-none d-sm-none d-md-none d-lg-block p-5"
        } else {
            "d-none"
        };
        let class_sidebar_mobile = if self.saved_sidebar_state {
            "offcanvas offcanvas-end show bg-silver d-block d-sm-block d-md-block d-lg-none d-xl-none d-xxl-none"
        } else {
            "offcanvas offcanvas-end"
        };
        let style_sidebar_mobile = if self.saved_sidebar_state {
            "visibility: visible;"
        } else {
            "display: none;"
        };
        // let on_list_change = ctx.link().callback(move |_| MySpaceMessage::FetchUserById);
        html! {
            <>
                <div class="w-100 h-100 d-flex flex-row justify-content-between scroll-y scroll-x-hidden">
                    <div class="d-flex flex-column w-100 p-3 p-md-5 p-lg-7">
                        {welcome_user}
                        // {your_files}
                        // <div class=files_class>
                        //     <UserFiles on_app_route=ctx.props().on_app_route.clone() auth_user=ctx.props().auth_user.clone()
                        //         auth_school=ctx.props().auth_school.clone() group_id=GroupId(group_id)
                        //         on_list_change=on_list_change school_id=ctx.props().school_id />
                        // </div>
                        // <div class="text-center">
                        //     <a onclick=on_more_files class="btn btn-white text-cyan-sky is-size-18 noir-bold lh-22">{lang::dict("See All Files")}</a>
                        // </div>
                        <br/>
                        <br/>
                        <br/>
                        <h1 class="text-primary-blue-dark noir-bold is-size-20 lh-24 mb-4">{lang::dict("Robots")}</h1>
                        <div>{maybe_user_robots}</div>
                        <h1 class="text-primary-blue-dark noir-bold is-size-20 lh-24 mb-5">{lang::dict("Discussions and Comments")}</h1>
                        <div class="pb-5">
                            <MessagesByUserId user_id={ctx.props().user_id.clone()}
                                user_profile={ctx.props().user_profile.clone()} />
                        </div>
                        // {maybe_modal}
                    </div>
                    <div class={class_search_modal}>
                        {"hello"}
                    </div>
                </div>
                <div class={class_right_sidebar}>
                    <div class="d-flex align-items-center justify-content-between mb-5">
                        <a class="button-search-univeral mt-3 me-5">
                            <span class="icon-text-search-universal">
                                <span>{lang::dict("Search")}</span>
                                <span class="icon">
                                    <i class="fas fa-search"></i>
                                </span>
                            </span>
                        </a>
                        {maybe_user_profile_pic.clone()}
                    </div>
                    <div class="my-4">
                        <span class="text-primary-blue-dark noir-bold is-size-24 lh-30">{lang::dict("My Notes")}</span>
                    </div>
                    <div class="section-right-post scroll-messages-y mh-80">{maybe_messages.clone()}</div>
                </div>
                <div class={class_sidebar_mobile} data-bs-scroll="true" data-bs-backdrop="false" tabindex="-1" id="offcanvasScrolling" aria-labelledby="offcanvasScrollingLabel" aria-modal="true" role="dialog" style={style_sidebar_mobile}>
                    <div class="offcanvas-header d-flex justify-content-end">
                        <button type="button" class="btn btn-outline-danger" data-bs-dismiss="offcanvas" onclick={&on_show_sidebar}>
                            <i class="fas fa-times"></i>
                        </button>
                    </div>
                    <div class="offcanvas-body pt-0">
                        <div class="d-flex align-items-center justify-content-between mb-5">
                            <a class="button-search-univeral mt-3 me-5">
                                <span class="icon-text-search-universal">
                                    <span>{lang::dict("Search")}</span>
                                    <span class="icon">
                                        <i class="fas fa-search"></i>
                                    </span>
                                </span>
                            </a>
                            {maybe_user_profile_pic.clone()}
                        </div>
                        <div class="my-4">
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-30">{lang::dict("My Notes")}</span>
                        </div>
                        <div class="section-right-post scroll-messages-y mh-80">{maybe_messages.clone()}</div>
                    </div>
            </div>
            </>
        }
    }
}