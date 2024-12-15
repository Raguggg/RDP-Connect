use std::process::Command;

use iced::widget::{button, column, text, text_input};
use iced::window::{self, Position};
use iced::{alignment, Alignment, Element, Size};

use regex::Regex;

#[allow(dead_code)]
fn is_connected(log_content: &str) -> bool {
    // Define the connection pattern (matching an IP address)
    let connection_pattern = r"connecting to peer (\d+\.\d+\.\d+\.\d+)";
    let connection_regex = Regex::new(connection_pattern).unwrap();

    // Define the security pattern (matching RDP, TLS, or NLA)
    let security_pattern = r"Enabling (RDP|TLS|NLA) security";
    let security_regex = Regex::new(security_pattern).unwrap();

    // Check if a connection is mentioned
    let connection_matches = connection_regex.is_match(log_content);

    // Check if a security protocol is mentioned
    let security_matches = security_regex.is_match(log_content);

    // If both connection and security protocol are found, we consider it connected
    connection_matches && security_matches
}
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
                // xfreerdp /v:<hostname_or_ip> /u:<username> /p:<password> /cert-ignore
                // create  this command

                let program = "xfreerdp";

                // let args = [];
                let args = vec![
                    format!("/v:{}", self.ip),
                    format!("/u:{}", self.username),
                    format!("/p:{}", self.password),
                    "/cert-ignore".to_string(),
                ];

                // let program = "python3";
                // let args = vec!["-u","/home/ragu/Desktop/rdp_linux/test.py"];

                Command::new(program)
                    .args(&args)
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
            column![text(title).size(14), content]
                .spacing(5)
                .align_x(alignment::Horizontal::Center)
        };

        let column = column![
            subtitle(
                "Server IP",
                text_input("", &self.ip)
                    .on_input(Message::IpChanged)
                    .padding(5)
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
            )
        ];
        // add spcaing between the input fields and connect button

        // connect button
        let connect_button = button(text("                            Connect").center())
            .width(iced::Length::Fill)
            .on_press(Message::Connect);
        let column = column![column, connect_button];

        let copyright_text = text("       © Copyright Ragu. All Rights Reserved")
        .size(15)
        .align_x(Alignment::Center)
        .color([0.7, 0.7, 0.7]);

    // Return the complete layout
    column
        .spacing(20)
        .padding(10)
        .push(copyright_text) // Push the copyright text to the end
        .into()



        // column.spacing(20).padding(10).into()
    }
}

fn main() -> Result<(), iced::Error> {
    // run the app from main function
    iced::application("RDP connect", RDPInput::update, RDPInput::view)
        .theme(|_state| iced::Theme::Dark) 
        .window(window::Settings {
            position: Position::Centered,
            resizable: false,
            size: Size::new(400.0, 300.0),

            ..Default::default()
        })
        .run_with(|| (RDPInput::new(), iced::Task::none()))
}

// fn main() {
//     // Sample log content (replace with actual log content you want to analyze)
//     let log_content = "[14:24:18:570] [23022:23022] [DEBUG][com.freerdp.client.common] - This is Build configuration: BUILD_TESTING=OFF ...
// [14:24:18:882] [23022:23023] [DEBUG][com.freerdp.core] - connecting to peer 37.27.189.41
// [14:24:18:401] [23022:23023] [DEBUG][com.freerdp.core.nego] - Enabling NLA security: TRUE";

//     // Check if the connection is established
//     let connection_status = is_connected(log_content);

//     // Print the result
//     println!("Is connected: {}", connection_status);
// }
