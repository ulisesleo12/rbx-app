use yew::prelude::*;

pub struct Msg;

pub struct LastRobotsPlaceholder;

impl Component for LastRobotsPlaceholder {
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
            <div class="card-last-robots is-loading-last-robot p-5">
                <div class="image-last-robot"></div>
                <div class="content-last-robot">
                    <h2></h2>
                    <p></p>
                </div>
            </div>
        }
    }
}