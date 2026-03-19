use yew::prelude::*;

pub struct Msg;

pub struct FullScreenLoaderSchools;

impl Component for FullScreenLoaderSchools {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, _ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div id="preloader">
                <div id="loader"></div>
            </div>
        }
    }
}