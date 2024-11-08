use iced::{button, Button, Column, Container, Element, Text};
use iced::{text_input, Alignment,Length,TextInput, Background, Color};
use crate::Message;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use std::error::Error;
use argon2::{Argon2,PasswordVerifier,password_hash::{SaltString, PasswordHash}};
use std::fmt;
use crate::set_user_id;
#[derive(Debug)]
struct Argon2Error(argon2::password_hash::Error);

impl fmt::Display for Argon2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Argon2 error: {}", self.0)
    }
}

impl std::error::Error for Argon2Error {}
fn check_user_exists(conn: &Arc<Mutex<Connection>>, id: &str, email: &str) -> Result<bool> {
    // Lock the Mutex to access the Connection
    let conn = conn.lock().map_err(|_| rusqlite::Error::InvalidQuery)?;
    
    let mut stmt = conn.prepare(
        "SELECT COUNT(*) FROM user_information WHERE id = ?1 AND email = ?2",
    )?;
    
    let count: i32 = stmt.query_row(params![id, email], |row| row.get(0))?;
    
    // Return true if the count is greater than 0, meaning the user exists
    Ok(count > 0)
}


fn verify_password(password: &str, hash: &str) -> Result<bool, Box<dyn Error>> {
    let argon2 = Argon2::default();
    let parsed_hash = PasswordHash::new(hash).map_err(Argon2Error)?;

    match argon2.verify_password(password.as_bytes(), &parsed_hash){
        Ok(_)=> Ok(true),
        Err(_)=>Ok(false),
    }

    
}

#[derive(Debug, Clone)]
pub struct LoginPage {
    email: String,
    email_input: text_input::State,
    userid: String,
    userid_input: text_input::State,
    password:String,
    password_input:text_input::State,
    login_button: button::State,
    back_button: button::State,
}

impl LoginPage {
    pub fn new() -> Self {
        LoginPage {
                email: String::new(),
                email_input: text_input::State::new(),
                userid: String::new(),
                userid_input: text_input::State::new(),
                password:String::new(),
                password_input:text_input::State::new(),
                login_button: button::State::new(),
                back_button:button::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EmailChanged(value)=>{
                self.email=value;
            }
            Message::UserIDChanged(value)=>{
                self.userid=value;
            }
            Message::PasswordChanged(value)=>{
                self.password=value;
            }
            
            _=>{}
            
        }
    }

    pub fn view(&mut self, d_conn: &Arc<Mutex<Connection>>) -> Element<super::Message> {
       
        let main_text = Text::new("Secure your financial future with us")
            .size(45)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

        // Subheading
        let sub_text = Text::new("Your financial future, our priority. Secure your finances with our trusted banking services.")
            .size(27)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

        let email_text=Text::new("Email Address")
            .size(25)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);


        let input_email = TextInput::new(
            &mut self.email_input,
            "eg. abc@gmail.com",
            &self.email,
            Message::EmailChanged,
        )
        .padding(15)
        .size(25);

        let userid_text=Text::new("User ID")
            .size(25)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

        let input_userid = TextInput::new(
            &mut self.userid_input,
            "xxxxxxxxxx",
            &self.userid,
            Message::UserIDChanged,
        )
        .padding(15)
        .size(25);

        let password_text=Text::new("Password")
            .size(25)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

        let input_password = TextInput::new(
            &mut self.password_input,
            "Password ",
            &self.password,
            Message::PasswordChanged,
        )
        .padding(15)
        .size(25);
       
        let userverified= match check_user_exists(d_conn, &self.userid, &self.email){
            Ok(b)=> b,
            Err(_)=>false,
        };
        
        let check_user= userverified && verify_user_password(d_conn, &self.userid, &self.password);

        let log_in_button=if check_user {
            set_user_id(self.userid.clone());
            
             Button::new(&mut self.login_button, Text::new("LOG IN"))
            .padding(15)
            .style(LogInButtonStyle) // Custom style for the send button
            .on_press(Message::GoToFunction)

        }else{
            Button::new(&mut self.login_button, Text::new("LOG IN"))
            .padding(15)
            .style(LogInButtonStyle) // Custom style for the send button
            .on_press(Message::GoToLogin)
        };
        
        let back_button = Button::new(&mut self.back_button, Text::new("Back"))
        .padding(8)
        .style(BackButtonStyle)
        .on_press(Message::GoToHome);


        let content = Column::new()
            .spacing(20)
            .padding(25)
            .align_items(Alignment::Center)
            .push(main_text)
            .push(sub_text)
            .push(email_text)
            .push(input_email)
            .push(userid_text)
            .push(input_userid)
            .push(password_text)
            .push(input_password)
            .push(log_in_button)
            .push(back_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

// Custom style for the send button
struct LogInButtonStyle;
impl iced::button::StyleSheet for LogInButtonStyle {
    fn active(&self) -> iced::button::Style {
        iced::button::Style {
            background: Some(Background::Color(Color::from_rgb(0.1, 0.3, 0.6))),
            border_radius: 10.0,
            text_color: Color::WHITE,
            ..iced::button::Style::default()
        }
    }
}

struct BackButtonStyle;
impl iced::button::StyleSheet for BackButtonStyle {
    fn active(&self) -> iced::button::Style {
        iced::button::Style {
            background: Some(Background::Color(Color::WHITE)),
            border_radius: 10.0,
            text_color: Color::BLACK,
            ..iced::button::Style::default()
        }
    }
}


fn verify_user_password(d_conn: &Arc<Mutex<Connection>>, userid: &str, input_password: &str) -> bool {
    // Lock the connection
    let conn = match d_conn.lock() {
        Ok(c) => c,
        Err(_) => return false,
    };

    // Prepare and execute the SQL statement to get the hashed password
    if let Ok(hashed_password) = conn.query_row(
        "SELECT hashed_password FROM user_information WHERE id = ?1",
        params![userid],
        |row| row.get::<_, String>(0), // Ensure we get a String from the row
    ) {
        // Parse the stored hashed password
        if let Ok(parsed_hash) = PasswordHash::new(&hashed_password) {
            // Verify the input password against the stored hash
            return Argon2::default().verify_password(input_password.as_bytes(), &parsed_hash).is_ok();
        }
    }

    false // Return false if any step fails
}
