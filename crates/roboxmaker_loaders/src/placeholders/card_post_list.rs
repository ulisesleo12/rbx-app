use yew::prelude::*;

pub struct Msg;

pub struct CardPostListPlaceholder;

impl Component for CardPostListPlaceholder {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="card-post-list is-loading-post-list p-4 mb-4 w-100">
                <div class="content-post-list-header">
                    <div class="image-post-list-2"></div>
                    <div class="menu-post-list"></div>
                </div>
                <div class="content-post-list">
                    <div class="image-post-list"></div>
                    <h2></h2>
                    <h3></h3>
                    <h4></h4>
                    <h5></h5>
                    <p></p>
                </div>
            </div>
        }
    }
}