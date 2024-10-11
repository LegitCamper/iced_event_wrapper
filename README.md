# iced_event_wrapper
simple widget to subscribe to events like keyboard input without having to implement your own `Widget`

## example of usage (examples/simple.rs): 
#### `cargo run --example simple`
```Rust
use iced::widget::{center, text};
use iced::{keyboard::Event, Task};
use iced_widget_wrapper::wrapper;

pub fn main() -> iced::Result {
    iced::application("Simple Wrapper example", App::update, App::view)
       .run()
}

#[derive(Debug, Clone)]
enum Message {
    OnKeyboardEvent(Event),
}

struct App {
    text_field: String,
}

impl Default for App {
    fn default() -> Self {
        App {
            text_field: String::from("Type Something"),
        }
    }
}

impl App {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::OnKeyboardEvent(iced::keyboard::Event::KeyPressed {
                key: _,
                modified_key: _,
                physical_key: _,
                location: _,
                modifiers: _,
                text,
            }) => {
                self.text_field = format!("Key presssed: {:?}", text);
                Task::none()
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let text = text(self.text_field.clone());
        let wrapper = wrapper(text).on_keyboard_event(Message::OnKeyboardEvent);

        center(wrapper).into()
    }
}
```
