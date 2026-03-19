use crate::user_page::UserPage;
use crate::user_select::{UserSelect, UserSelectOption};
use crate::users_card::UserCard;
use code_location::code_location;
use gloo_storage::Storage;
use log::*;
use uuid::Uuid;
use yew::prelude::*;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_graphql::{
    GraphQLService, GraphQLTask, Request, RequestTask, Subscribe, SubscriptionTask,
};
use roboxmaker_main::lang;
use roboxmaker_models::{group_model, school_model, user_model};
use roboxmaker_types::types::{AppRoute, GroupId, MyUserProfile, UserId};

#[derive(Debug, Clone, PartialEq)]
pub struct UserProfile {
    pub full_name: String,
    pub user_id: UserId,
    pub pic_path: String,
    pub user_staff: bool,
    pub user_teacher: bool,
    pub user_student: bool,
    pub school_name: String,
    pub license: String,
    pub on_dropdown_menu: bool,
    pub view_profile: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserFilter {
    Alls,
    Teachers,
    Students,
    Staff,
}

pub struct UserList {
    link: ComponentLink<Self>,
    props: UserListProps,
    graphql_task: Option<GraphQLTask>,
    user_sub: Option<SubscriptionTask>,
    user_delete_task: Option<RequestTask>,
    user_add_task: Option<RequestTask>,
    user_list: Vec<UserProfile>,
    filter: UserFilter,
    show_dropdown_filter: bool,
    user_selected: Option<UserProfile>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct UserListProps {
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
    pub saved_sidebar_state: bool,
    pub class_name: String,
}

#[derive(Debug)]
pub enum UserListMessage {
    AppRoute(AppRoute),
    FetchUsersByGroupId,
    Users(Option<user_model::users_list_by_group::ResponseData>),
    ChangeFilter(UserFilter),
    ShowDropdownFilter,
    RemoveUser(UserId),
    UserRemoved(Option<UserId>),
    AddUser(UserId),
    UserAdded(Option<UserId>),
    ChangeSidebarState,
    UserSelected(Option<UserProfile>),
}

impl Component for UserList {
    type Message = UserListMessage;
    type Properties = UserListProps;

    fn create(mut props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(UserListMessage::FetchUsersByGroupId);
        props.saved_sidebar_state =
            if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
                value
            } else {
                true
            };
        UserList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            user_sub: None,
            user_delete_task: None,
            user_add_task: None,
            user_list: vec![],
            filter: UserFilter::Alls,
            show_dropdown_filter: false,
            user_selected: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            UserListMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            UserListMessage::FetchUsersByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let group_id = self.props.group_id;

                    let vars = user_model::users_list_by_group::Variables {
                        group_id: group_id.0,
                    };

                    let task = user_model::UsersListByGroup::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| UserListMessage::Users(response),
                    );
                    self.user_sub = Some(task);
                }
            }
            UserListMessage::Users(response) => {
                self.user_list = response
                    .clone()
                    .and_then(|data| Some(data.user_profile))
                    .unwrap_or_default()
                    .iter()
                    .filter(|users| {
                        let user_staff = users.user_staff.is_some();
                        let user_teacher = users.user_teacher.is_some();
                        let user_student = users.user_student.is_some();

                        self.filter == UserFilter::Alls && {
                            user_teacher == true || user_student == true || user_staff == true
                        } || self.filter == UserFilter::Students
                            && user_student == true
                            && user_staff == false
                            && user_teacher == false
                            || self.filter == UserFilter::Teachers
                                && user_student == false
                                && user_staff == false
                                && user_teacher == true
                            || self.filter == UserFilter::Staff
                                && user_student == false
                                && user_staff == true
                                && user_teacher == false
                    })
                    .map(|item| {
                        let school_name = item
                            .group_member
                            .clone()
                            .and_then(|data| data.school_group)
                            .and_then(|school_group| Some(school_group.school))
                            .and_then(|school| school.school_profile)
                            .and_then(|school_profile| Some(school_profile.name))
                            .unwrap_or("Sin Colegio".to_string());

                        let license = item
                            .user_student
                            .clone()
                            .and_then(|data| data.license)
                            .and_then(|license| Some(license.license))
                            .unwrap_or("AAAAAAAAAAAAAAA".to_string());

                        // info!("VIEWLICENSE {:?}", license);
                        UserProfile {
                            full_name: item.full_name.clone(),
                            user_id: UserId(item.user_id),
                            pic_path: item.pic_path.clone().unwrap_or(
                                "https://files.roboxmaker.network/uploads/avatar.png".to_string(),
                            ),
                            user_staff: item.user_staff.is_some(),
                            user_teacher: item.user_teacher.is_some(),
                            user_student: item.user_student.is_some(),
                            school_name,
                            license,
                            on_dropdown_menu: false,
                            view_profile: false,
                        }
                    })
                    .collect();
                if let Some(user_selected) = self.user_selected.clone() {
                    for user in self.user_list.clone() {
                        if user.user_id == user_selected.user_id {
                            self.user_selected = Some(user)
                        }
                    }
                }
            }
            UserListMessage::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;
                // self.list_response_state = LoadResponse::Loading;
                self.link.send_message(UserListMessage::FetchUsersByGroupId);
            }
            UserListMessage::ShowDropdownFilter => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            UserListMessage::RemoveUser(user_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = group_model::group_member_delete::Variables {
                        group_id: self.props.group_id.0,
                        user_id: user_id.0,
                    };

                    let task = group_model::GroupMemberDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let user_id = if let Some(response) = response {
                                if response
                                    .clone()
                                    .delete_group_member
                                    .clone()
                                    .and_then(|data| Some(data.returning))
                                    .unwrap_or(vec![])
                                    .len()
                                    > 0
                                {
                                    Some(UserId(
                                        response
                                            .clone()
                                            .delete_group_member
                                            .clone()
                                            .and_then(|data| Some(data.returning[0].user_id))
                                            .unwrap_or(Uuid::default()),
                                    ))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            UserListMessage::UserRemoved(user_id)
                        },
                    );
                    self.user_delete_task = Some(task);
                }
            }
            UserListMessage::UserRemoved(user_id) => {
                if let Some(user_id) = user_id {
                    self.user_list.retain(|u| u.user_id != user_id);
                } else {
                    should_update = false;
                }
            }
            UserListMessage::AddUser(user_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = group_model::group_member_add::Variables {
                        group_id: self.props.group_id.0,
                        user_id: user_id.0,
                    };

                    let task = group_model::GroupMemberAdd::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let user_id = if let Some(user) = response {
                                user.insert_group_member_one
                                    .clone()
                                    .and_then(|data| Some(UserId(data.user_id)))
                            } else {
                                None
                            };
                            UserListMessage::UserAdded(user_id)
                        },
                    );
                    self.user_add_task = Some(task);
                }
            }
            UserListMessage::UserAdded(user_id) => {
                if let Some(user_id) = user_id {
                    self.user_list.push(UserProfile {
                        full_name: String::from(""),
                        user_id: user_id,
                        pic_path: String::from(""),
                        user_staff: true,
                        user_teacher: true,
                        user_student: true,
                        on_dropdown_menu: true,
                        view_profile: true,
                        school_name: String::from(""),
                        license: String::from(""),
                    });
                } else {
                    should_update = true;
                }
            }
            UserListMessage::ChangeSidebarState => {
                if let Some(element) =
                    gloo_utils::document().get_element_by_id("show-sidebar-right")
                {
                    if self.props.saved_sidebar_state {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", false);
                        self.props.saved_sidebar_state = false;
                        let _ =
                            element.set_attribute("class", "fa-angle-double-left fa-w-14 fa-2x");
                    } else {
                        let _ = gloo_storage::LocalStorage::set("saved_sidebar_state", true);
                        self.props.saved_sidebar_state = true;
                        let _ = element
                            .set_attribute("class", "fa fa-angle-double-right fa-w-14 fa-2x");
                    }
                }
            }
            UserListMessage::UserSelected(user_profile) => {
                self.user_selected = user_profile;
                self.props.saved_sidebar_state = true;
            }
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
        let group_id = self.props.group_id;
        let on_alls = self
            .link
            .callback(|_| UserListMessage::ChangeFilter(UserFilter::Alls));
        let on_students = self
            .link
            .callback(|_| UserListMessage::ChangeFilter(UserFilter::Students));
        let on_teachers = self
            .link
            .callback(|_| UserListMessage::ChangeFilter(UserFilter::Teachers));
        let on_staff = self
            .link
            .callback(|_| UserListMessage::ChangeFilter(UserFilter::Staff));
        let on_dropdown = self.link.callback(|_| UserListMessage::ShowDropdownFilter);

        let maybe_option_seleted = match self.filter {
            UserFilter::Alls => "Everyone",
            UserFilter::Teachers => "Teachers",
            UserFilter::Students => "Students",
            UserFilter::Staff => "Staff",
        };
        let class_dropdown = if self.show_dropdown_filter {
            "btn btn-secondary btn-see-degree dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-see-degree dropdown-toggle d-flex align-items-center justify-content-between"
        };
        let class_dropdown_list = if self.show_dropdown_filter {
            "dropdown-menu dropdown-menu-degree show"
        } else {
            "dropdown-menu dropdown-menu-degree"
        };

        let maybe_dropdown_by_user = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown">
                            <button class=class_dropdown type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick=on_dropdown>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(maybe_option_seleted)}</span>
                            </button>
                            <ul class=class_dropdown_list aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick=on_alls>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == UserFilter::Alls {true} else {false}} />
                                        <span class={if self.filter == UserFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-4"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-4"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_teachers>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == UserFilter::Teachers {true} else {false}} />
                                        <span class={if self.filter == UserFilter::Teachers {"text-blue-purple noir-regular is-size-18 lh-22 ps-4"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-4"}}>{lang::dict("Teachers")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_students>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == UserFilter::Students {true} else {false}}/>
                                        <span class={if self.filter == UserFilter::Students {"text-blue-purple noir-regular is-size-18 lh-22 ps-4"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-4"}}>{lang::dict("Students")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_staff>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == UserFilter::Staff {true} else {false}} />
                                        <span class={if self.filter == UserFilter::Staff {"text-blue-purple noir-regular is-size-18 lh-22 ps-4"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-4"}}>{lang::dict("Staff")}</span>
                                    </a>
                                </li>
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let members = self
            .user_list
            .iter()
            .filter(|users| {
                self.filter == UserFilter::Alls && {
                    users.user_teacher == true
                        || users.user_student == true
                        || users.user_staff == true
                } || self.filter == UserFilter::Students
                    && users.user_student == true
                    && users.user_staff == false
                    && users.user_teacher == false
                    || self.filter == UserFilter::Teachers
                        && users.user_student == false
                        && users.user_staff == false
                        && users.user_teacher == true
                    || self.filter == UserFilter::Staff
                        && users.user_student == false
                        && users.user_staff == true
                        && users.user_teacher == false
            })
            .map(|item| {
                let user_profile = item.clone();
                let user_id = item.user_id;
                let full_name = item.full_name.clone();
                let pic_path = item.pic_path.clone();
                let user_staff = item.user_staff;
                let user_teacher = item.user_teacher;
                let user_student = item.user_student;
                let on_dropdown_menu = item.on_dropdown_menu;

                let on_user_delete = self
                    .link
                    .callback(|user_id| UserListMessage::RemoveUser(user_id));
                let on_select = self
                    .link
                    .callback(move |_| UserListMessage::UserSelected(Some(user_profile.clone())));

                html! {

                    <UserCard user_id=user_id
                        on_app_route=self.props.on_app_route.clone()
                        on_user_delete=Some(on_user_delete)
                        saved_sidebar_state=self.props.saved_sidebar_state
                        full_name=full_name
                        pic_path=pic_path
                        user_staff=user_staff
                        user_teacher=user_teacher
                        user_student=user_student
                        view_profile=item.view_profile
                        on_dropdown_menu=on_dropdown_menu
                        onclick=on_select />

                }
            })
            .collect::<Html>();
        let maybe_dropdown = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                    {lang::dict("Members")} <span class="ps-1">{"("}{self.user_list.iter().cloned().len()}{")"}</span>
                </span>
                {maybe_dropdown_by_user}
            </div>
        };
        let on_app_route = self
            .link
            .callback(move |route: AppRoute| UserListMessage::AppRoute(route));

        let maybe_user_search = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = self.link.callback(|select_option| match select_option {
                    UserSelectOption::User(user_id) => UserListMessage::AddUser(user_id),
                });
                if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <UserSelect on_select=on_select
                            on_app_route=on_app_route.clone()
                            group_id=self.props.group_id />
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {});

        let on_show_sidebar = self
            .link
            .callback(move |_| UserListMessage::ChangeSidebarState);
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

        let on_direct_meet = self
            .link
            .callback(move |_| UserListMessage::AppRoute(AppRoute::MeetDirect(group_id)));

        let head_section = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between mb-5">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {self.props.class_name.clone()}
                </h1>
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick=on_direct_meet>
                    <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
                {btn_sidebar_show}
            </div>
        };
        let class_right_sidebar = if self.props.saved_sidebar_state {
            // "bg-silver col col-sm-3 col-md-3 col-lg-5 col-xl-4 col-xxl-3 d-none d-sm-none d-md-none d-lg-block p-5"
            "bg-silver col col-sm-8 col-md-5 col-lg-5 col-xl-4 col-xxl-3 p-5"
        } else {
            "d-none"
        };
        let maybe_user_profile_pic = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .and_then(|pic_path| {
                Some(html! {
                    <img class="img-card-72" src=pic_path.clone() alt="photo of user" />
                })
            })
            .unwrap_or(html! {<img class="img-card-72" src="/static/avatar.png"/>
            });
        let close_modal_callback = self.link.callback(|_| UserListMessage::UserSelected(None));
        let user_selected = self.user_selected.clone().and_then(|user| {
            Some(html! {
                <UserPage user_id=user.user_id
                    user_profile=self.props.user_profile.clone()
                    on_app_route=self.props.on_app_route.clone()
                    full_name=user.full_name.clone()
                    pic_path=user.pic_path.clone()
                    close_modal_callback=close_modal_callback.clone()
                    staff={user.user_staff}
                    teacher={user.user_teacher}
                    school_name={user.school_name}
                    license={user.license} />
            })
        }).unwrap_or(html! {
            <div class="d-flex flex-column align-items-center mt-335">
                <span class="text-purple-gray">
                    <i class="far fa-question-circle fas fa-2x"></i>
                </span>
                <p class="text-purple-gray noir-regular is-size-18 lh-22 text-center pt-5">{lang::dict("Select a person in the list to see their general information")}</p>
            </div>
        });

        let maybe_option = if !self.user_list.is_empty() {
            html! {
                <div class="d-flex flex-wrap">
                    {members}
                </div>
            }
        } else {
            html! {
                <div class="text-center">
                    <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No members here.")}</span>
                </div>
            }
        };

        html! {
            <div class="d-flex flex-row w-100 h-100">
                <div class="scroll-y w-100 h-100 pt-3 ps-3 pt-md-4 ps-md-4 pt-lg-7 ps-lg-7">
                    {head_section}
                    {maybe_dropdown}
                    {maybe_option}
                </div>
                <div class=class_right_sidebar>
                    <div class="d-flex flex-row align-items-center justify-content-between">
                        {maybe_user_search}
                        {maybe_user_profile_pic}
                    </div>
                    {user_selected}
                </div>
            </div>
        }
    }
}
