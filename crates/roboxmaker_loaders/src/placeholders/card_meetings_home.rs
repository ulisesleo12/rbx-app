use yew::prelude::*;

pub struct Msg;

pub struct CardMeetingssHomePlaceholder;

impl Component for CardMeetingssHomePlaceholder {
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
            <div class="card-meetings-home is-loading-meetings-home mb-4">
                <div class="content-meetings-home">
                    <h2></h2>
                    <h3></h3>
                    <h4></h4>
                    <p></p>
                </div>
            </div>
        }
    }
}