use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_main::lang;
use roboxmaker_models::grade_model;
use roboxmaker_user::users_list::UserList;
use roboxmaker_post::posts_list::PostsList;
use roboxmaker_robot::robots_list::RobotsList;
use roboxmaker_lesson::lesson_list::LessonList;
use roboxmaker_utils::functions::user_profile_data;
use roboxmaker_classes::classes_list::ClassesList;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, SchoolId, AppRoute, ClassGroupCategory, MyUserProfile};

#[derive(Debug, Clone, PartialEq)]
pub struct ClassProfile {
    pub class_name: String,
    pub group_id: GroupId,
    pub inventoty_group: Option<Uuid>,
}

pub struct DegreeContent {
    graphql_task: Option<GraphQLTask>,
    degree_data_task: Option<RequestTask>,
    category: ClassGroupCategory,
    categories: Vec<ClassGroupCategory>,
    grade_data_view: Vec<ClassProfile>,
    user_profile: Option<MyUserProfile>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct DegreeContentProps {
    pub group_id: GroupId,
    pub category: ClassGroupCategory,
    pub school_id: SchoolId,
}

#[derive(Debug)]
pub enum DegreeContentMessage {
    // AppRouteChanged(AppRoute),
    FetchClassGroups,
    ClassGroups(Option<grade_model::degree_content_by_id::ResponseData>),
    ChangeCategoryClass(ClassGroupCategory),
}

impl Component for DegreeContent {
    type Message = DegreeContentMessage;
    type Properties = DegreeContentProps;

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(DegreeContentMessage::FetchClassGroups);

        let user_profile = user_profile_data();

        roboxmaker_utils::functions::school_state();

        DegreeContent { 
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            degree_data_task: None,
            category: ClassGroupCategory::Posts,
            categories: vec![
                ClassGroupCategory::Posts,
                ClassGroupCategory::Members,
                ClassGroupCategory::Robots,
                ClassGroupCategory::Lessons,
                ClassGroupCategory::Classes,
            ],
            grade_data_view: vec![],
            user_profile,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            // DegreeContentMessage::AppRouteChanged(route) => ctx.props().on_app_route.emit(route),
            DegreeContentMessage::FetchClassGroups => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = grade_model::degree_content_by_id::Variables {
                        school_id: ctx.props().school_id.0,
                        group_id: ctx.props().group_id.0, 
                    };
                    
                    let task = grade_model::DegreeContentById::request(
                        graphql_task, 
                        &ctx, 
                        vars, 
                        |response| {
                            DegreeContentMessage::ClassGroups(response)
                        }
                    );
                    self.degree_data_task = Some(task);
                }
            },
            DegreeContentMessage::ClassGroups(response) => {

                self.grade_data_view = response
                    .clone()
                    .and_then(|data| Some(data.class_group))
                    .unwrap_or(vec![])
                    .iter()
                    .zip(response
                            .clone()
                            .and_then(|data| Some(data.inventory_group))
                            .unwrap_or(vec![]))
                    .map(|(class_group, inventoty)| {
                        let class_name = class_group.class_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        let group_id = class_group.group_id;
                        ClassProfile {
                            class_name,
                            group_id: GroupId(group_id),
                            inventoty_group: Some(inventoty.group_id),
                        }
                    })
                    .collect();
            }
            DegreeContentMessage::ChangeCategoryClass(category) => {
                self.category = category;
            }
        }
        should_update
    }
    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // trace!("{:?} => {:?}", ctx.props(), old_props);
        
        self.user_profile = user_profile_data();
        
        ctx.props() != old_props
    }
    fn view(&self, ctx: &Context<Self>) -> Html {

        let props_category = ctx.props().category;


        let class_group_posts = |class_group: &ClassProfile| {
            html! {
                <PostsList group_id={class_group.group_id }
                    inventory_group={class_group.inventoty_group.clone()}
                    user_profile={self.user_profile.clone()} 
                    class_name={class_group.class_name.clone()} 
                    school_id={ctx.props().school_id} />
            }
        };
        let class_group_members = |class_group: &ClassProfile| {
            html! {
                <UserList user_profile={self.user_profile.clone()} 
                    group_id={class_group.group_id}
                    class_name={class_group.class_name.clone()} />
            }
        };
        let class_group_robots = |class_group: &ClassProfile| {
            html! {
                <RobotsList user_id={None} group_id={class_group.group_id}
                    user_profile={self.user_profile.clone()} 
                    class_name={class_group.class_name.clone()} />
            }
        };
        let class_group_lessons = |class_group: &ClassProfile| {
            html! {
                <LessonList group_id={class_group.group_id} 
                    school_id={ctx.props().school_id}
                    user_profile={self.user_profile.clone()} 
                    inventory_group={class_group.inventoty_group.clone()} 
                    class_name={class_group.class_name.clone()} />
            }
        };
        let class_group_classes = |class_group: &ClassProfile| {
            html! {
                <ClassesList group_id={class_group.group_id} 
                    school_id={ctx.props().school_id}
                    user_profile={self.user_profile.clone()}
                    inventory_group={class_group.inventoty_group.clone()} 
                    class_name={class_group.class_name.clone()} />
            }
        };
        let class_group_level = |class_group: &ClassProfile| {
            let group_id = class_group.group_id;
            let school_id = ctx.props().school_id;

            
            let class_group_category_desktop = |&category| {
                let navigator = ctx.link().navigator().unwrap();

                let props_category = ctx.props().category;

                let on_click_category = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
                let is_active = if category == props_category {
                    "navbar-desktop"
                } else {
                    "inactive-navbar-btn-desktop"
                };
                match category {
                    ClassGroupCategory::Posts => html! {
                        <a onclick={on_click_category} class="d-flex justify-content-center">
                            <li class={is_active}>
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                                    <span class="icon">
                                        <img src="/icons/envelope-open-text.svg" style="height: 22px;" />
                                    </span>
                                    <span class="ps-2">{lang::dict("Posts")}</span>
                                </span>
                            </li>
                        </a>
                    },
                    ClassGroupCategory::Members => html! {
                        <a onclick={on_click_category} class="d-flex justify-content-center">
                            <li class={is_active}>
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                                    <span class="icon">
                                        <img src="/icons/user-class-2.svg" style="height: 22px;" />
                                    </span>
                                    <span class="ps-2">{lang::dict("Members")}</span>
                                </span>
                            </li>
                        </a>
                    },
                    ClassGroupCategory::Robots => html! {
                        <a onclick={on_click_category} class="d-flex justify-content-center">
                            <li class={is_active}>
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                                    <span class="icon">
                                        <img src="/icons/robot-2.svg" style="height: 22px;" />
                                    </span>
                                    <span class="ps-2">{lang::dict("Robots")}</span>
                                </span>
                            </li>
                        </a>
                    },
                    ClassGroupCategory::Lessons => html! {
                        <a onclick={on_click_category} class="d-flex justify-content-center">
                            <li class={is_active}>
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                                    <span class="icon">
                                        <i class="far fa-file-alt"></i>
                                    </span>
                                    <span class="ps-2">{lang::dict("Lessons")}</span>
                                </span>
                            </li>
                        </a>
                    },
                    ClassGroupCategory::Classes => html! {
                        <a onclick={on_click_category} class="d-flex justify-content-center">
                            <li class={is_active}>
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                                    <span class="icon">
                                        <img src="/icons/folders-2.svg" style="height: 22px;" />
                                    </span>
                                    <span class="ps-2">{lang::dict("Classes")}</span>
                                </span>
                            </li>
                        </a>
                    },
                }
            };
            let mobile_class_group_category = |&category| {

                let navigator = ctx.link().navigator().unwrap();

                let props_category = ctx.props().category;


                let on_click_category = Callback::from(move |_| navigator.push(&AppRoute::SchoolGroupSection{school_id, group_id, category}));
                
                let is_active = if category == props_category {
                    "nav-link bg-cyan-turquesa text-primary-blue-dark text-center p-2"
                } else {
                    "nav-link text-primary-blue-dark text-center p-2"
                };
                match category {
                    ClassGroupCategory::Posts => html! {
                        <li class="nav-item">
                            <a class={is_active} aria-current="page" onclick={on_click_category}>
                                <img src="/icons/envelope-open-text.svg" style="height: 22px;" />
                                <span class="ps-2">{lang::dict("Posts")}</span>
                            </a>
                        </li>
                    },
                    ClassGroupCategory::Members => html! {
                        <li class="nav-item">
                            <a class={is_active} aria-current="page" onclick={on_click_category}>
                                <img src="/icons/user-class-2.svg" style="height: 22px;" />
                                <span class="ps-2">{lang::dict("Members")}</span>
                            </a>
                        </li>
                    },
                    ClassGroupCategory::Robots => html! {
                        <li class="nav-item">
                            <a class={is_active} aria-current="page" onclick={on_click_category}>
                                <img src="/icons/robot-2.svg" style="height: 22px;" />
                                <span class="ps-2">{lang::dict("Robots")}</span>
                            </a>
                        </li>
                    },
                    ClassGroupCategory::Lessons => html! {
                        <li class="nav-item">
                            <a class={is_active} aria-current="page" onclick={on_click_category}>
                                <i class="far fa-file-alt fa-lg text-primary-blue-dark"></i>
                                <span class="ps-2">{lang::dict("Lessons")}</span>
                            </a>
                        </li>
                    },
                    ClassGroupCategory::Classes => html! {
                        <li class="nav-item">
                            <a class={is_active} aria-current="page" onclick={on_click_category}>
                                <img src="/icons/folders-2.svg" style="height: 22px;" />
                                <span class="ps-2">{lang::dict("Classes")}</span>
                            </a>
                        </li>
                    },
                }
            };
            let school_id = ctx.props().school_id;

            let navigator = ctx.link().navigator().unwrap();
            let on_schools_staff = Callback::from(move |_| navigator.push(&AppRoute::GradesBySchoolId{school_id}));

            let navigator_two = ctx.link().navigator().unwrap();
            let on_schools_teacher = Callback::from(move |_| navigator_two.push(&AppRoute::GradesByUserId{school_id}));

            // let on_schools_staff = ctx.link().callback(move |_| DegreeContentMessage::AppRouteChanged(AppRoute::GradesBySchoolId{school_id}));
            // let on_schools_teacher = ctx.link().callback(move |_| DegreeContentMessage::AppRouteChanged(AppRoute::GradesByUserId{school_id}));
            let go_to_degrees = self.user_profile
                .clone()
                .and_then(|user_auth| {
                    if user_auth.user_staff.is_some() {
                        Some(html! {
                            <a onclick={&on_schools_staff}>
                                <span class="text-purple-gray noir-bold is-size-16 lh-19">
                                    <i class="fas fa-arrow-left me-2"></i>
                                    <span>{lang::dict("To Degrees")}</span>
                                </span>
                            </a>
                        })
                    } else if user_auth.user_teacher.is_some() {
                        Some(html! {
                            <a onclick={&on_schools_teacher}>
                                <span class="text-purple-gray noir-bold is-size-16 lh-19">
                                    <i class="fas fa-arrow-left me-2"></i>
                                    <span>{lang::dict("To Degrees")}</span>
                                </span>
                            </a>
                        })
                    } else {
                        None
                    }
                })
                .unwrap_or(html! {});
            let maybe_categories = html! {
                <>
                    <div class="menu p-7 d-none d-sm-none d-md-none d-lg-block">
                        <div class="mt-7">{go_to_degrees}</div>
                        <ul class="menu-list-option">
                            {self.categories.iter().map(class_group_category_desktop).collect::<Html>()}
                        </ul>
                    </div>
                    <div class="mt-4">
                        <ul class="nav nav-tabs bg-silver justify-content-center d-lg-none">
                            {self.categories.iter().map(mobile_class_group_category).collect::<Html>()}
                        </ul>
                    </div>
                </>
            };
            let class_group_body = |class_group: &ClassProfile| match props_category {
                ClassGroupCategory::Posts => class_group_posts(class_group),
                ClassGroupCategory::Members => class_group_members(class_group),
                ClassGroupCategory::Robots => class_group_robots(class_group),
                ClassGroupCategory::Lessons => class_group_lessons(class_group),
                ClassGroupCategory::Classes => class_group_classes(class_group),
            };
            html! {
                <>
                    <div class="w-100 h-100 d-flex flex-column flex-lg-row">
                        {maybe_categories}
                        {class_group_body(class_group)}
                    </div>
                </>
            }
        };
        html! {
            {
                self.grade_data_view
                    .iter()
                    .map(|class_group| {
                        class_group_level(&class_group)
                    }).collect::<Html>()
            }
        }
    }
}
