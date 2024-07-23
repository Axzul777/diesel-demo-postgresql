mod console;
mod handlers;
mod models;
mod schema;

fn main() {
    let mut connection = handlers::establish_conection().unwrap();

    console::console_init(&mut connection);
}
