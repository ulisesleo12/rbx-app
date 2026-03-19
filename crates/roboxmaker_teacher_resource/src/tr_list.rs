use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use crate::tr_card::TeacherResourceCard;
use crate::tr_select::{SelectResource, SelectResourceOption};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_utils::funtions::get_creation_date;
use roboxmaker_models::{school_model, teacher_resource};
use roboxmaker_models::teacher_resource::teacher_resource_group_create;
use roboxmaker_models::teacher_resource::teacher_resource_list_by_group;
// use roboxmaker_models::teacher_resource::teacher_resourcen_group_create;
use roboxmaker_types::types::{GroupId, ResourceId, AppRoute, SchoolId, MyUserProfile, UserId};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask, Request, RequestTask};


#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ShowMoreResources {
    manual_and_guides: bool,
    software_and_tools: bool,
    presentation_and_didactic_material: bool,
    additional_resources: bool,
}


#[derive(Debug, Clone, PartialEq)]
pub struct TRProfile {
    pub title: String,
    pub timestamp: String,
    pub resource_id: ResourceId,
    pub author_full_name: String,
    pub author_pic_path: String,
    pub author_id: bool,
    pub archived: bool,
    pub send_to_degree: bool,
    pub school_name: String,
    pub school_logo: String,
    pub on_dropdown_menu: bool,
    pub tr_type: teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum,
}

pub struct TeacherResources {
    link: ComponentLink<Self>,
    props: TeacherResourcesProperties,
    graphql_task: Option<GraphQLTask>,
    resource_sub: Option<SubscriptionTask>,
    resource_delete_task: Option<RequestTask>,
    resource_add_task: Option<RequestTask>,
    show_dropdown_filter: bool,
    filter: ResourceFilter,
    resource_list: Vec<TRProfile>,
    more_resources: ShowMoreResources,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceFilter {
    Alls,
    Published,
    Unpublished,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct TeacherResourcesProperties {
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
pub enum Message {
    AppRoute(AppRoute),
    FetchResourcesByGroupId,
    Resources(Option<teacher_resource::teacher_resource_list_by_group::ResponseData>),
    AddResource(ResourceId),
    RemoveTR(ResourceId),
    RemoveTREntirely(ResourceId),
    CreateResource,
    TRAdded(Option<ResourceId>),
    TRRemoved(Option<ResourceId>),
    ShowDropdown,
    ChangeFilter(ResourceFilter),
    ShowMoreResource(ShowMoreResources),
    TeacherUpdateResources(ResourceId, bool, bool),
}

impl Component for TeacherResources {
    type Message = Message;
    type Properties = TeacherResourcesProperties;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Message::FetchResourcesByGroupId);

        let more_resources = ShowMoreResources {
            manual_and_guides: false,
            software_and_tools: false,
            presentation_and_didactic_material: false,
            additional_resources: false,
        };

        TeacherResources {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            resource_sub: None,
            resource_delete_task: None,
            resource_add_task: None,

            resource_list: vec![],
            show_dropdown_filter: false,
            filter: ResourceFilter::Alls,
            more_resources,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("TeacherResources: {:?}", msg);
        let mut should_update = true;
        match msg {
            Message::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            Message::FetchResourcesByGroupId => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = teacher_resource::teacher_resource_list_by_group::Variables {
                        group_id: self.props.group_id.0,
                    };

                    let task = teacher_resource::TeacherResourceListByGroup::subscribe(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                Message::Resources(response)
                            },
                    );
                    self.resource_sub = Some(task);
                }
            }
            Message::Resources(response) => { 
                self.resource_list = response
                    .clone()
                    .and_then(|data| Some(data.teacher_resource_group))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|item| {
                        let naive = chrono::NaiveDate::from_ymd_opt(2023, 01, 01).unwrap().and_hms_opt(23, 59, 59).unwrap();

                        let timestamp = item.teacher_resource_profile.clone().and_then(|data| Some(data.timestamp)).unwrap_or(naive);
                        
                        let time_fn = get_creation_date(timestamp);

                        let my_id = self.props.user_profile.clone().and_then(|user_by_pk| Some(user_by_pk.user_id)).unwrap_or(UserId(Uuid::default()));
                        let author_id = if item.teacher_resource_profile.clone().and_then(|data| Some(data.author_id)).unwrap_or(Uuid::default()) == my_id.0 {
                            true
                        } else {
                            false
                        };

                        let tr_type = item.teacher_resource_profile.clone().and_then(|data| data.teacher_resource_type).unwrap_or(teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum::AdditionalResources);

                        TRProfile { 
                            title: item.teacher_resource_profile.clone().and_then(|data| Some(data.title)).unwrap_or("".to_string()), 
                            timestamp: time_fn, 
                            resource_id: ResourceId(item.resource_id), 
                            author_full_name: item.teacher_resource_profile.clone().and_then(|data| Some(data.author)).clone().and_then(|author| author.user_profile).clone().and_then(|user_profile| Some(user_profile.full_name)).unwrap_or("".to_string()), 
                            author_pic_path: item.teacher_resource_profile.clone().and_then(|data| Some(data.author)).clone().and_then(|author| author.user_profile).clone().and_then(|user_profile| user_profile.pic_path).unwrap_or("".to_string()), 
                            author_id,
                            archived: item.archived, 
                            send_to_degree: item.send_to_grade, 
                            school_name: item.school_group.clone().and_then(|data| Some(data.school)).clone().and_then(|school| school.school_profile).clone().and_then(|school_profile| Some(school_profile.name)).unwrap_or("".to_string()), 
                            school_logo: item.school_group.clone().and_then(|data| Some(data.school)).clone().and_then(|school| school.school_profile).clone().and_then(|school_profile| school_profile.logo).unwrap_or("".to_string()), 
                            on_dropdown_menu: false,
                            tr_type,
                        }
                    }).collect();

            }
            Message::AddResource(resource_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                        
                    let vars = teacher_resource::teacher_resource_group_add::Variables { 
                        group_id: self.props.group_id.0,
                        resource_id: resource_id.0,
                    };

                    let task = teacher_resource::TeacherResourceGroupAdd::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let resource_id = if let Some(tr) = response {
                                tr.insert_teacher_resource_group_one.and_then(|data| Some(ResourceId(data.resource_id)))
                            } else {
                                None
                            };
                            Message::TRAdded(resource_id)
                        },
                    );
                    self.resource_add_task = Some(task);
                }
            }
            Message::RemoveTR(resource_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = teacher_resource::teacher_resource_group_delete::Variables { 
                        group_id: self.props.group_id.0,
                        resource_id: resource_id.0,
                    };

                    let task = teacher_resource::TeacherResourceGroupDelete::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let resource_id = if let Some(response) = response {
                                if response.delete_teacher_resource_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![]).len() > 0 {
                                    Some(ResourceId(response.delete_teacher_resource_group.clone().and_then(|data| Some(data.returning)).unwrap_or(vec![])[0].resource_id))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            Message::TRRemoved(resource_id)
                        },
                    );
                    self.resource_delete_task = Some(task);
                }
            }
            Message::RemoveTREntirely(resource_id) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = teacher_resource::delete_teacher_resource_by_id::Variables { 
                        resource_id: resource_id.0,
                    };

                    let task = teacher_resource::DeleteTeacherResourceById::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            let resource_id = if let Some(response) = response {
                                if response.delete_teacher_resources_by_pk.clone().and_then(|data| Some(data.id)).is_some() {
                                    let id = response.delete_teacher_resources_by_pk.clone().and_then(|data| Some(data.id)).unwrap();
                                    Some(ResourceId(id))
                                } else {
                                    None
                                }
                            } else {
                                None
                            };
                            Message::TRRemoved(resource_id)
                        },
                    );
                    self.resource_delete_task = Some(task);
                }
            }
            Message::CreateResource => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let local = chrono::Local::now().naive_local();

                    let teacher_resource_type = teacher_resource_group_create::RoboxTeacherResourceTypeEnum::ManualAndGuides;

                    if let Some(inventory_group_id) = self.props.inventory_group {
                        let vars = teacher_resource::teacher_resource_group_create::Variables { 
                            // title: String::from(lang::dict("~ New Lesson ~")),
                            title: String::from("~ New Resource ~"),
                            group_id: self.props.group_id.0,
                            inventory_group_id,
                            resource_id: Uuid::new_v4(),
                            timestamp: local,
                            teacher_resource_type,
                        };

                        // info!("{:?}", format!("{:?}", vars));
    
                        let task = teacher_resource::TeacherResourceGroupCreate::request(
                            graphql_task,
                            &self.link,
                            vars,
                            |response| {
                                let resource_id = if let Some(tr) = response {
                                    tr.insert_teacher_resource_group_one.and_then(|data| Some(ResourceId(data.resource_id)))
                                } else {
                                    None
                                };
                                Message::TRAdded(resource_id)
                            },
                        );
                        self.resource_add_task = Some(task);
                        self.link.send_message(Message::FetchResourcesByGroupId);
                    }
                }
            }
            Message::TRAdded(resource_id) => {
                let group_id = self.props.group_id;
                let school_id = self.props.school_id;
                let tr_type = teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum::AdditionalResources;

                if let Some(resource_id) = resource_id {
                    self.resource_list.push(TRProfile { 
                        title: String::from(""), timestamp: String::from(""), 
                        resource_id, 
                        author_full_name: String::from(""), 
                        author_pic_path: String::from(""), 
                        author_id: false, 
                        archived: false, send_to_degree: false, 
                        school_name: String::from(""), school_logo: String::from(""), 
                        on_dropdown_menu: false, 
                        tr_type,
                    });
                    self.link.send_message(Message::AppRoute(AppRoute::Resource(school_id, group_id, resource_id)));
                } else {
                    should_update = true;
                }
            }
            Message::TRRemoved(resource_id) => {
                if let Some(resource_id) = resource_id {
                    self.resource_list.retain(|u| u.resource_id != resource_id);
                } else {
                    should_update = true;
                }
            }
            Message::ShowDropdown => {
                self.show_dropdown_filter = !self.show_dropdown_filter;
            }
            Message::ChangeFilter(filter) => {
                self.filter = filter;
                self.show_dropdown_filter = false;
            }
            Message::ShowMoreResource(more_resources) => {
                self.more_resources = more_resources

            }
            Message::TeacherUpdateResources(resource_id, send_to_grade , archived) => {
                for resource in self.resource_list.iter_mut() {
                    if resource.resource_id == resource_id {
                        resource.send_to_degree = send_to_grade;
                        resource.archived = archived;
                    }
                }
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
        let on_direct_meet = self.link.callback(move |_| Message::AppRoute(AppRoute::MeetDirect(group_id)));
        let on_dropdown = self.link.callback(|_| Message::ShowDropdown);
        let on_alls = self.link.callback(|_| Message::ChangeFilter(ResourceFilter::Alls));
        let on_published = self.link.callback(|_| Message::ChangeFilter(ResourceFilter::Published));
        let on_unpublished = self.link.callback(|_| Message::ChangeFilter(ResourceFilter::Unpublished));

        // let on_change_list = self.link.callback(|(resource_id, send_to_grade, archived)| Message::TeacherUpdateResources(resource_id, send_to_grade, archived));
        // let on_lesson_delete = self.link.callback(|resource_id| Message::RemoveTR(resource_id));
        // let on_del_lesson_entirely = self.link.callback(|resource_id| Message::RemoveTREntirely(resource_id));


        let search_resources = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = self.link.callback(|select_option| match select_option {
                    SelectResourceOption::Resource(resource_id) => { Message::AddResource(resource_id) }
                });
                if item.user_staff.is_some() {
                    Some(html! {
                        <SelectResource on_select=on_select 
                            allow_create=true
                            on_app_route=self.props.on_app_route.clone()
                            school_id=self.props.school_id />
                    })
                } else {
                    None
                }
            })
            .unwrap_or(html! {});

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
        
        let header = html! {
            <div class="d-flex flex-wrap align-items-center justify-content-between mb-5">
                <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">
                    {self.props.class_name.clone()}
                </h1>
                <a class="btn btn-outline-light text-primary-blue-dark noir-regular is-size-18 lh-22" onclick=on_direct_meet>
                    <img class="me-3" src="/icons/video-2.svg" style="height: 30px;" />
                    <span>{lang::dict("Meet up")}</span>
                </a>
                {search_resources}
                {maybe_user_profile_pic}
            </div>
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

        let maybe_option_selected = match self.filter {
            ResourceFilter::Alls => "Everyone",
            ResourceFilter::Published => "Released",
            ResourceFilter::Unpublished => "Unpublished",
            // ResourceFilter::Archived => "Archived",
        };

        let filtered_resources: Vec<_> = self.resource_list.iter().filter(|resource| {
            self.filter == ResourceFilter::Alls && {resource.archived == true || resource.archived == false || resource.send_to_degree == true || resource.send_to_degree == false} ||
    
            self.filter == ResourceFilter::Published && resource.send_to_degree == true && resource.archived == false ||

            self.filter == ResourceFilter::Unpublished && resource.archived == false && resource.send_to_degree == false
        }).collect();

        let get_filter_resources_by_type = |resource_type: teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum| -> usize {
            filtered_resources
                .iter()
                .filter(|l| l.tr_type == resource_type)
                .count()
        };

        // let filter_resources_by_type = |resource_type: teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum| -> yew::virtual_dom::VNode {
        let filter_resources_by_type =  {
            filtered_resources
                .iter()
                // .filter(|l| l.tr_type == resource_type)
                .map(|item| {
                    let tr_profile = item.clone();
                    html! {
                        <TeacherResourceCard resource_id=item.resource_id.clone()
                            user_profile=self.props.user_profile.clone()
                            group_id=self.props.group_id
                            on_app_route=self.props.on_app_route.clone()
                            on_delete_resource=self.link.callback(|resource_id| Message::RemoveTR(resource_id))
                            on_delete_resource_by_id=self.link.callback(|resource_id| Message::RemoveTREntirely(resource_id))
                            auth_school=self.props.auth_school.clone()
                            on_change_list=self.link.callback(|(resource_id, send_to_grade, archived)| Message::TeacherUpdateResources(resource_id, send_to_grade, archived))
                            tr_profile={tr_profile}.clone()
                            archived=item.archived
                            send_to_grade=item.send_to_degree
                            school_id=self.props.school_id />
                    }
                }).collect::<Html>()
        };

        let _dropdown_resource_filter = self
            .props
            .user_profile
            .as_ref()
            .and_then(|user|{
                if user.user_staff.is_some() {
                    Some(html! {
                        <div class="dropdown me-5">
                            <button class=class_dropdown type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick=on_dropdown>
                                <img src="/icons/filter.svg" style="height: 22px;" />
                                <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22">{lang::dict(maybe_option_selected)}</span>
                            </button>
                            <ul class=class_dropdown_list aria-labelledby="dropdownMenuButton2">
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center mt-1 pe-0" onclick=on_alls>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == ResourceFilter::Alls { true } else { false }} />
                                        <span class={if self.filter == ResourceFilter::Alls {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Everyone")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_published>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == ResourceFilter::Published { true } else { false }} />
                                        <span class={if self.filter == ResourceFilter::Published {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Released")}</span>
                                    </a>
                                </li>
                                <li>
                                    <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_unpublished>
                                        <input class="bg-checkbox" type="checkbox" checked={if self.filter == ResourceFilter::Unpublished { true } else { false }} />
                                        <span class={if self.filter == ResourceFilter::Unpublished {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Unpublished")}</span>
                                    </a>
                                </li>
                                // <li>
                                //     <a class="dropdown-item d-flex flex-wrap align-items-center pe-0" onclick=on_archived>
                                //         <input class="bg-checkbox" type="checkbox" checked={if self.filter == ResourceFilter::Archived {true} else {false}} />
                                //         <span class={if self.filter == ResourceFilter::Archived {"text-blue-purple noir-regular is-size-18 lh-22 ps-2"} else {"text-gray-purple noir-regular is-size-18 lh-22 ps-2"}}>{lang::dict("Archived")}</span>
                                //     </a>
                                // </li>
                            </ul>
                        </div>
                    })
                } else {
                    Some(html! {})
                }
            })
            .unwrap_or(html! {});

        let new_resource = self
            .props
            .user_profile
            .as_ref()
            .and_then(|item| {
                let on_select = self.link.callback(move |_| Message::CreateResource);
                if item.user_staff.is_some() {
                    Some(html! {
                        <a class="button btn-create-card bg-primary-blue-dark d-flex align-items-center justify-content-center" onclick=on_select.clone()>
                            <span class="text-white noir-bold is-size-16 lh-20 d-flex align-items-center">
                                <i class="fas fa-plus me-2"></i>
                                <span>{"Nuevo recurso"}</span>
                            </span>
                        </a>
                    })
                } else {
                    None
                }
            }).unwrap_or(html! {});

        // let resource_manuals_and_guides = filter_resources_by_type(teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum::ManualAndGuides);
        // let resource_software_and_tools = filter_resources_by_type(teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum::SoftwareAndTools);
        // let resource_presentation_and_didactic_material = filter_resources_by_type(teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum::PresentationAndDidacticMaterial);
        // let resource_additional_resources = filter_resources_by_type(teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum::AdditionalResources);

        // <div class="text-center">
        //     <span class="text-gray-strong is-size-18 lh-20">{lang::dict("No teacher resources.")}</span>
        // </div>
        let show_more_resources = self.more_resources.clone();
        let link = self.link.clone();

        let count_resource_manuals_and_guides = get_filter_resources_by_type(teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum::ManualAndGuides);
        let count_resource_software_and_tools = get_filter_resources_by_type(teacher_resource_list_by_group::RoboxTeacherResourceTypeEnum::SoftwareAndTools);
        html! { 
            <div class="scroll-y w-100 h-100 p-3 p-md-4 p-lg-7 pb-6">
                { header }
                <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                    <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                        // { lang::dict("Electronics Resources") } <span class="ps-1">{"("}{ lesson_electronics_count }{")"}</span>
                        { "Manuales y guías" }
                    </span>
                    { new_resource }
                    // <div class="d-flex flex-wrap">
                    //     { dropdown_resource_filter }
                    // </div>
                </div>
                { filter_resources_by_type }
                <div class={ if self.more_resources.manual_and_guides { "d-flex flex-wrap" } else { "d-flex flex-wrap more-lesson-hidden" } }>
                    // { resource_manuals_and_guides }
                </div>
                {
                    if count_resource_manuals_and_guides > 3 {
                        html! {
                            <div class="d-flex justify-content-center">
                                <a onclick={ link.callback(move |_| Message::ShowMoreResource(ShowMoreResources { manual_and_guides: !show_more_resources.manual_and_guides, ..show_more_resources })) }>
                                    <span class="text-secondary-purple noir-bold is-size-18 lh-22 pt-5">{ "Ver más elementos" }</span>
                                </a>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                    <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                        // { lang::dict("Electronics Resources") } <span class="ps-1">{"("}{ lesson_electronics_count }{")"}</span>
                        { "Software y herramientas" }
                    </span>
                </div>
                <div class={ if self.more_resources.software_and_tools { "d-flex flex-wrap" } else { "d-flex flex-wrap more-lesson-hidden" } }>
                    // { resource_software_and_tools }
                </div>
                {
                    if count_resource_software_and_tools > 3 {
                        html! {
                            <div class="d-flex justify-content-center">
                                <a onclick={ link.callback(move |_| Message::ShowMoreResource(ShowMoreResources { software_and_tools: !show_more_resources.software_and_tools, ..show_more_resources })) }>
                                    // <span class="text-secondary-purple noir-bold is-size-18 lh-22 pt-5">{lang::dict("See All Electronics Resources")}</span>
                                    <span class="text-secondary-purple noir-bold is-size-18 lh-22 pt-5">{ "Ver más elementos" }</span>
                                </a>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <br />
                <br />
                <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                    <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                        // { lang::dict("Electronics Resources") } <span class="ps-1">{"("}{ lesson_electronics_count }{")"}</span>
                        { "Presentaciones y material didáctico" }
                    </span>
                </div>
                // { resource_presentation_and_didactic_material }
                <br />
                <br />
                <div class="d-flex flex-wrap align-items-center justify-content-between pb-4">
                    <span class="text-primary-blue-dark noir-bold is-size-24 lh-29 mb-3 mb-sm-3 mb-md-3 mb-lg-0">
                        // { lang::dict("Electronics Resources") } <span class="ps-1">{"("}{ lesson_electronics_count }{")"}</span>
                        { "Recursos complementarios" }
                    </span>
                </div>
                // { resource_additional_resources }
            </div>
        }
    }
}