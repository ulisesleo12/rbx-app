use log::*;
use uuid::Uuid;
use yew::prelude::*;
use gloo_storage::Storage;
use code_location::code_location;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_models::group_model;
use roboxmaker_user::user_robots::UserRobots;
use roboxmaker_user::my_profile::MyProfilePage;
use roboxmaker_user::last_robots_card::UserStyle;
use roboxmaker_searches::search_home::SearchView;
use roboxmaker_post::post_list_home::PostListHome;
use roboxmaker_robot::robot_list_home::RobotListHome;
use roboxmaker_user::members_list_home::MembersListHome;
use roboxmaker_lesson::lesson_list_home::LessonListHome;
use roboxmaker_classes::classes_list_home::ClassesListHome;
use roboxmaker_loaders::fullscreen_loader::FullScreenLoader;
use roboxmaker_meetings::meetings_list_home::MeetingsListHome;
use roboxmaker_utils::functions::{user_profile_data, school_profile_data};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, SchoolId, UserId, AppRoute, NewClassGroup, ClassGroupCategory, LoadFullScreen, LoadFullScreenFound, MyUserProfile, DataSchoolProfile};

pub struct Home {
    graphql_task: Option<GraphQLTask>,
    list_degree_task: Option<RequestTask>,
    class_groups: Vec<NewClassGroup>,
    group_id_selected: Option<GroupId>,
    show_dropdown: bool,
    user_section_on: bool,
    user_selected: Option<UserId>,
    loading_screen: LoadFullScreen,
    saved_sidebar_state: bool,
    user_profile: Option<MyUserProfile>,
    school_profile: Option<DataSchoolProfile>,
    school_selected: Option<SchoolId>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct HomeProps {
    pub user_id: UserId,
    #[prop_or(None)]
    pub on_user_profile: Option<Callback<UserId>>,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum HomeMessage {
    FetchClassGroups,
    ClassGroups(Option<group_model::class_group_by_user_id::ResponseData>),
    SelectClassGroup(GroupId),
    ShowDropdown,
    ShowUserHiddenSection,
    ShowUser(UserId),
    OnShowModalUser(bool),
    ChangeSidebarState,
}

impl Component for Home {
    type Message = HomeMessage;
    type Properties = HomeProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(HomeMessage::FetchClassGroups);
        
        let saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };
        let user_profile = user_profile_data();
        let school_profile = school_profile_data();

        roboxmaker_utils::functions::home_state();

        Home { 
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            list_degree_task: None,
            class_groups: vec![],
            group_id_selected: None,
            show_dropdown: false,
            user_section_on: false,
            user_selected: None,
            loading_screen: LoadFullScreen::Loading,
            saved_sidebar_state,
            user_profile,
            school_profile,
            school_selected: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            HomeMessage::FetchClassGroups => {
                let user_id = ctx.props().user_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = group_model::class_group_by_user_id::Variables {
                        user_id: user_id.0,
                    };

                    let task = group_model::ClassGroupByUserId::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            HomeMessage::ClassGroups(response)
                        },
                    );
                    self.list_degree_task = Some(task);
                }
            },
            HomeMessage::ClassGroups(class_groups) => {
                self.class_groups = class_groups
                    .clone()
                    .and_then(|data| Some(data.class_group))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|class_group| {
                        let class_name = class_group.class_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        let group_id = class_group.group_id;
                        let school_id = class_group.school_group.clone().and_then(|data| Some(data.school_id)).unwrap_or(Uuid::default());
                        NewClassGroup {
                            class_name,
                            group_id: GroupId(group_id),
                            school_id: SchoolId(school_id),
                        }
                    }).collect();
                self.group_id_selected = match self.class_groups.first() {
                    Some(group) => Some(group.group_id),
                    None => None,
                };

                self.school_selected = match self.class_groups.first() {
                    Some(school) => Some(school.school_id),
                    None => None,
                };

                if !class_groups.clone().and_then(|data| Some(data.class_group)).unwrap_or(vec![]).is_empty() {
                    self.loading_screen = LoadFullScreen::Load(LoadFullScreenFound::Found);
                } else {
                    self.loading_screen = LoadFullScreen::Load(LoadFullScreenFound::NotFound);
                }
            }
            HomeMessage::SelectClassGroup(group_id) => {
                self.group_id_selected = Some(group_id);
                self.show_dropdown = false;
            }
            HomeMessage::ShowDropdown => {
                self.show_dropdown = !self.show_dropdown;
            }
            HomeMessage::ShowUserHiddenSection => {
                self.user_section_on = !self.user_section_on;
            }
            HomeMessage::ShowUser(user_id) => {
                self.user_selected = Some(user_id);
                if let Some(on_user_profile) = &ctx.props().on_user_profile {
                    on_user_profile.emit(user_id)
                }
            }
            HomeMessage::OnShowModalUser(show) => {
                if !show {
                    self.user_selected = None;
                }
                self.user_section_on = show;
            }
            HomeMessage::ChangeSidebarState => {
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
        trace!("{:?} => {:?}", ctx.props(), old_props);
        
        self.user_profile = user_profile_data();
        self.school_profile = school_profile_data();

        true
    }
    fn view(&self, ctx: &Context<Self>) -> Html {


        let alls_class_groups = self.class_groups.iter().map(|class_group| {
            let group_id = class_group.group_id;
            let class_id_select = format!("{:?}", group_id);
            let on_show_list_degrees = ctx.link().callback(move |_| HomeMessage::SelectClassGroup(group_id));
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
                    <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{&class_group.class_name}</span>
                }
            } else {
                html! {}
            };
            html! {
                {maybe_class}
            }
        })
        .collect::<Html>();

        let on_show_sidebar = ctx.link().callback(move |_| HomeMessage::ChangeSidebarState);

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
        let on_dropdown = ctx.link().callback(|_| HomeMessage::ShowDropdown);
        let user_profile_name = self.user_profile.as_ref()
            .and_then(|item| {
            Some(html! {
                <div class="d-flex justify-content-between">
                    <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 pb-4 mb-1">{lang::dict("Hello, ")}
                        {&item.full_name}
                    </h1>
                    {btn_sidebar_show}
                </div>
            })
        }).unwrap_or(html! {});

        let class_dropdown = if self.show_dropdown {
            "btn btn-secondary btn-second-home dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-second-home dropdown-toggle d-flex align-items-center justify-content-between"
        };
        let class_dropdown_list = if self.show_dropdown {
            "dropdown-menu dropdown-menu-home show"
        } else {
            "dropdown-menu dropdown-menu-home"
        };

        let maybe_option_user = self
            .user_profile
            .as_ref()
            .and_then(|item|{
                if item.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown dropdown-h">
                            <button class={class_dropdown} type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick={on_dropdown}>
                                <img src="/icons/graduation_1.svg" style="height: 18px;" />
                                {change_class_group}
                            </button>
                            <ul class={class_dropdown_list} aria-labelledby="dropdownMenuButton2">
                                {alls_class_groups}
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Here is your summary")}</span>
                    })
                }
            })
            .unwrap_or(html! {});

        let class_group_posts = |class_group: &NewClassGroup| {
            let school_selected = self.school_selected.and_then(|id| Some(id.0)).unwrap_or_default();

            if self.group_id_selected == Some(class_group.group_id) {
                html! {
                    <div class="scroll-x-home">
                        // <PostListHome group_id={class_group.group_id}
                        //         user_profile={self.user_profile.clone()} 
                        //         school_id={SchoolId(school_selected)} />
                    </div>
                }
            } else {
                html! {}
            }
        };
        
        let class_group_robots = |class_group: &NewClassGroup| {
            if self.group_id_selected == Some(class_group.group_id) {
                html! {
                    <div class="scroll-x-home">
                        // <RobotListHome
                        //     group_id={class_group.group_id}
                        //     user_profile={self.user_profile.clone()} />
                    </div>
                }
            } else {
                    html! {}
                }
        };
                    
        let class_group_classes = |class_group: &NewClassGroup| {
            if self.group_id_selected == Some(class_group.group_id) {
                html! {
                    <div class="scroll-x-home">
                        // <ClassesListHome
                        //     group_id={class_group.group_id}
                        //     user_profile={self.user_profile.clone()} 
                        //     school_id={ctx.props().school_id} />
                    </div>
                }
            } else {
                html! {}
            }
        };

        let class_group_lessons = |class_group: &NewClassGroup| {
            if self.group_id_selected == Some(class_group.group_id) {
                html! {
                    <div class="scroll-x-home">
                        // <LessonListHome group_id={class_group.group_id}
                        //     user_profile={self.user_profile.clone()} 
                        //     school_id={ctx.props().school_id }
                        //     filter_lessons={false}
                        //     maybe_author={true} />
                    </div>
                }
            } else {
                html! {}
            }
        };

        let class_group_meetings = |class_group: &NewClassGroup| {
            if self.group_id_selected == Some(class_group.group_id) {
                html! {
                    <div class="d-flex flex-column">
                        // <MeetingsListHome group_id={class_group.group_id} />
                    </div>
                }
            } else {
                html! {}
            }
        };

        let class_group_members = |class_group: &NewClassGroup| {
            if self.group_id_selected == Some(class_group.group_id) {
                html! {
                    <div class="d-flex flex-column">
                        // <MembersListHome user_profile={self.user_profile.clone()}
                            // group_id={class_group.group_id} />
                    </div>
                }
            } else {
                html! {}
            }
        };
        let on_user_section = ctx.link().callback(move |_| HomeMessage::ShowUserHiddenSection);
        let maybe_user_profile_pic = self
            .user_profile
            .as_ref()
            .and_then(|item| {
                let pic_path = item.pic_path.clone();
                let full_name = item.full_name.clone();
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
        let maybe_section_user = self
            .user_profile
            .as_ref()
            .zip(
                self.school_profile
                    .as_ref()
                    .and_then(|auth_school| Some(auth_school.clone())),
            )
            .and_then(|(item, school_profile)| {

                let licence = item.license.clone();

                let maybe_user_type = if item.user_teacher.is_some() {
                    Some(html! {
                        <span class="text-purple-gray text-center noir-light is-size-14 lh-17 pt-2">{lang::dict("Teacher")}</span>
                    })
                } else {
                    Some(html! {
                        <span class="text-purple-gray text-center noir-light is-size-14 lh-17 pt-2">{lang::dict("Student")}</span>
                    })
                } .unwrap_or(html! {});

                let close_modal_callback = ctx.link().callback(|_| HomeMessage::OnShowModalUser(false));

                let maybe_email = item.email.clone();

                let list_class_group = self
                    .class_groups
                    .iter()
                    .map(|class_group| {      
                    html! {
                        <span class="text-brown noir-light is-size-18 lh-22 mb-2">{&class_group.class_name}</span>
                    }
                }).collect::<Html>();

                let maybe_license = if item.user_teacher.is_some() {
                    html! {}
                } else {
                    html! {
                        <>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("License")}</span>
                            <div class="mb-4"><span class="text-brown noir-light is-size-18 lh-22">{&licence}</span></div>
                        </>
                    }
                };

                Some(html! {
                    <>
                        <div class="d-none d-sm-none d-md-none d-lg-block">
                            <MyProfilePage user_id={ctx.props().user_id.clone()} 
                                user_profile={self.user_profile.clone()}
                                on_user_profile={ctx.props().on_user_profile.clone()}
                                show_user={self.user_section_on.clone()} 
                                close_modal_callback={close_modal_callback.clone()} />
                            <div class="d-flex justify-content-center">{maybe_user_type}</div>
                            // <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("License")}</span>
                            // <div class="mb-4"><span class="text-brown noir-light is-size-18 lh-22">{&licence}</span></div>
                            {maybe_license}
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("College")}<span>{"(s)"}</span></span>
                            <div class="mb-4"><span class="text-brown noir-light is-size-18 lh-22">{&school_profile.name}</span></div>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("Grade")}<span>{"(s)"}</span></span>
                            <div class="d-flex flex-column mb-3">{list_class_group}</div>
                            <span class="text-primary-blue-dark noir-bold is-size-14 lh-17 pb-2">{lang::dict("Email")}</span>
                            <br />
                            <span class="text-brown noir-light is-size-18 lh-22 pb-4">{maybe_email}</span>
                        </div>
                    </>
                })
            })
            .unwrap_or_default();  
        let class_group_level = |class_group: & NewClassGroup| {
            let group_id = class_group.group_id;
            let school_id = class_group.school_id;
            let class_group_id = format!("class-group-{}", group_id);
            let user_id = self.user_profile.clone().and_then(|data| Some(data.user_id)).unwrap_or(UserId(Uuid::default()));
            let class_name = class_group.class_name.clone().to_uppercase();
            let _class_lesson_view = if class_name.contains("KINDER") 
                || class_name.contains("PREPARATORIA") {
                true
            } else {
                false
            };

            let category = ClassGroupCategory::Posts;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_post = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            // let on_class_group_post = ctx.link().callback(move |_| {
            //     HomeMessage::AppRoute(AppRoute::SchoolGroupSection{school_id, group_id, category})
            // });
            let category = ClassGroupCategory::Classes;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_classes = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            // let on_class_group_classes = ctx.link().callback(move |_| {
            //     HomeMessage::AppRoute(AppRoute::SchoolGroupSection{school_id, group_id, category})
            // });
            let category = ClassGroupCategory::Lessons;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_lessons = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            // let on_class_group_lessons = ctx.link().callback(move |_| {
            //     HomeMessage::AppRoute(AppRoute::SchoolGroupSection{school_id, group_id, category})
            // });
            let category = ClassGroupCategory::Robots;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_robot = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            // let on_class_group_robot = ctx.link().callback(move |_| {
            //     HomeMessage::AppRoute(AppRoute::SchoolGroupSection{school_id, group_id, category})
            // });
            let category = ClassGroupCategory::Posts;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_post_st = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            // let on_class_group_post_st = ctx.link().callback(move |_| {
            //     HomeMessage::AppRoute(AppRoute::GroupSectionStudent{school_id, user_id, category})
            // });
            let category = ClassGroupCategory::Classes;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_classes_st = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            // let on_class_group_classes_st = ctx.link().callback(move |_| {
            //     HomeMessage::AppRoute(AppRoute::GroupSectionStudent{school_id, user_id, category})
            // });
            let category = ClassGroupCategory::Lessons;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_lessons_st = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            // let on_class_group_lessons_st = ctx.link().callback(move |_| {
            //     HomeMessage::AppRoute(AppRoute::GroupSectionStudent{school_id, user_id, category})
            // });
            let category = ClassGroupCategory::Robots;
            let navigator = ctx.link().navigator().unwrap();
            let on_class_group_robot_st = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
            // let on_class_group_robot_st = ctx.link().callback(move |_| {
            //     HomeMessage::AppRoute(AppRoute::GroupSectionStudent{school_id, user_id, category})
            // });
            let navigator = ctx.link().navigator().unwrap();
            let on_meetings = Callback::from(move |_| navigator.push(&AppRoute::Meetings));
            // let on_meetings = ctx.link().callback(move |_| HomeMessage::AppRoute(AppRoute::Meetings));
            let maybe_members = self.user_profile.as_ref().and_then(|user| {
                if user.user_staff.is_some() || user.user_teacher.is_some() {
                    Some(html! {
                        <>
                            <div class="my-4"><span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Members")}</span></div>
                            <div class="card-members-class bg-white px-4 pt-4">
                                {class_group_members(class_group)}
                            </div>
                        </>
                    })
                } else {
                    None
                }
            }).unwrap_or(html! {});
            let maybe_option = if self.user_section_on {
                html! {
                    {maybe_section_user}
                }
            } else {
                html! {
                    <>
                        <div>
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-29">{lang::dict("Latest Robots")}</span>
                            <UserRobots user_id={user_id.clone()} 
                                user_profile={self.user_profile.clone()}
                                maybe_style={UserStyle::ListHome} />
                        </div>
                        {maybe_members}
                    </>
                }
            };
            let class_right_sidebar = if self.saved_sidebar_state {
                "bg-silver col col-sm-3 col-md-3 col-lg-5 col-xl-4 col-xxl-3 d-none d-sm-none d-md-none d-lg-block p-5"
            } else {
                "d-none"
            };
            let maybe_post = self.user_profile.clone()
                .and_then(|user| {
                    if user.user_teacher.is_some() {
                        Some(html! {
                            <div class="d-flex justify-content-between align-items-center py-home-sections">
                                <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Posts")}</span>
                                <span class="icon-text d-flex align-items-center">
                                    <a onclick={on_class_group_post}>
                                        <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                                    </a>
                                    <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                        <i class="fas fa-arrow-right"></i>
                                    </span>
                                </span>
                            </div>
                        })
                    } else {
                        Some(html! {
                            <div class="d-flex justify-content-between align-items-center py-home-sections">
                                <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Posts")}</span>
                                <span class="icon-text d-flex align-items-center">
                                    <a onclick={on_class_group_post_st}>
                                        <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                                    </a>
                                    <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                        <i class="fas fa-arrow-right"></i>
                                    </span>
                                </span>
                            </div>
                        })
                    }
                }).unwrap_or(html! {});

            let maybe_classes = self.user_profile.clone()
                .and_then(|user| {
                    if user.user_teacher.is_some() {
                        Some(html! {
                            <div class="d-flex justify-content-between align-items-center py-home-sections">
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
                        })
                    } else {
                        Some(html! {
                            <div class="d-flex justify-content-between align-items-center py-home-sections">
                                <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Classes")}</span>
                                <span class="icon-text d-flex align-items-center">
                                    <a onclick={on_class_group_classes_st}>
                                        <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                                    </a>
                                    <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                        <i class="fas fa-arrow-right"></i>
                                    </span>
                                </span>
                            </div>
                        })
                    }
                }).unwrap_or(html! {});

            let maybe_lessons = self.user_profile.clone()
                .and_then(|user| {
                    if user.user_teacher.is_some() {
                        Some(html! {
                            <div class="d-flex justify-content-between align-items-center py-home-sections">
                                <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Lessons")}</span>
                                <span class="icon-text d-flex align-items-center">
                                    <a onclick={&{on_class_group_lessons}}>
                                        <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                                    </a>
                                    <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                        <i class="fas fa-arrow-right"></i>
                                    </span>
                                </span>
                            </div>
                        })
                    } else {
                        Some(html! {
                            <div class="d-flex justify-content-between align-items-center py-home-sections">
                                <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Lessons")}</span>
                                <span class="icon-text d-flex align-items-center">
                                    <a onclick={&{on_class_group_lessons_st}}>
                                        <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                                    </a>
                                    <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                        <i class="fas fa-arrow-right"></i>
                                    </span>
                                </span>
                            </div>
                        })
                    }
                }).unwrap_or(html! {});

            let maybe_robots = self.user_profile.clone()
                .and_then(|user| {
                    if user.user_teacher.is_some() {
                        Some(html! {
                            <div class="d-flex justify-content-between align-items-center py-home-sections">
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
                        })
                    } else {
                        Some(html! {
                            <div class="d-flex justify-content-between align-items-center py-home-sections">
                                <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Robots")}</span>
                                <span class="icon-text d-flex align-items-center">
                                    <a onclick={on_class_group_robot_st}>
                                        <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                                    </a>
                                    <span class="text-cyan-sky noir-medium is-size-16 lh-19">
                                        <i class="fas fa-arrow-right"></i>
                                    </span>
                                </span>
                            </div>
                        })
                    }
                }).unwrap_or(html! {});

            let maybe_meets = self.user_profile.clone()
                .and_then(|user| {
                    if user.user_teacher.is_some() {
                        Some(html! {
                            <div class="d-flex justify-content-between align-items-center py-home-sections">
                                <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Meetings")}</span>
                                <a onclick={on_meetings}>
                                    <span class="d-flex align-items-center">
                                        <span class="text-cyan-sky noir-medium is-size-16 lh-19 me-2">{"Ver todo"}</span>
                                        <span class="icon text-cyan-sky noir-medium is-size-16 lh-19">
                                            <i class="fas fa-arrow-right"></i>
                                        </span>
                                    </span>
                                </a>
                            </div>
                        })
                    } else {
                        Some(html! {
                            <div class="d-flex align-items-center py-home-sections">
                                <span class="text-primary-blue-dark noir-medium is-size-20 lh-24">{lang::dict("Meetings")}</span>
                            </div>
                        })
                    }
                }).unwrap_or(html! {});

            html! {
                <>
                    <div class="w-100 h-100 d-flex flex-row justify-content-between scroll-y scroll-x-hidden">
                        <div class="w-100 pt-3 ps-3 pt-md-4 ps-md-4 pt-lg-7 ps-lg-7">
                            {user_profile_name}
                            {maybe_option_user}
                            <div id={ class_group_id.clone() }>
                                {maybe_post}
                                {class_group_posts(class_group)}
                                {maybe_classes}
                                {class_group_classes(class_group)}
                                {maybe_lessons}
                                // {
                                //     if class_lesson_view {
                                //         html! {}
                                //     } else {
                                //     }
                                // }
                                {class_group_lessons(class_group)}
                                {maybe_robots}
                                {class_group_robots(class_group)}
                                {maybe_meets}
                                {class_group_meetings(class_group)}
                            </div>
                        </div>
                    </div>
                    <div class={class_right_sidebar}>
                        <div class="d-flex flex-wrap align-items-center justify-content-between mb-4">
                            <SearchView group_id={Some(group_id)}
                                user_profile={self.user_profile.clone()}
                                school_id={school_id.clone()} />
                            {maybe_user_profile_pic}
                        </div>
                        {maybe_option.clone()}
                    </div>
                </>
            }
        };
        let right_sidebar = |class_group_data: &NewClassGroup|{
            let group_id = class_group_data.group_id;
            // let school_id = ctx.props().auth_school.as_ref().and_then(|data| Some(data.id)).unwrap_or(Uuid::default());
            let school_id = Uuid::default();
            let on_user_section = ctx.link().callback(move |_| HomeMessage::ShowUserHiddenSection);
            let maybe_user_profile_pic = self
                .user_profile
                .as_ref()
                .and_then(|item| {
                    let pic_path = item.pic_path.clone();
                    let full_name = item.full_name.clone();
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

            let close_modal_callback = ctx.link().callback(|_| HomeMessage::OnShowModalUser(false));
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
            let maybe_option = if self.user_section_on {
                html! {
                    <MyProfilePage user_id={ctx.props().user_id}
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
                            <UserRobots user_id={ctx.props().user_id }
                                user_profile={self.user_profile.clone()}
                                maybe_style={UserStyle::ListHome} />
                        </div>
                        {maybe_members}
                    </>
                }
            };
            let mayber_sidebar = if self.saved_sidebar_state {
                html! {
                    <div class="d-flex flex-column justify-content-between w-100">
                        <div class="d-flex flex-wrap align-items-center justify-content-between mb-3">
                            <SearchView group_id={Some(group_id)}
                                user_profile={self.user_profile.clone()}
                                school_id={SchoolId(school_id)} />
                            {maybe_user_profile_pic.clone()}
                        </div>
                        {maybe_option}
                    </div>
                }
            } else {
                html! {}
            };
            html! {
                {mayber_sidebar}
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
        let home_view = match self.loading_screen {
            LoadFullScreen::Loading => {
                html! {
                    <FullScreenLoader />
                }
            },
            LoadFullScreen::Load(LoadFullScreenFound::Found) => {
                html! {
                    <>
                        {
                            self.class_groups
                                .iter()
                                .filter(|data| data.group_id == self.group_id_selected.unwrap_or(GroupId(Uuid::default())))
                                .map(|class_group|  {
                                    class_group_level.clone()(class_group)
                                }).collect::<Html>()
                        }
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
            {home_view}
        }
    }
}
