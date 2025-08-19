use iced::advanced::layout::Limits;
use iced::advanced::widget::tree::State;
use iced::advanced::widget::Tree;
use iced::advanced::{renderer, widget, Clipboard, Layout, Shell, Widget};
use iced::{
    alignment, event, mouse, Alignment, Background, Border, Color, Element, Event, Length, Padding,
    Point, Rectangle, Shadow, Size, Vector,
};

use crate::{
    node_element::{GraphNodeElement, ScalableWidget},
    styles::node::StyleSheet,
};

type OnTranslateFn<'a, Message> = Box<dyn Fn((f32, f32)) -> Message + 'a>;

pub struct Node<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    padding: Padding,
    style: <iced::Theme as StyleSheet>::Style,
    content: Element<'a, Message, Theme, Renderer>,
    position: Point,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    on_translate: Option<OnTranslateFn<'a, Message>>,
}

struct NodeState {
    drag_start_position: Option<Point>,
}

impl<'a, Message, Theme, Renderer> Node<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new<T>(content: T) -> Self
    where
        T: Into<Element<'a, Message, Theme, Renderer>>,
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

    pub fn style(mut self, style: impl Into<<iced::Theme as StyleSheet>::Style>) -> Self {
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

pub fn node<'a, Message, Theme, Renderer>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Node<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    Node::new(content)
}

impl<'a, Message, Theme, Renderer> ScalableWidget<Message, Theme, Renderer>
    for Node<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
        scale: f32,
    ) -> iced::advanced::layout::Node {
        let limits = limits
            .loose()
            .max_width(self.max_width)
            .max_height(self.max_height)
            .width(self.width)
            .height(self.height);

        let content = self.content.as_widget().layout(
            tree.children.first_mut().unwrap(),
            renderer,
            &limits
                .shrink((self.padding.horizontal(), self.padding.vertical()))
                .loose(),
        );

        let padding = self.padding.fit(content.size(), limits.max());
        let size = limits
            .shrink((padding.horizontal(), padding.vertical()))
            .resolve(self.width, self.height, content.size());

        let size = Size::new(size.width * scale, size.height * scale);

        let content = content
            .move_to(Point::new(padding.left, padding.top))
            .align(
                Alignment::from(self.horizontal_alignment),
                Alignment::from(self.vertical_alignment),
                size,
            );

        let node = iced::advanced::layout::Node::with_children(size, vec![content]);

        node.translate(Vector::new(self.position.x, self.position.y) * scale)
    }
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Node<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
    Theme: StyleSheet<Style = <iced::Theme as StyleSheet>::Style>,
{
    fn children(&self) -> Vec<Tree> {
        vec![iced::advanced::widget::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn state(&self) -> State {
        State::new(NodeState {
            drag_start_position: None,
        })
    }

    fn tag(&self) -> widget::tree::Tag {
        widget::tree::Tag::of::<widget::tree::State>()
    }

    fn layout(
        &self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        _limits: &Limits,
    ) -> iced::advanced::layout::Node {
        todo!("This should never be called.")
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
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
                    border: Border {
                        color: style.border_color,
                        width: style.border_width,
                        radius: style.border_radius.into(),
                    },
                    shadow: Shadow {
                        color: Color::TRANSPARENT,
                        offset: Vector::ZERO,
                        blur_radius: 0.0,
                    },
                },
                style
                    .background
                    .unwrap_or(Background::Color(Color::TRANSPARENT)),
            );
        }

        self.content.as_widget().draw(
            tree.children.first().unwrap(),
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
        let state = tree.state.downcast_mut::<NodeState>();

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
                if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = event {
                    state.drag_start_position = Some(cursor_position);
                    status = event::Status::Captured;
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

    fn size(&self) -> Size<Length> {
        Size::new(self.width, self.height)
    }
}

impl<'a, Message, Theme, Renderer> From<Node<'a, Message, Theme, Renderer>>
    for GraphNodeElement<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Renderer: renderer::Renderer + 'a,
    Theme: StyleSheet<Style = <iced::Theme as StyleSheet>::Style> + 'a,
{
    fn from(node: Node<'a, Message, Theme, Renderer>) -> Self {
        Self::new(node)
    }
}
