use yew::prelude::*;

pub struct Msg;

pub struct CardMembersHomePlaceholder;

impl Component for CardMembersHomePlaceholder {
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