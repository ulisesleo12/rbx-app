use log::*;
use uuid::Uuid;
use gloo_storage::Storage;
use yew_router::prelude::*;
use yew::{prelude::*, web_sys};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::school_model;
use roboxmaker_types::types::{AppRoute, ClassGroupCategory, ClassesId, GroupId, LessonId, MeetingsId, MyUserProfile, PageMode, PostId, ResourceId, RobotId, SchoolId, UserId};

pub struct Menu {
    link: ComponentLink<Self>,
    props: Properties,
    node_burger_ref: NodeRef,
    node_menu_ref: NodeRef,
    reset_toggle: bool,
    sidebar_menu_state: bool,
    toggle_menu_mobile: bool,
}

#[derive(Debug, Properties, Clone)]
pub struct Properties {
    pub route: Route<()>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_app_route: Callback<AppRoute>,
    pub group_id: GroupId,
    pub category: ClassGroupCategory,
    pub post_id: PostId,
    pub robot_id: RobotId,
    pub user_id: UserId,
    pub lesson_id: LessonId,
    pub classes_id: ClassesId,
    pub school_id: SchoolId,
    pub meetings_id: MeetingsId,
    pub resource_id: ResourceId,
    pub on_logout: Callback<MouseEvent>,
    pub page_mode: PageMode,
    pub quiz_id: Uuid,
}

#[derive(Debug)]
pub enum MenuMessage {
    AppRoute(AppRoute),
    Toggle,
    ChangeSidebarState,
    MenuToggleMobile,
}

impl Menu {
    pub fn toggle(&mut self) {
        self.node_burger_ref
            .cast::<web_sys::Element>()
            .unwrap()
            .class_list()
            .toggle("is-active")
            .unwrap();
        self.node_menu_ref
            .cast::<web_sys::Element>()
            .unwrap()
            .class_list()
            .toggle("is-active")
            .unwrap();
    }
}

impl Component for Menu {
    type Message = MenuMessage;
    type Properties = Properties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let sidebar_menu_state = if let Ok(value) = gloo_storage::LocalStorage::get("sidebar_menu_state") {
            value 
        } else {
            true
        };
        Menu {
            link,
            props,
            node_burger_ref: NodeRef::default(),
            node_menu_ref: NodeRef::default(),
            reset_toggle: false,
            sidebar_menu_state,
            toggle_menu_mobile: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            MenuMessage::AppRoute(route) => {
                if self.reset_toggle {
                    self.reset_toggle = false;
                    self.toggle();
                }
                self.props.on_app_route.emit(route);
            }
            MenuMessage::Toggle => {
                self.reset_toggle = !self.reset_toggle;
                self.toggle();
            }
            MenuMessage::ChangeSidebarState => {
                if let Some(element) = gloo_utils::document().get_element_by_id("sidebar-menu-state") {
                    if self.sidebar_menu_state {
                        let _ = gloo_storage::LocalStorage::set("sidebar_menu_state", false);
                        self.sidebar_menu_state = false;
                        let _ = element.set_attribute("class", "fa fa-angle-double-right fa-w-14 fa-2x");
                    } else {
                        let _ = gloo_storage::LocalStorage::set("sidebar_menu_state", true);
                        self.sidebar_menu_state = true;
                        let _ = element.set_attribute("class", "fa-angle-double-left fa-w-14 fa-2x");
                    }
                }
            }
            MenuMessage::MenuToggleMobile => {
                if let Some(element) = gloo_utils::document().get_element_by_id("toggle-menu-mobile") {
                    if self.toggle_menu_mobile {
                        self.toggle_menu_mobile = false;
                        let _ = element.set_attribute("class", "fas fa-bars fa-w-14 fa");
                    } else {
                        self.toggle_menu_mobile = true;
                        let _ = element.set_attribute("class", "fas fa-times fa-w-14 fa");
                    }
                }
                should_update = true;
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        // let user_id = self.props.user_id;
        let school_id_uuid = self.props.auth_school.clone().and_then(|data| Some(data.id)).unwrap_or(Uuid::default());
        let user_id = self.props.user_profile.clone().and_then(|item| Some(item.user_id)).unwrap_or(UserId(Uuid::default()));

        let on_show_sidebar = self.link.callback(move |_| MenuMessage::ChangeSidebarState);
        let on_schools_staff = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::Schools));
        let on_schools_teacher = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::GradesByUserId(SchoolId(school_id_uuid))));
        let on_my_section = self.link.callback(move |_| {
            MenuMessage::AppRoute(AppRoute::GroupSectionStudent(
                SchoolId(school_id_uuid.clone()),
                user_id.clone(),
                ClassGroupCategory::Posts,
            ))
        });
        let on_my_space = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::MySpace(user_id)));

        let on_menu_home = self
            .link
            .callback(|_| MenuMessage::AppRoute(AppRoute::Home));
            
        let btn_sidebar_menu_show = if self.sidebar_menu_state {
            html! {
                <a class="btn-show-sidebar-menu" onclick=&on_show_sidebar>
                    <span class="text-white">
                        <i class="fas fa-angle-double-left fas fa-2x" id="sidebar-menu-state"></i>
                    </span>
                </a>
            }
        } else {
            html! {
                <a class="btn-show-sidebar-menu" onclick=&on_show_sidebar>
                    <span class="text-white">
                        <i class="fas fa-angle-double-right fas fa-2x" id="sidebar-menu-state"></i>
                    </span>
                </a>
            }
        };

        let maybe_roboxmaker_logo = if self.sidebar_menu_state {
            html! {
                <img src="/static/logo-robox-maker.png" />
            }
        } else {
            html! {}
        };

        let maybe_menu = self.props.user_profile.as_ref()
            .zip(self.props.auth_school.as_ref()
            .and_then(|school| school.school_profile.as_ref()))
            .and_then(|(item, school_profile)| {
                let user_id = item.user_id;
                let school_id = self.props.school_id;
                let group_id = self.props.group_id;
                let grade_is_active = if self.props.route == Route::from(AppRoute::SchoolGroupSection(
                    school_id.clone(),
                    group_id,
                    self.props.category,
                )) || 
                self.props.route == Route::from(AppRoute::Quizzes(self.props.school_id, group_id, self.props.quiz_id)) || 
                self.props.route == Route::from(AppRoute::Post(self.props.school_id, group_id, self.props.post_id, self.props.page_mode)) || 
                self.props.route == Route::from(AppRoute::Robot(self.props.robot_id, group_id, self.props.user_id)) || 
                self.props.route == Route::from(AppRoute::Lesson(self.props.school_id, group_id, self.props.lesson_id)) || 
                self.props.route == Route::from(AppRoute::LessonView(self.props.school_id, group_id, self.props.lesson_id)) || 
                // self.props.route == Route::from(AppRoute::Classes(self.props.school_id, group_id, self.props.classes_id)) ||
                self.props.route == Route::from(AppRoute::Schools) ||
                self.props.route == Route::from(AppRoute::GradesBySchoolId(school_id)) ||
                self.props.route == Route::from(AppRoute::GradesByUserId(school_id)) ||
                self.props.route == Route::from(AppRoute::Meet(group_id, self.props.meetings_id)) ||
                self.props.route == Route::from(AppRoute::GroupSectionStudent(  
                    school_id.clone(),
                    user_id.clone(),
                    self.props.category,
                )) ||
                self.props.route == Route::from(AppRoute::MeetDirect(group_id)) {
                    "bg-menu-option-active"
                } else {
                    "bg-menu-option-inactive"
                };

                let school_id = SchoolId(school_profile.school_id);

                let on_menu_school_class_groups_student = self.link.callback(move |_| {
                    MenuMessage::AppRoute(AppRoute::GroupSectionStudent(
                        school_id.clone(),
                        user_id.clone(),
                        ClassGroupCategory::Posts,
                    ))
                });

                let home_is_active = if self.props.route == Route::from(AppRoute::Home) {
                    "bg-menu-option-active"
                } else {
                    "bg-menu-option-inactive"
                };
                let on_schools_staff = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::Schools));
                let on_schools_teacher = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::GradesByUserId(school_id)));
                let colleges_option = if item.user_staff.is_some() {
                    Some(html! {
                        <>
                            <a class=grade_is_active onclick=&on_schools_staff>
                                <img src="/icons/graduation.svg"
                                    style="height: 17px; width: 26px; object-fit: contain;" />
                                <span class="links_name">{lang::dict("Schools")}</span>
                            </a>
                            <span class="tooltip">{lang::dict("Schools")}</span>
                        </>
                    })
                } else if item.user_teacher.is_some() {
                    Some(html! {
                        <>
                            <a class=grade_is_active onclick=&on_schools_teacher>
                                <img src="/icons/graduation.svg"
                                    style="height: 17px;width: 26px;object-fit: contain;" />
                                <span class="links_name">{lang::dict("Degrees")}</span>
                            </a>
                            <span class="tooltip">{lang::dict("Degrees")}</span>
                        </>
                    })
                } else {
                    Some(html! {
                        <>
                            <a class=grade_is_active onclick=&on_menu_school_class_groups_student>
                                <img src="/icons/graduation.svg"
                                    style="height: 17px;width: 26px;object-fit: contain;" />
                                <span class="links_name">{lang::dict("My section")}</span>
                            </a>
                            <span class="tooltip">{lang::dict("My section")}</span>
                        </>
                    })
                }
                .unwrap_or(html! {});
                let on_my_space = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::MySpace(user_id)));

                let my_space_is_active = if self.props.route == Route::from(AppRoute::MySpace(user_id)) {
                    "bg-menu-option-active"
                } else {
                    "bg-menu-option-inactive"
                };
                let on_menu_meetings = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::Meetings));
                let meetings_is_active = if self.props.route == Route::from(AppRoute::Meetings) {
                    "bg-menu-option-active"
                } else {
                    "bg-menu-option-inactive"
                };
                let on_menu_quiz_panel = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::QuizzesPanel));
                let quiz_panel_is_active = if self.props.route == Route::from(AppRoute::QuizzesPanel) {
                    "bg-menu-option-active"
                } else {
                    "bg-menu-option-inactive"
                };
                let maybe_meetings = if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <>
                            <a class=meetings_is_active onclick=on_menu_meetings>
                                <img src="/icons/video.svg"
                                    style="height: 28px; width: 30px; object-fit: contain;" />
                                <span class="links_name ms-1">{lang::dict("Meetings")}</span>
                            </a>
                            <span class="tooltip">{lang::dict("Meetings")}</span>
                        </>
                    })
                } else {
                    Some(html! {})
                }.unwrap_or(html! {});
                
                let quiz_panel = if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <>
                            <a class={ quiz_panel_is_active } onclick={ on_menu_quiz_panel }>
                                <span class="ms-3">
                                    <i class="fas fa-list-alt fas fa-lg"></i>
                                </span>
                                <span class="links_name ms-3">{ "Resultados" }</span>
                            </a>
                            <span class="tooltip">{ "Resultados" }</span>
                        </>
                    })
                } else {
                    Some(html! {})
                }.unwrap_or(html! {});

                let class_sidebar_menu = if self.sidebar_menu_state {
                    "sidebar open card bg-primary-blue-light text-white vh-100 border-0 rounded-0 col col-sm-3 col-md-3 col-lg-2 col-xl-2 col-xxl-2 d-none d-sm-none d-md-none d-lg-block"
                } else {
                    "sidebar close-menu card bg-primary-blue-light text-white vh-100 border-0 rounded-0 col col-1 d-none d-sm-none d-md-none d-lg-block"
                };

                Some(html! {
                    <div class=class_sidebar_menu>
                        <img src="/static/arte-2.png" class="card-img card-img-menu" alt="..." />
                        <div class="card-img-overlay blur-img-menu h-100 d-flex flex-column justify-content-between">
                            <div class="d-flex justify-content-between">
                                { maybe_roboxmaker_logo }
                                { btn_sidebar_menu_show }
                            </div> 
                            <ul class="nav-list mb-3">
                                <li>
                                    <a class=home_is_active onclick={ &on_menu_home }>
                                        <span class="ms-3">
                                            <i class="fas fa-home fas fa-lg"></i>
                                        </span>
                                        <span class="links_name ms-3">{lang::dict("Home")}</span>
                                    </a>
                                    <span class="tooltip">{lang::dict("Home")}</span>
                                </li>
                                <li>
                                    { colleges_option }
                                </li>
                                <li>
                                    <a class=my_space_is_active onclick=&on_my_space>
                                        <span class="ms-3">
                                            <i class="far fa-sticky-note fas fa-lg"></i>
                                        </span>
                                        <span class="links_name ms-3">{lang::dict("My space")}</span>
                                    </a>
                                    <span class="tooltip">{lang::dict("My space")}</span>
                                </li>
                                <li>
                                    { maybe_meetings }
                                </li>
                                <li>
                                    { quiz_panel }
                                </li>
                            </ul>
                            <div class="">
                                <ul class="nav-list-2">
                                    <li>
                                        <a>
                                            <span class="text-white" style="margin-left: 13px;">
                                                <i class="far fa-question-circle fas fa-lg"></i>
                                            </span>
                                            <span class="links_name ms-3">{lang::dict("About...")}</span>
                                        </a>
                                        <span class="tooltip">{lang::dict("About...")}</span>
                                    </li>
                                    <li>
                                        <a onclick={&self.props.on_logout}>
                                            <span class="text-white ms-3">
                                                <i class="fas fa-sign-out-alt fas fa-lg"></i>
                                            </span>
                                            <span class="links_name ms-3">{lang::dict("Logout")}</span>
                                        </a>
                                        <span class="tooltip">{lang::dict("Logout")}</span>
                                    </li>
                                </ul>
                                <span class="text-white noir-bold is-size-12">{ "v2.0.62" }</span>
                            </div>
                        </div>
                    </div>
                })
            }).unwrap_or(html! {});
        let on_menu_toggle_mobile = self.link.callback(|_| MenuMessage::MenuToggleMobile);
        let class_dropdown = if self.toggle_menu_mobile {
            "nav-link dropdown-toggle menu-hidden-toggle focus-none show"
        } else {
            "nav-link dropdown-toggle menu-hidden-toggle focus-none"
        };
        let class_dropdown_item = if self.toggle_menu_mobile {
            "dropdown-menu show"
        } else {
            "dropdown-menu"
        };
        let maybe_toggle_menu = if self.toggle_menu_mobile {
            html! {
                <a class=class_dropdown data-bs-toggle="dropdown" role="button" aria-expanded="true" onmousedown=&on_menu_toggle_mobile>
                    <span class="text-white">
                        <i class="fas fa-times" id="toggle-menu-mobile"></i>
                    </span>
                </a>
            }
        } else {
            html! {
                <a class=class_dropdown data-bs-toggle="dropdown" role="button" aria-expanded="true" onmousedown=&on_menu_toggle_mobile>
                    <span class="text-white">
                        <i class="fas fa-bars" id="toggle-menu-mobile"></i>
                    </span>
                </a>
            }
        };
        let on_menu_meetings = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::Meetings));
        let on_menu_quiz_panel = self.link.callback(move |_| MenuMessage::AppRoute(AppRoute::QuizzesPanel));
        let menu_mobile = self.props.user_profile.as_ref()
            .and_then(|item| {
                let schools_option = if item.user_staff.is_some() {
                    html! {
                        <li>
                            <a class="dropdown-item bg-primary-blue-dark" onclick=&on_schools_staff>
                                <img src="/icons/graduation.svg" style="height: 18px; width: 28px; object-fit: contain;" />
                                <span class="text-white ms-1">{lang::dict("Schools")}</span>
                            </a>
                        </li>
                    }
                } else if item.user_teacher.is_some() {
                    html! {
                        <li>
                            <a class="dropdown-item bg-primary-blue-dark" onclick=&on_schools_teacher>
                                <img src="/icons/graduation.svg" style="height: 18px; width: 28px; object-fit: contain;" />
                                <span class="text-white ms-1">{lang::dict("Degrees")}</span>
                            </a>
                        </li>
                    }
                } else {
                    html! {
                        <li>
                            <a class="dropdown-item bg-primary-blue-dark" onclick=&on_my_section>
                                <img src="/icons/graduation.svg" style="height: 18px; width: 28px; object-fit: contain;" />
                                <span class="text-white ms-1">{lang::dict("My section")}</span>
                            </a>
                        </li>
                    }
                };

                let meets = if item.user_staff.is_some() || item.user_teacher.is_some() {
                    html! {
                        <li>
                            <a class="dropdown-item bg-primary-blue-dark" onclick=&on_menu_meetings>
                                <img src="/icons/video.svg" style="height: 26px; width: 30px; object-fit: contain;" />
                                <span class="text-white ms-1">{lang::dict("Meetings")}</span>
                            </a>
                        </li>
                    }
                } else {
                    html! {}
                };
                
                let quiz_panel = if item.user_staff.is_some() || item.user_teacher.is_some() {
                    html! {
                        <li>
                            <a class="dropdown-item bg-primary-blue-dark" onclick={ &on_menu_quiz_panel }>
                                // <img src="/icons/video.svg" style="height: 26px; width: 30px; object-fit: contain;" />
                                <span class="text-white">
                                    <i class="fas fa-list-alt fas fa-lg"></i>
                                </span>
                                <span class="text-white ms-2">{ "Resultados" }</span>
                            </a>
                        </li>
                    }
                } else {
                    html! {}
                };

                Some(html! {
                    <div class="w-100 d-sm-block d-md-block d-lg-none">
                        <ul class="nav nav-tabs bg-primary-blue-dark d-flex justify-content-between">
                            <li class="nav-item d-flex flex-wrap">
                                <a class="nav-link focus-none" aria-current="page" onclick=&on_menu_home>
                                    <img src="/static/logo-robox-maker.png" style="height: 25px;" />
                                </a>
                                <a class="nav-link focus-none" aria-current="page" onclick=&on_menu_home>
                                    <img src="/icons/home.svg" alt="" style="height: 23px;" />
                                </a>
                            </li>
                            <li class="nav-item dropdown">
                                {maybe_toggle_menu}
                                <ul class=class_dropdown_item style="right: 0px;">
                                    {schools_option}
                                    <li>
                                        <a class="dropdown-item bg-primary-blue-dark" onclick=&on_my_space>
                                            <span class="text-white is-size-22">
                                                <i class="far fa-sticky-note"></i>
                                            </span>
                                            <span class="text-white ms-1">{lang::dict("My space")}</span>
                                        </a>
                                    </li>
                                    { meets }
                                    { quiz_panel }
                                    <li><hr class="dropdown-divider" /></li>
                                    <li>
                                        <a class="dropdown-item bg-primary-blue-dark" onclick={&self.props.on_logout}>
                                            <span class="text-white">
                                                <i class="fas fa-sign-out-alt fas fa-lg"></i>
                                            </span>
                                            <span class="text-white ms-1">{lang::dict("Logout")}</span>
                                        </a>
                                    </li>
                                </ul>
                            </li>
                        </ul>
                    </div>
                })
            }).unwrap_or(html! {});
        html! {
            <>
                {menu_mobile}
                {maybe_menu}
            </>
        }
    }
}
