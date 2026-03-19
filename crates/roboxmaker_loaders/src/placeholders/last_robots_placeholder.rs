use yew::prelude::*;

pub struct Msg;

pub struct LastRobotsPlaceholder;

impl Component for LastRobotsPlaceholder {
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