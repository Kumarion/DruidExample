// Use druid
use druid::{Data, widget::{Label, Button, Flex}, Env, Widget, WindowDesc, AppLauncher};

// Derive Data trait
#[derive(Clone, Data)]

struct FunnyData {
    num: i32
}

// Functions to increment, decrement, multiply and subtract
fn increment(_ctx: &mut druid::EventCtx, data: &mut FunnyData, _env: &Env) {
    data.num += 1;
}

fn decrement(_ctx: &mut druid::EventCtx, data: &mut FunnyData, _env: &Env) {
    // Cant go below 0
    if data.num <= 0 {
        // return
        return;
    };

    data.num -= 1;
}

fn multiply_by_two(_ctx: &mut druid::EventCtx, data: &mut FunnyData, _env: &Env) {
    data.num *= 2;
}

fn subtract_by_two(_ctx: &mut druid::EventCtx, data: &mut FunnyData, _env: &Env) {
    // Cant go below 0
    if data.num <= 0 {
        // return
        return;
    };

    data.num -= 2;
}

fn ui_builder() -> impl Widget<FunnyData> {
    let label = Label::new(|data: &FunnyData, _: &Env| format!("Count: {}", data.num));

    let increment = Button::new("+")
        .on_click(|_ctx, data: &mut FunnyData, _env| increment(_ctx, data, _env));

    let decrement = Button::new("-")
        .on_click(|_ctx, data: &mut FunnyData, _env| decrement(_ctx, data, _env));

    let multiply_by_two = Button::new("Multiply by 2")
        .on_click(|_ctx, data: &mut FunnyData, _env| multiply_by_two(_ctx, data, _env));

    let subtract_by_two = Button::new("Subtract 2")
        .on_click(|_ctx, data: &mut FunnyData, _env| subtract_by_two(_ctx, data, _env));

    Flex::column()
    .with_child(label)
    .with_child(Flex::row()
    .with_child(increment)
    .with_child(decrement)
    .with_child(multiply_by_two)
    .with_child(subtract_by_two))
}

fn main() {
    let starter_num = 0;
    
    let main_window = WindowDesc::new(ui_builder())
        .title("First UI with Rust");

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(FunnyData { num: starter_num }).unwrap();
}