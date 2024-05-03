use iced::widget::{button, container, text, Container, Row};
use iced::Alignment;
use iced::Element;
use iced::Length;
use iced::Size;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Default)]
struct App {}

#[derive(Debug, EnumIter, Clone, Display)]
enum Message {
    Shutdown,
    Reboot,
    LogOut,
    Lock,
    Sleep,
}

impl App {
    fn view(&self) -> Container<Message> {
        let mut buttons = vec![];
        for message in Message::iter() {
            buttons.push(Element::new(
                button(text(message.to_string())).on_press(message),
            ));
        }
        let line = Row::from_vec(buttons)
            .align_items(Alignment::Center)
            .spacing(5);
        container(line)
            .width(Length::Fill)
            .height(Length::Fill)
            .center()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Shutdown => {
                println!("/!\\TODO")
            }
            Message::Reboot => {
                println!("/!\\TODO")
            }
            Message::LogOut => {
                println!("/!\\TODO")
            }
            Message::Lock => {
                println!("/!\\TODO")
            }
            Message::Sleep => {
                println!("/!\\TODO")
            }
        }
    }
}

fn main() -> Result<(), iced::Error> {
    iced::program("HyperDown", App::update, App::view)
        .settings(iced::Settings {
            window: iced::window::Settings {
                size: Size::new(400f32, 50f32),
                max_size: Some(Size::new(400f32, 50f32)),
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}
