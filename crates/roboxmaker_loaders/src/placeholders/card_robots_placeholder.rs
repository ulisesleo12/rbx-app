use yew::prelude::*;

pub struct Msg;

pub struct CardRobotsPlaceholder;

impl Component for CardRobotsPlaceholder {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="card-robots is-loading-robot p-5 me-5">
                <div class="image-robot"></div>
                <div class="content-robot">
                    <h3></h3>
                    <h2></h2>
                    <p></p>
                </div>
            </div>
        }
    }
}