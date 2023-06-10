use vizia::prelude::*;

#[derive(Lens)]
pub struct Counter {
    value: i32,
}

pub enum CounterEvent {
    Decrement,
    Increment,
    Set(i32),
}

pub enum CounterEventExt {
    OnUpdate(i32),
}

impl Model for Counter {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _meta| match app_event {
            CounterEvent::Decrement => {
                println!("decrement");
                self.value -= 1;
            }
            CounterEvent::Increment => {
                println!("increment");
                self.value += 1;
            }
            CounterEvent::Set(v) => {
                println!("set {}", v);
                self.value = *v;
            }
        });

        cx.emit(CounterEventExt::OnUpdate(self.value));
    }
}
impl View for Counter {}

impl Counter {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        vizia::view::View::build(Self { value: 0 }, cx, |cx| {
            HStack::new(cx, |cx| {
                Button::new(
                    cx,
                    |ex| ex.emit(CounterEvent::Decrement),
                    |cx| Label::new(cx, "Decrement"),
                );
                Button::new(
                    cx,
                    |ex| ex.emit(CounterEvent::Increment),
                    |cx| Label::new(cx, "Increment"),
                );
            })
            .child_space(Stretch(1.0))
            .col_between(Pixels(20.0));
            Label::new(cx, Counter::value);
        })
    }
}

#[derive(Lens)]
pub struct AppData {
    count: i32,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|counter_event, meta| match counter_event {
            CounterEventExt::OnUpdate(data) => self.count = *data,
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData { count: 0 }.build(cx);

        Counter::new(cx);
        Label::new(cx, AppData::count);
    })
    .title("Counter")
    .inner_size((400, 150))
    .run();
}
