use yew::prelude::*;

pub struct Msg;

pub struct CardDegreePlaceholder;

impl Component for CardDegreePlaceholder {
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
            <div class="card-degree is-loading-degree me-6 my-4">
                <div class="px-5 pt-5">
                    <div class="image-degree"></div>  
                </div>
                <div class="hr-degree"></div>  
                <div class="content-degree px-5 pb-4">
                    <h2></h2>
                    <h3></h3>
                    <h4></h4>
                    <h5></h5>
                    <p></p>
                </div>
            </div>
        }
    }
}