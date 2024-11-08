use iced::{button, Button, Column, Container, Element, Text, Row};
use iced::{text_input, Alignment,Length,TextInput, Background, Color};
use crate::Message;
#[derive(Debug, Clone)]
pub struct HomePage{
    login_button: button::State,
    signup_button: button::State,
}

impl HomePage {
    pub fn new()->Self{
        HomePage{
            login_button:button::State::new(),
            signup_button:button::State::new(),
        }
    }
    pub fn view(&mut self)-> Element<super::Message> {
        let main_text = Text::new("Secure your financial future with us")
        .size(40)
        .width(Length::Fill)
        .horizontal_alignment(iced::alignment::Horizontal::Left);


        let sub_text = Text::new("Your financial future, our priority. Secure your finances with our trusted banking services.")
            .size(20)
            .width(Length::Fill)
            .horizontal_alignment(iced::alignment::Horizontal::Left);


        let login_button=Button::new(&mut self.login_button, Text::new("LOG IN"))
        .padding(15)
        .style(LogInButtonStyle) 
        .on_press(Message::GoToLogin);

        let signup_button=Button::new(&mut self.signup_button, Text::new("Sign Up"))
                .on_press(Message::GoToSignup)
                .padding(15)
                .style(SignUpButtonStyle);

        let buttons= Row::new()
        .spacing(10)
        .padding(10)
        .push(login_button)
        .push(signup_button);


        let content = Column::new()
            .spacing(20)
            .padding(25)
            .align_items(Alignment::Start)
            .push(main_text)
            .push(sub_text)
            .push(buttons);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()

        
    }
}

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

struct SignUpButtonStyle;
impl iced::button::StyleSheet for SignUpButtonStyle {
    fn active(&self) -> iced::button::Style {
        iced::button::Style {
            background: Some(Background::Color(Color::WHITE)),
            border_radius: 10.0,
            border_color:Color::BLACK,
            border_width:1.0,
            text_color: Color::BLACK,
            ..iced::button::Style::default()
        }
    }
}
