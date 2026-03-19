use yew::prelude::*;

pub struct Msg;

pub struct CardMembersHomePlaceholder;

impl Component for CardMembersHomePlaceholder {
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
            <div class="card-members-home is-loading-members-home mb-4">
                <div class="image-members-home"></div>
                <div class="content-members-home">
                    <h2></h2>
                    <p></p>
                </div>
            </div>
        }
    }
}