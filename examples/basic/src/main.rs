use iced::widget::{container, text};
use iced::{Element, Length, Point, Sandbox, Settings};
use iced_node_editor::{connection, graph_container, node, Matrix};

pub fn main() -> iced::Result {
    // To resize the the resulting canvas for web: https://github.com/iced-rs/iced/issues/1265
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let (width, height) = (
            (window.inner_width().unwrap().as_f64().unwrap())
                as u32,
            (window.inner_height().unwrap().as_f64().unwrap())
                as u32,
        );

        Example::run(Settings{
            window: iced::window::Settings {
                size: (width, height),
                ..Default::default()
            },
            ..Default::default()
        })?;
    }

    #[cfg(not(target_arch = "wasm32"))]
    Example::run(Settings{
        window: iced::window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        ..Default::default()
    })?;


    Ok(())
}

struct Example {
    matrix: Matrix,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ScaleChanged(f32, f32, f32),
    TranslationChanged(f32, f32),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example {
            matrix: Matrix::identity(),
        }
    }

    fn title(&self) -> String {
        String::from("Iced Graph Editor - Basic Example")
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::Dark
    }

    fn update(&mut self, _message: Message) {
        match _message {
            Message::ScaleChanged(x, y, scale) => {
                self.matrix = self
                    .matrix
                    .translate(-x, -y)
                    .scale(if scale > 0.0 { 1.2 } else { 1.0 / 1.2 })
                    .translate(x, y);
            }
            Message::TranslationChanged(x, y) => self.matrix = self.matrix.translate(x, y),
        }
    }

    fn view(&self) -> Element<Message> {
        container(
            graph_container(vec![
                node(text("Iced"))
                    .center_x()
                    .center_y()
                    .width(Length::Units(200))
                    .height(Length::Units(75))
                    .into(),
                node(text("Node"))
                    .center_x()
                    .center_y()
                    .width(Length::Units(200))
                    .height(Length::Units(75))
                    .position(Point::new(250.0, 250.0))
                    .into(),
                node(text("Editor"))
                    .center_x()
                    .center_y()
                    .width(Length::Units(200))
                    .height(Length::Units(75))
                    .position(Point::new(500.0, 250.0))
                    .into(),
                connection(Point::new(200.0, 37.5), Point::new(250.0, 250.0 + 32.5)).into(),
                connection(
                    Point::new(450.0, 250.0 + 32.5),
                    Point::new(500.0, 250.0 + 32.5),
                )
                .into(),
            ])
            .on_translate(|p| Message::TranslationChanged(p.0, p.1))
            .on_scale(|x, y, s| Message::ScaleChanged(x, y, s))
            .width(Length::Fill)
            .height(Length::Fill)
            .matrix(self.matrix),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
