use std::sync::Mutex;

use iced::{Length, Point, Size, Vector};
use iced_graphics::triangle::{ColoredVertex2D, Mesh2D};
use iced_native::{
    renderer::{self},
    Widget,
};

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
    spline: Mutex<Vec<Vector>>,
}

impl<Message, Renderer> Connection<Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub fn new(from: Point, to: Point) -> Self {
        Connection {
            spline: Mutex::new(Vec::new()),
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
        let min_x = self.from.x.min(self.to.x);
        let max_x = self.from.x.max(self.to.x);
        let min_y = self.from.y.min(self.to.y);

        let width = (max_x - min_x).max(self.width);

        let from = Vector::new((self.from.x - min_x) * scale, (self.from.y - min_y) * scale);
        let to = Vector::new((self.to.x - min_x) * scale, (self.to.y - min_y) * scale);

        let control_scale = width.max(10.0_f32) * scale;

        let spline = generate_spline(from, control_scale, to, self.number_of_segments, 0.5_f32);

        let spline_min_x = spline
            .iter()
            .map(|p| p.x)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let spline_min_y = spline
            .iter()
            .map(|p| p.y)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let spline_max_x = spline
            .iter()
            .map(|p| p.x)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();
        let spline_max_y = spline
            .iter()
            .map(|p| p.y)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap();

        let spline = spline
            .iter()
            .map(|p| Vector::new(p.x - spline_min_x, p.y - spline_min_y))
            .collect();

        let node = iced_native::layout::Node::new(Size::new(
            (spline_max_x - spline_min_x).max(1.0),
            (spline_max_y - spline_min_y).max(1.0),
        ));

        let mut self_state = self.spline.lock().expect("Could not lock mutex");
        *self_state = spline;

        node.translate(Vector::new(min_x * scale + spline_min_x, min_y * scale + spline_min_y))
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

        let spline = self.spline.lock().unwrap();
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

fn dot_vector(vector: Vector, other: Vector) -> f32 {
    vector.x * other.x + vector.y * other.y
}

fn generate_spline(from: Vector, control_scale: f32, to: Vector, number_of_segments: usize, alpha: f32) -> Vec<Vector> {
    let mut spline = Vec::new();

    for i in 0..number_of_segments {
        let t = i as f32 / (number_of_segments - 1) as f32;
        let p = catmull_rom(
            Vector::new(from.x - control_scale, from.y),
            from,
            to,
            Vector::new(to.x + control_scale, to.y),
            t,
            alpha,
        );
        spline.push(p);
    }

    spline
}

// Code taken and adapted from https://en.wikipedia.org/wiki/Centripetal_Catmull%E2%80%93Rom_spline
fn get_t(t: f32, alpha: f32, p0: Vector, p1: Vector) -> f32 {
    let d = p1 - p0;
    let a = dot_vector(d, d);
    let b = a.powf(alpha * 0.5);
    b + t
}

fn catmull_rom(p0: Vector, p1: Vector, p2: Vector, p3: Vector, t: f32, alpha: f32) -> Vector {
    let t0 = 0.0;
    let t1 = get_t(t0, alpha, p0, p1);
    let t2 = get_t(t1, alpha, p1, p2);
    let t3 = get_t(t2, alpha, p2, p3);
    let t = t1 + (t2 - t1) * t;
    let a1 = p0 * ((t1 - t) / (t1 - t0)) + p1 *((t - t0) / (t1 - t0));
    let a2 = p1 * ((t2 - t) / (t2 - t1)) + p2 *((t - t1) / (t2 - t1));
    let a3 = p2 * ((t3 - t) / (t3 - t2)) + p3 *((t - t2) / (t3 - t2));
    let b1 = a1 * ((t2 - t) / (t2 - t0)) + a2 *((t - t0) / (t2 - t0));
    let b2 = a2 * ((t3 - t) / (t3 - t1)) + a3 *((t - t1) / (t3 - t1));
    let c = b1 * ((t2 - t) / (t2 - t1)) + b2 *((t - t1) / (t2 - t1));

    c
}