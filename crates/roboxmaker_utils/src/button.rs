use log::info;
use yew::prelude::*;


pub struct Button {}


#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Height {
    Small,
    Medium,
    Hight,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Width {
    Small,
    Medium,
    Hight,
}

// Colors: /style/scss/_variables.scss
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Primary,
    Secondary,
    Success,
    Info,
    Warning,
    Danger,
    Light,
    Dark,
    White,
    Lavanda,              
    Silver,
    Transparent,
    Brown,
    PrimaryBlueDark,
    PrimaryBlueLight,
    GrayPurpleTwo,
    LightSeaGreen,
    GrayDark,
    SecondaryPurple,
    GrayPurple,
    CyanBase,
    CyanSky,
    CyanTurquesa,
    PurpleGray,
    GrayBlue,
    LightBlue,
    LavandaLight,
    GrayStrong,
    BluePurple,
    PurpleOn,
    BlueTwo,
    RedDelete,
}

// FontSize: /style/scss/_variables.scss
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FontSize {
    H1FontSize,
    H2FontSize,
    H3FontSize,
    H4FontSize,
    H5FontSize,
    H6FontSize,
    FontSize48,
    FontSize36,
    FontSize32,
    FontSize24,
    FontSize22,
    FontSize20,
    FontSize18,
    FontSize16,
    FontSize14,
    FontSize13,
    FontSize12,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ButtonView {
    Style(Height, Width, Color, FontSize),
}

#[derive(Debug, Properties, Clone, PartialEq)]
pub struct Props {
}

#[derive(Debug)]
pub enum Msg {}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(_props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Button {}
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        info!("msg {:?}", msg);
        match msg {
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {

        html! {

        }
    }
}
