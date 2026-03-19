use log::*;
use uuid::Uuid;
use yew::prelude::*;
use gloo_storage::Storage;
use code_location::code_location;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;
use serde_derive::{Deserialize, Serialize};
use crate::get_class_profile_handler::{*, self};

use roboxmaker_main::lang;
use roboxmaker_user::user_robots::UserRobots;
use roboxmaker_user::my_profile::MyProfilePage;
use roboxmaker_user::last_robots_card::UserStyle;
use roboxmaker_searches::search_home::SearchView;
use roboxmaker_utils::functions::user_profile_data;
use roboxmaker_post::post_list_home::PostListHome;
use roboxmaker_robot::robot_list_home::RobotListHome;
use roboxmaker_models::{grade_model::{self, home_data_by_group_id, get_class_group_by_group_id}, meetings_model};
use roboxmaker_user::members_list_home::MembersListHome;
use roboxmaker_lesson::lesson_list_home::LessonListHome;
use roboxmaker_classes::classes_list_home::ClassesListHome;
use roboxmaker_loaders::fullscreen_loader::FullScreenLoader;
use roboxmaker_meetings::meetings_list_home::MeetingsListHome;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask, SubscriptionTask, Subscribe};
use roboxmaker_types::types::{GroupId, AppRoute, SchoolId, UserId, ClassGroupCategory, LoadFullScreen, LoadFullScreenFound, MyUserProfile};

#[derive(Debug, Clone, PartialEq)]
pub struct GroupData {
    pub class_name: String,
    pub group_id: GroupId,
    pub class_id: Uuid,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSchool {
    pub name: String,
    pub inventory_group: Uuid,
    pub school_id: SchoolId,
}

pub struct HomeStaff {
    graphql_task: Option<GraphQLTask>,
    list_schools_task: Option<RequestTask>,
    class_group_sub: Option<SubscriptionTask>,
    school_selected: Option<SchoolId>,
    group_id_selected: Option<GroupId>,
    data_school: Vec<DataSchool>,
    class_groups: Vec<GroupData>,
    show_dropdown_school: bool,
    show_dropdown_degree: bool,
    user_section_on: bool,
    user_selected: Option<UserId>,
    loading_screen: LoadFullScreen,
    saved_sidebar_state: bool,
    user_profile: Option<MyUserProfile>,
    class_profile: Option<ClassProfileData>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct HomeStaffProps {
    #[prop_or(None)]
    pub on_user_profile: Option<Callback<UserId>>,
}

#[derive(Debug)]
pub enum HomeStaffMessage {
    // AppRoute(AppRoute),
    FetchSchoolList,
    SchoolList(Option<meetings_model::list_schools_meets::ResponseData>),
    SchoolChangeData(SchoolId),
    FetchClassGroups,
    ClassGroups(Option<grade_model::groups_by_school_id_list_class::ResponseData>),
    GroupChangeData(GroupId),
    ShowDropdownSchool,
    ShowDropdownDegree,
    ChangeSidebarState,
    ShowUserHiddenSection,
    ShowUser(UserId),
    OnShowModalUser(bool),
    // FetchClassGroupData(GroupId),
    FetchClassGroupData(GroupId),
    ResponseClassGroupData(Option<get_class_group_by_group_id::ResponseData>),
}


impl Component for HomeStaff {
    type Message = HomeStaffMessage;
    type Properties = HomeStaffProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(HomeStaffMessage::FetchSchoolList);
        
        let saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };
        let user_profile = user_profile_data();

        roboxmaker_utils::functions::home_state();

        HomeStaff { 
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            list_schools_task: None,
            school_selected: None,
            group_id_selected: None,
            data_school: vec![],
            class_groups: vec![],
            show_dropdown_school: false,
            show_dropdown_degree: false,
            user_section_on: false,
            user_selected: None,
            loading_screen: LoadFullScreen::Loading,
            saved_sidebar_state,
            user_profile,
            class_group_sub: None,
            class_profile: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("HOME-STAFF {:?}", msg);
        let should_update = true;
        match msg {
            // HomeStaffMessage::AppRoute(route) => {
            //     ctx.props().on_app_route.emit(route)
            // }
            HomeStaffMessage::FetchSchoolList => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = meetings_model::list_schools_meets::Variables {};

                    let task = meetings_model::ListSchoolsMeets::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            HomeStaffMessage::SchoolList(response)
                        },
                    );
                    self.list_schools_task = Some(task);
                }
            }
            HomeStaffMessage::SchoolList(response) => {
                self.data_school = response
                    .clone()
                    .and_then(|data| Some(data.inventory_group))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|data_schools| {
                        let school_group = data_schools.school_group.clone();
                        let school = school_group.clone().and_then(|data| Some(data.school));
                        let school_profile = school.clone().and_then(|data| data.school_profile);
                        let name = school_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        let inventory_group = school_group.clone().and_then(|data| Some(data.group_id)).unwrap_or(Uuid::default());
                        let school_id = school_group.clone().and_then(|data| Some(data.school_id)).unwrap_or(Uuid::default());
                        DataSchool {
                            name,
                            inventory_group,
                            school_id: SchoolId(school_id),
                        }
                    }).collect();

                self.school_selected = match self.data_school.first() {
                    Some(school) => Some(school.school_id),
                    None => None,
                };
                
                if self.school_selected.is_some() {
                    ctx.link().send_message(HomeStaffMessage::FetchClassGroups);
                }
                
                if !response.clone().and_then(|data| Some(data.inventory_group)).unwrap_or(vec![]).is_empty() {
                    self.loading_screen = LoadFullScreen::Load(LoadFullScreenFound::Found);
                } else {
                    self.loading_screen = LoadFullScreen::Load(LoadFullScreenFound::NotFound);
                }
            }
            HomeStaffMessage::FetchClassGroups => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    if let Some(school_id) = self.school_selected { 
                        let vars = grade_model::groups_by_school_id_list_class::Variables {
                            school_id: school_id.0,
                        };
    
                        let task = grade_model::GroupsBySchoolIdListClass::request(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                HomeStaffMessage::ClassGroups(response)
                            },
                        );

                        self.list_schools_task = Some(task);
                        // info!("SELECTED - SCHOOL: {:?}", school_id);
                    }
                }
            },
            HomeStaffMessage::ClassGroups(response) => {
                info!("CLASS_GROUP RESP: {:?}", response.clone());

                self.class_groups = response
                    .clone()
                    .and_then(|data| Some(data.class_group))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|class_group| {
                        let class_name = class_group.class_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        let group_id = class_group.group_id;
                        GroupData {
                            class_name,
                            group_id: GroupId(group_id),
                            class_id: class_group.class_profile.clone().and_then(|item| Some(item.class_id)).unwrap_or(Uuid::default()),
                        }
                    })
                    .collect();

                self.group_id_selected = match self.class_groups.first(){
                    Some(group) => Some(group.group_id),
                    None => None
                };

                if self.group_id_selected.is_some() {
                    let group_id = self.group_id_selected.unwrap();
                    ctx.link().send_message(HomeStaffMessage::FetchClassGroupData(group_id));
                };
            }
            HomeStaffMessage::SchoolChangeData(school_id) => {
                self.school_selected = Some(school_id);
                self.show_dropdown_school = false;
                self.show_dropdown_degree = false;
                ctx.link().send_message(HomeStaffMessage::FetchClassGroups);
            }
            HomeStaffMessage::GroupChangeData(group_id) => {
                self.group_id_selected = Some(group_id);
                self.show_dropdown_degree = false;

                if self.group_id_selected.is_some() {
                    ctx.link().send_message(HomeStaffMessage::FetchClassGroupData(group_id));
                }
                info!("SELECTED: {:?}", group_id);

            }
            HomeStaffMessage::ShowDropdownSchool => {
                self.show_dropdown_school = !self.show_dropdown_school;
            }
            HomeStaffMessage::ShowDropdownDegree => {
                self.show_dropdown_degree = !self.show_dropdown_degree;
            }
            HomeStaffMessage::ChangeSidebarState => {
                if let Some(element) = gloo_utils::document().get_element_by_id("show-sidebar-right") {
                    if self.saved_sidebar_state {
                        gloo_storage::LocalStorage::set("saved_sidebar_state", false).ok();
                        self.saved_sidebar_state = false;
                        let _ = element.set_attribute("class", "fa-angle-double-left fa-w-14 fa-2x");
                    } else {
                        gloo_storage::LocalStorage::set("saved_sidebar_state", true).ok();
                        self.saved_sidebar_state = true;
                        let _ = element.set_attribute("class", "fa fa-angle-double-right fa-w-14 fa-2x");
                    }
                }
            }
            HomeStaffMessage::ShowUserHiddenSection => {
                self.user_section_on = !self.user_section_on;
            }
            HomeStaffMessage::ShowUser(user_id) => {
                self.user_selected = Some(user_id);
                if let Some(on_user_profile) = &ctx.props().on_user_profile {
                    on_user_profile.emit(user_id)
                }
            }
            HomeStaffMessage::OnShowModalUser(show) => {
                if !show {
                    self.user_selected = None;
                }
                self.user_section_on = show;
            }
            HomeStaffMessage::FetchClassGroupData(group_id) => {
                let scheduled_meetings = chrono::Local::now().date_naive();

                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    // if let Some(group_id) = self.group_id_selected {
                        let vars = grade_model::get_class_group_by_group_id::Variables {
                            group_id: group_id.0,
                            timestamp: get_class_group_by_group_id::OrderBy::Desc,
                            schedule_time: scheduled_meetings
                        };
    
                        let task = grade_model::GetClassGroupByGroupId::subscribe(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                HomeStaffMessage::ResponseClassGroupData(response)
                            },
                        );
    
                        self.class_group_sub = Some(task);
                    // }
                }
            }
            HomeStaffMessage::ResponseClassGroupData(response) => {
                if let Some(class_data) = response.clone().and_then(|class_profile| Some(class_profile.class_profile)) {
                    for class_profile in class_data.into_iter() {
                        self.class_profile = Some(ClassProfileData::get_class_profile(class_profile));
                    }
                }

                info!("GetClassProfileFunction {:?}", self.class_profile);
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        trace!("{:?} => {:?}", ctx.props(), old_props);

        self.user_profile = user_profile_data();

        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {


        // HIDDEN RIGHT SIDEBAR
        let on_show_sidebar = ctx.link().callback(move |_| HomeStaffMessage::ChangeSidebarState);
        
        let btn_sidebar_show = if self.saved_sidebar_state {
            html! {
                <>
                    <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                        <i class="fas fa-angle-double-right fas fa-2x" id="show-sidebar-right"></i>
                    </button>
                </>
            }
        } else {
            html! {
                <>
                    <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                        <i class="fas fa-angle-double-left fas fa-2x" id="show-sidebar-right"></i>
                    </button>
                </>
            }
        };
        // END HIDDEN RIGHT SIDEBAR

        // DROPDOWN SCHOOLS

        let all_schools = self.data_school.iter().map(|school_group| {
            let school_id = school_group.school_id;
            let school_id_select = format!("{:?}", school_group.school_id);
            let on_show_list_degrees = ctx.link().callback(move |_| HomeStaffMessage::SchoolChangeData(school_id));
            let school_selected = if self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default() == school_group.school_id.0 {
                true
            } else {
                false
            };
            let class_selected = if self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default() == school_group.school_id.0 {
                "dropdown-item bg-silver text-blue-purple noir-regular is-size-14 lh-20 d-flex align-items-center text-break-spaces"
            } else {
                "dropdown-item text-gray-purple noir-regular is-size-14 lh-20 d-flex align-items-center text-break-spaces"
            };
            html! {
                <li>
                    <a class={class_selected} onclick={on_show_list_degrees}>
                        <input class="bg-checkbox me-1 d-flex align-items-center" type="checkbox" value={school_id_select} checked={school_selected} />
                        {&school_group.name}
                    </a>
                </li>
            }
        })
        .collect::<Html>();
        let change_school = self.data_school.iter().map(|school_group| {
            let school_selected = if self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default() == school_group.school_id.0 {
                true
            } else {
                false
            };
            let maybe_school = if school_selected {
                html! {
                    <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22-2 text-secondary-purple noir-regular is-size-18 lh-22">{&school_group.name}</span>
                }
            } else {
                html! {}
            };
            html! {
                {maybe_school}
            }
        })
        .collect::<Html>();

        let class_dropdown_school = if self.show_dropdown_school {
            "btn btn-secondary btn-second-home dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-second-home dropdown-toggle d-flex align-items-center justify-content-between"
        };

        let class_dropdown_list_school = if self.show_dropdown_school {
            "dropdown-menu dropdown-menu-home show"
        } else {
            "dropdown-menu dropdown-menu-home"
        };

        let on_dropdown_school = ctx.link().callback(|_| HomeStaffMessage::ShowDropdownSchool);

        let dropdown_schools = html! {
            <div class="dropdown dropdown-h me-4">
                <button class={class_dropdown_school} type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick={on_dropdown_school}>
                    <img src="/icons/school-3.svg" style="height: 22px;" />
                    {change_school}
                </button>
                <ul class={class_dropdown_list_school} aria-labelledby="dropdownMenuButton2">
                    {all_schools}
                </ul>
            </div>
        };
        // END DROPDOWN SCHOOLS

        // DROPDOWN DEGREES

        let alls_class_groups = self.class_groups.iter().map(|class_group| {
            let group_id = class_group.group_id;
            let class_id_select = format!("{:?}", group_id);
            let on_show_list_degrees = ctx.link().callback(move |_| HomeStaffMessage::GroupChangeData(group_id));
            let class_group_selected = if self
                .group_id_selected
                .and_then(|id| Some(id.0))
                .unwrap_or_default()
                == class_group.group_id.0 {
                    true
                } else {
                    false
                };
            let class_selected = if self
                .group_id_selected
                .and_then(|id| Some(id.0))
                .unwrap_or_default()
                == class_group.group_id.0 {
                    "dropdown-item bg-silver text-blue-purple noir-regular is-size-14 lh-20 d-flex align-items-center"
                } else {
                    "dropdown-item text-gray-purple noir-regular is-size-14 lh-20 d-flex align-items-center"
                };
            html! {
                <li>
                    <a class={class_selected} onclick={on_show_list_degrees}>
                        <input class="bg-checkbox me-1 d-flex align-items-center" type="checkbox" value={class_id_select} checked={class_group_selected} />
                        {&class_group.class_name}
                    </a>
                </li>
            }
        })
        .collect::<Html>();

        let change_class_group = self.class_groups.iter().map(|class_group| {
            let class_group_selected = if self
                .group_id_selected
                .and_then(|id| Some(id.0))
                .unwrap_or_default()
                == class_group.group_id.0 {
                    true
                } else {
                    false
                };
            let maybe_class = if class_group_selected {
                html! {
                    <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22-2 text-secondary-purple noir-regular is-size-18 lh-22">{&class_group.class_name}</span>
                }
            } else {
                html! {}
            };
            html! {
                {maybe_class}
            }

        }).collect::<Html>(); 

        let class_dropdown = if self.show_dropdown_degree {
            "btn btn-secondary btn-second-home dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-second-home dropdown-toggle d-flex align-items-center justify-content-between"
        };

        let class_dropdown_list = if self.show_dropdown_degree {
            "dropdown-menu dropdown-menu-home show scroll-dropdown-home"
        } else {
            "dropdown-menu dropdown-menu-home"
        };

        let on_dropdown_degree = ctx.link().callback(|_| HomeStaffMessage::ShowDropdownDegree);
        let dropdown_degrees = html! {
            <div class="dropdown dropdown-h mt-3 mt-md-0">
                <button class={class_dropdown} type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick={on_dropdown_degree}>
                    <img src="/icons/graduation_1.svg" style="height: 18px;" />
                    {change_class_group}
                </button>
                <ul class={class_dropdown_list} aria-labelledby="dropdownMenuButton2">
                    {alls_class_groups}
                </ul>
            </div>
        };
        // END DROPDOWN DEGREES

        let welcome_class_view = self.user_profile.as_ref().and_then(|user_profile| {
            Some(html! {
                <div class="d-flex justify-content-between">
                    <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 pb-4 mb-1">{lang::dict("Hello, ")}
                        {&user_profile.full_name}
                    </h1>
                    {btn_sidebar_show}
                </div>
            })
        }).unwrap_or(html! {});

        let class_group_posts = |class_group: &GroupData| {
            let school_selected = self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default();
            if self.group_id_selected == Some(class_group.group_id) {
                if let Some(class_profile) = &self.class_profile {
                    html! {
                        <div class="scroll-x-home">
                            <PostListHome group_id={class_group.group_id}
                            user_profile={self.user_profile.clone()} 
                            school_id={SchoolId(school_selected)}
                            post_list={class_profile.post_profile.clone()} />
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            }
        };
        let class_group_classes = |class_group: &GroupData| {
            let school_selected = self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default();
            if self.group_id_selected == Some(class_group.group_id) {
                if let Some(class_profile) = &self.class_profile {
                    html! {
                        <div class="scroll-x-home">
                            <ClassesListHome group_id={class_group.group_id}
                                user_profile={self.user_profile.clone()}
                                school_id={SchoolId(school_selected)}
                                classes_list={class_profile.classes_profile.clone()} />
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            }
        };

        let class_group_lessons = |class_group: &GroupData| {
            let school_selected = self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default();
            if self.group_id_selected == Some(class_group.group_id) {
                if let Some(class_profile) = &self.class_profile {
                    html! {
                        <div class="scroll-x-home">
                            <LessonListHome group_id={class_group.group_id}
                                user_profile={self.user_profile.clone()}
                                school_id={SchoolId(school_selected)}
                                filter_lessons={false}
                                maybe_author={true}
                                lesson_list={class_profile.lesson_profile.clone()} />
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            }
        };

        let class_group_robots = |class_group: &GroupData| {
            if self.group_id_selected == Some(class_group.group_id) {
                if let Some(class_profile) = &self.class_profile {
                    html! {
                        <div class="scroll-x-home">
                            <RobotListHome
                                group_id={class_group.group_id}
                                user_profile={self.user_profile.clone()}
                                robot_list={class_profile.robot_profile.clone()} />
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            }
        };

        let class_group_meetings = |class_group: &GroupData| {
            if self.group_id_selected == Some(class_group.group_id) {
                if let Some(class_profile) = &self.class_profile {
                    html! {
                        <div class="d-flex flex-column">
                            <MeetingsListHome group_id={class_group.group_id}
                                meetings_list={class_profile.meets_profile.clone()} />
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            }
        };

        let class_group_members = |class_group: &GroupData| {
            if self.group_id_selected == Some(class_group.group_id) {
                if let Some(class_profile) = &self.class_profile {
                    html! {
                        <div class="d-flex flex-column">
                            <MembersListHome user_profile={self.user_profile.clone()}
                                group_id={class_group.group_id}
                                members_list={class_profile.members_profile.clone()} />
                        </div>
                    }
                } else {
                    html! {}
                }
            } else {
                html! {}
            }
        };
        let class_group_level = |class_group_data: &GroupData|{
            let group_id = class_group_data.group_id;
            let class_group_id = format!("class-group-{}", group_id);  
            let school_id = if let Some(school_id) = self.school_selected {
                school_id
            } else {
                SchoolId(Uuid::default())
            };
            
            let category = ClassGroupCategory::Posts;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_post = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));

            let category = ClassGroupCategory::Classes;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_classes = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));

            let category = ClassGroupCategory::Lessons;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_lessons = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
  
            let category = ClassGroupCategory::Robots;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_robot = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            
            let navigator = ctx.link().navigator().unwrap();
            let on_meetings = Callback::from(move |_| navigator.push(&AppRoute::Meetings));

            html! {
                <div id={ class_group_id.clone() }>
                    <div
                        class="d-flex justify-content-between align-items-center py-home-sections">
                        <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Posts")}</span>
                        <span class="d-flex align-items-center">
                            <a onclick={on_class_group_post}>
                                <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                            </a>
                            <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                <i class="fas fa-arrow-right"></i>
                            </span>
                        </span>
                    </div>
                    {class_group_posts(class_group_data)}
                    <div
                        class="d-flex justify-content-between align-items-center py-home-sections">
                        <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Classes")}</span>
                        <span class="icon-text d-flex align-items-center">
                            <a onclick={on_class_group_classes}>
                                <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                            </a>
                            <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                <i class="fas fa-arrow-right"></i>
                            </span>
                        </span>
                    </div>
                    {class_group_classes(class_group_data)}
                    <div
                        class="d-flex justify-content-between align-items-center py-home-sections">
                        <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Lessons")}</span>
                        <span class="icon-text d-flex align-items-center">
                            <a onclick={on_class_group_lessons}>
                                <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                            </a>
                            <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                <i class="fas fa-arrow-right"></i>
                            </span>
                        </span>
                    </div>
                    {class_group_lessons(class_group_data)}
                    <div
                        class="d-flex justify-content-between align-items-center py-home-sections">
                        <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Robots")}</span>
                        <span class="icon-text d-flex align-items-center">
                            <a onclick={on_class_group_robot}>
                                <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                            </a>
                            <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                <i class="fas fa-arrow-right"></i>
                            </span>
                        </span>
                    </div>
                    {class_group_robots(class_group_data)}
                    <div
                        class="d-flex justify-content-between align-items-center py-home-sections">
                        <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Meetings")}</span>
                        <a onclick={on_meetings}>
                            <span class="icon-text d-flex align-items-center">
                                <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                                <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                    <i class="fas fa-arrow-right"></i>
                                </span>
                            </span>
                        </a>
                    </div>
                    {class_group_meetings(class_group_data)}
                </div>
            }
        };

        let class_right_sidebar = if self.saved_sidebar_state {
            "bg-silver col col-sm-3 col-md-3 col-lg-5 col-xl-4 col-xxl-3 d-none d-sm-none d-md-none d-lg-block p-5"
        } else {
            "d-none"
        };

        let right_sidebar = |class_group_data: &GroupData|{
            let group_id = class_group_data.group_id;
            let school_id = if let Some(school_id) = self.school_selected {
                school_id
            } else {
                SchoolId(Uuid::default())
            };
            let on_user_section = ctx.link().callback(move |_| HomeStaffMessage::ShowUserHiddenSection);

            let maybe_user_profile_pic = self.user_profile.as_ref().and_then(|user| {
                    let pic_path = user.pic_path.clone();
                    let full_name = user.full_name.clone();
                    let name = {lang::dict("Picture of ")}.to_string() + &full_name;
                    let maybe_icon = if self.user_section_on {
                        html! {}
                    } else {
                        html! {
                            <span class="icon-my-profile">
                                <i class="far fa-edit"></i>
                            </span>
                        }
                    };
                    Some(html! {
                        <a onclick={&on_user_section}>
                            <div class="card" style="height: 72px; width: 72px; border-radius: 150px;">
                                <img src={pic_path} class="img-card-72" alt={name} />
                                <div class="card-img-overlay d-flex justify-content-end align-items-end p-0">
                                    {maybe_icon}
                                </div>
                            </div>
                        </a>
                    })
                }).unwrap_or(html! {});
            let close_modal_callback = ctx.link().callback(|_| HomeStaffMessage::OnShowModalUser(false));

            let maybe_members = self.user_profile.as_ref().and_then(|user| {
                if user.user_staff.is_some() || user.user_teacher.is_some() {
                    Some(html! {
                        <>
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 my-4">{lang::dict("Members")}</span>
                            <div class="card-members-class bg-white px-4 pt-4">
                                {class_group_members(class_group_data)}
                            </div>
                        </>
                    })
                } else {
                    None
                }
            }).unwrap_or(html! {});

            let user_id = self.user_profile.clone().and_then(|d| Some(d.user_id)).unwrap_or(UserId(Uuid::default()));
            let maybe_option = if self.user_section_on {
                html! {
                    <MyProfilePage user_id={user_id}
                        user_profile={self.user_profile.clone()}
                        on_user_profile={ctx.props().on_user_profile.clone()}
                        show_user={self.user_section_on}
                        close_modal_callback={close_modal_callback} />
                }
            } else {
                html! {
                    <>
                        <div>
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Latest Robots")}</span>
                            <UserRobots user_id={user_id} 
                                user_profile={self.user_profile.clone()}
                                maybe_style={UserStyle::ListHome} />
                        </div>
                        {maybe_members}
                    </>
                }
            };
            let maybe_sidebar = if self.saved_sidebar_state {
                html! {
                    <div class="d-flex flex-column justify-content-between w-100">
                        <div class="d-flex flex-wrap align-items-center justify-content-between mb-3">
                            <SearchView group_id={Some(group_id)}
                                user_profile={self.user_profile.clone()}
                                school_id={school_id.clone()} />
                            {maybe_user_profile_pic.clone()}
                        </div>
                        {maybe_option}
                    </div>
                }
            } else {
                html! {}
            };
            html! {
                {maybe_sidebar}
            }
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

        // let on_panel_add_user = ctx.link().callback(move |_| HomeStaffMessage::AppRoute(AppRoute::PanelAddUsers));

        let home_view_staff = match self.loading_screen {
            LoadFullScreen::Loading => {
                html! {
                    <FullScreenLoader />
                }
            },
            LoadFullScreen::Load(LoadFullScreenFound::Found) => {
                html! {
                    <>
                        <div class="w-100 h-100 d-flex flex-row justify-content-between scroll-y scroll-x-hidden">
                            <div class="w-100 pt-3 ps-3 pt-md-4 ps-md-4 pt-lg-7 ps-lg-7">
                                {welcome_class_view}
                                <div class="d-flex flex-wrap">
                                    {dropdown_schools}
                                    {dropdown_degrees}
                                </div>
                                // <button type="button" class="btn btn-outline-info btn-lg" onclick={&on_panel_add_user}>{"Panel Para Agregar Usuarios"}</button>
                                {   self.class_groups
                                    .iter()
                                    .filter(|data| data.group_id == self.group_id_selected.unwrap_or(GroupId(Uuid::default())))
                                    .map(|class_group|{
                                        class_group_level.clone()(class_group)
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                        <div class={class_right_sidebar}>
                            {   self.class_groups
                                .iter()
                                .filter(|data| data.group_id == self.group_id_selected.unwrap_or(GroupId(Uuid::default())))
                                .map(|class_group|{
                                    right_sidebar.clone()(class_group)
                                }).collect::<Html>()
                            }
                        </div>
                        <div class={class_sidebar_mobile} data-bs-scroll="true" data-bs-backdrop="false" tabindex="-1" id="offcanvasScrolling" aria-labelledby="offcanvasScrollingLabel" aria-modal="true" role="dialog" style={style_sidebar_mobile}>
                            <div class="offcanvas-header d-flex justify-content-end">
                                <button type="button" class="btn btn-outline-danger" data-bs-dismiss="offcanvas" onclick={&on_show_sidebar}>
                                    <i class="fas fa-times"></i>
                                </button>
                            </div>
                            <div class="offcanvas-body pt-0">
                                {   self.class_groups
                                    .iter()
                                    .filter(|data| data.group_id == self.group_id_selected.unwrap_or(GroupId(Uuid::default())))
                                    .map(|class_group|{
                                        right_sidebar.clone()(class_group)
                                    }).collect::<Html>()
                                }
                            </div>
                        </div>
                    </>
                }
            },
            LoadFullScreen::Load(LoadFullScreenFound::NotFound) => {
                html! {
                    <FullScreenLoader />
                }
            },
        };
        html! {
            {home_view_staff}
        }
    }
}