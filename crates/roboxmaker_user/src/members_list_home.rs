use log::*;
use yew::prelude::*;
use code_location::code_location;
use yew::{html, Component, ComponentLink, Html, ShouldRender};

use roboxmaker_main::lang;
use roboxmaker_models::{user_model, school_model};
use roboxmaker_types::types::{GroupId, UserId, AppRoute, MyUserProfile};
use roboxmaker_graphql::{GraphQLService, GraphQLTask, Subscribe, SubscriptionTask};
use roboxmaker_loaders::placeholders::card_members_home::CardMembersHomePlaceholder;

#[derive(Debug, Clone)]
enum LoadMembersFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadMembers {
    Loading,
    Load(LoadMembersFound),
}

#[derive(Debug, Clone, PartialEq)]
pub struct MemberProfile {
    pub full_name: String,
    pub pic_path: String,
    pub user_id: UserId,
    pub user_staff: bool,
    pub user_teacher: bool,
    pub user_student: bool,
}

pub struct MembersListHome {
    link: ComponentLink<Self>,
    props: MembersListHomeProps,
    graphql_task: Option<GraphQLTask>,
    task: Option<SubscriptionTask>,
    members_list: Vec<MemberProfile>,
    list_members_state: LoadMembers,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MembersListHomeProps {
    pub group_id: GroupId,
    pub on_app_route: Callback<AppRoute>,
    pub auth_school: Option<school_model::school_by_id::SchoolByIdSchoolByPk>,
    pub user_profile: Option<MyUserProfile>,
}

#[derive(Debug)]
pub enum MembersListHomeMessage {
    AppRoute(AppRoute),
    FetchMembersByGroupId,
    Members(Option<user_model::members_by_group_id::ResponseData>),
}

impl Component for MembersListHome {
    type Message = MembersListHomeMessage;
    type Properties = MembersListHomeProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(MembersListHomeMessage::FetchMembersByGroupId);
        MembersListHome {
            link,
            props,
            graphql_task: Some(GraphQLService::connect(&code_location!())),
            task: None,
            members_list: vec![],
            list_members_state: LoadMembers::Loading,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            MembersListHomeMessage::AppRoute(route) => {
                self.props.on_app_route.emit(route);
            }
            MembersListHomeMessage::FetchMembersByGroupId => {
                self.list_members_state = LoadMembers::Loading;
                let group_id = self.props.group_id;
                if let Some(graphql_task) = self.graphql_task.as_mut() {

                    let vars = user_model::members_by_group_id::Variables { 
                        group_id: group_id.0,
                    };

                    let task = user_model::MembersByGroupId::subscribe(
                        graphql_task,
                        &self.link,
                        vars,
                        |response| {
                            MembersListHomeMessage::Members(response)
                        },
                    );
                    self.task = Some(task);
                }
            }
            MembersListHomeMessage::Members(response) => {
                self.members_list = response
                    .clone()
                    .and_then(|data| Some(data.user_profile))
                    .unwrap_or(vec![])
                    .iter()
                    .map(|user_profile| {
                        let full_name = user_profile.full_name.clone();
                        let pic_path = user_profile.pic_path.clone().unwrap_or("".to_string());
                        let user_staff = user_profile.user_staff.is_some();
                        let user_teacher = user_profile.user_teacher.is_some();
                        let user_student = user_profile.user_student.is_some();
                        let user_id = UserId(user_profile.user_id);
                        MemberProfile {
                            full_name: full_name,
                            pic_path: pic_path,
                            user_id: user_id,
                            user_staff: user_staff,
                            user_teacher: user_teacher,
                            user_student: user_student,
                        }
                    }).collect();
                if !response.clone().and_then(|data| Some(data.user_profile)).unwrap_or(vec![]).is_empty() {
                    self.list_members_state = LoadMembers::Load(LoadMembersFound::Found);
                } else {
                    self.list_members_state = LoadMembers::Load(LoadMembersFound::NotFound);
                }
            }
        }
        should_render
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        info!("{:?} => {:?}", self.props, props);
        let mut should_render = false;

        if self.props.group_id != props.group_id {
            self.link.send_message(MembersListHomeMessage::FetchMembersByGroupId);
        }

        if self.props != props {
            self.props = props;
            should_render = true;
        } 
        
        should_render
    }

    fn view(&self) -> Html {
        let card_members_list = self.members_list.iter().map(|item| {
            let maybe_i_am = self.props.user_profile.as_ref().and_then(|user| {
                if user.user_id == item.user_id {
                    Some(html! {
                        <span class="ms-2">{"Tú"}</span>
                    })
                } else {
                    None
                }
            }).unwrap_or(html! {});
            let maybe_user_type = {
                if item.user_staff {
                    html! {
                        <span class="text-brown noir-light is-size-13 lh-22 ">{lang::dict("Staff")}</span>
                    }
                } else if item.user_teacher {
                    html! {
                        <span class="text-brown noir-light is-size-13 lh-22 ">{lang::dict("Teacher")}</span>
                    }
                } else {
                    html! {
                        <span class="text-brown noir-light is-size-13 lh-22 ">{lang::dict("Student")}</span>
                    }
                }
            };
            html! {
                <div class="card-members-user-home bg-white mb-4">
                    <img class="img-card-48" src=item.pic_path.clone() />
                    <div class="d-flex flex-column ms-3">
                        <span class="text-primary-blue-dark noir-bold is-size-18 lh-22">
                                {&item.full_name} {maybe_i_am}
                        </span>
                        <span class="text-brown noir-light is-size-13 lh-22 ">
                            {maybe_user_type}
                        </span>
                    </div>
                </div>
            }
        }).collect::<Html>();
        
        let members_list = match self.list_members_state {
            LoadMembers::Loading => {
                html! {
                    <>
                        <CardMembersHomePlaceholder />
                        <CardMembersHomePlaceholder />
                        <CardMembersHomePlaceholder />
                        <CardMembersHomePlaceholder />
                    </>
                }
            },
            LoadMembers::Load(LoadMembersFound::Found) => {
                html! {
                    {card_members_list}
                }
            },
            LoadMembers::Load(LoadMembersFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <p class="is-size-5">{lang::dict("No members here.")}</p>
                    </div>
                }
            },
        };
        html! {
            <>
                {members_list}
            </>
        }
    }
}