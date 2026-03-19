use yew::prelude::*;

pub struct Msg;

pub struct CardDegreePlaceholder;

impl Component for CardDegreePlaceholder {
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