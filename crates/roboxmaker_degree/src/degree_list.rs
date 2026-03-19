use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use serde_derive::{Deserialize, Serialize};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::{school_model, grade_model};
use roboxmaker_searches::search_degree_list::SearchDegreeList;
use roboxmaker_loaders::fullscreen_loader_degree::FullScreenLoaderDegree;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, SchoolId, UserId, AppRoute, ClassGroupCategory, MyUserProfile};


#[derive(Debug, Clone)]
enum LoadClassesFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadClasses {
    Loading,
    Load(LoadClassesFound),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchoolProfile {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListDegreesByShoolId {
    pub class_name: String,
    pub group_id: Uuid,
    pub school_id: Uuid,
    pub robots_group: usize,
    pub posts_group: usize,
    pub classes_group: usize,
    pub members_group: usize,
    pub view_degree: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchoolProfileUser {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ListDegreesByUserId {
    pub class_name: String,
    pub group_id: Uuid,
    pub school_id: Uuid,
    pub robots_group: usize,
    pub posts_group: usize,
    pub classes_group: usize,
    pub members_group: usize,
    pub view_degree: bool,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ListOfGradesFilter {
    UserGroups(UserId),
    SchoolGroups,
}

pub struct DegreeList {
    link: ComponentLink<Self>,
    props: DegreeListProps,
    graphql_task: Option<GraphQLTask>,
    degree_list_task: Option<RequestTask>,
    degree_delete_task: Option<RequestTask>,
    degrees_by_school_id: Vec<ListDegreesByShoolId>,
    school_profile: Vec<SchoolProfile>,
    degrees_by_user_id: Vec<ListDegreesByUserId>,
    school_profile_user: Vec<SchoolProfileUser>,
    list_classes_state: LoadClasses,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct DegreeListProps {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub school_id: SchoolId,
    pub filter: ListOfGradesFilter,
}

#[derive(Debug)]
pub enum DegreeListMessage {
    AppRoute(AppRoute),
    FetchGradesGroup(ListOfGradesFilter),
    GradesBySchoolId(Option<grade_model::list_of_grades_of_school_by_id::ResponseData>),
    GradesByUserId(Option<grade_model::list_of_grades_of_user_by_id::ResponseData>),
    DeleteClassGroup(GroupId),
    Response(Option<grade_model::delete_class_group_by_id::ResponseData>),
}

impl Component for DegreeList {
    type Message = DegreeListMessage;
    type Properties = DegreeListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(DegreeListMessage::FetchGradesGroup(props.filter));
        DegreeList { 
            link, 
            props, 
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            degree_list_task: None,
            degree_delete_task: None,
            degrees_by_school_id: vec![],
            school_profile: vec![],
            degrees_by_user_id: vec![],
            school_profile_user: vec![],
            list_classes_state: LoadClasses::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            DegreeListMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route.clone());
            }
            DegreeListMessage::FetchGradesGroup(filter) => match filter {
                ListOfGradesFilter::UserGroups(user_id) => {
                    self.list_classes_state = LoadClasses::Loading;
                    let school_id = self.props.school_id;
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = grade_model::list_of_grades_of_user_by_id::Variables {
                            user_id: user_id.0,
                            school_id: school_id.0,
                        };
                        let task = grade_model::ListOfGradesOfUserById::request(
                            graphql_task, 
                            &self.link, 
                            vars, 
                            |response| {
                                DegreeListMessage::GradesByUserId(response)
                            }
                        );
                        self.degree_list_task = Some(task);
                    }
                }
                ListOfGradesFilter::SchoolGroups => {
                    self.list_classes_state = LoadClasses::Loading;
                    let school_id = self.props.school_id;
                    if let Some(graphql_task) = self.graphql_task.as_mut() {
                        let vars = grade_model::list_of_grades_of_school_by_id::Variables {
                            school_id: school_id.0, 
                        };
                        let task = grade_model::ListOfGradesOfSchoolById::request(
                            graphql_task, 
                            &self.link, 
                            vars, 
                            |response| {
                                DegreeListMessage::GradesBySchoolId(response)
                            }
                        );
                        self.degree_list_task = Some(task);
                    }
                }
            }
            DegreeListMessage::GradesBySchoolId(response) => {
                self.degrees_by_school_id = response
                    .clone()
                    .and_then(|data| Some(data.class_group))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|class_group| {
                        let class_name = class_group.class_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        let group_id = class_group.group_id;
                        let school_id = class_group.school_group.clone().and_then(|data| Some(data.school_id)).unwrap_or(Uuid::default());
                        let robots_group = class_group.robot_groups.iter().map(|data| data).len();
                        let posts_group = class_group.post_groups.iter().map(|data| data).len();
                        let classes_group = class_group.classes_groups.iter().map(|data| data).len();
                        let members_group = class_group.group_members.iter().map(|data| data).len();
                        ListDegreesByShoolId {
                            class_name: class_name,
                            group_id: group_id,
                            school_id: school_id,
                            robots_group: robots_group,
                            posts_group: posts_group,
                            classes_group: classes_group,
                            members_group: members_group,
                            view_degree: false,
                        }
                    }).collect();
                self.school_profile = response
                    .clone()
                    .and_then(|data| Some(data.school))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|school_profile| {
                        let name = school_profile.school_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        SchoolProfile {
                            name: name,
                        }
                    }).collect();
                if !response.clone().and_then(|data| Some(data.class_group)).unwrap_or(vec![]).is_empty() {
                    self.list_classes_state = LoadClasses::Load(LoadClassesFound::Found);
                } else {
                    self.list_classes_state = LoadClasses::Load(LoadClassesFound::NotFound);
                }
            }
            DegreeListMessage::GradesByUserId(response) => {
                self.degrees_by_user_id = response
                    .clone()
                    .and_then(|data| Some(data.class_group))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|class_group| {
                        let class_name = class_group.class_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        let group_id = class_group.group_id;
                        let school_id = class_group.school_group.clone().and_then(|data| Some(data.school_id)).unwrap_or(Uuid::default());
                        let robots_group = class_group.robot_groups.iter().map(|data| data).len();
                        let posts_group = class_group.post_groups.iter().map(|data| data).len();
                        let classes_group = class_group.classes_groups.iter().map(|data| data).len();
                        let members_group = class_group.group_members.iter().map(|data| data).len();
                        ListDegreesByUserId {
                            class_name: class_name,
                            group_id: group_id,
                            school_id: school_id,
                            robots_group: robots_group,
                            posts_group: posts_group,
                            classes_group: classes_group,
                            members_group: members_group,
                            view_degree: false,
                        }
                    }).collect();
                self.school_profile_user = response
                    .clone()
                    .and_then(|data| Some(data.school))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|school_profile| {
                        let name = school_profile.school_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        SchoolProfileUser {
                            name: name,
                        }
                    }).collect();
                if !response.clone().and_then(|data| Some(data.class_group)).unwrap_or(vec![]).is_empty() {
                    self.list_classes_state = LoadClasses::Load(LoadClassesFound::Found);
                } else {
                    self.list_classes_state = LoadClasses::Load(LoadClassesFound::NotFound);
                }
            }
            DegreeListMessage::DeleteClassGroup(group_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = grade_model::delete_class_group_by_id::Variables {
                        group_id: group_id.0, 
                    };
                    let task = grade_model::DeleteClassGroupById::request(
                        graphql_task, 
                        &self.link, 
                        vars, 
                        |response| {
                            DegreeListMessage::Response(response)
                        }
                    );
                    self.degree_delete_task = Some(task);
                }
            }
            DegreeListMessage::Response(_resp) => {}
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

        let maybe_user_profile_name = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                Some(html! {
                    {&item.full_name}
                })
            })
            .unwrap_or(html! {});

        let school_name = self
            .school_profile
            .iter()
            .map(|school| {
                let name = school.name.clone();
                html! {
                    <div class="d-flex flex-column pt-4">
                        <div class="d-flex flex-wrap mb-2">
                            <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">{lang::dict("Degrees -")}</h1>
                            <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0 ps-2">{name.clone()}</h1>
                        </div>
                        <div class="d-flex flex-wrap pb-4">
                            <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("These are the grades of the school")}</span>
                            <span class="text-brown noir-regular is-size-18 lh-22 ps-2">{name.clone()} </span>
                        </div>
                    </div>
                }
            }).collect::<Html>();

        let school_name_user = self
            .school_profile_user
            .iter()
            .map(|school| {
                let name = school.name.clone();
                html! {
                    <div class="d-flex flex-column pt-4">
                        <div class="d-flex flex-wrap mb-2">
                            <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">{lang::dict("Degrees -")}</h1>
                            <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0 ps-2">{name.clone()}</h1>
                        </div>
                        <div class="d-flex flex-wrap pb-4">
                            <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("These are the grades of the school")}</span>
                            <span class="text-brown noir-regular is-size-18 lh-22 ps-2">{name.clone()} </span>
                        </div>
                    </div>
                }
            }).collect::<Html>();
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
        let list_of_grades_by_school_id = self.degrees_by_school_id.iter().map(|class_group| {
                let group_id = class_group.group_id;
                let school_id = class_group.school_id;
                let on_grade_view = self.link.callback(move |_| {
                    DegreeListMessage::AppRoute(AppRoute::SchoolGroupSection(
                        SchoolId(school_id.clone()),
                        GroupId(group_id.clone()),
                        ClassGroupCategory::Posts,
                    ))
                });
                html! {
                    <div class="me-sm-2 me-md-2 me-lg-6 my-1 my-lg-4">
                        <div class="card-class-sections bg-white">
                            <div class="px-5 pt-5">
                                <span class="icon-text d-flex justify-content-between">
                                    <span class="text-primary-blue-light noir-bold is-size-18 lh-22">{&class_group.class_name}</span>
                                    <span class="text-gray-purple noir-regular is-size-14 lh-18">
                                        <i class="fas fa-ellipsis-v"></i>
                                    </span>
                                </span>
                                <span class="d-flex align-items-center text-gray-purple noir-regular is-size-14 lh-18 mt-1">
                                    <span class="me-3">
                                        <i class="fas fa-user-tie fas"></i>
                                    </span>
                                    {maybe_user_profile_name.clone()}
                                </span>
                            </div>
                            <hr class="hr-degrees" />
                            <div class="d-flex justify-content-between px-5">
                                <div class="d-flex flex-column">
                                    <span class="text-purple-gray d-flex align-items-center pb-2">
                                        <img src="/icons/user-class.svg" style="height: 22px;" />
                                        <span class="noir-regular is-size-14 lh-18">
                                            <span class="px-2">{&class_group.members_group}</span>
                                            {lang::dict("Members")}
                                        </span>
                                    </span>
                                    <span class="text-purple-gray d-flex align-items-center">
                                        <i class="far fa-file-alt"></i>
                                        <span class="noir-regular is-size-14 lh-18">
                                            <span class="px-2">{&class_group.posts_group}</span>
                                            {lang::dict("Posts")}
                                        </span>
                                    </span>
                                </div>
                                <div class="d-flex flex-column">
                                    <span class="text-purple-gray d-flex align-items-center pb-2">
                                        <img src="/icons/robot.svg" style="height: 18px;" />
                                        <span class="noir-regular is-size-14 lh-18"><span
                                                class="px-2">{&class_group.robots_group}</span>{lang::dict("Robots")}</span>
                                    </span>
                                    // <span class="text-purple-gray d-flex align-items-center">
                                    //     <img src="/icons/folders.svg" style="height: 18px;" />
                                    //     <span class="noir-regular is-size-14 lh-18"><span
                                    //             class="px-2">{&class_group.classes_group}</span>{lang::dict("Classes")}</span>
                                    // </span>
                                </div>
                            </div>
                            <div class="text-center mt-5">
                                <a class="button bg-white" onmousedown=on_grade_view>
                                    <span class="text-secondary-purple noir-bold is-size-16 lh-20">{lang::dict("See Degree")}</span>
                                </a>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();
        let list_of_grades_by_user_id = self.degrees_by_user_id.iter().map(|class_group| {
                let group_id = class_group.group_id;
                let school_id = class_group.school_id;
                let on_grade_view = self.link.callback(move |_| {
                    DegreeListMessage::AppRoute(AppRoute::SchoolGroupSection(
                        SchoolId(school_id.clone()),
                        GroupId(group_id.clone()),
                        ClassGroupCategory::Posts,
                    ))
                });
                html! {
                    <div class="me-sm-2 me-md-2 me-lg-6 my-1 my-lg-4">
                        <div class="card-class-sections bg-white">
                            <div class="px-5 pt-5">
                                <span class="icon-text d-flex justify-content-between">
                                    <span class="text-primary-blue-light noir-bold is-size-18 lh-22">{&class_group.class_name}</span>
                                    <span class="text-gray-purple noir-regular is-size-14 lh-18">
                                        <i class="fas fa-ellipsis-v"></i>
                                    </span>
                                </span>
                                <span class="icon-text d-flex align-items-center text-gray-purple noir-regular is-size-14 lh-18 mt-1">
                                    <span class="me-3">
                                        <i class="fas fa-user-tie fas"></i>
                                    </span>
                                    {maybe_user_profile_name.clone()}
                                </span>
                            </div>
                            <hr class="hr-degrees" />
                            <div class="d-flex justify-content-between px-5">
                                <div class="d-flex flex-column">
                                    <span class="text-purple-gray d-flex align-items-center pb-2">
                                        <img src="/icons/user-class.svg" style="height: 22px;" />
                                        <span class="noir-regular is-size-14 lh-18">
                                            <span class="px-2">{&class_group.members_group}</span>
                                            {lang::dict("Members")}
                                        </span>
                                    </span>
                                    <span class="text-purple-gray d-flex align-items-center">
                                        <i class="far fa-file-alt"></i>
                                        <span class="noir-regular is-size-14 lh-18">
                                            <span class="px-2">{&class_group.posts_group}</span>
                                            {lang::dict("Posts")}
                                        </span>
                                    </span>
                                </div>
                                <div class="d-flex flex-column">
                                    <span class="text-purple-gray d-flex align-items-center pb-2">
                                        <img src="/icons/robot.svg" style="height: 18px;" />
                                        <span class="noir-regular is-size-14 lh-18">
                                            <span class="px-2">{&class_group.robots_group}</span>
                                            {lang::dict("Robots")}
                                        </span>
                                    </span>
                                    // <span class="text-purple-gray d-flex align-items-center">
                                    //     <img src="/icons/folders.svg" style="height: 18px;" />
                                    //     <span class="noir-regular is-size-14 lh-18">
                                    //         <span class="px-2">{&class_group.classes_group}</span>
                                    //         {lang::dict("Classes")}
                                    //     </span>
                                    // </span>
                                </div>
                            </div>
                            <div class="text-center mt-5">
                                <a class="button bg-white" onmousedown=on_grade_view>
                                    <span class="text-secondary-purple noir-bold is-size-16 lh-20">{lang::dict("See Degree")}</span>
                                </a>
                            </div>
                        </div>
                    </div>
                }
            })
            .collect::<Html>();

        let on_schools_staff = self.link.callback(move |_| DegreeListMessage::AppRoute(AppRoute::Schools));
        let maybe_option = self.props.user_profile.as_ref().and_then(|item| {
            if item.user_staff.is_some() {
                Some(html! {
                    <a onclick=on_schools_staff>
                        <span class="text-purple-gray noir-bold is-size-16 lh-19">
                            <i class="fas fa-arrow-left me-2"></i>
                            <span>{lang::dict("To Schools")}</span>
                        </span>
                    </a>
                })
            } else {
                None
            }
        }).unwrap_or(html! {});
        let option_list_grades_view = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                if item.user_staff.is_some() {
                    Some(html! {
                        <>
                            <div class="d-flex flex-wrap justify-content-between">
                                <div class="d-flex flex-column">
                                    {maybe_option}
                                    {school_name}
                                </div>
                                <div class="d-flex flex-wrap align-items-center">
                                    <SearchDegreeList on_app_route=self.props.on_app_route.clone()
                                        school_id=self.props.school_id />
                                    {maybe_user_profile_pic}
                                </div>
                            </div>
                            <div class="d-flex flex-wrap">
                                {list_of_grades_by_school_id}
                            </div>
                        </>
                    })
                } else if item.user_teacher.is_some() {
                    Some(html! {
                        <>
                            <div class="d-flex flex-wrap justify-content-between">
                                {school_name_user}
                                <div class="d-flex flex-wrap align-items-center">
                                    <SearchDegreeList on_app_route=self.props.on_app_route.clone()
                                        school_id=self.props.school_id />
                                    {maybe_user_profile_pic}
                                </div>
                            </div>
                            <div class="d-flex flex-wrap">
                                {list_of_grades_by_user_id}
                            </div>
                        </>
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {});

        let list_degrees = match self.list_classes_state {
            LoadClasses::Loading => {
                html! {
                    <div class="w-100 h-100 d-flex align-items-center justify-content-center p-6">
                        <FullScreenLoaderDegree />
                    </div>
                }
            },
            LoadClasses::Load(LoadClassesFound::Found) => {
                html! {
                    <div class="w-100 h-100 d-flex flex-row justify-content-between scroll-y">
                        <div class="w-100 pt-3 ps-3 pt-md-4 ps-md-4 pt-lg-7 ps-lg-7">
                            {option_list_grades_view}
                        </div>
                    </div>
                }
            },
            LoadClasses::Load(LoadClassesFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <p class="text-gray is-size-22 lh-20">{lang::dict("No degree here.")}</p>
                    </div>
                }
            },
        };
        let _list_of_grades_by_school_id = self.degrees_by_school_id
            .iter()
            // .filter(|data| data.class_name.contains("Profesor"))
            .map(|item| {

            let _group_id = item.group_id.clone();
            // let on_delete_degree = self.link.callback(move |_| DegreeListMessage::DeleteClassGroup(GroupId(group_id)));
            html! {
                <div class="d-flex align-items-center justify-content-evenly mb-4">
                    <span class="text-primary-blue-light noir-bold is-size-18 lh-22">{&item.class_name}</span>
                    <span class="text-primary-blue-light noir-bold is-size-18 lh-22">{"<----------------------------->"}</span>
                    <span class="text-primary-blue-light noir-bold is-size-14 lh-22">{&item.group_id}</span>
                    // <button type="button" class="btn btn-outline-danger" onclick={&on_delete_degree}>{"DELETE CLASS GROUP"}</button>
                </div>
            }
        }).collect::<Html>();
        html! {
            <>
                // <div class="px-6 mx-6">{list_of_grades_by_school_id}</div>
                {list_degrees}
            </>
        }
    }
}
