fn main() {
    Application::new(|cx| {
        AppData { counter: i32 }.build(cx);
        Counter::new(cx, AppData::value);

        Label::new(cx, AppData::value); // Bind the label to the count data
    })
    .title("Check")
    .inner_size((400, 100))
    .run();
}