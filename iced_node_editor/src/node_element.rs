use iced_native::{renderer, Widget, layout};
use std::borrow::Borrow;

pub struct GraphNodeElement<'a, Message, Renderer> {
    widget: Box<dyn GraphWidget<'a, Message, Renderer> + 'a>,
}

pub trait GraphWidget<'a, Message, Renderer: renderer::Renderer>:
    Widget<Message, Renderer> + ScalableWidget<Message, Renderer>
{
    fn as_widget(&self) -> &(dyn Widget<Message, Renderer> + 'a);
    fn as_widget_mut(&mut self) -> &mut (dyn Widget<Message, Renderer> + 'a);
    fn as_scalable_widget(&self) -> &(dyn ScalableWidget<Message, Renderer> + 'a);
}

impl<'a, T, Message, Renderer: renderer::Renderer> GraphWidget<'a, Message, Renderer> for T
where
    T: Widget<Message, Renderer> + ScalableWidget<Message, Renderer> + 'a,
{
    fn as_widget(&self) -> &(dyn Widget<Message, Renderer> + 'a) {
        self
    }

    fn as_widget_mut(&mut self) -> &mut (dyn Widget<Message, Renderer> + 'a) {
        self
    }

    fn as_scalable_widget(&self) -> &(dyn ScalableWidget<Message, Renderer> + 'a) {
        self
    }
}

pub trait ScalableWidget<Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn layout(
        &self,
        renderer: &Renderer,
        limits: &layout::Limits,
        scale: f32
    ) -> layout::Node;
}

impl<'a, Message, Renderer> GraphNodeElement<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    pub fn new(widget: impl GraphWidget<'a, Message, Renderer> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    pub fn as_widget(&self) -> &dyn Widget<Message, Renderer> {
        self.widget.as_widget()
    }

    pub fn as_widget_mut(&mut self) -> &mut dyn Widget<Message, Renderer> {
        self.widget.as_widget_mut()
    }

    pub fn as_scalable_widget(&self) -> &dyn ScalableWidget<Message, Renderer> {
        self.widget.as_scalable_widget()
    }
}

impl<'a, Message, Renderer> Borrow<dyn Widget<Message, Renderer> + 'a>
    for GraphNodeElement<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn borrow(&self) -> &(dyn Widget<Message, Renderer> + 'a) {
        self.widget.as_widget().borrow()
    }
}

impl<'a, Message, Renderer> Borrow<dyn Widget<Message, Renderer> + 'a>
    for &GraphNodeElement<'a, Message, Renderer>
where
    Renderer: renderer::Renderer,
{
    fn borrow(&self) -> &(dyn Widget<Message, Renderer> + 'a) {
        self.widget.as_widget().borrow()
    }
}
