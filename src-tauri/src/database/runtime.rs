use polodb_core::{bson::doc, Collection, Database};
use std::{thread, time};

const DATABASE_FILE_NAME: &'static str = "database/primary-polo.db";

/// Starts Database
///
/// If db file doesn't exist in normal location, create one
///     else
/// open database and return
pub fn start_db(app_data_dir: &mut std::path::PathBuf) -> Database {
    assert!(app_data_dir.is_dir());
    // Tell the user that the resolved app directory isn't a directory
    std::fs::create_dir_all(&app_data_dir.join("database")).unwrap();

    let db_path = app_data_dir.join(DATABASE_FILE_NAME);

    dbg!("{:?}", db_path.as_os_str());

    if !db_path.exists() {
        println!("Generated new database");
    }

    let mut db = Database::open_file(db_path).unwrap();
    setup_indicies(&mut db);

    db
}

pub fn setup_indicies(db: &mut Database) {
    let items_inv: Collection<serde_json::Value> = db.collection("items");

    items_inv
        .create_index(polodb_core::IndexModel {
            keys: doc! {
                "model": 1
            },
            options: Some(polodb_core::IndexOptions {
                unique: Some(true),
                ..Default::default()
            }),
        })
        .unwrap();

    dbg!(items_inv.name());
}
