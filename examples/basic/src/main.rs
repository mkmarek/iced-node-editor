use iced::widget::{container, text};
use iced::{Element, Length, Point, Sandbox, Settings};
use iced_node_editor::{connection, graph_container, node, Matrix};

pub fn main() -> iced::Result {
    // To resize the the resulting canvas for web: https://github.com/iced-rs/iced/issues/1265
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().unwrap();
        let (width, height) = (
            (window.inner_width().unwrap().as_f64().unwrap()) as u32,
            (window.inner_height().unwrap().as_f64().unwrap()) as u32,
        );

        Example::run(Settings {
            window: iced::window::Settings {
                size: (width, height),
                ..Default::default()
            },
            ..Default::default()
        })?;
    }

    #[cfg(not(target_arch = "wasm32"))]
    Example::run(Settings {
        window: iced::window::Settings {
            size: (800, 600),
            ..Default::default()
        },
        ..Default::default()
    })?;

    Ok(())
}

struct NodeState {
    position: Point,
    text: String,
}

struct Example {
    matrix: Matrix,
    nodes: Vec<NodeState>,
    connections: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    ScaleChanged(f32, f32, f32),
    TranslationChanged(f32, f32),
    MoveNode(usize, f32, f32),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example {
            matrix: Matrix::identity(),
            nodes: vec![
                NodeState {
                    position: Point::new(0.0, 0.0),
                    text: String::from("Iced"),
                },
                NodeState {
                    position: Point::new(250.0, 250.0),
                    text: String::from("Node"),
                },
                NodeState {
                    position: Point::new(500.0, 250.0),
                    text: String::from("Editor"),
                },
            ],
            connections: vec![(0, 1), (1, 2)],
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
            Message::MoveNode(i, x, y) => {
                self.nodes[i].position =
                    Point::new(self.nodes[i].position.x + x / self.matrix.get_scale(), self.nodes[i].position.y + y / self.matrix.get_scale());
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let mut graph_content = Vec::new();

        for (i, n) in self.nodes.iter().enumerate() {
            graph_content.push(
                node(text(&n.text))
                    .center_x()
                    .center_y()
                    .on_translate(move |p| Message::MoveNode(i, p.0, p.1))
                    .width(Length::Units(200))
                    .height(Length::Units(75))
                    .position(n.position)
                    .into(),
            );
        }

        for (_i, c) in self.connections.iter().enumerate() {
            graph_content.push(
                connection(
                    Point::new(
                        self.nodes[c.0].position.x + 200.0,
                        self.nodes[c.0].position.y + 37.5,
                    ),
                    Point::new(
                        self.nodes[c.1].position.x,
                        self.nodes[c.1].position.y + 37.5,
                    ),
                )
                .into(),
            );
        }

        container(
            graph_container(graph_content)
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
