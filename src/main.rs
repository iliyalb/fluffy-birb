use iced::{widget::{container, Column, Space, Row, Image}, Element, application, Subscription, Color, Length};
use iced::keyboard::Key;
use iced::keyboard::key::Named;

use std::time::Duration;


const DAY_COLOR: Color = Color::from_rgb(0.53, 0.81, 0.92); // skyblue
const NIGHT_COLOR: Color = Color::from_rgb(0.29, 0.0, 0.51); // dark violet
const CYCLE_DURATION: f32 = 10.0; // seconds for a full day/night cycle

#[derive(Debug, Clone)]
enum Message {
    Tick,
    Boost,
}

struct State {
    phase: f32, // 0.0..1.0
    bird_x: f32, // 0.0 (left) .. 1.0 (right)
    bird_y: f32, // 0.0 (top) .. 1.0 (bottom)
    bird_velocity_x: f32,
    bird_velocity_y: f32,
}

const GRAVITY: f32 = 0.005;
const BOOST: f32 = -0.04;
const TERMINAL_VELOCITY: f32 = 0.04;

fn update(state: &mut State, message: Message) {
    match message {
        Message::Tick => {
            // Advance phase
            let increment = 1.0 / (CYCLE_DURATION * 60.0); // 60 ticks per second
            state.phase = (state.phase + increment) % 1.0;

            // Bird horizontal movement
            state.bird_x += state.bird_velocity_x;
            if state.bird_x < 0.0 {
                state.bird_x = 1.0; // Respawn at right
            }

            // Bird vertical physics
            state.bird_velocity_y += GRAVITY;
            if state.bird_velocity_y > TERMINAL_VELOCITY {
                state.bird_velocity_y = TERMINAL_VELOCITY;
            }
            state.bird_y += state.bird_velocity_y;
            if state.bird_y > 1.0 {
                state.bird_y = 1.0;
                state.bird_velocity_y = 0.0;
            } else if state.bird_y < 0.0 {
                state.bird_y = 0.0;
                state.bird_velocity_y = 0.0;
            }
        }
        Message::Boost => {
            state.bird_velocity_y = BOOST;
        }
    }
}

fn view<'a>(state: &'a State) -> Element<'a, Message> {
    let t = 0.5 - 0.5 * (f32::cos(state.phase * std::f32::consts::TAU)); // Smooth cycle
    let r = DAY_COLOR.r + (NIGHT_COLOR.r - DAY_COLOR.r) * t;
    let g = DAY_COLOR.g + (NIGHT_COLOR.g - DAY_COLOR.g) * t;
    let b = DAY_COLOR.b + (NIGHT_COLOR.b - DAY_COLOR.b) * t;
    let bg = Color { r, g, b, a: 1.0 };

    // Calculate horizontal and vertical position
    let bird_y = state.bird_y.clamp(0.0, 1.0);
    let bird_x = state.bird_x.clamp(0.0, 1.0);
    let scale = 100u16;
    let top_portion = (bird_y * scale as f32).round() as u16;
    let bottom_portion = ((1.0 - bird_y) * scale as f32).round() as u16;
    let top_space = Length::FillPortion(top_portion.max(1));
    let bottom_space = Length::FillPortion(bottom_portion.max(1));
    let left_space = Length::FillPortion((bird_x * 100.0) as u16);
    let right_space = Length::FillPortion(((1.0 - bird_x) * 100.0) as u16);

    let birb = Image::new("assets/birb.png")
    .width(Length::Fixed(32.0))
    .height(Length::Fixed(32.0));

    container(
        Column::new()
            .push(Space::new(Length::Shrink, top_space))
            .push(
                Row::new()
                    .push(Space::new(left_space, Length::Shrink))
                    .push(birb)
                    .push(Space::new(right_space, Length::Shrink))
            )
            .push(Space::new(Length::Shrink, bottom_space))
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .style(move |_| iced::widget::container::Style {
        background: Some(bg.into()),
        text_color: Some(Color::WHITE),
        ..Default::default()
    })
    .into()
}

fn subscription(_state: &State) -> Subscription<Message> {
    use iced::keyboard;

    // Timer
    let timer = iced_futures::Subscription::run(|| {
        futures::stream::unfold((), |_| async {
            async_std::task::sleep(Duration::from_millis(16)).await;
            Some(((), ()))
        })
    })
    .map(|_| Message::Tick);

    // Keyboard
    let keyboard = keyboard::on_key_press(|key, _modifiers| {
        if let Key::Named(Named::Space) = key {
            Some(Message::Boost)
        } else {
            None
        }
    });

    Subscription::batch(vec![timer, keyboard])
}

impl Default for State {
    fn default() -> Self {
        State {
            phase: 0.0,
            bird_x: 1.0, // Start at the right edge
            bird_y: 0.5, // Start in the middle vertically
            bird_velocity_x: -0.005, // Move left
            bird_velocity_y: 0.0,
        }
    }
}

fn main() -> iced::Result {
    application(
        "Fluffy Birb",
        update,
        view,
    )
    .subscription(|state| subscription(state))
    .run()
}
