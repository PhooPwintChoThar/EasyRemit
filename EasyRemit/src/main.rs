mod home;
mod login;
mod signup;
mod transfer;
mod success;
mod function;
use rusqlite::{params, Connection, Result};
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use iced::{Application, Command, Element, Settings};
mod db;  
use crate::db::{DB_CONN, execute_with_retry};

pub fn main() -> Result<(), iced::Error> {

    let settings = Settings {
        window: iced::window::Settings {
            size: (430, 732),
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    };
    
    EasyRemit::run(settings);
    Ok(())
}

pub static TAMOUNT: Lazy<Mutex<Option<i64>>> = Lazy::new(|| Mutex::new(None));

pub fn set_t_amount(amount: i64) {
    let mut t_amount = TAMOUNT.lock().unwrap();
    *t_amount = Some(amount);
}
pub fn get_t_amount() -> Option<i64> {
    let t_amount = TAMOUNT.lock().unwrap();
    t_amount.clone()
}

pub static USER_ID: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
pub static RECEIVER_ID: Lazy<Mutex<Option<String>>> = Lazy::new(|| Mutex::new(None));
pub fn set_user_id(id: String) {
    let mut user_id_lock = USER_ID.lock().unwrap();
    *user_id_lock = Some(id);
}

pub fn set_receiver_id(id:String){
    let mut receiver_id_lock = RECEIVER_ID.lock().unwrap();
    *receiver_id_lock = Some(id);
}
    
    pub fn get_receiver_id() -> Option<String> {
    let receiver_id_lock = RECEIVER_ID.lock().unwrap();
    receiver_id_lock.clone()
}


// Function to retrieve the user ID globally
pub fn get_user_id() -> Option<String> {
    let user_id_lock = USER_ID.lock().unwrap();
    user_id_lock.clone()
}


struct EasyRemit {
    current_page: Page,
}

#[derive(Debug, Clone)]
enum Page {
    Home(home::HomePage),
    Login(login::LoginPage),
    Signup(signup::SignupPage),
    Function(function::FunctionPage),
    Transfer(transfer::TransferPage),
    Success(success::SuccessPage),
}

impl Application for EasyRemit {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let home_page = home::HomePage::new();
        (
            EasyRemit {
                current_page: Page::Home(home_page),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Easy Remit")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match &mut self.current_page {
            Page::Login(page) => {
                page.update(message.clone());
            }
            Page::Signup(page) => {
                page.update(message.clone());
            }
            Page::Transfer(page) => {
                page.update(message.clone());
            }
        
            // Other pages do not need to handle these messages
            _ => {}
        }
        match message {
            Message::GoToLogin => {
                let login_page = login::LoginPage::new();
                self.current_page = Page::Login(login_page);
            }
            Message::GoToSignup => {
                let signup_page = signup::SignupPage::new();
                self.current_page = Page::Signup(signup_page);
            }
            Message::GoToFunction => {
               
                let function_page = function::FunctionPage::new();
                self.current_page = Page::Function(function_page);
            }
            Message::GoToTransfer => {
                let transfer_page = transfer::TransferPage::new();
                self.current_page = Page::Transfer(transfer_page);
            }
            Message::GoToSuccess => {
                
                
                let success_page = success::SuccessPage::new();
                self.current_page = Page::Success(success_page);
            }

            Message::GoToHome => {
                
                
                let home_page = home::HomePage::new();
                self.current_page = Page::Home(home_page);
            }
            _=>{}
        }
        Command::none()
    }
    fn view(&mut self) -> Element<Self::Message> {
        match &mut self.current_page {
            Page::Home(page) => page.view(),
            Page::Login(page) => page.view(&DB_CONN),
            Page::Signup(page) => page.view(&DB_CONN),
            Page::Function(page) => page.view(&DB_CONN),
            Page::Transfer(page) => page.view(&DB_CONN),
            Page::Success(page) => page.view(&DB_CONN),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    GoToHome,
    GoToLogin,
    GoToSignup,
    GoToFunction,
    GoToTransfer,
    GoToSuccess,
    EmailChanged(String),
    PassportChanged(String),
    NameChanged(String),
    BirthDateChanged(String),
    PasswordChanged(String),
    UserIDChanged(String),
    InputChanged(String),
    KeypadPressed(char),
    
}