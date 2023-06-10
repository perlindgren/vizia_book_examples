use vizia::prelude::*;

#[derive(Lens)]
pub struct AppData {
    count: i32,
}

pub enum AppEvent {
    Increment,
    Decrement,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, meta| match app_event {
            AppEvent::Decrement => self.count -= 1,
            AppEvent::Increment => self.count += 1,
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { count: 0 }.build(cx);

        HStack::new(cx, |cx| {
            Button::new(
                cx,
                |ex| ex.emit(AppEvent::Decrement),
                |cx| Label::new(cx, "Decrement"),
            );
            Button::new(
                cx,
                |ex| ex.emit(AppEvent::Increment),
                |cx| Label::new(cx, "Increment"),
            );
            // Bind the label to the count data
            Label::new(cx, AppData::count);
        })
        .child_space(Stretch(1.0))
        .col_between(Pixels(20.0));
    })
    .title("Counter")
    .inner_size((400, 100))
    .run();
}
