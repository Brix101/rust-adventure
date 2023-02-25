use sqlx::{migrate::MigrateDatabase, FromRow, Row, Sqlite, SqlitePool};

const DB_URL: &str = "sqlite://phonebook.db";

#[derive(Clone, FromRow, Debug)]
struct User {
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

    let result = sqlx::query("INSERT INTO users (name, number) VALUES (?,?)")
        .bind("brix")
        .bind("09514502870")
        .execute(&db)
        .await
        .unwrap();

    println!("Query result: {:?}", result);

    let user_results = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&db)
        .await
        .unwrap();

    for user in user_results {
        println!(
            "[{}] name: {}, number: {}",
            user.id, &user.name, &user.number
        );
    }

    // let delete_result = sqlx::query("DELETE FROM users WHERE name=$1")
    //     .bind("bobby")
    //     .execute(&db)
    //     .await
    //     .unwrap();

    // println!("Delete result: {:?}", delete_result);
}

