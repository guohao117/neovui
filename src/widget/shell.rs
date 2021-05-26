use iced::{
    button, pane_grid, scrollable, slider, Align, Button, Column, Container, Element,
    HorizontalAlignment, Length, Scrollable, Slider, Text,
};

use super::circle::Circle;
use super::style::{self};

#[derive(Debug, Clone, Copy)]
pub enum Message {
    RadiusChanged(f32),
    Split(pane_grid::Axis, pane_grid::Pane),
    SplitFocused(pane_grid::Axis),
    FocusAdjacent(pane_grid::Direction),
    Clicked(pane_grid::Pane),
    Dragged(pane_grid::DragEvent),
    Resized(pane_grid::ResizeEvent),
    Close(pane_grid::Pane),
    CloseFocused,
}

pub struct Window {
    pub id: usize,
    scroll: scrollable::State,
    pub radius: f32,
    slider: slider::State,
    split_horizontally: button::State,
    split_vertically: button::State,
    close: button::State,
}

impl Window {
    pub fn new(id: usize) -> Self {
        Window {
            id,
            scroll: scrollable::State::new(),
            radius: 50.0,
            slider: slider::State::new(),
            split_horizontally: button::State::new(),
            split_vertically: button::State::new(),
            close: button::State::new(),
        }
    }
    pub fn view(&mut self, pane: pane_grid::Pane, total_panes: usize) -> Element<Message> {
        let Window {
            scroll,
            radius,
            slider,
            split_horizontally,
            split_vertically,
            close,
            ..
        } = self;

        let button = |state, label, message, style| {
            Button::new(
                state,
                Text::new(label)
                    .width(Length::Fill)
                    .horizontal_alignment(HorizontalAlignment::Center)
                    .size(16),
            )
            .width(Length::Fill)
            .padding(8)
            .on_press(message)
            .style(style)
        };

        let mut controls = Column::new()
            .spacing(5)
            .max_width(150)
            .push(button(
                split_horizontally,
                "Split horizontally",
                Message::Split(pane_grid::Axis::Horizontal, pane),
                style::Button::Primary,
            ))
            .push(button(
                split_vertically,
                "Split vertically",
                Message::Split(pane_grid::Axis::Vertical, pane),
                style::Button::Primary,
            ))
            .push(Circle::new(*radius))
            .push(Text::new(format!("Radius: {:.2}", *radius)))
            .push(Slider::new(slider, 1.0..=100.0, *radius, Message::RadiusChanged).step(0.01));

        if total_panes > 1 {
            controls = controls.push(button(
                close,
                "Close",
                Message::Close(pane),
                style::Button::Destructive,
            ));
        }

        let content = Scrollable::new(scroll)
            .width(Length::Fill)
            .spacing(10)
            .align_items(Align::Center)
            .push(controls);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(5)
            .center_y()
            .into()
    }
}
