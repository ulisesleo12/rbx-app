use log::*;
use uuid::Uuid;
use yew::prelude::*;
use gloo_storage::Storage;
use yew::{html, Component, Html};
use gloo_timers::callback::Interval;

use roboxmaker_main::lang;
use yew_router::scope_ext::RouterScopeExt;
use roboxmaker_types::types::{AppRoute, SchoolId, ClassGroupCategory, UserId, MyUserProfile, DataSchoolProfile};

pub struct Menu {
    node_burger_ref: NodeRef,
    node_menu_ref: NodeRef,
    reset_toggle: bool,
    sidebar_menu_state: bool,
    toggle_menu_mobile: bool,
    home_state: bool,
    school_state: bool,
    myspace_state: bool,
    meets_state: bool,
    state_interval: Option<Interval>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Properties {
    pub user_profile: Option<MyUserProfile>,
    pub on_logout: Callback<MouseEvent>,
    pub school_profile: Option<DataSchoolProfile>
}

#[derive(Debug)]
pub enum MenuMessage {
    Toggle,
    ChangeSidebarState,
    MenuToggleMobile,
    States,
    GetState,
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

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(MenuMessage::GetState);
        let sidebar_menu_state = if let Ok(value) = gloo_storage::LocalStorage::get("sidebar_menu_state") {
            value 
        } else {
            true
        };

        Menu {
            node_burger_ref: NodeRef::default(),
            node_menu_ref: NodeRef::default(),
            reset_toggle: false,
            sidebar_menu_state,
            toggle_menu_mobile: false,
            home_state: false,
            school_state: false,
            myspace_state: false,
            meets_state: false,
            state_interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            MenuMessage::GetState => {
                if ctx.props().user_profile.is_some() && ctx.props().school_profile.is_some() {
                    let link = ctx.link().clone();
    
                    self.state_interval = Some(Interval::new(1_000, move || {
                        link.send_message(MenuMessage::States);
                    }))
                }
            }
            MenuMessage::States => {
        
                self.home_state = if let Ok(value) = gloo_storage::LocalStorage::get("home-state") {
                    value
                } else {
                    true
                };
                self.school_state = if let Ok(value) = gloo_storage::LocalStorage::get("school-state") {
                    value
                } else {
                    true
                };
                self.myspace_state = if let Ok(value) = gloo_storage::LocalStorage::get("myspace-state") {
                    value
                } else {
                    true
                };
                self.meets_state = if let Ok(value) = gloo_storage::LocalStorage::get("meets-state") {
                    value
                } else {
                    true
                };
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

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {

        if ctx.props().user_profile.is_some() && ctx.props().school_profile.is_some() {
            // ctx.link().send_message(MenuMessage::States);
            ctx.link().send_message(MenuMessage::GetState);
        }

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // let user_id = ctx.props().user_id;
        // let school_id_uuid = ctx.props().auth_school.clone().and_then(|data| Some(data.id)).unwrap_or(Uuid::default());
        let school_id_uuid = Uuid::default();
        let school_id = SchoolId(school_id_uuid);
        let user_id = ctx.props().user_profile.clone().and_then(|data| Some(data.user_id)).unwrap_or(UserId(Uuid::default()));

        let on_show_sidebar = ctx.link().callback(move |_| MenuMessage::ChangeSidebarState);
        let navigator = ctx.link().navigator().unwrap();
        let on_schools_staff = Callback::from(move |_| navigator.push(&AppRoute::Schools));
        // let on_schools_staff = ctx.link().callback(move |_| MenuMessage::AppRoute(AppRoute::Schools));
        let navigator = ctx.link().navigator().unwrap();
        let on_schools_teacher = Callback::from(move |_| navigator.push(&AppRoute::GradesByUserId{school_id}));
        // let on_schools_teacher = ctx.link().callback(move |_| MenuMessage::AppRoute(AppRoute::GradesByUserId{school_id}));
        let category = ClassGroupCategory::Posts;

        let navigator = ctx.link().navigator().unwrap();
        let on_my_section = Callback::from(move |_| navigator.push(&AppRoute::GroupSectionStudent{
            school_id,
            user_id,
            category,
        }));
        // let on_my_section = ctx.link().callback(move |_| {
        //     MenuMessage::AppRoute(AppRoute::GroupSectionStudent{
        //         school_id,
        //         user_id,
        //         category,
        //     })
        // });
        let navigator = ctx.link().navigator().unwrap();
        let on_my_space = Callback::from(move |_| navigator.push(&AppRoute::MySpace{user_id}));
        // let on_my_space = ctx.link().callback(move |_| MenuMessage::AppRoute(AppRoute::MySpace{user_id}));

        let navigator = ctx.link().navigator().unwrap();
        let on_menu_home = Callback::from(move |_| navigator.push(&AppRoute::Home));
        
        // let on_menu_home = ctx
        //     .link()
        //     .callback(|_| MenuMessage::AppRoute(AppRoute::Home));
            
        let btn_sidebar_menu_show = if self.sidebar_menu_state {
            html! {
                <a class="btn-show-sidebar-menu" onclick={&on_show_sidebar}>
                    <span class="text-white">
                        <i class="fas fa-angle-double-left fas fa-2x" id="sidebar-menu-state"></i>
                    </span>
                </a>
            }
        } else {
            html! {
                <a class="btn-show-sidebar-menu" onclick={&on_show_sidebar}>
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

        let maybe_menu = ctx.props().user_profile.as_ref()
            .zip(ctx.props().school_profile.as_ref()
            .and_then(|school| Some(school.clone())))
            .and_then(|(item, school_profile)| {

                let grade_is_active = if self.school_state {
                    "bg-menu-option-active"
                } else {
                    "bg-menu-option-inactive"
                };

                let school_id = school_profile.school_id;

                let category = ClassGroupCategory::Posts;

                let navigator = ctx.link().navigator().unwrap();
                let on_menu_school_class_groups_student = Callback::from(move |_| navigator.push(&AppRoute::GroupSectionStudent{
                    school_id,
                    user_id,
                    category,
                }));

                let home_is_active = if self.home_state {
                    "bg-menu-option-active"
                } else {
                    "bg-menu-option-inactive"
                };
                let navigator = ctx.link().navigator().unwrap();
                let on_schools_staff = Callback::from(move |_| navigator.push(&AppRoute::Schools));

                let navigator = ctx.link().navigator().unwrap();
                let on_schools_teacher = Callback::from(move |_| navigator.push(&AppRoute::GradesByUserId{school_id}));
                let colleges_option = if item.user_staff.is_some() {
                    Some(html! {
                        <>
                            <a class={grade_is_active} onclick={&on_schools_staff}>
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
                            <a class={grade_is_active} onclick={&on_schools_teacher}>
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
                            <a class={grade_is_active} onclick={&on_menu_school_class_groups_student}>
                                <img src="/icons/graduation.svg"
                                    style="height: 17px;width: 26px;object-fit: contain;" />
                                <span class="links_name">{lang::dict("My section")}</span>
                            </a>
                            <span class="tooltip">{lang::dict("My section")}</span>
                        </>
                    })
                }
                .unwrap_or(html! {});

                let navigator = ctx.link().navigator().unwrap();
                let on_my_space = Callback::from(move |_| navigator.push(&AppRoute::MySpace{user_id}));

                let my_space_is_active = if self.myspace_state {
                    "bg-menu-option-active"
                } else {
                    "bg-menu-option-inactive"
                };

                let navigator = ctx.link().navigator().unwrap();
                let on_menu_meetings = Callback::from(move |_| navigator.push(&AppRoute::Meetings));

                let meetings_is_active = if self.meets_state {
                    "bg-menu-option-active"
                } else {
                    "bg-menu-option-inactive"
                };
                let maybe_meetings = if item.user_staff.is_some() || item.user_teacher.is_some() {
                    Some(html! {
                        <>
                            <a class={meetings_is_active} onclick={on_menu_meetings}>
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

                let class_sidebar_menu = if self.sidebar_menu_state {
                    "sidebar open card bg-primary-blue-light text-white vh-100 border-0 rounded-0 col col-sm-3 col-md-3 col-lg-2 col-xl-2 col-xxl-2 d-none d-sm-none d-md-none d-lg-block"
                } else {
                    "sidebar close-menu card bg-primary-blue-light text-white vh-100 border-0 rounded-0 col col-1 d-none d-sm-none d-md-none d-lg-block"
                };

                Some(html! {
                    <div class={class_sidebar_menu}>
                        <img src="/static/arte-2.png" class="card-img card-img-menu" alt="..." />
                        <div class="card-img-overlay blur-img-menu h-100 d-flex flex-column justify-content-between">
                            <div class="d-flex justify-content-between">
                                {maybe_roboxmaker_logo}
                                {btn_sidebar_menu_show}
                            </div> 
                            <ul class="nav-list mb-3">
                                <li>
                                    <a class={home_is_active} onclick={&on_menu_home}>
                                        <span class="ms-3">
                                            <i class="fas fa-home fas fa-lg"></i>
                                        </span>
                                        <span class="links_name ms-3">{lang::dict("Home")}</span>
                                    </a>
                                    <span class="tooltip">{lang::dict("Home")}</span>
                                </li>
                                <li>
                                    {colleges_option}
                                </li>
                                <li>
                                    <a class={my_space_is_active} onclick={&on_my_space}>
                                        <span class="ms-3">
                                            <i class="far fa-sticky-note fas fa-lg"></i>
                                        </span>
                                        <span class="links_name ms-3">{lang::dict("My space")}</span>
                                    </a>
                                    <span class="tooltip">{lang::dict("My space")}</span>
                                </li>
                                <li>
                                    {maybe_meetings}
                                </li>
                            </ul>
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
                                    <a onclick={&ctx.props().on_logout}>
                                        <span class="text-white ms-3">
                                            <i class="fas fa-sign-out-alt fas fa-lg"></i>
                                        </span>
                                        <span class="links_name ms-3">{lang::dict("Logout")}</span>
                                    </a>
                                    <span class="tooltip">{lang::dict("Logout")}</span>
                                </li>
                            </ul>
                        </div>
                    </div>
                })
            }).unwrap_or(html! {});
        let on_menu_toggle_mobile = ctx.link().callback(|_| MenuMessage::MenuToggleMobile);
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
                <a class={class_dropdown} data-bs-toggle="dropdown" role="button" aria-expanded="true" onmousedown={&on_menu_toggle_mobile}>
                    <span class="text-white">
                        <i class="fas fa-times" id="toggle-menu-mobile"></i>
                    </span>
                </a>
            }
        } else {
            html! {
                <a class={class_dropdown} data-bs-toggle="dropdown" role="button" aria-expanded="true" onmousedown={&on_menu_toggle_mobile}>
                    <span class="text-white">
                        <i class="fas fa-bars" id="toggle-menu-mobile"></i>
                    </span>
                </a>
            }
        };

        let navigator = ctx.link().navigator().unwrap();
        let on_menu_meetings = Callback::from(move |_| navigator.push(&AppRoute::Meetings));

        let menu_mobile = ctx.props().user_profile.as_ref()
            .and_then(|item| {
                let schools_option = if item.user_staff.is_some() {
                    html! {
                        <li>
                            <a class="dropdown-item bg-primary-blue-dark" onclick={&on_schools_staff}>
                                <img src="/icons/graduation.svg" style="height: 18px; width: 28px; object-fit: contain;" />
                                <span class="text-white ms-1">{lang::dict("Schools")}</span>
                            </a>
                        </li>
                    }
                } else if item.user_teacher.is_some() {
                    html! {
                        <li>
                            <a class="dropdown-item bg-primary-blue-dark" onclick={&on_schools_teacher}>
                                <img src="/icons/graduation.svg" style="height: 18px; width: 28px; object-fit: contain;" />
                                <span class="text-white ms-1">{lang::dict("Degrees")}</span>
                            </a>
                        </li>
                    }
                } else {
                    html! {
                        <li>
                            <a class="dropdown-item bg-primary-blue-dark" onclick={&on_my_section}>
                                <img src="/icons/graduation.svg" style="height: 18px; width: 28px; object-fit: contain;" />
                                <span class="text-white ms-1">{lang::dict("My section")}</span>
                            </a>
                        </li>
                    }
                };

                let meets = if item.user_staff.is_some() || item.user_teacher.is_some() {
                    html! {
                        <li>
                            <a class="dropdown-item bg-primary-blue-dark" onclick={&on_menu_meetings}>
                                <img src="/icons/video.svg" style="height: 26px; width: 30px; object-fit: contain;" />
                                <span class="text-white ms-1">{lang::dict("Meetings")}</span>
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
                                <a class="nav-link focus-none" aria-current="page" onclick={&on_menu_home}>
                                    <img src="/static/logo-robox-maker.png" style="height: 25px;" />
                                </a>
                                <a class="nav-link focus-none" aria-current="page" onclick={&on_menu_home}>
                                    <img src="/icons/home.svg" alt="" style="height: 23px;" />
                                </a>
                            </li>
                            <li class="nav-item dropdown">
                                {maybe_toggle_menu}
                                <ul class={class_dropdown_item} style="right: 0px;">
                                    {schools_option}
                                    <li>
                                        <a class="dropdown-item bg-primary-blue-dark" onclick={&on_my_space}>
                                            <span class="text-white is-size-22">
                                                <i class="far fa-sticky-note"></i>
                                            </span>
                                            <span class="text-white ms-1">{lang::dict("My space")}</span>
                                        </a>
                                    </li>
                                    {meets}
                                    <li><hr class="dropdown-divider" /></li>
                                    <li>
                                        <a class="dropdown-item bg-primary-blue-dark" onclick={&ctx.props().on_logout}>
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
