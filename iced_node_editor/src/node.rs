use iced::{
    alignment, Alignment, Background, Color, Element, Length, Padding, Point, Size, Vector,
};
use iced_native::{renderer, Widget};

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
    max_width: u32,
    max_height: u32,
    padding: Padding,
    style: <Renderer::Theme as StyleSheet>::Style,
    content: Element<'a, Message, Renderer>,
    position: Point,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
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
            max_width: u32::MAX,
            max_height: u32::MAX,
            padding: Padding::ZERO,
            style: Default::default(),
            content: content.into(),
            position: Point::new(0.0, 0.0),
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
        }
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
        limits: &iced_native::layout::Limits,
        scale: f32,
    ) -> iced_native::layout::Node {
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

        let node = iced_native::layout::Node::with_children(size, vec![content]);

        node.translate(Vector::new(self.position.x, self.position.y) * scale)
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Node<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn children(&self) -> Vec<iced_native::widget::Tree> {
        vec![iced_native::widget::Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut iced_native::widget::Tree) {
        tree.diff_children(std::slice::from_ref(&self.content))
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        todo!("This should never be called.")
    }

    fn draw(
        &self,
        tree: &iced_native::widget::Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as iced_native::Renderer>::Theme,
        renderer_style: &renderer::Style,
        layout: iced_native::Layout<'_>,
        cursor_position: iced::Point,
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
            cursor_position,
            viewport,
        );
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
