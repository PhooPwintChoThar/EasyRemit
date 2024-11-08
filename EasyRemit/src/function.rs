use iced::{button, Button, Column, Container, Element, Text};
use iced::{Length, Row, alignment::Horizontal, Alignment, scrollable, Scrollable,Background, Color};
use crate::Message;
use crate::get_user_id;
use rusqlite::{params, Connection, Result};
use std::sync::{Arc, Mutex};
use crate::db::execute_with_retry;
use aes::{Aes128};
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;
use base64::decode;



type Aes128Cbc = Cbc<Aes128, Pkcs7>;


#[derive(Debug)]
struct AppError(String);

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn decrypt(encrypted_data: &[u8], key: &[u8; 16], iv: &[u8; 16]) -> Result<Vec<u8>, AppError> {
    let cipher = Aes128Cbc::new_from_slices(key, iv).map_err(|e| AppError(e.to_string()))?;
    let mut buffer = encrypted_data.to_vec();
    let decrypted_data = cipher.decrypt(&mut buffer).map_err(|e| AppError(e.to_string()))?;
    
    Ok(decrypted_data.to_vec())
}
#[derive(Debug, Clone)]
pub struct FunctionPage {
    transfer_button: button::State,
    scrollable_state: scrollable::State,
    logout_button:button::State,
}

impl FunctionPage {
    pub fn new() -> Self {
        FunctionPage {
            transfer_button: button::State::new(),
            scrollable_state: scrollable::State::new(),
            logout_button:button::State::new(),
        }
    }

    pub fn view<'a>(&'a mut self, db_conn: &'a Arc<Mutex<Connection>>) -> Element<'a, super::Message> {
        let conn = db_conn.lock().expect("Failed to acquire lock");
        let user_id=match get_user_id(){
            Some(id)=>id,
            None=>"NULL".to_owned(),
        };

        let user_name = execute_with_retry(|| {
            conn.query_row(
                "SELECT name FROM user_information WHERE id = ?1",
                params![user_id.clone()],
                |row| row.get::<_, String>(0),
            )
        }, 3).unwrap_or_else(|_| "Null".to_string());

        let balance = execute_with_retry(|| {
            conn.query_row(
                "SELECT balance FROM user_information WHERE id = ?1",
                params![user_id.clone()],
                |row| row.get::<_, i64>(0),
            )
        }, 3).unwrap_or(0);



        let total_balance = Container::new(
            Row::new()
                .spacing(10)
                .align_items(Alignment::Center)
                .push(icon())  // Placeholder icon
                .push(
                    Column::new()
                        .spacing(5)
                        .align_items(Alignment::Start)
                        .push(Text::new("TOTAL BALANCE").size(14).color([0.6, 0.6, 0.6]))
                        .push(Text::new(balance.to_string()).size(25).color([0.1, 0.1, 0.1])),
                ),
        )
        .width(Length::Fill)
        .padding(20)
        .style(styles::Card);
        const KEY: [u8; 16] = *b"mysecretkey12345";
        const IV: [u8; 16] = *b"uniqueiv12345678";
        let encrypted_birth_date=execute_with_retry(|| {
            conn.query_row(
                "SELECT encrypted_birthdate FROM user_information WHERE id = ?1",
                params![user_id.clone()],
                |row| row.get::<_, String>(0),
            )
        }, 3).unwrap_or_else(|_| "Null".to_string());

        let encrypted_birthdate_bytes = decode(encrypted_birth_date).map_err(|e| AppError(e.to_string())).unwrap();

        let encrypted_passport=execute_with_retry(|| {
            conn.query_row(
                "SELECT encrypted_passport FROM user_information WHERE id = ?1",
                params![user_id.clone()],
                |row| row.get::<_, String>(0),
            )
        }, 3).unwrap_or_else(|_| "Null".to_string());


        let encrypted_passport_bytes = decode(encrypted_passport).map_err(|e| AppError(e.to_string())).unwrap();
        let birth_date=match decrypt(&encrypted_birthdate_bytes, &KEY, &IV){
            Ok(i)=>String::from_utf8(i)
            .unwrap_or_else(|_| "Invalid UTF-8 data".to_string()),
            Err(_)=>"00/00/0000".to_string(),

        };

        let passport=match decrypt(&encrypted_passport_bytes, &KEY, &IV){
            Ok(i)=>String::from_utf8(i)
            .unwrap_or_else(|_| "Invalid UTF-8 data".to_string()),
            Err(_)=>"AA000000".to_string(),

        };
        let userid_text=format!("Account Number : {}",user_id.clone());
        let birth_text=format!("Birth Date : {}", birth_date);
        let passport_text=format!("Passport : {}", passport);


        // Card information section
        let card_info = Container::new(
            Column::new()
                .spacing(8)
                .push(
                    Row::new()
                        .push(Text::new(user_name.clone()).size(22).color([0.1, 0.1, 0.1]))
                        .push(
                            Text::new("VISA")
                                .size(20)
                                .color([0.7, 0.7, 0.7])
                                .width(Length::Fill)
                                .horizontal_alignment(Horizontal::Right)
                        )
                )
                .push(Text::new(userid_text).size(17).color([0.5, 0.5, 0.5]))
                .push(Text::new(birth_text).size(13).color([0.5, 0.5, 0.5]))
                .push(Text::new(passport_text).size(13).color([0.5, 0.5, 0.5]))
                .push(Text::new("Expiry  : 10/30").size(13).color([0.5, 0.5, 0.5]))
        )
        .padding(20)
        .width(Length::Fill)
        .style(styles::Card);

        // Create a vector to hold transaction rows
        let transaction_rows = execute_with_retry(|| {
            let mut rows = Vec::new();
            let mut stmt = conn.prepare(
                "SELECT th.sender_id, th.amount, th.receiver_id,
                        COALESCE(s.name, 'New User''s Privilege') as sender_name,
                        COALESCE(r.name, 'New User''s Privilege') as receiver_name
                FROM transaction_history th
                LEFT JOIN user_information s ON th.sender_id = s.id
                LEFT JOIN user_information r ON th.receiver_id = r.id
                WHERE th.sender_id = ?1 OR th.receiver_id = ?1",
            )?;

            let transactions = stmt.query_map(params![user_id.clone()], |row| {
                Ok((
                    row.get::<_, String>(0)?,  // sender_id
                    row.get::<_, i64>(1)?,     // amount
                    row.get::<_, String>(2)?,  // receiver_id
                    row.get::<_, String>(3)?,  // sender_name
                    row.get::<_, String>(4)?,  // receiver_name
                ))
            })?;
           
            for transaction in transactions {
                let (sender_id, amount, receiver_id, sender_name, receiver_name) =
                    transaction.expect("Failed to retrieve transaction");

                let (amount_str, related_name) = if sender_id == user_id.clone() {
                    (format!("-${}", amount), receiver_name)
                } else {
                    (format!("+${}", amount), sender_name)
                };

                rows.push(transaction_row(related_name, amount_str));
            }
            Ok(rows)
        }, 3).unwrap_or_default();

        let mut transactions_column = Column::new().spacing(15);
        for row in transaction_rows {
            transactions_column = transactions_column.push(row);
        }

        let scrollable_transactions = Scrollable::new(&mut self.scrollable_state)
            .padding(8)
            .width(Length::Fill)
            .height(Length::Units(200))
            .push(transactions_column);

        let transactions_container = Container::new(
            Column::new()
                .spacing(8)
                .push(Text::new("Transactions").size(20).color([0.1, 0.1, 0.1]))
                .push(scrollable_transactions),
        )
        .padding(20)
        .style(styles::Card);

        let transfer_button = Button::new(&mut self.transfer_button, Text::new("Transfer"))
            .on_press(super::Message::GoToTransfer)
            .padding(12)
            .style(styles::TransferButton);

        let logout_button = Button::new(&mut self.logout_button, Text::new("Log Out"))
            .padding(1)
            .style(LogOutButtonStyle)
            .on_press(Message::GoToHome);

        let content = Column::new()
            .spacing(13)
            .align_items(Alignment::Center)
            .push(total_balance)
            .push(card_info)
            .padding(30)
            .push(transactions_container)
            .push(transfer_button)
            .push(logout_button);

        Container::new(content)
            .padding(20)
            .center_x()
            .center_y()
            .width(Length::Fill)
            .into()
    }
}


// Placeholder icon (a simple circle)
fn icon<'a>() -> Element<'a, Message> {
    Container::new(Text::new("👤"))  // Emoji as placeholder icon
        .width(Length::Units(30))
        .height(Length::Units(30))
        .center_x()
        .center_y()
        .style(styles::Icon)
        .into()
}

// Custom styles for the UI components
mod styles {
    use iced::{button, container, Background, Color};

    pub struct Card;

    impl container::StyleSheet for Card {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(Color::WHITE)),
                border_radius: 15.0,
                border_width: 1.0,
                border_color: Color::from_rgb(0.2, 0.4, 0.6),
                text_color: Some(Color::from_rgb(0.0, 0.0, 0.0)),
            }
        }
    }

    pub struct Icon;

    impl container::StyleSheet for Icon {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                border_radius: 15.0,
                ..container::Style::default()
            }
        }
    }

    pub struct TransferButton;

    impl button::StyleSheet for TransferButton {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.3, 0.6))),
                border_radius: 8.0,
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }

    pub struct ButtonContainer;

    impl container::StyleSheet for ButtonContainer {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.3, 0.6))),
                border_radius: 15.0,
                ..container::Style::default()
            }
        }
    }
}

// Helper function to create a transaction row with an icon
fn transaction_row<'a>(name: String, amount: String) -> Row<'a, Message> {
    Row::new()
        .spacing(15)
        .align_items(Alignment::Center)
        .push(icon())  // Placeholder icon for user
        .push(Text::new(name).size(18).color([0.1, 0.1, 0.1]))
        .push(
            Text::new(amount.clone())
                .size(18)
                .color(if amount.starts_with('+') { [0.2, 0.6, 0.2] } else { [0.8, 0.1, 0.1] })
                .width(Length::Fill)
                .horizontal_alignment(iced::alignment::Horizontal::Right),
        )
}

struct LogOutButtonStyle;
impl iced::button::StyleSheet for LogOutButtonStyle {
    fn active(&self) -> iced::button::Style {
        iced::button::Style {
            background: Some(Background::Color(Color::WHITE)),
            border_radius: 10.0,
            text_color: Color::BLACK,
            ..iced::button::Style::default()
        }
    }
}