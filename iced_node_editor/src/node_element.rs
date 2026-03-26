use iced::advanced::widget::Tree;
use iced::advanced::{layout, renderer, Widget};
use std::borrow::Borrow;

pub struct GraphNodeElement<'a, Message, Theme, Renderer> {
    widget: Box<dyn GraphWidget<'a, Message, Theme, Renderer> + 'a>,
}

pub trait GraphWidget<'a, Message, Theme, Renderer: renderer::Renderer>:
    Widget<Message, Theme, Renderer> + ScalableWidget<Message, Theme, Renderer>
{
    fn as_widget(&self) -> &(dyn Widget<Message, Theme, Renderer> + 'a);
    fn as_widget_mut(&mut self) -> &mut (dyn Widget<Message, Theme, Renderer> + 'a);
    fn as_scalable_widget(&self) -> &(dyn ScalableWidget<Message, Theme, Renderer> + 'a);
    fn as_scalable_widget_mut(
        &mut self,
    ) -> &mut (dyn ScalableWidget<Message, Theme, Renderer> + 'a);
}

impl<'a, T, Message, Theme, Renderer: renderer::Renderer> GraphWidget<'a, Message, Theme, Renderer>
    for T
where
    T: Widget<Message, Theme, Renderer> + ScalableWidget<Message, Theme, Renderer> + 'a,
{
    fn as_widget(&self) -> &(dyn Widget<Message, Theme, Renderer> + 'a) {
        self
    }

    fn as_widget_mut(&mut self) -> &mut (dyn Widget<Message, Theme, Renderer> + 'a) {
        self
    }

    fn as_scalable_widget(&self) -> &(dyn ScalableWidget<Message, Theme, Renderer> + 'a) {
        self
    }

    fn as_scalable_widget_mut(
        &mut self,
    ) -> &mut (dyn ScalableWidget<Message, Theme, Renderer> + 'a) {
        self
    }
}

pub trait ScalableWidget<Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
        scale: f32,
    ) -> layout::Node;
}

impl<'a, Message, Theme, Renderer> GraphNodeElement<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new(widget: impl GraphWidget<'a, Message, Theme, Renderer> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn as_widget(&self) -> &dyn Widget<Message, Theme, Renderer> {
        self.widget.as_widget()
    }

    pub fn as_widget_mut(&mut self) -> &mut dyn Widget<Message, Theme, Renderer> {
        self.widget.as_widget_mut()
    }

    pub fn as_scalable_widget(&self) -> &dyn ScalableWidget<Message, Theme, Renderer> {
        self.widget.as_scalable_widget()
    }

    pub fn as_scalable_widget_mut(&mut self) -> &mut dyn ScalableWidget<Message, Theme, Renderer> {
        self.widget.as_scalable_widget_mut()
    }
}

impl<'a, Message, Theme, Renderer> Borrow<dyn Widget<Message, Theme, Renderer> + 'a>
    for GraphNodeElement<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn borrow(&self) -> &(dyn Widget<Message, Theme, Renderer> + 'a) {
        self.widget.as_widget()
    }
}

impl<'a, Message, Theme, Renderer> Borrow<dyn Widget<Message, Theme, Renderer> + 'a>
    for &GraphNodeElement<'a, Message, Theme, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn borrow(&self) -> &(dyn Widget<Message, Theme, Renderer> + 'a) {
        self.widget.as_widget()
    }
}
