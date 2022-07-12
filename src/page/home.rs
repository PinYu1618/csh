use seed::{prelude::*, *};
use super::PageTrait;

const HEADER1: &str = "世鴻耳鼻喉科聯合診所";

#[allow(dead_code)]
pub struct HomePage {
    text: &'static str,
    received_text: Option<&'static str>,
    receiver_handle: SubHandle,
}

pub enum Msg {
    Clicked,
    TextReceived(&'static str),
}

impl PageTrait for HomePage {
    type Message = Msg;

    fn init(orders: &mut impl Orders<Self::Message>) -> Self {
        Self {
            text: "",
            received_text: None,
            receiver_handle: orders.subscribe_with_handle(Msg::TextReceived),
        }
    }

    fn update(&mut self, msg: Self::Message, _orders: &mut impl Orders<Self::Message>) {
        match msg {
            Msg::Clicked => self.text = "HomePage button clicked!",
            Msg::TextReceived(text) => self.received_text = Some(text),
        }
    }

    fn view(&self) -> Vec<Node<Self::Message>> {
        vec![
            h1![
                HEADER1,
            ],
            button![
                "HomePage button",
                ev(Ev::Click, |_| Msg::Clicked)
            ],
            plain![self.text],
            div![
                &self.received_text,
            ]
        ]
    }
}