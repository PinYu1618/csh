#![allow(clippy::wildcard_imports)]
// TODO: Remove this line
#![allow(unused)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

fn init(_: Url, _orders: &mut impl Orders<Msg>) -> Model {
    todo!()
}

// ------ ------
//     Model
// ------ ------

struct Model {
    pages: Vec<Page>, // TODO: fix performance (`Indexmap`)
    address_comp: Address,
    contact_comp: PhoneNumber,
    activated_item: ActivatedItem,
    base_url: Url,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            pages: Vec::default(),
            address_comp: Address::default(),
            contact_comp: PhoneNumber::default(),
            activated_item: ActivatedItem::default(),
            base_url: Url::new(),
        }
    }
}

struct Page {
    id: ID,
    title: String,
    completed: bool,
}

enum ActivatedItem {
    None,
    Page(ActivePage),
    Address(Address),
    PhoneNumber(PhoneNumber),
}

impl Default for ActivatedItem {
    fn default() -> Self {
        Self::None
    }
}

struct ActivePage {
    id: ID,
    title: String,
    input_element: ElRef<web_sys::HtmlInputElement>,
}

struct Address {
    input_element: ElRef<web_sys::HtmlInputElement>,
}

impl Default for Address {
    fn default() -> Self {
        todo!()
    }
}

struct PhoneNumber {
    input_element: ElRef<web_sys::HtmlInputElement>,
}

impl Default for PhoneNumber {
    fn default() -> Self {
        todo!()
    }
}

type ID = i32;

// ------ ------
//    Update
// ------ ------

enum Msg {
    UrlChanged(subs::UrlChanged),

    SelectPage(ID),
    LeavePage(ID),

    SelectAddr,

    SelectPhone,
    UnselectPhone,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => {
            log!("UrlChanged", url);
        }

        Msg::SelectPage(id) => {
            log!("SelectPage");
        }
        Msg::LeavePage(id) => {
            log!("LeavePage");
        }

        Msg::SelectAddr => {
            log!("SelectAddr");
        }

        Msg::SelectPhone => {
            log!("SelectPhone");
        }
        Msg::UnselectPhone => {
            log!("UnselectPhone");
        }
        _ => {}
    }
}

// ------ ------
//     View
// ------ ------

fn view(model: &Model) -> Node<Msg> {
    div!["I'm a placeholder",]
}

// ------ ------
//     Start
// ------ ------

pub fn start() {
    //App::start("app", init, update, view);
}
