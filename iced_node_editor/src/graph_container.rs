use iced::{
    event, mouse, Background, Color, Element, Event, Length, Point, Rectangle, Size, Vector,
};
use iced_native::{
    layout,
    renderer::{self},
    widget::{self, Operation},
    Clipboard, Layout, Shell, Widget,
};

use crate::{
    matrix::Matrix,
    styles::graph_container::{Appearance, StyleSheet},
    GraphNodeElement,
};

pub struct GraphContainer<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    width: Length,
    height: Length,
    max_width: u32,
    max_height: u32,
    style: <Renderer::Theme as StyleSheet>::Style,
    content: Vec<GraphNodeElement<'a, Message, Renderer>>,
    matrix: Matrix,
    on_translate: Option<Box<dyn Fn((f32, f32)) -> Message + 'a>>,
    on_scale: Option<Box<dyn Fn(f32, f32, f32) -> Message + 'a>>,
}

struct GraphContainerState {
    drag_start_position: Option<Point>,
}

impl<'a, Message, Renderer> GraphContainer<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub fn new(content: Vec<GraphNodeElement<'a, Message, Renderer>>) -> Self
    {
        GraphContainer {
            on_translate: None,
            on_scale: None,
            matrix: Matrix::identity(),
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: u32::MAX,
            max_height: u32::MAX,
            style: Default::default(),
            content,
        }
    }

    pub fn on_translate<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn((f32, f32)) -> Message,
    {
        self.on_translate = Some(Box::new(f));
        self
    }

    pub fn on_scale<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn(f32, f32, f32) -> Message,
    {
        self.on_scale = Some(Box::new(f));
        self
    }

    pub fn matrix(mut self, m: Matrix) -> Self {
        self.matrix = m;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }

    pub fn max_width(mut self, max_width: u32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn max_height(mut self, max_height: u32) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }
}

pub fn graph_container<'a, Message, Renderer>(
    content: Vec<GraphNodeElement<'a, Message, Renderer>>,
) -> GraphContainer<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    GraphContainer::new(content)
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for GraphContainer<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn children(&self) -> Vec<widget::Tree> {
        let mut children = Vec::new();

        for node in &self.content {
            children.push(widget::Tree::new(node));
        }

        children
    }

    fn diff(&self, tree: &mut widget::Tree) {
        tree.diff_children(self.content.as_slice())
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(GraphContainerState {
            drag_start_position: None,
        })
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits
            .loose()
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height);

        let mut content = Vec::new();

        let scale = self.matrix.get_scale();
        let offset = self.matrix.get_translation();

        for node in &self.content {
            let mut node = node.as_scalable_widget().layout(_renderer, &limits, scale);
            node = node.translate(Vector::new(offset.0, offset.1));

            content.push(node);
        }

        let size = limits.resolve(Size::ZERO);

        layout::Node::with_children(size, content)
    }

    fn operate(
        &self,
        tree: &mut widget::Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, &mut |operation| {
            self.content
                .iter()
                .zip(&mut tree.children)
                .zip(layout.children())
                .for_each(|((child, state), layout)| {
                    child
                        .as_widget()
                        .operate(state, layout, renderer, operation);
                })
        });
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let mut status = event::Status::Ignored;
        let mut state = tree.state.downcast_mut::<GraphContainerState>();

        if let Some(start) = state.drag_start_position {
            match event {
                Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)) => {
                    state.drag_start_position = None;
                }
                Event::Mouse(mouse::Event::CursorMoved { .. }) => {
                    let delta = cursor_position - start;
                    state.drag_start_position = Some(cursor_position);
                    if let Some(f) = &self.on_translate {
                        let message = f((delta.x, delta.y));
                        shell.publish(message);
                    }
                    status = event::Status::Captured;
                }
                _ => {}
            }
        } else {
            status = self
                .content
                .iter_mut()
                .zip(&mut tree.children)
                .zip(layout.children())
                .map(|((child, state), layout)| {
                    child.as_widget_mut().on_event(
                        state,
                        event.clone(),
                        layout,
                        cursor_position,
                        renderer,
                        clipboard,
                        shell,
                    )
                })
                .fold(event::Status::Ignored, event::Status::merge);
        }

        if status == event::Status::Ignored {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    state.drag_start_position = Some(cursor_position);
                }
                Event::Mouse(mouse::Event::WheelScrolled { delta }) => {
                    if let Some(f) = &self.on_scale {
                        match delta {
                            mouse::ScrollDelta::Lines { y, .. } => {
                                let message = f(cursor_position.x, cursor_position.y, y);
                                shell.publish(message);
                            }
                            mouse::ScrollDelta::Pixels { y, .. } => {
                                let message = f(cursor_position.x, cursor_position.y, y);
                                shell.publish(message);
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        status
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget().mouse_interaction(
                    state,
                    layout,
                    cursor_position,
                    viewport,
                    renderer,
                )
            })
            .max()
            .unwrap_or_default()
    }

    fn draw(
        &self,
        state: &widget::Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) {
        let style = theme.appearance(&self.style);

        let bounds = layout.bounds();

        renderer.with_layer(bounds, |renderer| {
            draw_background(renderer, bounds, style);

            let offset = self.matrix.get_translation();
            let scale = self.matrix.get_scale();
            let normalized_scale = normalize_scale(scale);

            let biggest_spacing = style
                .minor_guidelines_spacing
                .unwrap()
                .max(style.major_guidelines_spacing.unwrap())
                .max(style.mid_guidelines_spacing.unwrap());

            draw_guidelines(
                renderer,
                bounds,
                offset,
                normalized_scale,
                style.minor_guidelines_spacing.unwrap(),
                biggest_spacing,
                style.minor_guidelines_color.unwrap(),
            );

            draw_guidelines(
                renderer,
                bounds,
                offset,
                normalized_scale,
                style.mid_guidelines_spacing.unwrap(),
                biggest_spacing,
                style.mid_guidelines_color.unwrap(),
            );

            draw_guidelines(
                renderer,
                bounds,
                offset,
                normalized_scale,
                style.major_guidelines_spacing.unwrap(),
                biggest_spacing,
                style.major_guidelines_color.unwrap(),
            );

            let mut children_layout = layout.children();
            for i in 0..self.content.len() {
                let layout = children_layout.next().unwrap();
                let node = self.content[i].as_widget();

                let child_bounds = layout.bounds();
                let intersect = child_bounds.intersection(&bounds);

                if intersect.is_none() {
                    continue;
                }

                let intersect = intersect.unwrap();

                if intersect.width < 1.0 || intersect.height < 1.0 {
                    continue;
                }

                node.draw(
                    &state.children[i],
                    renderer,
                    theme,
                    renderer_style,
                    layout,
                    cursor_position,
                    &viewport,
                );
            }
        });
    }
}

impl<'a, Message, Renderer> From<GraphContainer<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(graph_container: GraphContainer<'a, Message, Renderer>) -> Self {
        Self::new(graph_container)
    }
}

fn draw_background<'a, Renderer>(renderer: &mut Renderer, bounds: Rectangle, style: Appearance)
where
    Renderer: renderer::Renderer,
{
    renderer.fill_quad(
        renderer::Quad {
            bounds,
            border_radius: [0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32].into(),
            border_width: 0.0_f32,
            border_color: Color::BLACK,
        },
        style
            .background
            .unwrap_or(Background::Color(Color::from_rgb8(44, 44, 44))),
    );
}

fn draw_guidelines<'a, Renderer>(
    renderer: &mut Renderer,
    bounds: Rectangle,
    offset: (f32, f32),
    scale: f32,
    grid_spacing: f32,
    biggest_grid_spacing: f32,
    color: Color,
) where
    Renderer: renderer::Renderer,
{
    if grid_spacing * scale < 5.0_f32 {
        return;
    }

    let edge = biggest_grid_spacing * scale;

    let offset_x = offset.0 % edge;
    let offset_y = offset.1 % edge;

    let from_x = -edge + offset_x + bounds.x;
    let to_x = bounds.x + bounds.width + edge;
    let step = grid_spacing * scale;
    let number_of_steps = ((to_x - from_x) / step).abs().ceil() as usize;

    for x in 0..number_of_steps {
        let x = from_x + (x as f32 * step);

        if x <= bounds.x || x >= bounds.x + bounds.width {
            continue;
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x,
                    y: bounds.y,
                    width: 1.0_f32,
                    height: bounds.height,
                },
                border_radius: [0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32].into(),
                border_width: 0.0_f32,
                border_color: Color::BLACK,
            },
            Background::Color(color),
        );
    }

    let from_y = -edge + offset_y + bounds.y;
    let to_y = bounds.y + bounds.height + edge;
    let step = grid_spacing * scale;
    let number_of_steps = ((to_y - from_y) / step).abs().ceil() as usize;

    for y in 0..number_of_steps {
        let y = from_y + (y as f32 * step);

        if y <= bounds.y || y >= bounds.y + bounds.height {
            continue;
        }

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y,
                    width: bounds.width,
                    height: 1.0_f32,
                },
                border_radius: [0.0_f32, 0.0_f32, 0.0_f32, 0.0_f32].into(),
                border_width: 0.0_f32,
                border_color: Color::BLACK,
            },
            Background::Color(color),
        );
    }
}

fn normalize_scale(scale: f32) -> f32 {
    let log_2 = scale.log2().floor();

    if log_2.abs() > f32::EPSILON {
        return scale / 2.0_f32.powf(log_2);
    } else {
        return scale;
    }
}
