use std::process::Command;

use iced::widget::{button, column, text, text_input};
use iced::Element;

struct RDPInput {
    ip: String,
    username: String,
    password: String,
}

#[derive(Debug, Clone)]
enum Message {
    IpChanged(String),
    UsernameChanged(String),
    PasswordChanged(String),
    Connect,
}

// Implement our RDPInput
impl RDPInput {
    fn new() -> Self {
        Self {
            ip: String::new(),
            username: String::new(),
            password: String::new(),
        }
    }

    fn update(&mut self, message: Message) -> iced::Task<Message> {
        // handle emitted messages
        match message {
            Message::IpChanged(ip) => self.ip = ip,
            Message::UsernameChanged(username) => self.username = username,
            Message::PasswordChanged(password) => self.password = password,
            Message::Connect => {
                println!(
                    "IP: {}, Username: {}, Password: {}",
                    self.ip, self.username, self.password
                );
                let program = "obs";
                // let args = [];

                Command::new(program)
                    // .args(&args)
                    .spawn()
                    // .status()
                    .expect("Failed to execute command");
                // close the app
                println!("Exiting...");
                std::process::exit(0);
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {
        //  let row = widget::row![
        //     widget::button("-").on_press(Message::DecrementCount),
        //     widget::text(self.count.to_string()),
        //     widget::button("+").on_press(Message::IncrementCount)
        // ];
        //  row.into()
        let subtitle = |title, content: Element<'static, Message>| {
            column![text(title).size(14), content].spacing(5)
        };

        let column = column![
            subtitle(
                "IP",
                text_input("", &self.ip)
                    .on_input(Message::IpChanged)
                    .padding(50)
                    .into()
            ),
            subtitle(
                "Username",
                text_input("", &self.username)
                    .on_input(Message::UsernameChanged)
                    .into()
            ),
            subtitle(
                "Password",
                text_input("", &self.password)
                    .on_input(Message::PasswordChanged)
                    .into()
            ),
            subtitle(
                "Connect",
                button("Connect").on_press(Message::Connect).into()
            )
        ];

        column.into()
    }
}

fn main() -> Result<(), iced::Error> {
    // run the app from main function
    iced::application("RDPInput Example", RDPInput::update, RDPInput::view)
        .run_with(|| (RDPInput::new(), iced::Task::none()))
}
