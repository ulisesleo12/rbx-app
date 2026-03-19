use yew::prelude::*;

pub struct Msg;

pub struct CardCommentsPlaceholder;

impl Component for CardCommentsPlaceholder {
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