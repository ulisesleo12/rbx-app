use yew::prelude::*;

pub struct Msg;

pub struct UserRobotsPlaceholder;

impl Component for UserRobotsPlaceholder {
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