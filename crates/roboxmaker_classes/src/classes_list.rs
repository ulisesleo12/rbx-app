use log::*;
use std::vec;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use crate::classes_select::ClassesSelect;
use yew::{html, Component, ComponentLink, Html, ShouldRender};
use crate::{classes_card::ClassesCard, classes_select::ClassesSelectOption};

use roboxmaker_main::lang;
use roboxmaker_storage::image_view::ImgView;
use roboxmaker_models::{school_model, classes_model};
use roboxmaker_searches::search_classes_group::SearchClassesGroup;
use roboxmaker_types::types::{ClassesId, GroupId, AppRoute, SchoolId, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};


#[derive(Debug, Clone, PartialEq)]
pub struct ClassesProfile {
    pub topic: String,
    pub classes_id: ClassesId,
    pub archived: bool,
    pub published: bool,
}

pub struct ClassesList {
    link: ComponentLink<Self>,
    props: ClassesListProperties,
    graphql_task: Option<GraphQLTask>,
    classes_sub: Option<SubscriptionTask>,
    classes_delete_task: Option<RequestTask>,
    classes_add_task: Option<RequestTask>,
    show_dropdown_filter: bool,
    filter: ClassesFilter,
    classes_list: Vec<ClassesProfile>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ClassesFilter {
    Alls,
    Published,
    Unpublished,
    Archived,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ClassesListProperties {
    pub group_id: GroupId,
    pub school_id: SchoolId,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_app_route: Callback<AppRoute>,
    pub on_list_change: Option<Callback<()>>,
    pub inventory_group: Option<Uuid>,
    pub class_name: String,
}

#[derive(Debug)]
pub enum ClassesListMessage {
    AppRoute(AppRoute),
    FetchClassesByGroupId,
    Classes(Option<classes_model::classes_list_by_group::ResponseData>),
    AddClasses(ClassesId),
    RemoveClasses(ClassesId),
    CreateClasses,
    ClassesAdded(Option<ClassesId>),
    ClassesRemoved(Option<ClassesId>),
    ShowDropdown,
    ChangeFilter(ClassesFilter),
}

impl Component for ClassesList {
    type Message = ClassesListMessage;
    type Properties = ClassesListProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(ClassesListMessage::FetchClassesByGroupId);
        ClassesList {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            classes_sub: None,
            classes_delete_task: None,
            classes_add_task: None,
            show_dropdown_filter: false,
            filter: ClassesFilter::Alls,
            classes_list: vec![],
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let mut should_update = true;
        match msg {
            ClassesListMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            ClassesListMessage::FetchClassesByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = classes_model::classes_list_by_group::Variables {
                        group_id: self.props.group_id.0,
                    };

                    let task = classes_model::ClassesListByGroup::subscribe(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                ClassesListMessage::Classes(response)
                            },
                    );
                    self.classes_sub = Some(task);
                }
            }
            ClassesListMessage::Classes(response) => {
                self.classes_list = response
                    .clone()
                    .and_then(|data| Some(data.classes_group))
                    .unwrap_or(vec![])
                    .iter()
                    .filter(|classes | {
    
                        self.filter == ClassesFilter::Alls && {classes.published == true || classes.published == false || classes.archived == false} ||
    
                        self.filter == ClassesFilter::Published && classes.published == true && classes.archived == false ||
    
                        self.filter == ClassesFilter::Unpublished && classes.archived == false && classes.published == false ||
    
                        self.filter == ClassesFilter::Archived && classes.archived == true && classes.published == false
    
                    })
                    .map(|item| {
                        ClassesProfile { 
                            topic: item.classes_profile.clone().and_then(|data| Some(data.topic)).unwrap_or("".to_string()), 
                            classes_id: ClassesId(item.classes_id), 
                            archived: item.archived, 
                            published: item.published, 
                        }
                    }).collect();
            }
            ClassesListMessage::AddClasses(classes_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                        
                    let vars = classes_model::classes_group_add::Variables { 
                        group_id: self.props.group_id.0,
                        classes_id: classes_id.0,
                    };

                    let task = classes_model::ClassesGroupAdd::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let classes_id = if let Some(classes) = response {
                                classes.insert_classes_group_one.clone().and_then(|data| Some(ClassesId(data.classes_id)))
                            } else {
                                None
                            };
                            ClassesListMessage::ClassesAdded(classes_id)
                        },
                    );
                    self.classes_add_task = Some(task);
                }
            }
            ClassesListMessage::RemoveClasses(classes_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = classes_model::classes_group_delete::Variables { 
                        group_id: self.props.group_id.0,
                        classes_id: classes_id.0,
                    };

                    let task = classes_model::ClassesGroupDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let classes_id = if let Some(response) = response {
                                if response.delete_classes_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![]).len() > 0 {
                                    Some(ClassesId(response.delete_classes_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![])[0].classes_id))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            ClassesListMessage::ClassesRemoved(classes_id)
                        },
                    );
                    self.classes_delete_task = Some(task);
                }
            }
            ClassesListMessage::CreateClasses => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let local = chrono::Local::now().naive_local();

                    if let Some(inventory_group_id) = self.props.inventory_group {
                        let vars = classes_model::classes_group_create::Variables { 
                            topic: String::from(lang::dict("~ New Classes ~")),
                            content: String::from(""),
                            group_id: self.props.group_id.0,
                            inventory_group_id,
                            classes_id: Uuid::new_v4(),
                            timestamp: local,
                        };
    
                        let task = classes_model::ClassesGroupCreate::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                let classes_id = if let Some(classes) = response {
                                    classes.insert_classes_group_one.and_then(|data| Some(ClassesId(data.classes_id)))
                                } else {
                                    None
                                };
                                ClassesListMessage::ClassesAdded(classes_id)
                            },
                        );
                        self.classes_add_task = Some(task);
                        self.link.send_message(ClassesListMessage::FetchClassesByGroupId);
                    }
                }
            }
            ClassesListMessage::ClassesAdded(classes_id) => {
                let group_id = self.props.group_id;
                let school_id = self.props.school_id;
                if let Some(classes_id) = classes_id {
                    self.classes_list.push(ClassesProfile {classes_id, published: false, archived: false, topic: String::from("") });
                    
                    self.link.send_message(ClassesListMessage::AppRoute(AppRoute::Classes(school_id, group_id, classes_id)));
                } else {
                    should_update = true;
                }
            }
            ClassesListMessage::ClassesRemoved(classes_id) => {
                if let Some(classes_id) = classes_id {
                    self.classes_list.retain(|u| u.classes_id != classes_id);
                } else {
                    should_update = true;
                }
            }
            ClassesListMessage::ShowDropdown => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            ClassesListMessage::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;
                self.link.send_message(ClassesListMessage::FetchClassesByGroupId);
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
        let on_alls = self.link.callback(|_| ClassesListMessage::ChangeFilter(ClassesFilter::Alls));
        let on_published = self.link.callback(|_| ClassesListMessage::ChangeFilter(ClassesFilter::Published));
        let on_unpublished = self.link.callback(|_| ClassesListMessage::ChangeFilter(ClassesFilter::Unpublished));
        let on_archived = self.link.callback(|_| ClassesListMessage::ChangeFilter(ClassesFilter::Archived));
        let on_dropdown = self.link.callback(|_| ClassesListMessage::ShowDropdown);
        let on_classes_delete = self.link.callback(|classes_id| ClassesListMessage::RemoveClasses(classes_id));
        let maybe_option_seleted = match self.filter {
            ClassesFilter::Alls => "Everyone",
            ClassesFilter::Published => "Released",
            ClassesFilter::Unpublished => "Unpublished",
            ClassesFilter::Archived => "Archived",
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
            .and_then(|user|{
                if user.user_staff.is_some() || user.user_teacher.is_some() {
                    Some(html! {
                        <div class="dropdown me-5">
                            <button class=class_dropdown type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick=on_dropdown>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(maybe_option_seleted)}</span>
                            </button>
                            <ul class=class_dropdown_list aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick=on_alls>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == ClassesFilter::Alls {true} else {false}} />
                                        <span class={if self.filter == ClassesFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_published>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == ClassesFilter::Published {true} else {false}} />
                                        <span class={if self.filter == ClassesFilter::Published {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Released")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_unpublished>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == ClassesFilter::Unpublished {true} else {false}} />
                                        <span class={if self.filter == ClassesFilter::Unpublished {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Unpublished")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_archived>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == ClassesFilter::Archived {true} else {false}} />
                                        <span class={if self.filter == ClassesFilter::Archived {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Archived")}</span>
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
        
        let maybe_new = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user|{
                if user.user_staff.is_some() || user.user_teacher.is_some() {
                    let on_select = self
                        .link
                        .callback(move |_| ClassesListMessage::CreateClasses);
                    Some(html! {
                        <>
                            <a class="button btn-create-card bg-primary-blue-dark d-flex align-items-center justify-content-center" onmousedown=on_select.clone()>
                                <span class="text-white noir-bold is-size-16 lh-20 d-flex align-items-center">
                                    <i class="fas fa-plus me-2"></i>
                                    <span>{lang::dict("New Module")}</span>
                                </span>
                            </a>
                        </>
                    })
                } else {Some(html! {})}
            })
            .unwrap_or(html! {});

        let classes_list = self.classes_list.iter().map(|item| {
            html! {
                <ClassesCard user_profile=self.props.user_profile.clone() 
                    classes_id=item.classes_id.clone()
                    group_id=self.props.group_id
                    school_id=self.props.school_id
                    on_app_route=self.props.on_app_route.clone()
                    on_classes_delete=on_classes_delete.clone()
                    topic=item.topic.clone() />
            }
        }).collect::<Html>();
        let maybe_classes_search = self
            .props
            .user_profile
            .as_ref()
            .and_then(|auth_user| {
                let on_select = self.link.callback(|select_option| match select_option {
                    ClassesSelectOption::Classes(classes_id) => ClassesListMessage::AddClasses(classes_id),
                });
                if auth_user.user_staff.is_some() || auth_user.user_teacher.is_some() {
                    Some(html! {
                        <ClassesSelect on_select=on_select 
                            group_id=self.props.group_id
                            user_profile=self.props.user_profile.clone()
                            on_app_route=self.props.on_app_route.clone()
                            school_id=self.props.school_id />
                    })
                } else {
                    Some(html! {
                        <SearchClassesGroup on_app_route=self.props.on_app_route.clone()
                            group_id=self.props.group_id
                            school_id=self.props.school_id />
                    })
                }
            })
            .unwrap_or(html! {});
        let group_id = self.props.group_id; 
        let on_direct_meet = self.link.callback(move |_| ClassesListMessage::AppRoute(AppRoute::MeetDirect(group_id)));
        let maybe_meet = {
            html! {
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick=on_direct_meet>
                    <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
            }
        };
        let pic_path = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user_profile| Some(user_profile.pic_path.clone()))
            .unwrap_or("/static/avatar.png".to_string());

        let head_section = html! {
            <div class="d-flex flex-wrap align-items-lg-center justify-content-between mb-md-5 mb-lg-6">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {self.props.class_name.clone()}
                </h1>
                <div class="d-flex flex-wrap justify-content-between align-items-center col-12 col-xl-5 mb-4 mb-lg-0">
                    {maybe_meet}
                    <div class="px-5">{maybe_classes_search}</div>
                    // {maybe_user_profile_pic}
                    <ImgView pic_path={pic_path} />
                </div>
            </div>
        };
        let maybe_dropdown = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                    {lang::dict("Classes")} <span class="ps-1">{"("}{self.classes_list.len()}{")"}</span>
                </span>
                <div class="d-flex flex-wrap">
                    {maybe_dropdown_by_user}
                    {maybe_new}
                </div>
            </div>
        };
        let maybe_option = if !self.classes_list.is_empty() {
            html! {
                <div class="d-flex flex-column">
                    {classes_list}
                </div>
            }
        } else {
            html! {
                <div class="text-center">
                    <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No classes here.")}</span>
                </div>
            }
        };

        html! { 
            <div class="scroll-y w-100 h-100 p-3 p-md-4 p-lg-7">
                {head_section}
                {maybe_dropdown}
                {maybe_option}
            </div>
        }
    }
}
