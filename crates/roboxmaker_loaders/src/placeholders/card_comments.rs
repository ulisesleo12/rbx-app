use yew::prelude::*;

pub struct Msg;

pub struct CardCommentsPlaceholder;

impl Component for CardCommentsPlaceholder {
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
            <div class="card-comments is-loading-comments p-4">
                <div class="image-comments"></div>
                <div class="txt-comments">
                    <h2></h2>
                    <h4></h4>
                    <div class="enabled-comments">
                        <p></p>
                        <h3></h3>
                        <h5></h5>
                    </div>
                </div>
            </div>
        }
    }
}