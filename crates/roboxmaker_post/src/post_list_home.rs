use log::*;
use yew::prelude::*;
use roboxmaker_main::lang;
use yew::{html, Component, Html};
use yew_router::scope_ext::RouterScopeExt;

use roboxmaker_loaders::placeholders::card_post_placeholder::CardPostPlaceholder;
use roboxmaker_types::types::{GroupId, AppRoute, SchoolId, MyUserProfile, PostProfile};


#[derive(Debug, Clone)]
enum LoadPostFound {
    Found,
    NotFound,
}

#[derive(Debug, Clone)]
enum LoadPosts {
    Loading,
    Load(LoadPostFound),
}

pub struct PostListHome {
    post_list: Vec<PostProfile>,
    list_posts_state: LoadPosts,

}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct PostListHomeProps {
    pub group_id: GroupId,
    pub user_profile: Option<MyUserProfile>,
    pub school_id: SchoolId,
    pub post_list: Vec<PostProfile>,
}

#[derive(Debug)]
pub enum PostListHomeMessage {
    FetchPostsByGroupId,
}

impl Component for PostListHome {
    type Message = PostListHomeMessage;
    type Properties = PostListHomeProps;

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(PostListHomeMessage::FetchPostsByGroupId);

        PostListHome {
            post_list: vec![],
            list_posts_state: LoadPosts::Loading,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("{:?}", msg);
        let should_render = true;
        match msg {
            PostListHomeMessage::FetchPostsByGroupId => {

                self.list_posts_state = LoadPosts::Loading;

                self.post_list = ctx.props().post_list.clone();

                if !self.post_list.is_empty() {
                    self.list_posts_state = LoadPosts::Load(LoadPostFound::Found);
                } else {
                    self.list_posts_state = LoadPosts::Load(LoadPostFound::NotFound);
                }
            }
        }
        should_render
    }

    fn changed(&mut self, ctx: &Context<Self>, old_props: &Self::Properties) -> bool {
        // info!("{:?} => {:?}", ctx.props(), old_props);

        if ctx.props().post_list != old_props.post_list {
            ctx.link().send_message(PostListHomeMessage::FetchPostsByGroupId);
        }
    
        ctx.props() !=  old_props

    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let group_id = ctx.props().group_id;
        let school_id = ctx.props().school_id;
        let user = ctx.props().user_profile.clone();

        let post_card_list = self
            .post_list
            .iter()
            .map(|item| {
            let post_id = item.post_id;

            let navigator = ctx.link().navigator().unwrap();
            let on_post_view = Callback::from(move |_| navigator.push(&AppRoute::PostView{school_id, group_id, post_id}));


            let maybe_time = if user.clone().and_then(|d| d.user_staff).is_some() || user.clone().and_then(|d| d.user_teacher).is_some() {
                html! {
                    <span class="text-brown noir-light is-size-13 lh-22 ">{&item.timestamp}</span>
                }
            } else {
                html! {
                    <span class="text-brown noir-light is-size-13 lh-22 ">{&item.maybe_timestamp}</span>
                }
            };
            html! {
                <div class="card-post-view-home bg-white d-flex flex-column justify-content-between align-items-center p-5 me-5">
                    <a onclick={&on_post_view}>
                        <div class="module-message-post line-clamp-message-post">
                            <span class="text-blue-two text-justify noir-medium is-size-18 lh-22 ">
                                {&item.topic}
                            </span>
                        </div>
                    </a>
                    <a class="w-100" onclick={&on_post_view}>
                        <div class="d-flex align-items-center justify-content-between">
                            <img src={item.pic_path.clone()} class="img-card-32" />
                            <span class="text-dark noir-light is-size-14 lh-17 text-truncate col-5 mb-0">
                                {&item.full_name}
                            </span>
                            <div class="ms-2">
                                <span class="text-brown noir-light is-size-13 lh-22  d-flex align-items-center">
                                    <i class="far fa-clock me-1"></i>
                                    <div class="d-flex flex-wrap">
                                        // <span class="text-brown noir-light is-size-13 lh-22 ">{&item.timestamp}</span>
                                        {maybe_time}
                                    </div>
                                </span>
                            </div>
                        </div>
                    </a>
                </div>
            }
        }).collect::<Html>();
        let posts_list = match self.list_posts_state {
            LoadPosts::Loading => {
                html! {
                    <>
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                        <CardPostPlaceholder />
                    </>
                }
            },
            LoadPosts::Load(LoadPostFound::Found) => {
                html! {
                    {post_card_list}
                }
            },
            LoadPosts::Load(LoadPostFound::NotFound) => {
                html! {
                    <div class="text-center">
                        <p class="is-size-5">{lang::dict("No posts here.")}</p>
                    </div>
                }
            },
        };
        html! {
            <div class="d-flex flex-row">   
                {posts_list}
            </div>
        }
    }
}