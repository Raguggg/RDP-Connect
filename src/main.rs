use std::{env, fs, io};
use std::process::Command;


use encoding_rs::UTF_16LE;
use iced::widget::{button, column, text, text_input};
use iced::window::{self, Position};
use iced::{alignment, Alignment, Element, Size};
use native_dialog::{MessageDialog, MessageType};
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


fn is_xfreerdp_installed() -> bool {
    let output = Command::new("xfreerdp")
        .arg("--version")
        .output();

    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}


fn connect_rdp(username: &str, password: &str, ip: &str, rdp_file_path: &str) -> bool {
    let program = "xfreerdp";

    // Building the argument list conditionally
    let mut args = vec![];

    if !rdp_file_path.is_empty() {
        args.push(format!("{}", rdp_file_path)); // Add the RDP file path if it's not empty
    }else {
        
        args.push(format!("/v:{}", ip )); // Add the IP if rdp_file_path is empty\
        args.push(format!("/u:{}", username)); // Add username
    }
   
    args.push(format!("/p:{}", password)); // Add password
    args.push("/cert-ignore".to_string()); // Add the cert-ignore flag
    args.push("/floatbar".to_string()); // Add  float-bar
    args.push("/dynamic-resolution".to_string()); 

    // Executing the command
    let _status = Command::new(program)
        .args(&args)
        .spawn()
        .expect("Failed to execute command");
        // .wait()
        // .expect("Failed to wait for command");

    // status.success()
    return  true;
}

fn pop_error(){
    MessageDialog::new()
    .set_type(MessageType::Info)
    .set_title("Please install xfreerdp")
    .set_text("This Software give only UI Please install `xfreerdp` ")
    .show_alert()
    .unwrap();
   
}

struct RDPInput {
    ip: String,
    username: String,
    password: String,
    rdp_file_path:String,
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
            rdp_file_path:String::new(),
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
                if connect_rdp(&self.username,&self.password,&self.ip,&self.rdp_file_path){
                    std::process::exit(0);
                }

                // let program = "xfreerdp";

                // // let args = [];
                // let args = vec![
                //     format!("/v:{}", self.ip),
                //     format!("/u:{}", self.username),
                //     format!("/p:{}", self.password),
                //     "/cert-ignore".to_string(),
                // ];


                // Command::new(program)
                //     .args(&args)
                //     .spawn()
                //     // .status()
                //     .expect("Failed to execute command");
                // close the app
                println!("Exiting...");
                // std::process::exit(0);
            }
        }
        iced::Task::none()
    }

    fn view(&self) -> iced::Element<'_, Message> {

        let subtitle = |title, content: Element<'static, Message>| {
            column![text(title).size(14), content]
                .spacing(5)
                .align_x(alignment::Horizontal::Center)
        };

        let column = column![
            subtitle(
                "Server IP",
                text_input(&self.ip, &self.ip)
                    .on_input(Message::IpChanged)
                    .padding(5)
                    .into()
            ),
            subtitle(
                "Username",
                text_input(&self.username, &self.username)
                    .on_input(Message::UsernameChanged)
                    .into()
            ),
            subtitle(
                "Password",
                text_input("", &self.password)
                    .on_input(Message::PasswordChanged)
                    .on_submit(Message::Connect)
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






fn parse_rdp_file(file_path: &str) -> Option<(String, String)> {
    // Read the raw bytes of the file
    let bytes = match fs::read(file_path) {
        Ok(b) => b,
        Err(e) => {
            println!("Error reading file {}: {}", file_path, e);
            return None;
        }
    };

    // Decode the bytes as UTF-16LE
    let (decoded, _, _) = UTF_16LE.decode(&bytes);
    let content = decoded.to_string();

    if content.is_empty() {
        println!("Error decoding file content for file: {}", file_path);
        return None;
    }

    println!("File content read from {}", file_path);

    // Regular expression to match the IP and username from the file
    let ip_regex = match Regex::new(r"full address\s*:\s*s:([0-9]+\.[0-9]+\.[0-9]+\.[0-9]+)") {
        Ok(r) => r,
        Err(e) => {
            println!("Error compiling IP regex: {}", e);
            return None;
        }
    };

    let username_regex = match Regex::new(r"username\s*:\s*s:([^\r\n]+)") {
        Ok(r) => r,
        Err(e) => {
            println!("Error compiling username regex: {}", e);
            return None;
        }
    };

    // Capture IP
    let ip = match ip_regex.captures(&content) {
        Some(caps) => caps.get(1).map_or_else(|| {
            println!("IP not found in file content: {}", file_path);
            None
        }, |m| Some(m.as_str().to_string())),
        None => {
            println!("No IP match found in content for file: {}", file_path);
            None
        }
    };

    // Capture username
    let username = match username_regex.captures(&content) {
        Some(caps) => caps.get(1).map_or_else(|| {
            println!("Username not found in file content: {}", file_path);
            None
        }, |m| Some(m.as_str().to_string())),
        None => {
            println!("No username match found in content for file: {}", file_path);
            None
        }
    };

    if let (Some(ip), Some(username)) = (ip, username) {
        println!("Parsed IP: {}, Username: {}", ip, username);
        Some((ip, username))
    } else {
        println!("Failed to parse IP or username for file: {}", file_path);
        None
    }
}


fn main() -> Result<(), iced::Error> {
    if !is_xfreerdp_installed() {
        pop_error();
        let io_error = io::Error::new(iced::futures::io::ErrorKind::Other, "xfreerdp is not installed");
        return Err(iced::Error::ExecutorCreationFailed(io_error));
    }

    let args: Vec<String> = env::args().collect();
    let mut rdp_input = RDPInput::new();
    print!("{}-len",args.len());
    if args.len() > 1 {
        println!("file is selected");
        let file_path = &args[1];
        print!("{:?}",file_path);
        if let Some((ip, username)) = parse_rdp_file(file_path) {
        
        
        rdp_input = RDPInput {
            ip,
            username,
            password: String::new(),
            rdp_file_path: file_path.clone(),
        };
    }

    }else{
        print!("file not passed");
        
    }
   
    // run the app from main function
    iced::application("RDP connect", RDPInput::update, RDPInput::view)
        .theme(|_state| iced::Theme::Dark) 
        .window(window::Settings {
            position: Position::Centered,
            resizable: false,
            size: Size::new(400.0, 300.0),

            ..Default::default()
        })
        .run_with(|| (rdp_input, iced::Task::none()))
}
