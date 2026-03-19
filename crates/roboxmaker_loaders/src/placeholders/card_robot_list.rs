use yew::prelude::*;

pub struct Msg;

pub struct CardRobotListPlaceholder;

impl Component for CardRobotListPlaceholder {
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
            <div class="card-robot-list is-loading-robot-list p-4 mb-md-3 mb-lg-5 me-md-3 me-lg-5">
                <div class="image-robot-list"></div>
                <div class="txt-robot-list">
                    <div class="display-robot-list">
                        <h2></h2>
                        <div class="menu-robot-list"></div>
                    </div>
                    <div class="enabled-robot-list">
                        <p></p>
                        <h3></h3>
                    </div>
                </div>
            </div>
        }
    }
}