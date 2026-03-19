use log::*;
use uuid::Uuid;
use yew::prelude::*;
use yew_router::route::Route;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_post::posts_list::PostsList;
use roboxmaker_robot::robots_list::RobotsList;
use roboxmaker_quizzes::quizzes_list::QuizList;
use roboxmaker_lesson::lesson_list::LessonList;
// use roboxmaker_classes::classes_list::ClassesList;
use roboxmaker_teacher_resource::tr_list::TeacherResources;
use roboxmaker_models::{school_model, grade_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, SchoolId, AppRoute, ClassGroupCategory, UserId, MyUserProfile};

#[derive(Debug, Clone, PartialEq)]
pub struct ClassProfile {
    pub class_name: String,
    pub group_id: GroupId,
}

pub struct DegreeContentStudent {
    link: ComponentLink<Self>,
    props: DegreeContentStudentProps,
    category: ClassGroupCategory,
    categories: Vec<ClassGroupCategory>,
    grade_data_view: Vec<ClassProfile>,
    graphql_task: Option<GraphQLTask>,
    degree_data_task: Option<RequestTask>,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct DegreeContentStudentProps {
    pub user_id: UserId,
    pub category: ClassGroupCategory,
    pub route: Route<()>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_app_route: Callback<AppRoute>,
    pub school_id: SchoolId,
    pub saved_sidebar_state: bool,
}

#[derive(Debug)]
pub enum DegreeContentStudentMessage {
    AppRouteChanged(AppRoute),
    FetchClassGroups,
    ClassGroups(Option<grade_model::content_degree_by_user_id::ResponseData>),
    ChangeCategoryClass(ClassGroupCategory),
}

impl Component for DegreeContentStudent {
    type Message = DegreeContentStudentMessage;
    type Properties = DegreeContentStudentProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(DegreeContentStudentMessage::FetchClassGroups);

        DegreeContentStudent { 
            link, 
            props,
            category: ClassGroupCategory::Posts,
            categories: vec![
                ClassGroupCategory::Posts,
                ClassGroupCategory::Members,
                ClassGroupCategory::Robots,
                ClassGroupCategory::Lessons,
                // ClassGroupCategory::Classes,
                // ClassGroupCategory::TeacherResources,
                ClassGroupCategory::Quizzes,
            ],
            grade_data_view: vec![],
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            degree_data_task: None
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            DegreeContentStudentMessage::AppRouteChanged(route) => self.props.on_app_route.emit(route),
            DegreeContentStudentMessage::FetchClassGroups => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = grade_model::content_degree_by_user_id::Variables {
                        school_id: self.props.school_id.0,
                        limit: 1,
                        user_id: self.props.user_id.0, 
                    };
                    let task = grade_model::ContentDegreeByUserId::request(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            DegreeContentStudentMessage::ClassGroups(response)
                        }
                    );
                    self.degree_data_task = Some(task);
                }
            },
            DegreeContentStudentMessage::ClassGroups(class_group_data) => {
                self.grade_data_view = class_group_data
                    .and_then(|data| Some(data.class_group))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|class_group| {
                        let class_name = class_group.class_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        let group_id = class_group.group_id;
                        ClassProfile {
                            class_name: class_name,
                            group_id: GroupId(group_id),
                        }
                    })
                    .collect();
            }
            DegreeContentStudentMessage::ChangeCategoryClass(category) => {
                self.category = category;
            }
        }
        should_update
    }
    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        trace!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props != props {
            self.props = props;
            should_render = true;
        }

        should_render
    }
    fn view(&self) -> Html {

        let props_category = self.props.category;
        let class_group_posts = |class_group: &ClassProfile| {
            html! {
                <PostsList group_id=class_group.group_id user_profile=self.props.user_profile.clone()
                    auth_school=self.props.auth_school.clone() on_app_route=self.props.on_app_route.clone()
                    class_name=class_group.class_name.clone() school_id=self.props.school_id />
            }
        };
        let class_group_members = |_class_group: &ClassProfile| {
            html! {
                // <UserList user_profile=self.props.user_profile.clone() group_id=class_group.group_id
                //     on_app_route=self.props.on_app_route.clone()
                //     class_name=class_group.class_name.clone()
                //     saved_sidebar_state=self.props.saved_sidebar_state.clone() />
            }
        };
        let class_group_robots = |class_group: &ClassProfile| {
            html! {
                <RobotsList user_id=None group_id=class_group.group_id
                    user_profile=self.props.user_profile.clone() on_app_route=self.props.on_app_route.clone()
                    class_name=class_group.class_name.clone() />
            }
        };
        let class_group_lessons = |class_group: &ClassProfile| {
            let class_name = class_group.class_name.clone().to_uppercase();
            if class_name.contains("KINDER") 
                || class_name.contains("PREPARATORIA") {
                html! {}
            } else {
                html! {
                    <LessonList group_id=class_group.group_id school_id=self.props.school_id
                        user_profile=self.props.user_profile.clone() auth_school=self.props.auth_school.clone()
                        on_app_route=self.props.on_app_route.clone()
                        class_name=class_group.class_name.clone() />
                }
            }
        };
        // let class_group_classes = |class_group: &ClassProfile| {
        //     html! {
        //         <ClassesList group_id=class_group.group_id school_id=self.props.school_id
        //             user_profile=self.props.user_profile.clone() auth_school=self.props.auth_school.clone()
        //             on_app_route=self.props.on_app_route.clone()
        //             class_name=class_group.class_name.clone() />
        //     }
        // };
        let class_group_resources = |class_group: &ClassProfile| {
            html! {
                <TeacherResources group_id=class_group.group_id school_id=self.props.school_id
                    user_profile=self.props.user_profile.clone() auth_school=self.props.auth_school.clone()
                    on_app_route=self.props.on_app_route.clone()
                    class_name=class_group.class_name.clone() />
            }
        };
        let class_group_quizzes = |class_group: &ClassProfile| {
            html! {
                <QuizList group_id=class_group.group_id school_id=self.props.school_id
                    user_profile=self.props.user_profile.clone() auth_school=self.props.auth_school.clone()
                    on_app_route=self.props.on_app_route.clone()
                    class_name=class_group.class_name.clone() />
            }
        };
        let class_group_level = |class_group: &ClassProfile| {
            let school_id = self.props.school_id;
            let user_id = self.props.user_profile.clone().and_then(|user_by_pk| Some(user_by_pk.user_id)).unwrap_or(UserId(Uuid::default()));

            let class_name = class_group.class_name.clone().to_uppercase();

            let class_group_category_desktop = |&category| {
                let on_click_category = self.link.callback(move |_| 
                    DegreeContentStudentMessage::AppRouteChanged(AppRoute::GroupSectionStudent(school_id, user_id, category)));
                let is_active = if category == props_category {
                    "navbar-desktop"
                } else {
                    "inactive-navbar-btn-desktop"
                };
                match category {
                    ClassGroupCategory::Posts => html! {
                        <a onclick=on_click_category class="d-flex justify-content-center">
                            <li class=is_active>
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
                    },
                    ClassGroupCategory::Robots => html! {
                        <a onclick=on_click_category class="d-flex justify-content-center">
                            <li class=is_active>
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                                    <span class="icon">
                                        <img src="/icons/robot-2.svg" style="height: 22px;" />
                                    </span>
                                    <span class="ps-2">{lang::dict("Robots")}</span>
                                </span>
                            </li>
                        </a>
                    },
                    ClassGroupCategory::Lessons => if class_name.contains("KINDER") || class_name.contains("PREPARATORIA") {
                        html! {}
                    } else {
                        html! {
                            <a onclick=on_click_category class="d-flex justify-content-center">
                                <li class=is_active>
                                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                                        <span class="icon">
                                            <i class="far fa-file-alt"></i>
                                        </span>
                                        <span class="ps-2">{lang::dict("Lessons")}</span>
                                    </span>
                                </li>
                            </a>
                        }
                    } 
                    // ClassGroupCategory::Classes => html! {
                    //     <a onclick=on_click_category class="d-flex justify-content-center">
                    //         <li class=is_active>
                    //             <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                    //                 <span class="icon">
                    //                     <img src="/icons/folders-2.svg" style="height: 22px;" />
                    //                 </span>
                    //                 <span class="ps-2">{lang::dict("Classes")}</span>
                    //             </span>
                    //         </li>
                    //     </a>
                    // },
                    ClassGroupCategory::TeacherResources => html! {
                        // <a onclick=on_click_category class="d-flex justify-content-center">
                        //     <li class=is_active>
                        //         <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                        //             <span class="icon">
                        //                 // <img src="/icons/folders-2.svg" style="height: 22px;" />
                        //                 <i class="fas fa-laptop"></i>
                        //             </span>
                        //             <span class="ps-2">{lang::dict("Teacher resources")}</span>
                        //         </span>
                        //     </li>
                        // </a>
                    },
                    ClassGroupCategory::Quizzes => html! {
                        <a onclick=on_click_category class="d-flex justify-content-center">
                            <li class=is_active>
                                <span class="text-primary-blue-dark noir-bold is-size-18 lh-22 d-flex align-items-center">
                                    <span class="icon">
                                        <i class="fas fa-list-ol fa-lg"></i>
                                    </span>
                                    <span class="ps-2">{"Evaluaciones"}</span>
                                </span>
                            </li>
                        </a>
                    },
                }
            };
            let mobile_class_group_category = |&category| {
                let user_id = self.props.user_profile.clone().and_then(|user_by_pk| Some(user_by_pk.user_id)).unwrap_or(UserId(Uuid::default()));

                let on_click_category = self.link.callback(move |_| 
                    DegreeContentStudentMessage::AppRouteChanged(AppRoute::GroupSectionStudent(school_id, user_id, category)));
                let is_active = if category == props_category {
                    "nav-link bg-cyan-turquesa text-primary-blue-dark text-center p-2"
                } else {
                    "nav-link text-primary-blue-dark text-center p-2"
                };
                match category {
                    ClassGroupCategory::Posts => html! {
                        <li class="nav-item">
                            <a class=is_active aria-current="page" onclick=on_click_category>
                                <img src="/icons/envelope-open-text.svg" style="height: 22px;" />
                                <span class="ps-2">{lang::dict("Posts")}</span>
                            </a>
                        </li>
                    },
                    ClassGroupCategory::Members => html! {
                    },
                    ClassGroupCategory::Robots => html! {
                        <li class="nav-item">
                            <a class=is_active aria-current="page" onclick=on_click_category>
                                <img src="/icons/robot-2.svg" style="height: 22px;" />
                                <span class="ps-2">{lang::dict("Robots")}</span>
                            </a>
                        </li>
                    },
                    // ClassGroupCategory::Lessons => html! {
                    //     <li class="nav-item">
                    //         <a class=is_active aria-current="page" onclick=on_click_category>
                    //             <i class="far fa-file-alt fa-lg text-primary-blue-dark"></i>
                    //             <span class="ps-2">{lang::dict("Lessons")}</span>
                    //         </a>
                    //     </li>
                    // },
                    ClassGroupCategory::Lessons => if class_name.contains("KINDER") || class_name.contains("PREPARATORIA") {
                        html! {}
                    } else {
                        html! {
                            <li class="nav-item">
                                <a class=is_active aria-current="page" onclick=on_click_category>
                                    <i class="far fa-file-alt fa-lg text-primary-blue-dark"></i>
                                    <span class="ps-2">{lang::dict("Lessons")}</span>
                                </a>
                            </li>
                        }
                    },
                    // ClassGroupCategory::Classes => html! {
                    //     <li class="nav-item">
                    //         <a class=is_active aria-current="page" onclick=on_click_category>
                    //             <img src="/icons/folders-2.svg" style="height: 22px;" />
                    //             <span class="ps-2">{lang::dict("Classes")}</span>
                    //         </a>
                    //     </li>
                    // },
                    ClassGroupCategory::TeacherResources => html! {
                        // <li class="nav-item">
                        //     <a class=is_active aria-current="page" onclick=on_click_category>
                        //         // <img src="/icons/folders-2.svg" style="height: 22px;" />
                        //         <i class="fas fa-laptop fa-lg text-primary-blue-dark"></i>
                        //         <span class="ps-2">{lang::dict("Teacher resources")}</span>
                        //     </a>
                        // </li>
                    },
                    ClassGroupCategory::Quizzes => html! {
                        <li class="nav-item">
                            <a class=is_active aria-current="page" onclick=on_click_category>
                                <i class="fas fa-list-ol fa-lg text-primary-blue-dark"></i>
                                <span class="ps-2">{"Evaluaciones"}</span>
                            </a>
                        </li>
                    },
                }
            };
            let school_id = self.props.school_id;
            let on_schools_staff = self.link.callback(move |_| DegreeContentStudentMessage::AppRouteChanged(AppRoute::GradesBySchoolId(school_id)));
            let on_schools_teacher = self.link.callback(move |_| DegreeContentStudentMessage::AppRouteChanged(AppRoute::GradesByUserId(school_id)));
            let go_to_degrees = self
                .props
                .user_profile
                .as_ref()
                .and_then(|user_auth| {
                    if user_auth.user_staff.is_some() {
                        Some(html! {
                            <a onclick=&on_schools_staff>
                                <span class="text-purple-gray noir-bold is-size-16 lh-19">
                                    <i class="fas fa-arrow-left me-2"></i>
                                    <span>{lang::dict("To Degrees")}</span>
                                </span>
                            </a>
                        })
                    } else if user_auth.user_teacher.is_some() {
                        Some(html! {
                            <a onclick=&on_schools_teacher>
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
                // ClassGroupCategory::Classes => class_group_classes(class_group),
                ClassGroupCategory::TeacherResources => class_group_resources(class_group),
                ClassGroupCategory::Quizzes => class_group_quizzes(class_group),
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
