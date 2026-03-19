use yew::prelude::*;

pub struct Msg;

pub struct CardRobotsPlaceholder;

impl Component for CardRobotsPlaceholder {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
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