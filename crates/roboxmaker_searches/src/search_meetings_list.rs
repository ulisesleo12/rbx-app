use log::*;
use uuid::Uuid;
use chrono::Local;
use yew::{prelude::*, web_sys};
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::meetings_model;
use roboxmaker_types::types::{AppRoute, GroupId, MeetingsId};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Request, RequestTask};

#[derive(Debug, Clone)]
enum LoadSearchFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadSearch {
    Static,
    Load(LoadSearchFound),
}
pub struct SearchMeetingsList {
    link: ComponentLink<Self>,
    props: SearchMeetingsListProps,
    search_node: NodeRef,
    graphql_task: Option<GraphQLTask>,
    search_task: Option<RequestTask>,
    meetings_search: Vec<meetings_model::search_meetings_all_schools::SearchMeetingsAllSchoolsMeetings>,
    maybe_section_search: bool,
    list_search_state: LoadSearch,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct SearchMeetingsListProps {
    pub on_app_route: Callback<AppRoute>,
}

#[derive(Debug)]
pub enum SearchMeetingsListMessage {
    AppRoute(AppRoute),
    FetchMeetingsSearch(String),
    Meetings(Option<meetings_model::search_meetings_all_schools::ResponseData>),
    OnFocus,
    OnBlur,
    HiddenModal,
}

impl Component for SearchMeetingsList {
    type Message = SearchMeetingsListMessage;
    type Properties = SearchMeetingsListProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        SearchMeetingsList { 
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            search_task: None,
            search_node: NodeRef::default(),
            meetings_search: vec![],
            maybe_section_search: false,
            list_search_state: LoadSearch::Static,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            SearchMeetingsListMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route)
            }
            SearchMeetingsListMessage::FetchMeetingsSearch(search) => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {
                    let scheduled_meetings = Local::now().date_naive();
                    let end_of_meetings = Local::now().time();
                    let vars = meetings_model::search_meetings_all_schools::Variables { 
                        search: search, 
                        scheduled_meetings: scheduled_meetings,
                        end_of_meetings: end_of_meetings,
                    };

                    let task = meetings_model::SearchMeetingsAllSchools::request(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            SearchMeetingsListMessage::Meetings(response)
                        },
                    );
                    self.search_task = Some(task);
                }
            }
            SearchMeetingsListMessage::Meetings(response) => {
                self.meetings_search = response.clone()
                    .and_then(|data| Some(data.meetings)).unwrap_or(vec![]);

                if !response.clone().and_then(|data| Some(data.meetings)).unwrap_or(vec![]).is_empty() {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::Found);
                } else {
                    self.list_search_state = LoadSearch::Load(LoadSearchFound::NotFound);
                }
            }
            SearchMeetingsListMessage::OnFocus => {
                self.maybe_section_search = true;
            }
            SearchMeetingsListMessage::OnBlur => {
                if let Some(input) = self.search_node.cast::<web_sys::HtmlInputElement>() {
                    input.set_value("");
                }
            }
            SearchMeetingsListMessage::HiddenModal => {
                self.maybe_section_search = !self.maybe_section_search;
                self.meetings_search = vec![];
                self.list_search_state = LoadSearch::Static;
            }
        }
        should_update
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        trace!("{:?} => {:?}", self.props, props);
        let mut should_render = false;
        if self.props != self.props {
            self.props = props;
            should_render = true;
        }
        should_render
    }
    fn view(&self) -> Html {
        let on_focus = self.link.callback(move |_| SearchMeetingsListMessage::OnFocus);
        let on_blur = self.link.callback(move |_| SearchMeetingsListMessage::OnBlur);
        let on_hidden_modal = self.link.callback(move |_| SearchMeetingsListMessage::HiddenModal);
        let on_search = self.link.callback(|search: InputData| {
            info!("search: {:?}", search);
            let search = if search.value.len() > 0 {
                format!("%{}%", search.value)
            } else {
                search.value
            };
            SearchMeetingsListMessage::FetchMeetingsSearch(search)
        });
        let meetings_response = self
            .meetings_search
            .iter()
            .map(|meets| {
                let meetings_groups = meets.meetings_groups.clone();
                let meetings_profile = meetings_groups.iter().map(|data| {
                    let title = data.meetings_profile.clone().and_then(|data| Some(data.title)).unwrap_or("".to_string());
                    let group_id = GroupId(data.group_id);
                    let id = data.meetings_profile.clone().and_then(|data| Some(data.meet_id)).unwrap_or(Uuid::default());
                    let meetings_id = MeetingsId(id);
                    let on_go_meet = self.link.callback(move |_| SearchMeetingsListMessage::AppRoute(AppRoute::Meet(group_id, meetings_id)));
                    html! {
                        <div class="m-4">
                            <div class="card card-search-u vh-15">
                                <div class="module-message-universal line-clamp-message-universal p-2 h-80">
                                    <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                        {&title}
                                    </span>
                                </div>
                                <div class="card-body border-top d-flex px-5 py-2">
                                    <a class="btn btn-outline-secondary btn-sm mx-auto" onclick={&on_go_meet}>
                                        <span>
                                            {lang::dict("Go to video call")}
                                        </span>
                                    </a>
                                </div>
                            </div>
                        </div>
                    }
                }).collect::<Html>();
                html! {
                    {meetings_profile}
                }
            })
            .collect::<Html>();
        let maybe_message_response = match self.list_search_state {
            LoadSearch::Static => {
                html! {
                    <div class="text-center">
                        <span class="text-brown noir-regular is-size-18 lh-22">{lang::dict("Write in the search engine to see your list of")}<span class="ps-2">{lang::dict("Meetings")}</span></span>
                    </div>
                }
            },
            LoadSearch::Load(LoadSearchFound::Found) => {
                html! {
                    <div class="d-flex flex-wrap justify-content-center">{ meetings_response }</div>
                }
            },
            LoadSearch::Load(LoadSearchFound::NotFound) => {
                html! {
                    <div class="d-flex justify-content-center">
                        <span class="text-danger is-size-20 lh-20">{"No se encontró en "}{lang::dict("Meetings")}</span>
                    </div>
                }
            },
        };
        let class_search_modal = if self.maybe_section_search {
            "modal fade show"
        } else {
            "modal fade"
        };
        let class_search_scroll = if self.maybe_section_search {
            "display: block;"
        } else {
            "display: none;"
        };
        html! {
            <>
                <a class="button-search-univeral mt-3 me-5" onclick=&on_hidden_modal>
                    <span class="icon-text-search-universal">
                        <span>{lang::dict("Search")}</span>
                        <span class="icon">
                            <i class="fas fa-search"></i>
                        </span>
                    </span>
                </a>
                <div class=class_search_modal id="exampleModalScrollable" tabindex="-1" aria-labelledby="exampleModalScrollableTitle" style=class_search_scroll aria-modal="true" role="dialog">
                    <div class="modal-dialog modal-dialog-scrollable modal-xl">
                        <div class="modal-content">
                            <div class="modal-header">
                                <div class="input-group">
                                    <span class="input-group-text text-primary-blue-dark input-group-search">
                                        <i class="fas fa-search"></i>
                                    </span>
                                    <input type="text" class="form-control input-style-class" ref=self.search_node.clone()
                                        oninput=on_search.clone() onfocus=on_focus.clone() onblur=on_blur.clone() placeholder=lang::dict("Search") />
                                </div>
                                <a class="btn bg-purple-on ms-5" onclick=&on_hidden_modal>
                                    <span class="text-white">
                                        <i class="fas fa-times"></i>
                                    </span>
                                </a>
                            </div>
                            <div class="modal-body vh-100">
                                {maybe_message_response}
                            </div>
                        </div>
                    </div>
                </div>
            </>
        }
    }
}
