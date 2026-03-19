use yew::prelude::*;

pub struct Msg;

pub struct CardMeetingssHomePlaceholder;

impl Component for CardMeetingssHomePlaceholder {
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