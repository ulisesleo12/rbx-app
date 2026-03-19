use yew::prelude::*;

pub struct Msg;

pub struct FullScreenLoaderDegree;

impl Component for FullScreenLoaderDegree {
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
            <div class="infinityChrome">
                <div></div>
                <div></div>
                <div></div>
            </div>
        }
    }
}