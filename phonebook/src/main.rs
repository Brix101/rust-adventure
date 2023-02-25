use sqlx::{
    migrate::MigrateDatabase, sqlite::SqliteQueryResult, FromRow, Pool, Row, Sqlite, SqlitePool,
};
use std::{borrow::Borrow, io};

const DB_URL: &str = "sqlite://phonebook.db";

#[derive(Clone, FromRow, Debug)]
struct Contact {
    id: i64,
    name: String,
    number: String,
}

#[tokio::main]
async fn main() {
    if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
        println!("Creating database {}", DB_URL);
        match Sqlite::create_database(DB_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(DB_URL).await.unwrap();

    let crate_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let migrations = std::path::Path::new(&crate_dir).join("./migrations");

    let migration_results = sqlx::migrate::Migrator::new(migrations)
        .await
        .unwrap()
        .run(&db)
        .await;

    match migration_results {
        Ok(_) => println!("Migration success"),
        Err(error) => {
            panic!("error: {}", error);
        }
    }

    println!("migration: {:?}", migration_results);

    let result = sqlx::query(
        "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
    )
    .fetch_all(&db)
    .await
    .unwrap();

    for (idx, row) in result.iter().enumerate() {
        println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
    }

    loop {
        println!("<>0<>0<>0<>0<> Select Command <>0<>0<>0<>0<>0<>");
        println!("== [1]Create | [2]Read | [3]Update | [4]Delete | [0]quit ==");

        let mut command_input = String::new();
        io::stdin()
            .read_line(&mut command_input)
            .expect("Failed to read line");

        let command: u32 = match command_input.trim().parse() {
            Ok(num) => num,
            Err(e) => {
                println!("{e}, Please type a number!");
                continue;
            }
        };

        match command {
            1 => {
                let result = create_contact(db.borrow().clone()).await;

                println!("Query result: {:?}", result);

                continue;
            }
            2 => {
                let contacts = get_all_contact(db.borrow().clone()).await;

                for contact in contacts {
                    println!("[{}] name: {}, number: {}",contact.id, &contact.name, &contact.number);
                }
                continue;
            }
            3 => {
                println!("Update to be emplemented");

                continue;
            }
            4 => {
                let result = delete_contact(db.borrow().clone()).await;

                println!("Delete result: {:?}", result);

                continue;
            }
            0 => break,
            _ => continue,
        }
    }
}

async fn get_all_contact(db: Pool<Sqlite>) -> Vec<Contact> {
    let contact_results = sqlx::query_as::<_, Contact>("SELECT * FROM users")
        .fetch_all(&db)
        .await
        .unwrap();

    println!("===== Read =====");
    return contact_results;
}

async fn create_contact(db: Pool<Sqlite>) -> SqliteQueryResult {

    let mut name_input = String::new();
    let mut number_input = String::new();

    println!("Name :");
    io::stdin()
        .read_line(&mut name_input)
        .expect("Failed to read line");

    println!("Number :");
    io::stdin()
        .read_line(&mut number_input)
        .expect("Failed to read line");


    let result = sqlx::query("INSERT INTO users (name, number) VALUES (?,?)")
        .bind(name_input)
        .bind(number_input)
        .execute(&db)
        .await
        .unwrap();

    return result;
}

async fn delete_contact(db: Pool<Sqlite>) -> SqliteQueryResult {
    let mut delete_input = String::new();


    println!("Id to delete: ");
    io::stdin()
        .read_line(&mut delete_input)
        .expect("Failed to read line");

    let result = sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(delete_input)
        .execute(&db)
        .await
        .unwrap();

    return result;
}

