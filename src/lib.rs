use iced::advanced::layout::{Layout, Limits, Node};
use iced::advanced::renderer::Style;
use iced::advanced::widget::{tree, Tree, Widget};
use iced::advanced::{Clipboard, Shell};
use iced::keyboard;
use iced::mouse::{self, Cursor};
use iced::widget::canvas::event::Status;
use iced::{Element, Event, Length, Point, Rectangle, Renderer, Size, Theme};

pub fn wrapper<'a, Message>(
    content: impl Into<Element<'a, Message, Theme, Renderer>>,
) -> Wrapper<'a, Message> {
    Wrapper::new(content)
}

#[derive(Default)]
struct WrapperState {
    bounds: Option<Size>,
}

/// Wraps widgets to allow for mouse interactions and events without having to implement them yourself
pub struct Wrapper<'a, Message> {
    content: Element<'a, Message, Theme, Renderer>,
    always_ignore_events: bool,
    on_keyboard_event: Option<Box<dyn Fn(keyboard::Event) -> Message + 'a>>,
    on_mouse_event: Option<Box<dyn Fn(mouse::Event, Point) -> Message + 'a>>,
    on_bounds_change: Option<Box<dyn Fn(Size) -> Message + 'a>>,
}

impl<'a, Message> Wrapper<'a, Message> {
    pub fn new(content: impl Into<Element<'a, Message, Theme, Renderer>>) -> Self {
        Wrapper {
            content: content.into(),
            always_ignore_events: false,
            on_keyboard_event: None,
            on_mouse_event: None,
            on_bounds_change: None,
        }
    }

    /// Allows user to set callback for keyboard events
    pub fn on_keyboard_event(
        mut self,
        on_keyboard_event: impl Fn(keyboard::Event) -> Message + 'a,
    ) -> Self {
        self.on_keyboard_event = Some(Box::new(on_keyboard_event));
        self
    }

    /// Allows user to set callback for mouse events within widget
    pub fn on_mouse_event(
        mut self,
        on_mouse_event: impl Fn(mouse::Event, Point) -> Message + 'a,
    ) -> Self {
        self.on_mouse_event = Some(Box::new(on_mouse_event));
        self
    }

    /// Allows user to set callback for when the bounds changes
    pub fn on_bounds_change(mut self, on_bounds_change: impl Fn(Size) -> Message + 'a) -> Self {
        self.on_bounds_change = Some(Box::new(on_bounds_change));
        self
    }

    /// Allows the widget to return `Status::Ignored` instead of capturing events
    pub fn always_ignore_events(mut self) -> Self {
        self.always_ignore_events = true;
        self
    }
}
impl<Message> Widget<Message, Theme, Renderer> for Wrapper<'_, Message>
where
    Renderer: iced::advanced::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<WrapperState>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(WrapperState::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.content]);
    }

    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        let state = tree.state.downcast_mut::<WrapperState>();
        state.bounds = Some(limits.max());

        // generate the child layout
        let child_layout = self
            .content
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits);

        Node::with_children(child_layout.size(), vec![child_layout])
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &Style,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) -> Status {
        let state = tree.state.downcast_mut::<WrapperState>();

        if let Some(event) = &self.on_bounds_change {
            if let Some(state_bounds) = state.bounds {
                shell.publish(event(state_bounds));
            }
        }

        match event {
            Event::Keyboard(event) => {
                if let Some(msg) = &self.on_keyboard_event {
                    shell.publish(msg(event));
                    if self.always_ignore_events {
                        Status::Ignored
                    } else {
                        Status::Captured
                    }
                } else {
                    Status::Ignored
                }
            }
            Event::Mouse(event) => {
                if let Some(msg) = &self.on_mouse_event {
                    if let Some(point) = cursor.position_in(layout.bounds()) {
                        shell.publish(msg(event, point));
                    }
                    if self.always_ignore_events {
                        Status::Ignored
                    } else {
                        Status::Captured
                    }
                } else {
                    Status::Ignored
                }
            }
            Event::Window(_event) => Status::Ignored,
            Event::Touch(_event) => Status::Ignored,
        }
    }
}

impl<'a, Message: 'a, Theme, Renderer> From<Wrapper<'a, Message>>
    for Element<'a, Message, Theme, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Wrapper<'a, Message>: Widget<Message, Theme, Renderer>,
{
    fn from(wrapper: Wrapper<'a, Message>) -> Self {
        Self::new(wrapper)
    }
}
