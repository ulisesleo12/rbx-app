use yew::prelude::*;

pub struct Msg;

pub struct CardRobotMySpacePlaceholder;

impl Component for CardRobotMySpacePlaceholder {
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
            <div class="card-robot-my-space is-loading-robot-my-space p-4">
                <div class="image-robot-my-space"></div>
                <div class="txt-robot-my-space">
                    <div class="display-robot-my-space">
                        <h2></h2>
                        <div class="options-robot-my-space">
                            // <div class="reset-robot-my-space"></div>
                            <div class="menu-robot-my-space"></div>
                        </div>
                    </div>
                    <div class="enabled-robot-my-space">
                        <p></p>
                        <h3></h3>
                    </div>
                </div>
            </div>
        }
    }
}