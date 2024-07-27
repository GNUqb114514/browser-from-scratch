use iced::{Sandbox, Settings};
use interface::Browser;

mod interface;
mod consts;
mod history;

fn main() -> Result<(), iced::Error>{
    Browser::run(Settings::default())
}
