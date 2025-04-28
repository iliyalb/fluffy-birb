use iced::{widget::{column, text, container}, Element, application, Subscription, Color};

use std::time::Duration;


const DAY_COLOR: Color = Color::from_rgb(0.53, 0.81, 0.92); // skyblue
const NIGHT_COLOR: Color = Color::from_rgb(0.29, 0.0, 0.51); // dark violet
const CYCLE_DURATION: f32 = 10.0; // seconds for a full day/night cycle

#[derive(Debug, Clone)]
enum Message {
    Tick,
}

struct State {
    phase: f32, // 0.0..1.0
}

fn update(state: &mut State, message: Message) {
    match message {
        Message::Tick => {
            // Advance phase
            let increment = 1.0 / (CYCLE_DURATION * 60.0); // 60 ticks per second
            state.phase = (state.phase + increment) % 1.0;
        }
    }
}

fn view<'a>(state: &'a State) -> Element<'a, Message> {
    let t = 0.5 - 0.5 * (f32::cos(state.phase * std::f32::consts::TAU)); // Smooth cycle
    let r = DAY_COLOR.r + (NIGHT_COLOR.r - DAY_COLOR.r) * t;
    let g = DAY_COLOR.g + (NIGHT_COLOR.g - DAY_COLOR.g) * t;
    let b = DAY_COLOR.b + (NIGHT_COLOR.b - DAY_COLOR.b) * t;
    let bg = Color { r, g, b, a: 1.0 };

    container(
        column![text("Hello, world!")]
    )
    .width(iced::Length::Fill)
    .height(iced::Length::Fill)
    .style(move |_| iced::widget::container::Style {
        background: Some(bg.into()),
        text_color: Some(Color::WHITE),
        ..Default::default()
    })
    .into()
}


fn subscription(_state: &State) -> Subscription<Message> {
    iced_futures::Subscription::run(|| {
        futures::stream::unfold((), |_| async {
            async_std::task::sleep(Duration::from_millis(16)).await;
            Some(((), ()))
        })
    })
    .map(|_| Message::Tick)
}


impl Default for State {
    fn default() -> Self {
        State { phase: 0.0 }
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
