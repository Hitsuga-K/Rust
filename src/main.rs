fn build_greeting(user_name: &str) -> String{
    let message: String = format!("Hello user {user_name} wellcone teanfkiw");
    message
}

fn create_user_label(name: &str, role: &str) -> String{
    format!("{name} - {role}")
}
fn calculate_task_score(priority: u32, complexity: u32) -> u32 {
    priority * complexity
}

fn main() {
    let app_name: &str = "TeamFlow Desktop";
    let lesson_number: i32 = 1;
    let is_rust_installed: bool = true;

    println!("Приложение: {app_name}");
    println!("Номер урока: {lesson_number}");
    println!("Rust установлен?: {is_rust_installed}");

    // В расте по умолчанию переменные не изменяемые
    let mut counter: i32 = 0;
    counter = counter + 1;

    let name: &str = "Oleg";
    let greeting: String = build_greeting(name);
    println!("{greeting}");


    let user_name = "Labubu";
    let user_role = "Killer";
    let label = create_user_label(user_name, user_role);

    println!("User: {label}");


    let priority: u32 = 3;
    let complexity: u32 = 5;
    let score = calculate_task_score(priority, complexity);
    println!("Score: {score}");
}
