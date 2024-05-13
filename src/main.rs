use std::process::exit;
use std::process::Command;

use iced::alignment::Horizontal;
use iced::border::Radius;
use iced::widget::button::{Status, Style};
use iced::widget::{button, column, container, row, text, Container, Row};
use iced::{
    Alignment, Background, Border, Color, Element, Font, Length, Pixels, Renderer, Shadow, Size,
    Theme,
};
use strum::{Display, EnumIter, EnumMessage, IntoEnumIterator};

struct App {
    show_confirm: bool,
    after_confirm: Message,
    font_size: u8,
}

#[derive(Debug, EnumIter, Clone, Display, Default, Copy, EnumMessage)]
enum Message {
    #[strum(to_string = "", message = "shutdown")]
    #[default]
    Shutdown,
    #[strum(to_string = "", message = "reboot")]
    Reboot,
    #[strum(to_string = "󱤛", message = "log out")]
    LogOut,
    #[strum(to_string = "󱅞", message = "lock session")]
    Lock,
    #[strum(to_string = "")]
    Confirm,
    #[strum(to_string = "")]
    Cancel,
}

impl Default for App {
    fn default() -> Self {
        Self {
            show_confirm: false,
            after_confirm: Message::Shutdown,
            font_size: 55,
        }
    }
}

impl App {
    fn view(&self) -> Container<Message> {
        let mut buttons = vec![];
        for message in Message::iter() {
            if message.to_string().as_str() != "" {
                buttons.push(Element::new(
                    button(text(message.to_string()).size(Pixels::from(self.font_size as u16)))
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
                                radius: Radius::from(20),
                            },
                            shadow: Shadow {
                                color: Color::TRANSPARENT,
                                ..Default::default()
                            },
                        }),
                ));
            }
        }
        let confirm: Container<Message, Theme, Renderer> = container(
            column![
                text(format!(
                    "Are you sure you want to {}?",
                    self.after_confirm.get_message().unwrap()
                ))
                .horizontal_alignment(Horizontal::Center),
                row![
                    button("Yes, do it!")
                        .on_press(Message::Confirm)
                        .style(|_, _| Style {
                            background: Some(Background::from(Color::from_rgb8(166, 218, 149))),
                            text_color: Color::from_rgb8(24, 25, 38),
                            border: Border {
                                radius: Radius::from(10),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                    button("God please no.")
                        .on_press(Message::Cancel)
                        .style(|_, _| Style {
                            background: Some(Background::from(Color::from_rgb8(245, 169, 127))),
                            text_color: Color::from_rgb8(24, 25, 38),
                            border: Border {
                                radius: Radius::from(10),
                                ..Default::default()
                            },
                            ..Default::default()
                        }),
                ]
                .spacing(5)
                .align_items(Alignment::Center)
            ]
            .spacing(4)
            .width(Length::Fill)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
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
                .height(Length::Shrink)
                .center()
        }
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Confirm => match self.after_confirm {
                Message::Shutdown => {
                    println!("shutdown -h now");
                    let _ = Command::new("shutdown").args(["-h", "now"]).spawn();
                    exit(0)
                }
                Message::Reboot => {
                    println!("reboot");
                    let _ = Command::new("reboot").spawn();
                    exit(0)
                }
                Message::LogOut => {
                    println!("hyprctl dispatch exit");
                    let _ = Command::new("hyprctl").args(["dispatch", "exit"]).spawn();
                    exit(0)
                }
                Message::Lock => {
                    let _ = Command::new("hyprlock").spawn();
                    exit(0)
                }
                _ => {}
            },
            Message::Cancel => {
                exit(0);
            }
            _ => {
                self.font_size = 24;
                self.show_confirm = true;
                self.after_confirm = message;
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
            default_font: Font::with_name("Hack Nerd Font Mono"),
            default_text_size: Pixels::from(12),
            ..Default::default()
        })
        .theme(|_| Theme::CatppuccinMacchiato)
        .run()
}
