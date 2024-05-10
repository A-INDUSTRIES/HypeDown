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
    confirmed: bool,
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
    #[strum(to_string = "")]
    Confirm,
    #[strum(to_string = "")]
    Cancel,
}

impl App {
    fn view(&self) -> Container<Message> {
        let mut buttons = vec![];
        for message in Message::iter() {
            if message.to_string().as_str() != "" {
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
        }
        let confirm: Container<Message, Theme, Renderer> = container(column![
            text("Are you sure?"),
            row![button("Yes"), button("Cancel")]
        ])
        .center();
        let line = Row::from_vec(buttons)
            .align_items(Alignment::Center)
            .spacing(5);
        if self.show_confirm {
            container(column![container(line).center(), confirm])
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
        self.show_confirm = !self.show_confirm;
        self.after_confirm = message.clone();
        match message {
            Message::Shutdown => {}
            Message::Reboot => {}
            Message::LogOut => {}
            Message::Lock => {}
            Message::Confirm => {
                self.confirmed = true;
            }
            Message::Cancel => {
                self.show_confirm = false;
                self.confirmed = false;
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
