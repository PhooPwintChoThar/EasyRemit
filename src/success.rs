use iced::{
    button, Button, Column, Container, Element, Text,
    container, Alignment, Length, Row, Space, Background, Color,
};
use crate::Message;
use crate::{get_user_id, get_t_amount, get_receiver_id};
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use crate::db::execute_with_retry;
use rodio::{Decoder, OutputStream, Source};
use std::fs::File;

// Define colors for the theme
const BACKGROUND_COLOR: Color = Color::from_rgb(0.96, 0.96, 0.96); // Light Gray (#F5F5F5)
const TEXT_COLOR: Color = Color::from_rgb(0.2, 0.2, 0.2); // Dark Gray (#333333)
const PRIMARY_COLOR: Color = Color::from_rgb(0.1, 0.3, 0.6); // Blue for title (#1A4D9);

// Add your sound file path here
const SUCCESS_SOUND: &str = "successsound.wav";



#[derive(Debug, Clone)]
pub struct SuccessPage {
    confirm_button: button::State,
}

impl SuccessPage {
   
        pub fn new() -> Self {
            // Initialize audio playback
            if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
                if let Ok(file) = File::open(SUCCESS_SOUND) {
                    // Create a decoder for the audio file
                    if let Ok(source) = Decoder::new(file) {
                        println!("Playing success sound..."); // Debugging message
                        // Play the audio
                        match stream_handle.play_raw(source.convert_samples()) {
                            Ok(_) => println!("Sound is playing."),
                            Err(e) => eprintln!("Error playing sound: {:?}", e),
                        }
                        // Optional delay to allow the sound to finish
                        std::thread::sleep(std::time::Duration::from_millis(600)); 
                    } else {
                        eprintln!("Failed to create decoder for audio file");
                    }
                } else {
                    eprintln!("Failed to open audio file");
                }
            } else {
                eprintln!("Failed to initialize audio stream");
            }
    
            SuccessPage {
                confirm_button: button::State::new(),
            }
        }

    pub fn view(&mut self, db_conn: &Arc<Mutex<Connection>>) -> Element<super::Message> {
      

        let conn = db_conn.lock().expect("Failed to acquire lock");
        conn.busy_timeout(std::time::Duration::from_secs(5)).unwrap();

        let sender_id = match get_user_id() { Some(id) => id, None => "NULL".to_owned(), };
        let receiver_id = match get_receiver_id() { Some(id) => id, None => "NULL".to_owned(), };
        let transferred_amount = match get_t_amount() { Some(amount) => amount, None => 0, };

        match conn.execute("INSERT INTO transaction_history (sender_id, amount, receiver_id) VALUES (?1, ?2, ?3)", params![sender_id, transferred_amount, receiver_id]) {
            Ok(_) => println!("Transaction added successfully"),
            Err(e) => eprintln!("Failed to add transaction: {:?}", e),
        };

        let s_balance = execute_with_retry(|| {
            conn.query_row(
                "SELECT balance FROM user_information WHERE id = ?1",
                params![sender_id.clone()],
                |row| row.get::<_, i64>(0),
            )
        }, 3).unwrap_or(0);

        let r_balance = execute_with_retry(|| {
            conn.query_row(
                "SELECT balance FROM user_information WHERE id = ?1",
                params![receiver_id.clone()],
                |row| row.get::<_, i64>(0),
            )
        }, 3).unwrap_or(0);

        let sql = "UPDATE user_information SET balance=?1 WHERE id=?2";
        conn.execute(sql, params![s_balance - transferred_amount, sender_id.clone()]).expect("Failed to update sender balance");
        conn.execute(sql, params![r_balance + transferred_amount, receiver_id.clone()]).expect("Failed to update receiver balance");

        let sender_name = execute_with_retry(|| {
            conn.query_row(
                "SELECT name FROM user_information WHERE id = ?1",
                params![sender_id.clone()],
                |row| row.get::<_, String>(0),
            )
        }, 3).unwrap_or_else(|_| "Null".to_string());

        let receiver_name = execute_with_retry(|| {
            conn.query_row(
                "SELECT name FROM user_information WHERE id = ?1",
                params![receiver_id.clone()],
                |row| row.get::<_, String>(0),
            )
        }, 3).unwrap_or_else(|_| "Null".to_string());

        // Title with primary color
        let title = Text::new("EasyRemit").size(30).color(PRIMARY_COLOR);
        let transaction_successful = Text::new("Transaction successful").size(25).color(TEXT_COLOR);
        let amount = Text::new("Amount").size(20).color(TEXT_COLOR);
        let value = Text::new(transferred_amount.to_string()).size(40).color(TEXT_COLOR);
        let from_label = Text::new("From").size(20).color(Color::WHITE);
        let from_name = Text::new(sender_name.clone()).size(20).color(Color::WHITE);
        let to_label = Text::new("To").size(20).color(Color::WHITE);
        let to_name = Text::new(receiver_name.clone()).size(20).color(Color::WHITE);
        let tagline = Text::new("\"Instant Transfer, Anytime, Anywhere\"").size(18).color(TEXT_COLOR);

        // Layout for "From" and "To"
        let from_row = Row::new()
            .push(Space::with_width(Length::Units(30)))
            .push(from_label)
            .push(Space::with_width(Length::Units(180)))
            .push(from_name)
            .align_items(Alignment::Center);

        let to_row = Row::new()
            .push(Space::with_width(Length::Units(30)))
            .push(to_label)
            .push(Space::with_width(Length::Units(180)))
            .push(to_name)
            .align_items(Alignment::Center);

        let styled_from_row = Container::new(from_row)
            .width(Length::Fill)
            .style(RowContainerStyle);

        let styled_to_row = Container::new(to_row)
            .width(Length::Fill)
            .style(RowContainerStyle);

        let filled_space_above_from_row = Container::new(Space::with_height(Length::Units(20)))
            .width(Length::Fill)
            .style(RowContainerStyle);

        let filled_space_below_to_row = Container::new(Space::with_height(Length::Units(20)))
            .width(Length::Fill)
            .style(RowContainerStyle);

        let filled_space_between_rows = Container::new(Space::with_height(Length::Units(10)))
            .width(Length::Fill)
            .style(RowContainerStyle);

        let ok_button = Button::new(&mut self.confirm_button, Text::new(" Ok "))
            .on_press(Message::GoToFunction)
            .padding(10)
            .style(CustomButtonStyle);

        let main_content = Column::new()
            .align_items(Alignment::Center)
            .push(title)
            .push(Space::with_height(Length::Units(80)))
            .push(transaction_successful)
            .push(Space::with_height(Length::Units(50)))
            .push(amount)
            .push(Space::with_height(Length::Units(10)))
            .push(value)
            .push(Space::with_height(Length::Units(20)))
            .push(filled_space_above_from_row)
            .push(styled_from_row)
            .push(filled_space_between_rows)
            .push(styled_to_row)
            .push(filled_space_below_to_row)
            .push(Space::with_height(Length::Units(60)))
            .push(tagline)
            .push(Space::with_height(Length::Units(20)))
            .push(ok_button);

        Container::new(main_content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(CustomContainerStyle)
            .into()
    
}
}



// Custom container style for background color
struct CustomContainerStyle;

impl container::StyleSheet for CustomContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(BACKGROUND_COLOR)),
            text_color: Some(TEXT_COLOR),
            ..container::Style::default()
        }
    }
}

// Custom container style for from_row and to_row with primary color
struct RowContainerStyle;

impl container::StyleSheet for RowContainerStyle {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(PRIMARY_COLOR)),
            text_color: Some(Color::WHITE), // Text color to be white for readability
            ..container::Style::default()
        }
    }
}
struct CustomButtonStyle;

impl iced::button::StyleSheet for CustomButtonStyle {
    fn active(&self) -> iced::button::Style {
        iced::button::Style {
            background: Some(Background::Color(PRIMARY_COLOR)), // Set button background color
            text_color: Color::WHITE, // Set button text color
            ..iced::button::Style::default()
        }
    }
}
