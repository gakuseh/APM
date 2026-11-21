use slint::SharedString;

slint::include_modules!();

fn new_task_handler(name: SharedString, duration: i32, due_date: Date) -> bool {
    println!(
        "Create new task. Name: {}. Duration: {}. Due date: {}-{}-{}",
        name, duration, due_date.year, due_date.month, due_date.day
    );

    true
}
fn main() {
    let main_component = Main::new().unwrap();

    main_component.on_new_task(new_task_handler);

    main_component.run().unwrap();
}
