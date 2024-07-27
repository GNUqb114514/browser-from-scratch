use iced::{Sandbox, Settings};
use interface::Browser;

mod interface;
mod consts;
mod history;

#[cfg(test)]
mod test;

fn main() -> Result<(), iced::Error>{
    Browser::run(Settings::default())
}
