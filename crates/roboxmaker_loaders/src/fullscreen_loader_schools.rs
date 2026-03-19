use yew::prelude::*;

pub struct Msg;

pub struct FullScreenLoaderSchools;

impl Component for FullScreenLoaderSchools {
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
            <div id="preloader">
                <div id="loader"></div>
            </div>
        }
    }
}