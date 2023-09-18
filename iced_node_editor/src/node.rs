use iced::advanced::{renderer, widget, Clipboard, Layout, Shell, Widget};
use iced::{
    alignment, event, mouse, Alignment, Background, Color, Element, Event, Length, Padding, Point,
    Rectangle, Size, Vector,
};

use crate::{
    node_element::{GraphNodeElement, ScalableWidget},
    styles::node::StyleSheet,
};

pub struct Node<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    padding: Padding,
    style: <Renderer::Theme as StyleSheet>::Style,
    content: Element<'a, Message, Renderer>,
    position: Point,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    on_translate: Option<Box<dyn Fn((f32, f32)) -> Message + 'a>>,
}

struct NodeState {
    drag_start_position: Option<Point>,
}

impl<'a, Message, Renderer> Node<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message, Renderer>>,
    {
        Node {
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: f32::MAX,
            max_height: f32::MAX,
            padding: Padding::ZERO,
            style: Default::default(),
            content: content.into(),
            position: Point::new(0.0, 0.0),
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            on_translate: None,
        }
    }

    pub fn on_translate<F>(mut self, f: F) -> Self
    where
        F: 'a + Fn((f32, f32)) -> Message,
    {
        self.on_translate = Some(Box::new(f));
        self
    }

    pub fn position(mut self, position: Point) -> Self {
        self.position = position;
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

    pub fn padding<P: Into<Padding>>(mut self, padding: P) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn max_width(mut self, max_width: f32) -> Self {
        self.max_width = max_width;
        self
    }

    pub fn max_height(mut self, max_height: f32) -> Self {
        self.max_height = max_height;
        self
    }

    pub fn style(mut self, style: impl Into<<Renderer::Theme as StyleSheet>::Style>) -> Self {
        self.style = style.into();
        self
    }

    pub fn align_x(mut self, alignment: alignment::Horizontal) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    pub fn align_y(mut self, alignment: alignment::Vertical) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    pub fn center_x(mut self) -> Self {
        self.horizontal_alignment = alignment::Horizontal::Center;
        self
    }

    pub fn center_y(mut self) -> Self {
        self.vertical_alignment = alignment::Vertical::Center;
        self
    }
}

pub fn node<'a, Message, Renderer>(
    content: impl Into<Element<'a, Message, Renderer>>,
) -> Node<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    Node::new(content)
}

impl<'a, Message, Renderer> ScalableWidget<Message, Renderer> for Node<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn layout(
        &self,
        renderer: &Renderer,
        limits: &iced::advanced::layout::Limits,
        scale: f32,
    ) -> iced::advanced::layout::Node {
        let limits = limits
            .loose()
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height);

        let mut content = self
            .content
            .as_widget()
            .layout(renderer, &limits.pad(self.padding).loose());

        let padding = self.padding.fit(content.size(), limits.max());
        let size = limits.pad(padding).resolve(content.size());
        let size = Size::new(size.width * scale, size.height * scale);

        content.move_to(Point::new(padding.left.into(), padding.top.into()));
        content.align(
            Alignment::from(self.horizontal_alignment),
            Alignment::from(self.vertical_alignment),
            size,
        );

        let node = iced::advanced::layout::Node::with_children(size, vec![content]);

        node.translate(Vector::new(self.position.x, self.position.y) * scale)
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Node<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn children(&self) -> Vec<iced::advanced::widget::Tree> {
        vec![iced::advanced::widget::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut iced::advanced::widget::Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn state(&self) -> widget::tree::State {
        widget::tree::State::new(NodeState {
            drag_start_position: None,
        })
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &iced::advanced::layout::Limits,
    ) -> iced::advanced::layout::Node {
        todo!("This should never be called.")
    }

    fn draw(
        &self,
        tree: &iced::advanced::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced::advanced::Renderer>::Theme,
        renderer_style: &renderer::Style,
        layout: iced::advanced::Layout<'_>,
        cursor: iced::advanced::mouse::Cursor,
        viewport: &iced::Rectangle,
    ) {
        let style = theme.appearance(&self.style);
        let bounds = layout.bounds();

        if style.background.is_some() || style.border_width > 0.0 {
            renderer.fill_quad(
                renderer::Quad {
                    bounds,
                    border_radius: style.border_radius.into(),
                    border_width: style.border_width,
                    border_color: style.border_color,
                },
                style
                    .background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        self.content.as_widget().draw(
            tree,
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color.unwrap_or(renderer_style.text_color),
            },
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }

    fn on_event(
        &mut self,
        tree: &mut widget::Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle<f32>,
    ) -> event::Status {
        let mut status = event::Status::Ignored;
        let mut state = tree.state.downcast_mut::<NodeState>();

        if let Some(cursor_position) = cursor.position() {
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
                status = self.content.as_widget_mut().on_event(
                    &mut tree.children[0],
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            }
        }

        if let Some(cursor_position) = cursor.position() {
            if status == event::Status::Ignored && layout.bounds().contains(cursor_position) {
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                        state.drag_start_position = Some(cursor_position);
                        status = event::Status::Captured;
                    }
                    _ => {}
                }
            }
        }

        status
    }

    fn mouse_interaction(
        &self,
        tree: &widget::Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }
}

impl<'a, Message, Renderer> From<Node<'a, Message, Renderer>>
    for GraphNodeElement<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
    Renderer::Theme: StyleSheet,
{
    fn from(node: Node<'a, Message, Renderer>) -> Self {
        Self::new(node)
    }
}
