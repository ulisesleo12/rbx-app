use log::info;
use yew::prelude::*;

use roboxmaker_types::types::PostPageContent;

pub struct PostContentAuthor {
    props: PostContentAuthorProps,
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct PostContentAuthorProps {
    pub post_page_content: PostPageContent,
}

#[derive(Debug)]
pub enum ResponderMsg {}

impl Component for PostContentAuthor {
    type Message = ResponderMsg;
    type Properties = PostContentAuthorProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        PostContentAuthor {
            props,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("{:?}", msg);
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        html! {
            <div class="mt-7">
                <div class="d-flex flex-wrap align-items-center justify-content-between pt-5 mb-2">
                    <div class="d-flex align-items-center">
                        <img class="img-card-32" src={ self.props.post_page_content.author_pic_path.clone() } alt="" style="height: 32px; object-fit: cover;" />
                        <span class="text-dark noir-light is-size-18 lh-22 ps-2">{ &self.props.post_page_content.author_full_name }</span>
                    </div>
                    <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                        <i class="far fa-clock"></i>
                        <span class="ps-2">{ &self.props.post_page_content.timestamp }</span>
                    </span>
                    <span class="text-gray-purple-two noir-light is-size-18 lh-22 d-flex align-items-center">
                        <i class="fas fa-graduation-cap"></i>
                        <span class="ps-2">{ self.props.post_page_content.class_name.clone() }</span>
                    </span>
                </div>
            </div>
        }
    }
}