mod date_time;

use date_time::DateTimeWidget;
use sauron::html::text;
use sauron::prelude::*;
use sauron::{node, Cmd, Component, Node, Program};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Msg {
    Increment,
    Decrement,
    Mount(web_sys::Node),
    DateTimeMsg(date_time::Msg),
    TimeChange,
}

pub struct App {
    count: i32,
    date_time: DateTimeWidget<Msg>,
}

impl App {
    pub fn new() -> Self {
        let mut date_time = DateTimeWidget::new("2020-12-30", "10:00", false);
        date_time.on_time_change(|_e| {
            log::info!("Time has changed..");
            Msg::TimeChange
        });
        App {
            count: 0,
            date_time,
        }
    }
}

impl Component<Msg> for App {
    fn view(&self) -> Node<Msg> {
        div(
            vec![on_mount(|me| Msg::Mount(me.target_node))],
            vec![
                input(
                    vec![
                        r#type("button"),
                        value("+"),
                        key("inc"),
                        on_click(|_| Msg::Increment),
                    ],
                    vec![],
                ),
                div(vec![], vec![text(self.count)]),
                input(
                    vec![
                        r#type("button"),
                        value("-"),
                        key("dec"),
                        on_click(|_| Msg::Decrement),
                    ],
                    vec![],
                ),
                self.date_time.view().map_msg(Msg::DateTimeMsg),
            ],
        )
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        match msg {
            Msg::Increment => {
                self.count += 1;
                Cmd::none()
            }
            Msg::Decrement => {
                self.count -= 1;
                Cmd::none()
            }
            Msg::Mount(target_node) => {
                log::trace!("app is mounted to {:?}", target_node);
                Cmd::none()
            }
            // this is only here for the purpose of mounting
            // the date time widget.
            // We want the date-time widget to have it's own lifecycle
            Msg::DateTimeMsg(dmsg) => {
                match dmsg {
                    date_time::Msg::Mount(_) => {
                        log::trace!("mount event pass through..");
                        let cmd = self.date_time.update(dmsg);
                    }
                    _ => {
                        log::trace!("not wiring {:?}", dmsg);
                    }
                }
                Cmd::none()
            }
            Msg::TimeChange => {
                log::debug!("Time is changed in out date time widget");
                Cmd::none()
            }
        }
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();
    Program::mount_to_body(App::new());
}
