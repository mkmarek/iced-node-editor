use std::sync::Mutex;

use iced::{Length, Point, Size, Vector};
use iced_graphics::triangle::{ColoredVertex2D, Mesh2D};
use iced_native::{renderer::{self}, Widget};

use crate::{
    mesh_renderer::MeshRenderer,
    node_element::{GraphNodeElement, ScalableWidget},
    styles::connection::StyleSheet,
};

pub struct Connection<Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    from: Point,
    to: Point,
    width: f32,
    number_of_segments: usize,
    style: <Renderer::Theme as StyleSheet>::Style,

    phantom_message: std::marker::PhantomData<Message>,
    scale: Mutex<f32>
}

impl<Message, Renderer> Connection<Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub fn new(from: Point, to: Point) -> Self {
        Connection {
            scale: Mutex::new(1.0),
            from,
            to,
            width: 1.0,
            number_of_segments: 10,
            phantom_message: std::marker::PhantomData,
            style: Default::default(),
        }
    }

    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    pub fn number_of_segments(mut self, number_of_segments: usize) -> Self {
        self.number_of_segments = number_of_segments;
        self
    }
}

pub fn connection<'a, Message, Renderer>(from: Point, to: Point) -> Connection<Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    Connection::new(from, to)
}

impl<'a, Message, Renderer> ScalableWidget<Message, Renderer> for Connection<Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &iced_native::layout::Limits,
        scale: f32,
    ) -> iced_native::layout::Node {
        let width = (self.from.x - self.to.x).abs().max(self.width);
        let height = (self.from.y - self.to.y).abs().max(self.width);

        let node = iced_native::layout::Node::new(Size::new(width * scale, height * scale));

        let mut self_state = self.scale.lock().expect("Could not lock mutex");
        *self_state = scale;

        node.translate(Vector::new(self.from.x, self.from.y) * scale)
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Connection<Message, Renderer>
where
    Renderer: renderer::Renderer + MeshRenderer,
    Renderer::Theme: StyleSheet,
{
    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        todo!("This should never be called.")
    }

    fn draw(
        &self,
        _tree: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        _renderer_style: &renderer::Style,
        layout: iced_native::Layout<'_>,
        _cursor_position: iced::Point,
        _viewport: &iced::Rectangle,
    ) {
        let bounds = layout.bounds();
        let style = theme.appearance(&self.style);

        let min_x = self.from.x.min(self.to.x);
        let min_y = self.from.y.min(self.to.y);
        let max_x = self.from.x.max(self.to.x);
        let max_y = self.from.y.max(self.to.y);

        let width = (max_x - min_x).max(self.width);
        let height = (max_y - min_y).max(self.width);

        let from = Vector::new(
            (self.from.x - min_x) / width * bounds.width,
            (self.from.y - min_y) / height * bounds.height,
        );
        let to = Vector::new(
            (self.to.x - min_x) / width * bounds.width,
            (self.to.y - min_y) / height * bounds.height,
        );

        let control_scale = width.max(30.0_f32) * *self.scale.lock().unwrap();

        let control_a = Vector {
            x: from.x + control_scale,
            y: from.y,
        };
        let control_b = Vector {
            x: to.x - control_scale,
            y: to.y,
        };

        let midpoint = (from + to) * 0.5_f32;

        let mut spline = generate_spline(from, control_a, midpoint, self.number_of_segments);
        spline.extend(generate_spline(midpoint, control_b, to, self.number_of_segments));

        let (vertices, indices) = line_to_polygon(&spline, self.width / 2.0);

        let mesh = Mesh2D {
            vertices: vertices
                .iter()
                .map(|p| ColoredVertex2D {
                    position: [p.x, p.y],
                    color: style.color.unwrap().into_linear(),
                })
                .collect(),
            indices,
        };

        renderer.with_translation(Vector::new(bounds.x, bounds.y), |renderer| {
            renderer.draw_mesh(mesh);
        });
    }

    fn width(&self) -> Length {
        Length::Units(((self.from.x - self.to.x).abs() + self.width).ceil() as u16)
    }

    fn height(&self) -> Length {
        Length::Units(((self.from.y - self.to.y).abs() + self.width).ceil() as u16)
    }
}

impl<'a, Message, Renderer> From<Connection<Message, Renderer>>
    for GraphNodeElement<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + MeshRenderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(node: Connection<Message, Renderer>) -> Self {
        Self::new(node)
    }
}

fn line_to_polygon(points: &[Vector], width: f32) -> (Vec<Vector>, Vec<u32>) {
    let mut result = Vec::new();
    let mut indices = Vec::new();

    let mut last = points[0];
    for point in points.iter().skip(1) {
        let dir = normalize_vector(*point - last);
        let normal = Vector::new(dir.y, -dir.x);

        result.push(last + normal * width);
        result.push(*point + normal * width);
        result.push(*point - normal * width);
        result.push(last - normal * width);

        let start = result.len() as u32 - 4;
        indices.push(start);
        indices.push(start + 1);
        indices.push(start + 2);

        indices.push(start);
        indices.push(start + 2);
        indices.push(start + 3);
        
        last = *point;
    }

    (result, indices)
}

fn normalize_vector(vector: Vector) -> Vector {
    let length = (vector.x * vector.x + vector.y * vector.y).sqrt();
    if length == 0.0 {
        Vector::new(0.0, 0.0)
    } else {
        Vector::new(vector.x / length, vector.y / length)
    }
}

fn spline(start: Vector, control: Vector, end: Vector, t: f32) -> Vector {
    let u = 1.0_f32 - t;
    let tt = t * t;
    let uu = u * u;
    let uuu = uu * u;
    let ttt = tt * t;

    let a = start * uuu;
    let b = control * uu * t * 3.0_f32;
    let c = end * u * tt * 3.0_f32;
    let d = end * ttt;

    a + b + c + d
}

fn generate_spline(start: Vector, control: Vector, end: Vector, num_segments: usize) -> Vec<Vector> {
    let mut spline_points = vec![];

    for i in 0..=num_segments {
        spline_points.push(spline(start, control, end, i as f32 / num_segments as f32));
    }

    spline_points
}