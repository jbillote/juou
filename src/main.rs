use iced::widget::text;
use iced::{Element, Event, Fill, Font, Subscription, Task, event, window};

pub fn main() -> iced::Result {
    iced::application(Juou::new, Juou::update, Juou::view)
        .font(include_bytes!("../fonts/NotoSansJP-Regular.ttf").as_slice())
        .default_font(Font::with_name("Noto Sans JP"))
        .subscription(Juou::subscription)
        .run()
}

#[derive(Default)]
struct Juou {
    scale_factor: f32,
}

#[derive(Clone, Copy)]
enum Message {
    WindowRescaled(f32),
}

impl Juou {
    fn new() -> (Self, Task<Message>) {
        (
            Self { scale_factor: 1.0 },
            window::latest()
                .and_then(window::scale_factor)
                .map(Message::WindowRescaled),
        )
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::WindowRescaled(scale_factor) => {
                self.scale_factor = scale_factor;
                Task::none()
            }
        }
    }

    fn subscription(&self) -> Subscription<Message> {
        event::listen_with(|event, _, _| {
            let Event::Window(window::Event::Rescaled(scale_factor)) = event else {
                return None;
            };

            Some(Message::WindowRescaled(scale_factor))
        })
    }

    fn view(&self) -> Element<'_, Message> {
        text("十王")
            .size(30)
            .line_height(1.5)
            .width(Fill)
            .height(Fill)
            .center()
            .into()
    }
}
