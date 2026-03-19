use yew::prelude::*;

pub struct Msg;

pub struct CardPostListPlaceholder;

impl Component for CardPostListPlaceholder {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
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