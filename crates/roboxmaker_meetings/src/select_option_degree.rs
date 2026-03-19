use log::*;
use uuid::Uuid;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use crate::create_meet::ModalCreateMeet;
use crate::{ClassGroupMeetData, ClassGroupMeetings};

use roboxmaker_models::{school_model, meetings_model};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};
use roboxmaker_types::types::{SchoolId, GroupId, MeetingsId, MyUserProfile};

pub struct SelectOptionDegree {
    graphql_task: Option<GraphQLTask>,
    degree_list_task: Option<RequestTask>,
    group_id_selected: Option<GroupId>,
    class_groups: Vec<ClassGroupMeetData>,
    show_dropdown: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SelectOptionDegreeProps {
    pub school_id: SchoolId,
    pub inventory_group_id: Uuid,
    pub user_profile: Option<MyUserProfile>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub on_list_change: Option<Callback<()>>,
    pub close_modal_callback_meet: Callback<bool>,
    pub close_modal_callback_failed: Callback<bool>,
}

#[derive(Debug)]
pub enum SelectOptionDegreeMessage {
    FetchClassGroups,
    ClassGroups(Option<meetings_model::class_groups_by_school_id_meetigns::ResponseData>),
    SelectClassGroup(GroupId),
    ShowDropdown,
}

impl Component for SelectOptionDegree {
    type Message = SelectOptionDegreeMessage;
    type Properties = SelectOptionDegreeProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(SelectOptionDegreeMessage::FetchClassGroups);

        SelectOptionDegree { 
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            degree_list_task: None,
            group_id_selected: None,
            class_groups: vec![],
            show_dropdown: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            SelectOptionDegreeMessage::FetchClassGroups => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = meetings_model::class_groups_by_school_id_meetigns::Variables { 
                        school_id: ctx.props().school_id.0,
                    };

                    let task = meetings_model::ClassGroupsBySchoolIdMeetigns::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            SelectOptionDegreeMessage::ClassGroups(response)
                        },
                    );
                    self.degree_list_task = Some(task);
                }
            }
            SelectOptionDegreeMessage::ClassGroups(class_groups) => {
                self.class_groups = class_groups
                    .clone()
                    .and_then(|data| Some(data.class_group))
                    .unwrap_or(vec! [])
                    .iter()
                    .map(|class_group| {
                        let class_name = class_group.class_profile.clone().and_then(|data| Some(data.name)).unwrap_or("".to_string());
                        let group_id = class_group.group_id;
                        let school_id = class_group.school_group.clone().and_then(|data| Some(data.school_id)).unwrap_or(Uuid::default());
                        let meetings: Vec<ClassGroupMeetings> = class_group.meetings_groups.iter().map(|meets| {
                            let meetings_id = meets.meet_id;
                            ClassGroupMeetings {
                                meetings_id: MeetingsId(meetings_id),
                            }
                        }).collect();
                        ClassGroupMeetData {
                            class_name: class_name,
                            group_id: GroupId(group_id),
                            school_id: SchoolId(school_id),
                            meetings: meetings,
                        }
                    }).collect();
                self.group_id_selected = match self.class_groups.first() {
                    Some(group) => Some(group.group_id),
                    None => None,
                };
            }
            SelectOptionDegreeMessage::SelectClassGroup(group_id) => {
                self.group_id_selected = Some(group_id);
                self.show_dropdown = false;
            }
            SelectOptionDegreeMessage::ShowDropdown => {
                self.show_dropdown = !self.show_dropdown;
            }
        }
        should_update
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if ctx.props() != old_props {
            should_render = true;
        } 

        should_render
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_dropdown = ctx.link().callback(|_| SelectOptionDegreeMessage::ShowDropdown);
        let alls_class_groups = self.class_groups.iter().map(|class_group| {
            let group_id = class_group.group_id;
            let class_id_select = format!("{:?}", group_id);
            let on_show_list_degrees = ctx.link().callback(move |_| SelectOptionDegreeMessage::SelectClassGroup(group_id));
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
                    "dropdown-item bg-silver text-blue-purple noir-regular is-size-14 lh-20 d-flex align-items-center text-break-spaces"
                } else {
                    "dropdown-item text-gray-purple noir-regular is-size-14 lh-20 d-flex align-items-center text-break-spaces"
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
                    <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22-meet">{&class_group.class_name}</span>
                }
            } else {
                html! {}
            };
            html! {
                {maybe_class}
            }
        })
        .collect::<Html>();
        let class_group_meetings = |class_group: &ClassGroupMeetData| {
            if self.group_id_selected == Some(class_group.group_id) {
                let meetings = class_group.meetings.clone();
                let class_name = class_group.class_name.clone();
                let school_id =class_group.school_id;
                let on_list_change = ctx.link().callback(move |_| SelectOptionDegreeMessage::FetchClassGroups);
                html! {
                    <div class="d-flex flex-column align-items-center">
                        <ModalCreateMeet meetings={meetings}
                            allow_edit={true}
                            class_name={class_name}
                            school_id={school_id}
                            group_id={class_group.group_id}
                            close_modal_callback_failed={ctx.props().close_modal_callback_failed.clone()}
                            close_modal_callback_meet={ctx.props().close_modal_callback_meet.clone()}
                            inventory_group_id={ctx.props().inventory_group_id}
                            auth_school={ctx.props().auth_school.clone()}
                            on_list_change={Some(on_list_change)}   />
                    </div>
                }
            } else {
                html! {}
            }
        };
        let class_dropdown = if self.show_dropdown {
            "btn btn-secondary btn-second-meet dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-second-meet dropdown-toggle d-flex align-items-center justify-content-between"
        };
        let class_dropdown_list = if self.show_dropdown {
            "dropdown-menu dropdown-menu-home show scroll-dropdown-home w-100"
        } else {
            "dropdown-menu dropdown-menu-home"
        };
        let maybe_option_user = html! {
            <div class="dropdown dropdown-h">
                <button class={class_dropdown} type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick={on_dropdown}>
                    <img src="/icons/graduation-4.svg" style="height: 18px;" />
                    {change_class_group}
                </button>
                <ul class={class_dropdown_list} aria-labelledby="dropdownMenuButton2">
                    {alls_class_groups}
                </ul>
            </div>
        };
        let class_group_level = |class_group: &ClassGroupMeetData| {
            let group_id = class_group.group_id;
            let class_group_id = format!("class-group-{}", group_id);
            html! {
                <>
                    <div id={ class_group_id.clone() } class="d-flex flex-column justify-content-center align-items-center mt-5">
                        {maybe_option_user}
                        {class_group_meetings(class_group)}
                    </div>
                </>
            }
        };
        html! {
            <>
                {
                    self.class_groups
                    .iter()
                    .filter(|data| data.group_id == self.group_id_selected.unwrap_or(GroupId(Uuid::default())))
                    .map(|class_group|{
                        class_group_level.clone()(class_group)
                    }).collect::<Html>()
                    
                }
            </>
        }
    }
}