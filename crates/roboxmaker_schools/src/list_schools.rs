use log::*;
use uuid::Uuid;
use gloo_storage::Storage;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, Html};
use serde_derive::{Deserialize, Serialize};
use yew_router::scope_ext::RouterScopeExt;
use crate::{data_schools::DataSchools, school_view::SchoolPage};

use roboxmaker_main::lang;
use roboxmaker_models::school_model;
use roboxmaker_searches::search_school_list::SearchSchoolList;
use roboxmaker_loaders::fullscreen_loader_schools::FullScreenLoaderSchools;
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};
use roboxmaker_types::types::{SchoolId, AppRoute, LoadResponse, LoadResponseFound, MyUserProfile};


#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SchoolProfile {
    pub name: String,
    pub logo: String,
    pub address: String,
    pub web_site: String,
    pub code: String,
    pub mission: String,
    pub motto: String,
    pub telephone: String,
    pub vision: String,
    pub we_are: String,
    pub school_id: Uuid,
}

pub struct ListOfSchoolsView {
    graphql_task: Option<GraphQLTask>,
    school_sub: Option<SubscriptionTask>,
    school_list: Vec<SchoolProfile>,
    list_schools_state: LoadResponse,
    school_selected: Option<SchoolProfile>,
    saved_sidebar_state: bool,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct ListOfSchoolsProps {
    // pub on_app_route: Callback<AppRoute>,
    pub user_profile: Option<MyUserProfile>,
}

#[derive(Debug)]
pub enum ListOfSchoolsMessage {
    // AppRoute(AppRoute),
    FetchSchools,
    Schools(Option<school_model::list_school_roboxmaker::ResponseData>),
    SchoolChangeData(Option<SchoolProfile>),
    HideSchoolProfile,
    ChangeSidebarState,
}

impl Component for ListOfSchoolsView {
    type Message = ListOfSchoolsMessage;
    type Properties = ListOfSchoolsProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(ListOfSchoolsMessage::FetchSchools);

        let saved_sidebar_state = if let Ok(value) = gloo_storage::LocalStorage::get("saved_sidebar_state") {
            value 
        } else {
            true
        };

        roboxmaker_utils::functions::school_state();

        ListOfSchoolsView {
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            school_sub: None,
            school_list: vec![],
            school_selected: None,
            list_schools_state: LoadResponse::Loading,
            saved_sidebar_state,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_update = true;
        match msg {
            // ListOfSchoolsMessage::AppRoute(route) => {
            //     ctx.props().on_app_route.emit(route);
            // }
            ListOfSchoolsMessage::FetchSchools => {
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = school_model::list_school_roboxmaker::Variables { };

                    let task = school_model::ListSchoolRoboxmaker::subscribe(
                            graphql_task,
                            &ctx,
                            vars,
                            |response| {
                                ListOfSchoolsMessage::Schools(response)
                            },
                    );
                    self.school_sub = Some(task);
                }
            }
            ListOfSchoolsMessage::Schools(response) => {
                self.school_list = response
                    .clone()
                    .and_then(|data| Some(data.school_profile))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|school| {
                        SchoolProfile {
                            name: school.name.clone(),
                            logo: school.logo.clone().unwrap_or("https://files.roboxmaker.com/uploads/school.png".to_string()),
                            address: school.address.clone().unwrap_or(lang::dict("No address").to_string()),
                            web_site: school.biography.clone().unwrap_or(lang::dict("Add school website").to_string()),
                            code: school.code.clone(),
                            mission: school.mission.clone().unwrap_or(lang::dict("Add a school mission").to_string()),
                            motto: school.motto.clone().unwrap_or(lang::dict("motto school").to_string()),
                            telephone: school.telephone.clone().unwrap_or(lang::dict("No telephone").to_string()),
                            vision: school.vision.clone().unwrap_or(lang::dict("Add a school vision").to_string()),
                            we_are: school.we_are.clone().unwrap_or(lang::dict("No information about us").to_string()),
                            school_id: school.school_id,
                        }
                    })
                    .collect();
                if !response.clone().and_then(|data| Some(data.school_profile)).unwrap_or(vec![]).is_empty() {
                    self.list_schools_state = LoadResponse::Load(LoadResponseFound::Found);
                } else {
                    self.list_schools_state = LoadResponse::Load(LoadResponseFound::NotFound);
                }
                if let Some(school_profile)  = &self.school_selected.clone() {
                    for school in self.school_list.clone() {
                        if school.school_id == school_profile.school_id {
                            self.school_selected = Some(school)
                        }
                    }
                }
            }
            ListOfSchoolsMessage::SchoolChangeData(school_selected) => {
                self.school_selected = school_selected;

                self.saved_sidebar_state = true;
            }
            ListOfSchoolsMessage::HideSchoolProfile=> {
                self.school_selected = None;
            }
            ListOfSchoolsMessage::ChangeSidebarState => {
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
        let on_show_sidebar = ctx.link().callback(move |_| ListOfSchoolsMessage::ChangeSidebarState);
        let btn_sidebar_show = if self.saved_sidebar_state {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-right fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        } else {
            html! {
                <button type="button" class="btn btn-outline-primary-blue-dark rounded-start rounded-0" onclick={&on_show_sidebar}>
                    <i class="fas fa-angle-double-left fas fa-2x" id="show-sidebar-right"></i>
                </button>
            }
        };

        let maybe_user_profile_name = ctx
            .props()
            .user_profile
            .as_ref()
            .and_then(|item| {
                Some(html! {
                    {&item.full_name}
                })
            })
            .unwrap_or(html! {});

        let welcome_class_view = html! {
            <div class="d-flex align-items-center justify-content-between">
                <div class="d-flex flex-column">
                    <h1 class="text-primary-blue-dark text-uppercase noir-bold is-size-36 lh-43 mb-0">{lang::dict("Hello, ")}
                        {maybe_user_profile_name}
                    </h1>
                    <span class="text-brown noir-regular is-size-18 lh-22">{"Estos son los colegios a tu cargo"}</span>
                </div>
                {btn_sidebar_show}
            </div>
        };
        let collages = html! {
            <div class="mt-6 mb-5">
                <span class="text-primary-blue-dark noir-bold is-size-20 lh-25">{"Colegios"}</span>
            </div>
        };
        let all_schools = self.school_list.iter().map(|item| {
            let navigator = ctx.link().navigator().unwrap();

            let school = Some(item.clone());
            let school_id = SchoolId(item.school_id);
            let on_show_school = Callback::from(move |_| navigator.push(&AppRoute::GradesBySchoolId{school_id}));
            let on_show_info = ctx.link().callback(move |_| ListOfSchoolsMessage::SchoolChangeData(school.clone()));
            
            html! {
                <div class="schools-card schools-card mb-2 mb-md-5 mb-lg-7 me-md-3 me-lg-5">
                    <div class="d-flex flex-row px-5 pt-5">
                        <img src={item.logo.clone()} alt="photo of user" class="img-card-56" />
                        <div class="d-flex flex-column w-100 ps-3">
                            <div class="d-flex justify-content-between">
                                <span class="text-primary-blue-light noir-bold is-size-18 lh-22 text-truncate" style="width: 260px;">{&item.name}</span>
                                <span class="text-gray-purple">
                                    <i class="fas fa-ellipsis-v"></i>
                                </span>
                            </div>
                            <span class="text-gray-purple noir-regular is-size-14 lh-18 d-flex align-items-center">
                                <span class="me-1">
                                    <i class="fas fa-map-marked"></i>
                                </span>
                                <span class="addres-school-text">{&item.address}</span>
                            </span>
                        </div>
                    </div>
                    <hr class="hr-schools" />
                    <DataSchools user_profile={ctx.props().user_profile.clone()}
                        school_id={school_id} />
                    <div class="ps-5">
                        <div class="d-flex flex-wrap">
                            <a class="see-info-school-btn text-purple-gray noir-bold is-size-16 lh-20 me-2" onclick={on_show_info.clone()}>{"Ver Info"}</a>
                            <a class="open-school-btn bg-primary-blue-dark text-white noir-bold is-size-16 lh-20" onclick={on_show_school.clone()}>{"Abrir"}</a>
                        </div>
                    </div>
                </div>
            }
        }).collect::<Html>();

        let select_on_school = self.school_selected.clone().and_then(|item| {
            let school_profile = item.clone();
            let on_hidden_info = ctx.link().callback(move |_| ListOfSchoolsMessage::SchoolChangeData(None));

            Some(html! {
                <SchoolPage user_profile={ctx.props().user_profile.clone()}
                    school_profile={school_profile}
                    close_school_profile={on_hidden_info} />  
            })
        }).unwrap_or(html! {
            <div class="d-flex flex-column align-items-center">
                <span class="text-purple-gray mt-335">
                    <i class="far fa-question-circle fas fa-2x"></i>
                </span>
                <p class="text-purple-gray noir-regular is-size-18 lh-22 text-center pt-5">{lang::dict("Select “View Info” of any School in the list to see its General information")}</p>
            </div>
        });

        let select_on_school_mobile = self.school_selected.clone().and_then(|item| {
            let school_profile = item.clone();
            let on_hidden_info = ctx.link().callback(move |_| ListOfSchoolsMessage::SchoolChangeData(None));

            Some(html! {
                <SchoolPage user_profile={ctx.props().user_profile.clone()}
                    school_profile={school_profile}
                    close_school_profile={on_hidden_info} />  
            })
        }).unwrap_or(html! {
            <div class="d-flex flex-column align-items-center">
                <span class="text-purple-gray mt-335">
                    <i class="far fa-question-circle fas fa-2x"></i>
                </span>
                <p class="text-purple-gray noir-regular is-size-18 lh-22 text-center pt-5">{lang::dict("Select “View Info” of any School in the list to see its General information")}</p>
            </div>
        });

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
        let pic_path = ctx.props().user_profile.clone().and_then(|d| Some(d.pic_path)).unwrap_or_default();
        let maybe_select_school = html! {
            <div class="w-100 h-100 d-flex flex-row justify-content-between">
                <div class="w-100 scroll-y pt-3 ps-3 pt-md-4 ps-md-4 pt-lg-7 ps-lg-7">
                    {welcome_class_view}
                    {collages}
                    <div class="d-flex flex-wrap">
                        {all_schools}
                    </div>
                </div>
                <div class={class_right_sidebar}>
                    <div class="d-flex align-items-center justify-content-between pb-4">
                        <SearchSchoolList />
                        <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                    </div>
                    {select_on_school.clone()}
                </div>
                <div class={class_sidebar_mobile} data-bs-scroll="true" data-bs-backdrop="false" tabindex="-1" id="offcanvasScrolling" aria-labelledby="offcanvasScrollingLabel" aria-modal="true" role="dialog" style={style_sidebar_mobile}>
                    <div class="offcanvas-header d-flex justify-content-end">
                        <button type="button" class="btn btn-outline-danger" data-bs-dismiss="offcanvas" onclick={&on_show_sidebar}>
                            <i class="fas fa-times"></i>
                        </button>
                    </div>
                    <div class="offcanvas-body pt-0">
                        <div class="d-flex align-items-center justify-content-between pb-4">
                            <SearchSchoolList />
                            <img class="img-card-72" src={pic_path.clone()} alt="photo of user" />
                        </div>
                        {select_on_school_mobile.clone()}
                    </div>
                </div>
            </div>
        };
        let schools_view = match self.list_schools_state {
            LoadResponse::Loading => {
                html! {
                    <FullScreenLoaderSchools />
                }
            },
            LoadResponse::Load(LoadResponseFound::Found) => {
                html! {
                    {maybe_select_school}
                }
            },
            LoadResponse::Load(LoadResponseFound::NotFound) => {
                html! {
                    <FullScreenLoaderSchools />
                }
            },
        };
        html! {
            {schools_view}
        }
    }
}