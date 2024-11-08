use iced::{button, Button, Column, Container, Element, Text};
use iced::{ text_input, Alignment,Length,Row,  TextInput, Background, Color};
use crate::Message;
use rusqlite::{params, Connection};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;
use crate::{set_receiver_id, set_t_amount,get_user_id};

static USER_ID: Lazy<Mutex<String>> = Lazy::new(|| {
    Mutex::new(match get_user_id() {
        Some(id) => id,              // If there is a user ID, use it
        None => "None".to_string(),  // Default to "None" if no user ID
    })
});


static mut ERROR:bool=false;


#[derive(Debug, Clone)]
pub struct TransferPage {
    transfer_button: button::State,
    recipient: String,
    recipient_input: text_input::State,
    amount: String,
    keypad_buttons: [button::State; 12],
    back_button:button::State,
}

impl TransferPage {
    pub fn new() -> Self {
        TransferPage {
                recipient: String::new(),
                recipient_input: text_input::State::new(),
                amount: String::from("0"),
                keypad_buttons: Default::default(), 
                transfer_button: button::State::new(),
                back_button:button::State::new(),
        }
    }

    pub fn update(&mut self, message: Message) {

        match message {
            Message::InputChanged(input) => {
                self.recipient = input;
            }
            Message::KeypadPressed(digit) => {
                if digit == 'X' {
                    self.amount.pop();
                    if self.amount.is_empty(){
                        self.amount="0".to_string();
                    }
                } else {
                    if &self.amount[0..1]=="0"{
                        self.amount.pop();
                        self.amount.push(digit);
                    }else{
                        self.amount.push(digit);
                    }
                }
            }

            
            _=>{}
            
        }
    }
    pub fn view(&mut self,db_conn:&Arc<Mutex<Connection>>) -> Element<super::Message> {
        
        let conn = db_conn.lock().expect("Failed to acquire lock");
        let user_id = match get_user_id(){
            Some(id)=>id,
            None=>"NULL".to_string(),
        };
        let balance1= conn.query_row(
            "SELECT balance FROM user_information WHERE id = ?1",
            params![user_id.clone()], |row| row.get::<_, i64>(0),);
        let balance:i64=match balance1{
            Ok(t)=> t,
            Err(_)=>0,

        };

        let mut stmt = conn.prepare("SELECT EXISTS(SELECT 1 FROM user_information WHERE id = ?)").expect("Failed to prepare statement");
        let exists: bool = stmt.query_row(params![self.recipient.clone()], |row| row.get(0)).unwrap_or(false);
        let amountt:i64=self.amount.parse().expect("error");   
        let invalid=self.recipient == user_id.clone() || balance-amountt<0 || !exists || self.amount=="0".to_string();
        unsafe{
        
        if invalid{
            ERROR=true;
        }else{
            ERROR=false;
        }
        let display_string=format!("VISA CARD\n{}",user_id.clone());
        let profile_circle = Container::new(Text::new(""))
            .width(Length::Units(50)) // Set the width for the circle
            .height(Length::Units(50)) // Set the height for the circle
            .padding(5)
            .center_x()
            .center_y()
            .style(ProfileCircleStyle); // Apply a style for the circle

        // Card display with the profile circle included
        let card_display = Row::new()
        .spacing(10)
        .align_items(Alignment::Center) // Align items in the center
        .push(
            Container::new(
                Row::new()
                    .align_items(Alignment::Center)
                    .push(profile_circle) // Add the circle inside the card
                    .push(
                        Container::new(
                            Text::new(display_string)
                                    
                                .size(20)
                                .horizontal_alignment(iced::alignment::Horizontal::Left)
                                .vertical_alignment(iced::alignment::Vertical::Center),
                        )
                        .padding(10),
                    ),
            )
            .style(CardStyle) // Apply card style here
            .width(Length::Units(300)) // Set the width of the card display
            .height(Length::Units(100)) // Set the height of the card display
            .padding(20) // Add padding around the card
        );


        let balance_display = Text::new(format!("Your balance: {}",balance.clone())).size(16);

        let input = TextInput::new(
            &mut self.recipient_input,
            "Recipient's ID: ",
            &self.recipient,
            Message::InputChanged,
        )
        .padding(10)
        .size(20);

        let amount_display = Text::new(format!("${}", self.amount)).size(50);
        
        let error_text = if ERROR{

            Text::new("Invalid Amount or Invalid Receiver")
                .color(Color::from_rgb(1.0, 0.0, 0.0))
                .size(16)
                  
        } else {
            Text::new("")
            .size(4)    
            
        };

        // Split the keypad_buttons into non-overlapping slices
        let (first_row, rest) = self.keypad_buttons.split_at_mut(3);
        let (second_row, rest) = rest.split_at_mut(3);
        let (third_row, fourth_row) = rest.split_at_mut(3);

        // Creating keypad rows
        let keypad = Column::new()
            .spacing(10)
            .padding(20)
            .align_items(Alignment::Center)
            .push(create_keypad_row(first_row, ['1', '2', '3']))
            .push(create_keypad_row(second_row, ['4', '5', '6']))
            .push(create_keypad_row(third_row, ['7', '8', '9']))
            .push(create_keypad_row(fourth_row, ['.', '0', 'X']));

       
        let send_button = if invalid {
            ERROR=true;

            Button::new(&mut self.transfer_button, Text::new("SEND"))
            .padding(15)
            .style(SendButtonStyle) // Custom style for the send button
            .on_press(Message::GoToTransfer)

        }else{
            ERROR=false;
            let t_amount:i64=self.amount.trim().parse().unwrap();
            set_receiver_id(self.recipient.clone());
            set_t_amount(t_amount);
            Button::new(&mut self.transfer_button, Text::new("SEND"))
            .padding(15)
            .style(SendButtonStyle) // Custom style for the send button
            .on_press(Message::GoToSuccess)

        };

        let back_button = Button::new(&mut self.back_button, Text::new("Back"))
        .padding(3)
        .style(BackButtonStyle)
        .on_press(Message::GoToFunction);


        let content = Column::new()
            .spacing(10)
            .padding(20)
            .align_items(Alignment::Center)
            .push(card_display)
            .push(input)
            .push(amount_display)
            .push(error_text)
            .push(balance_display)
            .push(keypad)
            .push(send_button)
            .push(back_button);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    
        
    }}
}




// Custom style for the profile circle
struct ProfileCircleStyle;
impl iced::container::StyleSheet for ProfileCircleStyle {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            background: Some(Background::Color(Color::from_rgb(0.8, 0.8, 0.8))), // Circle color
            border_radius: 25.0, // Make it a circle
            ..iced::container::Style::default()
        }
    }
}

// Custom style for the card display
struct CardStyle;
impl iced::container::StyleSheet for CardStyle {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            background: Some(Background::Color(Color::WHITE)), // Paler color
            border_radius: 15.0,
            border_width: 1.0,
            border_color: Color::from_rgb(0.2, 0.4, 0.6),
            ..iced::container::Style::default()
        }
    }
}

// Custom style for the send button
struct SendButtonStyle;
impl iced::button::StyleSheet for SendButtonStyle {
    fn active(&self) -> iced::button::Style {
        iced::button::Style {
            background: Some(Background::Color(Color::from_rgb(0.1, 0.3, 0.6))),
            border_radius: 10.0,
            text_color: Color::WHITE,
            ..iced::button::Style::default()
        }
    }
}

fn create_keypad_row<'a>(
    states: &'a mut [button::State],
    labels: [char; 3],
) -> Row<'a, Message> {
    // Ensure that the slice has exactly 3 elements to avoid index out of bounds
    if states.len() != 3 {
        panic!("Expected exactly 3 button states, but got {}", states.len());
    }

    // Use split_at_mut to get mutable references without borrowing conflicts
    let (first_state, rest) = states.split_at_mut(1);
    let (second_state, third_state) = rest.split_at_mut(1);

    Row::new()
        .spacing(10)
        .push(create_keypad_button(&mut first_state[0], labels[0])) // Use first state
        .push(create_keypad_button(&mut second_state[0], labels[1])) // Use second state
        .push(create_keypad_button(&mut third_state[0], labels[2])) // Use third state
}

fn create_keypad_button<'a>(
    state: &'a mut button::State,
    label: char,
) -> Button<'a, Message> {
    Button::new(state, Text::new(label.to_string()).size(30))
        .padding(10)
        .on_press(Message::KeypadPressed(label))
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
