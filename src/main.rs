use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, text, Container, Row};
use iced::Alignment;
use iced::Background;
use iced::Border;
use iced::Color;
use iced::Element;
use iced::Font;
use iced::Length;
use iced::Pixels;
use iced::Renderer;
use iced::Shadow;
use iced::Size;
use iced::Theme;
use strum::{Display, EnumIter, IntoEnumIterator};

#[derive(Default)]
struct App {
    show_confirm: bool,
    after_confirm: Message,
}

#[derive(Debug, EnumIter, Clone, Display, Default)]
enum Message {
    #[strum(to_string = "")]
    #[default]
    Shutdown,
    #[strum(to_string = "")]
    Reboot,
    #[strum(to_string = "󱤛")]
    LogOut,
    #[strum(to_string = "󱅞")]
    Lock,
}

impl App {
    fn view(&self) -> Container<Message> {
        let mut buttons = vec![];
        for message in Message::iter() {
            buttons.push(Element::new(
                button(text(message.to_string()).size(Pixels::from(50)))
                    .on_press(message)
                    .style(|_, status| Style {
                        background: match status {
                            Status::Hovered => {
                                Some(Background::from(Color::from_rgb8(244, 219, 214)))
                            }
                            Status::Pressed => {
                                Some(Background::from(Color::from_rgb8(238, 153, 160)))
                            }
                            _ => Some(Background::from(Color::from_rgb8(183, 189, 248))),
                        },
                        text_color: Color::from_rgb8(24, 25, 38),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.1,
                            radius: Radius::from(200),
                        },
                        shadow: Shadow {
                            color: Color::TRANSPARENT,
                            ..Default::default()
                        },
                    })
                    .width(Length::from(66))
                    .height(Length::from(75)),
            ));
        }
        let confirm: Container<Message, Theme, Renderer> = container(column![
            text("Are you sure?"),
            row![button("Yes"), button("Cancel")]
        ]);
        let line = Row::from_vec(buttons)
            .align_items(Alignment::Center)
            .spacing(5);
        if self.show_confirm {
            container(column![container(line), confirm])
                .width(Length::Fill)
                .height(Length::Fill)
                .center()
        } else {
            container(line)
                .width(Length::Fill)
                .height(Length::Fill)
                .center()
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Shutdown => {
                self.show_confirm = true;
            }
            Message::Reboot => {
                self.show_confirm = true;
            }
            Message::LogOut => {
                self.show_confirm = true;
            }
            Message::Lock => {
                self.show_confirm = true;
            }
        }
    }
}

fn main() -> Result<(), iced::Error> {
    iced::program("HyperDown", App::update, App::view)
        .settings(iced::Settings {
            window: iced::window::Settings {
                size: Size::new(300f32, 95f32),
                ..Default::default()
            },
            default_font: Font::with_name("Hack Nerd Font"),
            default_text_size: Pixels::from(12),
            ..Default::default()
        })
        .theme(|_| Theme::CatppuccinMacchiato)
        .run()
}
