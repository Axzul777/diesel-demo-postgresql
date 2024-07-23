use crate::handlers::{create_new_user, delete_user, get_users};
use diesel::pg::PgConnection;
use std::io;

pub fn console_init(cnn_cmd: &mut PgConnection) {
    let mut commands = String::new();

    loop {
        commands.clear();

        println!("+");

        io::stdin().read_line(&mut commands).expect("Reading Error");

        println!("Updated: {:?}", commands.trim());

        let parts: Vec<&str> = commands.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "exit" => {
                break;
            }
            "help" => {
                println!("list, delete <username>, new <username>");
            }
            "new" => match parts.get(1) {
                Some(name) => {
                    //println!("Adding new user...");
                    create_new_user(cnn_cmd, name);
                    //println!("done");
                }
                None => println!("Unexpect! No args founds"),
            },
            "delete" => match parts.get(1) {
                Some(name) => {
                    //println!("Deleting user ...");
                    delete_user(cnn_cmd, name);
                    //print!("done");
                }
                None => println!("Unexpect! No args founds"),
            },
            "list" => get_users(cnn_cmd),
            &_ => println!("Unexpect! Unknow command"),
        }
    }
}
