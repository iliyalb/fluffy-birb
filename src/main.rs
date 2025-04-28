use iced::{widget::{column, text}, Element, Settings, application};

#[derive(Debug, Clone)]
enum Message {}

fn update(_state: &mut (), _message: Message) {}

fn view<'a>(_state: &'a ()) -> Element<'a, Message> {
    column![text("Hello, world!")].into()
}

fn main() -> iced::Result {
    application(
        "Fluffy Birb", // title
        update,
        view,
    )
    .run()
}
