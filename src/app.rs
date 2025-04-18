use crate::models::ShortcutCategory;
use iced::{
    widget::{button, column, container, row, text},
    Application, Command, Element, Length, Theme,
    keyboard, event,
};

#[derive(Debug)]
pub struct MacShortcutTips {
    categories: Vec<ShortcutCategory>,
    current_category: usize,
}

#[derive(Debug, Clone)]
pub enum Message {
    NextCategory,
    PreviousCategory,
}

impl Application for MacShortcutTips {
    fn theme(&self) -> Theme {
        Theme::Light
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
    fn subscription(&self) -> iced::Subscription<Message> {
        iced::subscription::events_with(
            |event, _| {
                if let event::Event::Keyboard(keyboard_event) = event {
                    if let keyboard::Event::KeyPressed { key_code, .. } = keyboard_event {
                        match key_code {
                            keyboard::KeyCode::Left => Some(Message::PreviousCategory),
                            keyboard::KeyCode::Right => Some(Message::NextCategory),
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
        )
    }
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                categories: crate::utils::load_shortcuts(),
                current_category: 0,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Mac Shortcut Tips")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::NextCategory => {
                if self.current_category < self.categories.len() - 1 {
                    self.current_category += 1;
                }
            }
            Message::PreviousCategory => {
                if self.current_category > 0 {
                    self.current_category -= 1;
                }
            }

        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let category = &self.categories[self.current_category];
        
        let nav_button_width = Length::Fixed(40.0);
        let nav_button_height = Length::Fixed(30.0);
        
        let navigation = row![
            button(
                container(text("←").size(16))
                    .width(nav_button_width)
                    .height(nav_button_height)
                    .center_x()
                    .center_y()
            )
            .on_press(Message::PreviousCategory)
            .style(iced::theme::Button::Secondary),
            
            container(text(&category.name).size(20))
                .width(Length::Fixed(150.0))
                .center_x(),
                
            button(
                container(text("→").size(16))
                    .width(nav_button_width)
                    .height(nav_button_height)
                    .center_x()
                    .center_y()
            )
            .on_press(Message::NextCategory)
            .style(iced::theme::Button::Secondary)
        ]
        .spacing(10)
        .padding(10)
        .align_items(iced::Alignment::Center);

        let shortcuts = category
            .shortcuts
            .iter()
            .fold(column![].spacing(5), |column, shortcut| {
                column.push(
                    row![
                        container(text(&shortcut.name))
                            .width(Length::Fixed(120.0)),
                        container(text(&shortcut.key_combination))
                            .width(Length::Fixed(100.0)),
                        container(text(&shortcut.description))
                            .width(Length::Fixed(200.0))
                    ]
                    .spacing(10),
                )
            });

        container(column![navigation, shortcuts].spacing(10))
            .width(Length::Fixed(450.0))
            .height(Length::Fixed(400.0))
            .padding(10)
            .center_x()
            .center_y()
            .into()
    }
}
