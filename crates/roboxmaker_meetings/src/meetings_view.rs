use log::*;
use uuid::Uuid;
use chrono::Local;
use web_sys::Node;
use std::time::Duration;
use gloo_storage::Storage;
use code_location::code_location;
use yew::{html, Component, Html};
use gloo_timers::callback::Timeout;
use yew::{prelude::*, virtual_dom::VNode};
use crate::last_meetings::LastMeetingsList;
use crate::select_option_degree::SelectOptionDegree;
use wasm_bindgen::{prelude::{Closure, wasm_bindgen}, JsValue};
use crate::list_meetings_by_school::{MeetingsListBySchool};

use roboxmaker_main::lang;
use roboxmaker_models::meetings_model;
use roboxmaker_searches::search_meetings_list::SearchMeetingsList;
use roboxmaker_types::types::{SchoolId, DataSchool, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

pub struct MeetingsView {
    graphql_task: Option<GraphQLTask>,
    list_schools_task: Option<RequestTask>,
    show_modal_meet: bool,
    data_school: Vec<DataSchool>,
    show_dropdown: bool,
    school_selected: Option<SchoolId>,
    meeting_created_successfully: bool,
    meeting_created_failed: bool,
    job: Option<Timeout>,
    node: Node,
    date_selected: String,
    is_loading: bool,
    saved_sidebar_state: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MeetingsProps {
    pub user_profile: Option<MyUserProfile>,
}

#[derive(Debug)]
pub enum MeetingsMessage {
    FetchSchoolList,
    SchoolList(Option<meetings_model::list_schools_meets::ResponseData>),
    ShowMeets,
    ShowDropdown,
    SchoolChangeData(SchoolId),
    OnHiddenModal(bool),
    OnHiddenModalFailed(bool),
    MeetingCreated,
    MeetingFailed,
    DateFromVCalendar(String),
    ChangeSidebarState,
}

impl MeetingsView {
    fn date_selected(
        &mut self,
        date: String,
        callback: Callback<String>,
    ) {
        let on_result_selected = Closure::wrap(Box::new(move |data: String| {
            callback.emit(data)
        }) as Box<dyn Fn(String)>);
        
        app_vcalendar(
            &self.node,
            date,
            &on_result_selected,
        );
        
        on_result_selected.forget();
    }
}

impl Component for MeetingsView {
    type Message = MeetingsMessage;
    type Properties = MeetingsProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(MeetingsMessage::FetchSchoolList);

        let node = web_sys::window()
            .and_then(|window| window.document())
            .and_then(|document| document.create_element("div").ok())
            .and_then(|div| {
                let _ = div.set_id("MyApp");
                Some(Node::from(div))
            });

        let saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };
        roboxmaker_utils::functions::meets_state();

        MeetingsView { 
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            list_schools_task: None,
            show_modal_meet: false,
            data_school: vec![],
            show_dropdown: false,
            school_selected: None,
            meeting_created_successfully: false,
            meeting_created_failed: false,
            job: None,
            node: node.unwrap(),
            date_selected: String::default(),
            is_loading: true,
            saved_sidebar_state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            MeetingsMessage::DateFromVCalendar(date_vcalendar) => {
                self.date_selected = date_vcalendar;
            }
            MeetingsMessage::FetchSchoolList => {
                self.is_loading = false;
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let vars = meetings_model::list_schools_meets::Variables {};

                    let task = meetings_model::ListSchoolsMeets::request(
                        graphql_task,
                        &ctx,
                        vars,
                        |response| {
                            MeetingsMessage::SchoolList(response)
                        },
                    );
                    self.list_schools_task = Some(task);
                }
            }
            MeetingsMessage::SchoolList(response) => {
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
                        let invenoty_id = school_group.clone().and_then(|data| Some(data.group_id)).unwrap_or(Uuid::default());
                        let school_id = school_group.clone().and_then(|data| Some(data.school_id)).unwrap_or(Uuid::default());
                        DataSchool {
                            name: name,
                            inventory_id: invenoty_id,
                            school_id: SchoolId(school_id),
                        }
                    }).collect();

                self.school_selected = match self.data_school.first()  {
                    Some(school) => Some(school.school_id),
                    None => None,
                };
            }
            MeetingsMessage::ShowDropdown => {
                self.show_dropdown = !self.show_dropdown;
            }
            MeetingsMessage::ShowMeets => {
                self.show_modal_meet = !self.show_modal_meet
            }
            MeetingsMessage::SchoolChangeData(school_id) => {
                self.school_selected = Some(school_id);
                self.show_dropdown = false;
            }
            MeetingsMessage::MeetingCreated => {
                self.meeting_created_successfully = false;
            }
            MeetingsMessage::MeetingFailed => {
                self.meeting_created_failed = false;
            }
            MeetingsMessage::OnHiddenModal(show) => {
                self.meeting_created_successfully = show;
                if !show {
                    self.show_modal_meet = false;
                    self.meeting_created_successfully = true;

                    let duration = Duration::from_secs(2).as_secs() as u32;

                    let link = ctx.link().clone();
                    let handle = Timeout::new( duration, move || {
                        link.send_message(MeetingsMessage::MeetingCreated)
                    });
                    self.job = Some(handle);
                } else {
                    self.meeting_created_successfully = false;
                }
            }
            MeetingsMessage::OnHiddenModalFailed(show) => {
                self.meeting_created_failed = show;
                if !show {
                    self.show_modal_meet = false;
                    self.meeting_created_failed = true;
                    let duration = Duration::from_secs(2).as_secs() as u32;

                    let link = ctx.link().clone();
                    let handle = Timeout::new( duration, move || {
                        link.send_message(MeetingsMessage::MeetingFailed)
                    });
                    self.job = Some(handle);

                } else {
                    self.meeting_created_failed = false;
                }
            }
            MeetingsMessage::ChangeSidebarState => {
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
        info!("{:?} => {:?}", ctx.props(), old_props);
        let mut should_render = false;

        if ctx.props() != old_props {
            should_render = true;
        } 

        should_render
    }
    fn view(&self, ctx: &Context<Self>) -> Html {
        let pic_path = ctx.props().user_profile.clone().and_then(|d| Some(d.pic_path)).unwrap_or_default();
        let full_name = ctx.props().user_profile.clone().and_then(|d| Some(d.full_name)).unwrap_or_default();
        let close_modal_callback_meet = ctx.link().callback(|_| MeetingsMessage::OnHiddenModal(false));
        let close_modal_callback_failed = ctx.link().callback(|_| MeetingsMessage::OnHiddenModalFailed(false));
        let on_show_meets = ctx.link().callback(|_| MeetingsMessage::ShowMeets);
        let on_hidden_modal_created = ctx.link().callback(|_| MeetingsMessage::MeetingCreated);
        let on_hidden_modal_failed = ctx.link().callback(|_| MeetingsMessage::MeetingFailed);
        let on_show_sidebar = ctx.link().callback(move |_| MeetingsMessage::ChangeSidebarState);
        let btn_sidebar_show = if self.saved_sidebar_state {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0 h-45" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-right fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        } else {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0 h-45" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-left fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        };

        let button_meet = html! {
            <button class="button-meeting-create bg-primary-blue-dark d-flex align-items-center justify-content-center me-md-4 me-lg-4" onclick={&on_show_meets} disabled={self.is_loading}>
                <img src="/icons/video.svg" style="height: 25px;" />
                <span class="text-white text-center noir-bold is-size-16 lh-20" style="margin-left: 8px;">{lang::dict("Start Meeting")}</span>
            </button>
        };

        let alls_school_and_meetings = self.data_school.iter().map(|school| {
            let school_id = school.school_id;
            let school_name = school.name.clone();
            html! {
                <MeetingsListBySchool school_id={school_id}
                    school_name={school_name}
                    date_selected={self.date_selected.clone()} />
            }
        }).collect::<Html>();

        let last_meetings = self.data_school.iter().map(|school_group| {
            let school_id = school_group.school_id;
            html! {
                <LastMeetingsList school_name={school_group.name.clone()}
                    school_id={school_id} />
            }
        }).collect::<Html>();

        let last_meetings_mobile = self.data_school.iter().map(|school_group| {
            let school_id = school_group.school_id;
            html! {
                <LastMeetingsList school_name={school_group.name.clone()}
                    school_id={school_id} />
            }
        }).collect::<Html>();

        let on_dropdown = ctx.link().callback(|_| MeetingsMessage::ShowDropdown);
        let dropdown_degrees = self.data_school.iter().map(|school_group| {
            let school_id = school_group.school_id;
            let inventory_group_id = school_group.inventory_id;
            let school_selected = if self
                .school_selected
                .and_then(|id| Some(id.0))
                .unwrap_or_default()
                == school_group.school_id.0 {
                    true
                } else {
                    false
                };

            let on_list_change = ctx.link().callback(move |_| MeetingsMessage::FetchSchoolList);
            let maybe_option = if school_selected {
                html! {
                    <SelectOptionDegree school_id={school_id }
                        on_list_change={on_list_change}
                        close_modal_callback_meet={close_modal_callback_meet.clone()}
                        close_modal_callback_failed={close_modal_callback_failed.clone()}
                        inventory_group_id={inventory_group_id}
                        user_profile={ctx.props().user_profile.clone()}
                        auth_school={None} />
                }
            } else {
                html! {}
            };
            html! {
                {maybe_option}
            }
        })
        .collect::<Html>();

        let change_school = self.data_school.iter().map(|class_group| {
            let class_school_selected = if self
                .school_selected
                .and_then(|id| Some(id.0))
                .unwrap_or_default()
                == class_group.school_id.0 {
                    true
                } else {
                    false
                };
            let maybe_class = if class_school_selected {
                html! {
                    <span class="universal-select-option text-secondary-purple noir-regular is-size-18 lh-22-meet">{&class_group.name}</span>
                }
            } else {
                html! {}
            };
            html! {
                {maybe_class}
            }
        })
        .collect::<Html>();

        let class_search_modal = if self.show_modal_meet {
            "modal fade show"
        } else {
            "modal fade"
        };
        let class_search_scroll = if self.show_modal_meet {
            "display: block;"
        } else {
            "display: none;"
        };
        let class_dropdown = if self.show_dropdown {
            "btn btn-secondary btn-second-meet dropdown-toggle show d-flex align-items-center justify-content-between"
        } else {
            "btn btn-secondary btn-second-meet dropdown-toggle d-flex align-items-center justify-content-between"
        };
        let class_dropdown_list = if self.show_dropdown {
            "dropdown-menu dropdown-menu-home show w-100"
        } else {
            "dropdown-menu dropdown-menu-home"
        };
        let list_schools = self.data_school.iter().map(|school_group| {
            let school_id = school_group.school_id;
            let school_id_select = format!("{:?}", school_group.school_id);
            let school_selected = if self
                .school_selected
                .and_then(|id| Some(id.0))
                .unwrap_or_default()
                == school_group.school_id.0 {
                    true
                } else {
                    false
                };
            let class_selected = if self
                .school_selected      
                .and_then(|id| Some(id.0))  
                .unwrap_or_default()
                == school_group.school_id.0 {
                    "dropdown-item bg-silver text-blue-purple noir-regular is-size-14 lh-20 d-flex align-items-center text-break-spaces"
                } else {
                    "dropdown-item text-gray-purple noir-regular is-size-14 lh-20 d-flex align-items-center text-break-spaces"
                };
            let on_school = ctx.link().callback(move |_| MeetingsMessage::SchoolChangeData(school_id));
            html! {
                <li>
                    <a class={class_selected} onclick={on_school}>
                        <input class="bg-checkbox me-1 d-flex align-items-center" type="checkbox" value={school_id_select} checked={school_selected} />
                        {&school_group.name}
                    </a>
                </li>
            }
        })
        .collect::<Html>();
        let modal_meet = html! {
            <div class={class_search_modal} id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style={class_search_scroll} aria-modal="true" role="dialog">
                <div class="modal-dialog modal-dialog-scrollable modal-xl">
                    <div class="modal-content">
                        <div class="modal-header">
                            <p class="modal-card-title text-primary-blue-dark noir-bold is-size-18 lh-22 mb-0">{"Nueva Reunión"}</p>
                            <a class="btn bg-purple-on ms-5" onclick={&on_show_meets}>
                                <span class="text-white">
                                    <i class="fas fa-times"></i>
                                </span>
                            </a>
                        </div>
                        <div class="modal-body vh-100 d-flex flex-column align-items-center">
                            <div class="dropdown dropdown-h">
                                <button class={class_dropdown} type="button" id="dropdownMenuButton2" data-bs-toggle="dropdown" aria-expanded="false" onclick={on_dropdown}>
                                    <img src="/icons/school-3.svg" style="height: 22px;" />
                                    {change_school}
                                </button>
                                <ul class={class_dropdown_list} aria-labelledby="dropdownMenuButton2">
                                    // {self.list_schools_vnode.clone()}
                                    {list_schools}
                                </ul>
                            </div>
                            {dropdown_degrees}
                        </div>
                    </div>
                </div>
            </div>
        };

        let class_modal_meeting_created = if self.meeting_created_successfully {
            "modal fade show"
        } else {
            "modal fade"
        };
        let class_modal_meeting_created_two = if self.meeting_created_successfully {
            "display: block;"
        } else {
            "display: none;"
        };
        let modal_meeting_created = html! {
            <div class={class_modal_meeting_created} id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true" style={class_modal_meeting_created_two}>
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header border-bottom-0">
                            <h5 class="modal-title text-success" id="exampleModalLabel">{"INFO"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" onclick={on_hidden_modal_created}></button>
                        </div>
                        <div class="modal-body vh-15 text-center">
                            <span class="text-success is-size-24">{"Reunión Creada Con Exito"}</span>
                        </div>
                    </div>
                </div>
            </div>
        };
        let class_modal_meeting_failed = if self.meeting_created_failed {
            "modal fade show"
        } else {
            "modal fade"
        };
        let class_modal_meeting_failed_two = if self.meeting_created_failed {
            "display: block;"
        } else {
            "display: none;"
        };
        let modal_meeting_failed = html! {
            <div class={class_modal_meeting_failed} id="exampleModal" tabindex="-1" aria-labelledby="exampleModalLabel" aria-hidden="true" style={class_modal_meeting_failed_two}>
                <div class="modal-dialog">
                    <div class="modal-content">
                        <div class="modal-header border-bottom-0">
                            <h5 class="modal-title text-danger" id="exampleModalLabel">{"INFO"}</h5>
                            <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close" onclick={on_hidden_modal_failed}></button>
                        </div>
                        <div class="modal-body vh-15 text-center">
                            <span class="text-danger text-center is-size-24">{"Fallo al crear Reunión"}</span>
                        </div>
                    </div>
                </div>
            </div>
        };
        let class_right_sidebar = if self.saved_sidebar_state {
            "bg-silver col col-sm-3 col-md-3 col-lg-5 col-xl-4 col-xxl-3 d-none d-sm-none d-md-none d-lg-block p-5"
        } else {
            "d-none"
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
        html! {
            <>
                <div class="d-flex flex-row flex-column scroll-y h-100 w-100 p-3 p-md-5 p-lg-7">
                    <div class="d-flex is-align-items-cemter justify-content-between">
                        <div class="d-flex flex-column">
                            <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">{lang::dict("Hello, ")}{full_name}</h1>
                            <div class="pt-2 pb-5 mb-4">
                                <span class="text-gray-strong noir-bold is-size-18 lh-22">{lang::dict("These are the meetings in progress")}</span>
                            </div>
                        </div>
                        <div class="d-flex flex-wrap justify-content-end">
                            {button_meet}
                            {btn_sidebar_show}
                        </div>
                    </div>
                    <div class="mb-4">{ VNode::VRef(self.node.clone()) }</div>
                    {alls_school_and_meetings}
                    {modal_meet}
                    {modal_meeting_created}
                    {modal_meeting_failed}
                </div>
                <div class={class_right_sidebar}>
                    <div class="d-flex flex-row align-items-center justify-content-between pb-5">
                        <SearchMeetingsList />
                        <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                    </div>
                    <div class="d-flex flex-column">
                        <div class="pb-4">
                            <span class="text-primary-blue-dark noir-bold is-size-24 lh-30">{lang::dict("Upcoming Meetings")}</span>
                        </div>
                        <div class="scroll-last-meets" style="overflow-y: scroll; height: 75vh; overflow-x: hidden;">{last_meetings}</div>
                    </div>
                </div>
                <div class={class_sidebar_mobile} data-bs-scroll="true" data-bs-backdrop="false" tabindex="-1" id="offcanvasScrolling" aria-labelledby="offcanvasScrollingLabel" aria-modal="true" role="dialog" style={style_sidebar_mobile}>
                    <div class="offcanvas-header d-flex justify-content-end">
                        <button type="button" class="btn btn-outline-danger" data-bs-dismiss="offcanvas" onclick={&on_show_sidebar}>
                            <i class="fas fa-times"></i>
                        </button>
                    </div>
                    <div class="offcanvas-body pt-0">
                        <div class="d-flex flex-row align-items-center justify-content-between pb-5">
                            <SearchMeetingsList />
                            <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                        </div>
                        <div class="d-flex flex-column">
                            <div class="pb-4">
                                <span class="text-primary-blue-dark noir-bold is-size-24 lh-30">{lang::dict("Upcoming Meetings")}</span>
                            </div>
                            <div class="scroll-last-meets" style="overflow-y: scroll; height: 75vh; overflow-x: hidden;">{last_meetings_mobile}</div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
            let date = Local::now().date_naive().to_string();
    
            let on_result_selected = ctx.link().callback(move |date| MeetingsMessage::DateFromVCalendar(date));
            if first_render {
                self.date_selected(
                    date,
                    on_result_selected
                );
            }
        
    }
}

#[wasm_bindgen(module = "/src/v-calendar.js")]
extern "C" {
    #[wasm_bindgen(js_name = "app_vcalendar")]
    fn render_calendar_js(node: &Node, date_meet: String, on_result_selected: &JsValue);
}

fn app_vcalendar(node: &Node, date_meet: String, on_result_selected: &Closure<dyn Fn(String)>,) {
    render_calendar_js(node, date_meet, on_result_selected.as_ref())
}