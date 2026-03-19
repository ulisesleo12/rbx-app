use log::*;
use yew::prelude::*;
use yew::{html, Component, Html};

use roboxmaker_main::lang;
use roboxmaker_types::types::{GroupId, MyUserProfile, MemberProfile};
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

pub struct MembersListHome {
    members_list: Vec<MemberProfile>,
    list_members_state: LoadMembers,
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct MembersListHomeProps {
    pub group_id: GroupId,
    pub user_profile: Option<MyUserProfile>,
    pub members_list: Vec<MemberProfile>,
}

#[derive(Debug)]
pub enum MembersListHomeMessage {
    FetchMembersByGroupId,
}

impl Component for MembersListHome {
    type Message = MembersListHomeMessage;
    type Properties = MembersListHomeProps;

    fn create(ctx: &Context<Self>) -> Self {

        ctx.link().send_message(MembersListHomeMessage::FetchMembersByGroupId);

        MembersListHome {
            members_list: vec![],
            list_members_state: LoadMembers::Loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            MembersListHomeMessage::FetchMembersByGroupId => {
                self.list_members_state = LoadMembers::Loading;

                self.members_list = ctx.props().members_list.clone();

                if !self.members_list.is_empty() {
                    self.list_members_state = LoadMembers::Load(LoadMembersFound::Found);
                } else {
                    self.list_members_state = LoadMembers::Load(LoadMembersFound::NotFound);
                }
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // info!("{:?} => {:?}", ctx.props(), old_props);

        if ctx.props().members_list != old_props.members_list {
            ctx.link().send_message(MembersListHomeMessage::FetchMembersByGroupId);
        }

        ctx.props() != old_props
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        let card_members_list = self.members_list.iter().map(|item| {
            let maybe_i_am = ctx.props().user_profile.as_ref().and_then(|user| {
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
                    <img class="img-card-48" src={item.pic_path.clone()} />
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