use iced::{button, Button, Column, Container, Element, Text};
use iced::{text_input, Alignment,Length,TextInput, Background, Color};
use crate::Message;
use regex::Regex;
use rusqlite::{params, Connection};
use argon2::{Argon2, PasswordHasher, password_hash::{SaltString, PasswordHash}};
use aes::{Aes128, BlockEncrypt};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use rand::{Rng, thread_rng};
use std::error::Error;
use std::fmt;
use std::io::{self, Write};
use base64::encode;
use std::sync::{Arc, Mutex};
use crate::set_user_id;

static mut ERROR:bool=true;
type Aes128Cbc = Cbc<Aes128, Pkcs7>;

// Custom Error Type for Argon2 Errors
#[derive(Debug)]
struct Argon2Error(argon2::password_hash::Error);

impl fmt::Display for Argon2Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Argon2 error: {}", self.0)
    }
}

impl std::error::Error for Argon2Error {}
struct EncryptedData {
    encrypted_string: String, // Field to store the encrypted string
}
fn hash_password(password: &str) -> Result<String, Box<dyn Error>> {
    let salt: SaltString = SaltString::generate(&mut rand::thread_rng());
    let argon2 = Argon2::default();
    
    let hash = argon2.hash_password(password.as_bytes(), &salt).map_err(Argon2Error)?;
    Ok(hash.to_string())
}

fn encrypt(data: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Result<EncryptedData, AppError> {
    let cipher = Aes128Cbc::new_from_slices(key, iv)
        .map_err(|e| AppError(e.to_string()))?;

    let mut buffer = data.to_vec();
    let pos = buffer.len();
    let padding_length = 16 - (pos % 16);
    buffer.resize(pos + padding_length, 0); // Resize buffer for padding

    let encrypted_data = cipher.encrypt(&mut buffer, pos)
        .map_err(|e| AppError(e.to_string()))?;

    // Use Base64 encoding to convert the encrypted data into a String
    let encrypted_string = encode(encrypted_data);

    // Return the struct that holds the encrypted string
    Ok(EncryptedData { encrypted_string })
}


// Custom App Error for general application errors
#[derive(Debug)]
struct AppError(String);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl std::error::Error for AppError {}

fn create_user_id(birth_date: &str) -> String {
    let year = &birth_date[7..9]; 
    let random_id: u64 = thread_rng().gen_range(0..10_000_000); 
    format!("{}{:010}", year, random_id) 
}


#[derive(Debug, Clone)]
pub struct SignupPage {
    email: String,
    email_input: text_input::State,
    passport: String,
    passport_input: text_input::State,
    name: String,
    name_input: text_input::State,
    birth_date: String,
    birth_date_input: text_input::State,
    password:String,
    password_input:text_input::State,
    signup_button: button::State,
    back_button:button::State,
    valid_fields: [bool; 5],
}



impl SignupPage {
    pub fn new() -> Self {
        SignupPage {
                email: String::new(),
                email_input: text_input::State::new(),
                passport: String::new(),
                passport_input: text_input::State::new(),
                name: String::new(),
                name_input: text_input::State::new(),
                birth_date: String::new(),
                birth_date_input: text_input::State::new(),
                password:String::new(),
                password_input:text_input::State::new(),
                signup_button: button::State::new(),
                back_button:button::State::new(),
                valid_fields: [false; 5],
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::EmailChanged(value)=>{
                self.email = value.clone();
                self.valid_fields[0] = value.contains("@");
            }
            Message::PassportChanged(value)=>{
                let passport_regex = Regex::new(r"^[A-Z]{2}\d{6}$").unwrap();
                self.passport = value.clone();
                self.valid_fields[1] = passport_regex.is_match(&value); 
            }
            Message::NameChanged(value)=>{
                self.name = value.clone();
                self.valid_fields[2] = !value.is_empty();}
            Message::BirthDateChanged(value)=>{
                let birthdate_regex = Regex::new(r"^\d{2}/\d{2}/\d{4}$").unwrap();
                self.birth_date = value.clone();
                self.valid_fields[3] = birthdate_regex.is_match(&value);
            }
            Message::PasswordChanged(value)=>{
                let password_regex = Regex::new(r"^\d{6}$").unwrap();
                    self.password = value.clone();
                    self.valid_fields[4] = password_regex.is_match(&value);
            }
            _=>{}
            
        }
    }

    pub fn view(&mut self, db_conn: &Arc<Mutex<Connection>> ) -> Element<super::Message> {
        let conn = db_conn.lock().expect("Failed to acquire lock");
        let all_valid = self.valid_fields.iter().all(|&valid| valid);
        // Main heading
        let main_text = Text::new("Secure your financial future with us")
            .size(40)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

        // Subheading
        let sub_text = Text::new("Your financial future, our priority. Secure your finances with our trusted banking services.")
            .size(20)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

        

        unsafe {
            if all_valid{
                ERROR=false;
            }
            
            let error_text = if !ERROR{
            Text::new("All entries are valid. You may proceed.")
            .color(Color::from_rgb(0.0, 1.0, 0.0))
            .size(16)
            
        } else {
            Text::new("Input validation failed. Ensure all fields meet the described criteria.")
                .color(Color::from_rgb(1.0, 0.0, 0.0))
                .size(16)
            
        };

        let email_text=Text::new("Email Address")
            .size(20)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);


        let input_email = TextInput::new(
            &mut self.email_input,
            "eg. abc123@gmail.com ",
            &self.email,
            Message::EmailChanged,
        )
        .padding(10)
        .size(20);

        let passport_text=Text::new("Passport Number")
            .size(20)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);


        let input_passport = TextInput::new(
            &mut self.passport_input,
            "eg. AB123456 ",
            &self.passport,
            Message::PassportChanged,
        )
        .padding(10)
        .size(20);

        let name_text=Text::new("Name")
            .size(20)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);


        let input_name = TextInput::new(
            &mut self.name_input,
            "Name as on passport ",
            &self.name,
            Message::NameChanged,
        )
        .padding(10)
        .size(20);


        let birth_text=Text::new("Birth Date")
            .size(20)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);


        let input_birth = TextInput::new(
            &mut self.birth_date_input,
            "dd/mm/yy (eg. 06/04/2004) ",
            &self.birth_date,
            Message::BirthDateChanged,
        )
        .padding(10)
        .size(20);

        let password_text=Text::new("Password")
            .size(20)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);

        let input_password = TextInput::new(
            &mut self.password_input,
            "6 Digits(eg. 123456)",
            &self.password,
            Message::PasswordChanged,
        )
        .padding(10)
        .size(20);

        
        let sign_up_button=if all_valid 
        {   
            let user_id = create_user_id(&self.birth_date);
            let hashed_password = match hash_password(&self.password){
                Ok(s)=>s,
                Err(e)=>"error occured in hashing".to_string(),
            };
             
    // Encryption key and IV
            const KEY: [u8; 16] = *b"mysecretkey12345";
            const IV: [u8; 16] = *b"uniqueiv12345678";

    //Encrypt passport number and birth date
            let encrypted_passport:String= match encrypt(self.passport.as_bytes(), &KEY, &IV){
                Ok(encrypted)=>{encrypted.encrypted_string}
                Err(e)=>{"a".to_string()}
            };
            let encrypted_birth_date:String= match encrypt(self.birth_date.as_bytes(), &KEY, &IV){
                Ok(encrypted)=>{encrypted.encrypted_string}
                Err(e)=>{"a".to_string()}
            };
            
            conn.execute("INSERT INTO user_information (id,email,name,encrypted_passport,encrypted_birthdate,hashed_password,balance) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)", params![user_id.clone(), self.email.clone(), self.name.clone(),encrypted_passport.clone(),encrypted_birth_date.clone(),hashed_password.clone(),500],
            ).unwrap();
            conn.execute("INSERT INTO transaction_history (sender_id, amount, receiver_id) VALUES (?1, ?2, ?3)", params!["New User's Privilege", 500, user_id.clone()]).unwrap();
            
            set_user_id(user_id.clone());
            // When all fields are valid, the button activates and sends the Message::GoToFunction
             Button::new(&mut self.signup_button, Text::new("Sign Up"))
                .on_press(Message::GoToFunction)
                .padding(12)
                .style(SignUpButtonStyle)

           
        } else {
            
            // When fields are not valid, the button is still rendered but does nothing
            Button::new(&mut self.signup_button, Text::new("Sign Up"))
                .on_press(Message::GoToSignup)
                .padding(12)
                .style(SignUpButtonStyle)
        };

        let back_button = Button::new(&mut self.back_button, Text::new("Back"))
        .padding(3)
        .style(BackButtonStyle)
        .on_press(Message::GoToHome);

        let content = Column::new()
            .spacing(12)
            .padding(15)
            .align_items(Alignment::Center)
            .push(main_text)
            .push(sub_text)
            .push(error_text)
            .push(email_text)
            .push(input_email)
            .push(passport_text)
            .push(input_passport)
            .push(name_text)
            .push(input_name)
            .push(birth_text)
            .push(input_birth)
            .push(password_text)
            .push(input_password)
            .push(sign_up_button)
            .push(back_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()}
        
    }
}

// Custom style for the send button
struct SignUpButtonStyle;
impl iced::button::StyleSheet for SignUpButtonStyle {
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
