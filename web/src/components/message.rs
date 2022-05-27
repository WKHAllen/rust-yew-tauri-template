use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct MessageProps {
    pub text: String,
}

#[function_component(Message)]
pub fn message(props: &MessageProps) -> Html {
    let MessageProps { text } = props.clone();

    html! {
        <div class="message">{text}</div>
    }
}
