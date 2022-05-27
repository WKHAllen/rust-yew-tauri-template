use super::super::{services, types::AppConfig};
use super::Message;
use yew::prelude::*;

pub enum AppMessage {
    SetMessage(String),
}

pub struct App {
    init: bool,
    message: String,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            init: false,
            message: String::from(""),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if !self.init {
            let config = services::config::load_config();
            ctx.link()
                .callback(AppMessage::SetMessage)
                .emit(config.message);
        }

        html! {
            <div class="app">
                <div class="content">
                    <Message text={self.message.clone()} />
                </div>
            </div>
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppMessage::SetMessage(text) => {
                self.init = true;
                self.message = text;
                services::config::save_config(&AppConfig {
                    message: self.message.clone(),
                });
            }
        };

        true
    }
}
