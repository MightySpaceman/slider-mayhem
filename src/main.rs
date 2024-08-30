use iced::{
    alignment,
    application::StyleSheet,
    widget::{
        button, column, container, horizontal_rule, horizontal_space, row, scrollable, slider,
        text, text_input, vertical_slider, Column,
    },
    Element, Length, Padding, Sandbox, Settings,
};

struct State {
    slider_values: Vec<f32>,
    num_sliders: i32,
    button_pressed: bool,
}

#[derive(Debug, Clone)]
enum Message {
    SliderMoved(usize, f32),
    SliderSliderMoved(i32),
    ButtonPressed,
}

impl Sandbox for State {
    type Message = Message;

    fn new() -> State {
        Self {
            slider_values: Vec::from([25.0, 25.0, 25.0, 25.0, 25.0, 25.0, 50.0, 15.0]),
            num_sliders: 10,
            button_pressed: false,
        }
    }

    fn title(&self) -> String {
        String::from("Slider mayhem")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::SliderMoved(index, value) => {
                update_slider_pos(index, value, &mut self.slider_values, self.button_pressed);
            }
            Message::SliderSliderMoved(value) => {
                self.num_sliders = value;
                self.slider_values = update_sliders(&self.slider_values, self.num_sliders);
                println!("{:?}", self.slider_values);
            }
            Message::ButtonPressed => self.button_pressed = !self.button_pressed,
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let button_text: String;
        if self.button_pressed {
            button_text = String::from("LOCKED")
        } else {
            button_text = String::from("Unlocked")
        }
        container(
            column!(
                button(text(button_text)).on_press(Message::ButtonPressed),
                horizontal_rule(20),
                slider(
                    0..=50,
                    self.num_sliders,
                    |value| Message::SliderSliderMoved(value)
                ),
                horizontal_rule(20),
                row!(render_sliders(&self.slider_values))
            )
            .align_items(iced::Alignment::Center),
        )
        .padding(100)
        .height(Length::Fill)
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Center)
        .align_y(alignment::Vertical::Center)
        .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }
}

// updates the vec of sliders based on the amount required by the slider slider
fn update_sliders(items: &Vec<f32>, num_sliders: i32) -> Vec<f32> {
    let mut new_vec = Vec::new();

    for i in 0..num_sliders {
        new_vec.push(if items.get(i as usize) == None {
            0.0
        } else {
            items[i as usize]
        })
    }

    new_vec
}

// Handles the updating of slider positions - takes a boolean to "lock" them so that dragging one
// moves the rest
fn update_slider_pos(index: usize, value: f32, slider_values: &mut Vec<f32>, button_pressed: bool) {
    let delta_pos = value - slider_values[index];
    if button_pressed {
        for i in 0..slider_values.len() {
            slider_values[i] = slider_values[i] + delta_pos;
        }
    } else {
        slider_values[index] = slider_values[index] + delta_pos;
    }
}

// Generates a scrollable box of sliders from a vec
fn render_sliders(items: &Vec<f32>) -> Element<'static, Message> {
    let mut column = Column::new()
        .spacing(20)
        .align_items(iced::Alignment::Center)
        .width(Length::Fill);

    for (index, value) in items.into_iter().enumerate() {
        column = column.push(slider(
            0.0..=100.0,
            items.get(index).unwrap().clone(),
            move |value| Message::SliderMoved(index, value),
        ));
    }

    scrollable(container(column))
        .height(250.0)
        .width(300)
        .into()
}

fn main() -> iced::Result {
    State::run(Settings::default())
}
