use log::*;
use uuid::Uuid;
use yew::prelude::*;
use gloo_storage::Storage;
use code_location::code_location;
use yew::{html, Component, Html};
use serde_derive::{Deserialize, Serialize};

use roboxmaker_main::lang;
use roboxmaker_loaders::fullscreen_loader::FullScreenLoader;
use roboxmaker_models::{school_model, grade_model, meetings_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{GroupId, AppRoute, SchoolId, UserId, LoadFullScreen, LoadFullScreenFound, MyUserProfile};

#[derive(Debug, Clone, PartialEq)]
pub struct GroupData {
    pub class_name: String,
    pub group_id: GroupId,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DataSchool {
    pub name: String,
    pub inventory_group: Uuid,
    pub school_id: SchoolId,
}

pub struct AddUsers {
    graphql_task: Option<GraphQLTask>,
    list_schools_task: Option<RequestTask>,
    school_selected: Option<SchoolId>,
    group_id_selected: Option<GroupId>,
    data_school: Vec<DataSchool>,
    class_groups: Vec<GroupData>,
    show_dropdown_school: bool,
    show_dropdown_degree: bool,
    user_section_on: bool,
    loading_screen: LoadFullScreen,
    saved_sidebar_state: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct AddUsersProps {
    pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_id: UserId,
    pub on_user_profile: Option<Callback<UserId>>,
    pub saved_sidebar_state: bool,
}

#[derive(Debug)]
pub enum AddUsersMessage {
    AppRoute(AppRoute),
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
}

impl Component for AddUsers {
    type Message = AddUsersMessage;
    type Properties = AddUsersProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(AddUsersMessage::FetchSchoolList);

        let saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };

        AddUsers { 
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            list_schools_task: None,
            school_selected: None,
            group_id_selected: None,
            data_school: vec![],
            class_groups: vec![],
            show_dropdown_school: false,
            show_dropdown_degree: false,
            user_section_on: false,
            loading_screen: LoadFullScreen::Loading,
            saved_sidebar_state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            AddUsersMessage::AppRoute(route) => {
                ctx.props().on_app_route.emit(route)
            }
            AddUsersMessage::FetchSchoolList => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = meetings_model::list_schools_meets::Variables {};

                    let task = meetings_model::ListSchoolsMeets::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            AddUsersMessage::SchoolList(response)
                        },
                    );
                    self.list_schools_task = Some(task);
                }
            }
            AddUsersMessage::SchoolList(response) => {
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
                if !response.clone().and_then(|data| Some(data.inventory_group)).unwrap_or(vec![]).is_empty() {
                    self.loading_screen = LoadFullScreen::Load(LoadFullScreenFound::Found);
                } else {
                    self.loading_screen = LoadFullScreen::Load(LoadFullScreenFound::NotFound);
                }
                if self.school_selected.is_some() {
                    ctx.link().send_message(AddUsersMessage::FetchClassGroups);
                }
            }
            AddUsersMessage::FetchClassGroups => {
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
                                AddUsersMessage::ClassGroups(response)
                            },
                        );
                        self.list_schools_task = Some(task);
                        info!("SELECTED: {:?}", school_id);
                    }
                }
            },
            AddUsersMessage::ClassGroups(response) => {
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
                        }
                    })
                    .collect();

                self.group_id_selected = match self.class_groups.first(){
                    Some(group) => Some(group.group_id),
                    None => None
                };
            }
            AddUsersMessage::SchoolChangeData(school_id) => {
                self.school_selected = Some(school_id);
                self.show_dropdown_school = false;
                self.show_dropdown_degree = false;
                ctx.link().send_message(AddUsersMessage::FetchClassGroups);
            }
            AddUsersMessage::GroupChangeData(group_id) => {
                self.group_id_selected = Some(group_id);
                self.show_dropdown_degree = false;
                info!("SELECTED: {:?}", group_id);

            }
            AddUsersMessage::ShowDropdownSchool => {
                self.show_dropdown_school = !self.show_dropdown_school;
            }
            AddUsersMessage::ShowDropdownDegree => {
                self.show_dropdown_degree = !self.show_dropdown_degree;
            }
            AddUsersMessage::ChangeSidebarState => {
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
            AddUsersMessage::ShowUserHiddenSection => {
                self.user_section_on = !self.user_section_on;
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        trace!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if ctx.props() != old_props {
            should_render = true;
        }
        should_render
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        // DROPDOWN SCHOOLS

        let all_schools = self.data_school.iter().map(|school_group| {
            let school_id = school_group.school_id;
            let school_id_select = format!("{:?}", school_group.school_id);
            let on_show_list_degrees = ctx.link().callback(move |_| AddUsersMessage::SchoolChangeData(school_id));
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
                    {&school_group.name}</a>
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

        let on_dropdown_school = ctx.link().callback(|_| AddUsersMessage::ShowDropdownSchool);

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
            let on_show_list_degrees = ctx.link().callback(move |_| AddUsersMessage::GroupChangeData(group_id));
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

        let on_dropdown_degree = ctx.link().callback(|_| AddUsersMessage::ShowDropdownDegree);
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

        let welcome_class_view = ctx.props().user_profile.as_ref().and_then(|user_profile| {
            Some(html! {
                <div class="d-flex justify-content-between">
                    <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 pb-4 mb-1">{lang::dict("Hello, ")}
                        {&user_profile.full_name}
                    </h1>
                </div>
            })
        }).unwrap_or(html! {});


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