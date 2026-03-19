use yew::prelude::*;

pub struct Msg;

pub struct UserRobotsPlaceholder;

impl Component for UserRobotsPlaceholder {
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
            <div class="card-user-robots is-loading-user-robot p-5">
                <div class="image-user-robot"></div>
                <div class="content-user-robot">
                    <h2></h2>
                    <p></p>
                </div>
            </div>
        }
    }
}