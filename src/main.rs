use leptos::*;
use rusqlite::{Connection, Result};

mod components;

use components::navbar::Navbar;

fn main() {
    // on serve create a database if not exits
        // Handle the database creation result
   // if let Err(e) = create_db() {
   //     eprintln!("Failed to create or connect to the database: {}", e);
    //    std::process::exit(1); // Optionally exit the program if the database setup fails
    //}
    mount_to_body(|| view! {
        <div>
            <Navbar /> // Include the Navbar at the top
            <p>"Hello, world!"</p>
        </div>
    });
}

/// this function creates a sqllite db
fn create_db() -> Result<()> {
    let conn = Connection::open("cats.db")?;
    // populate tables
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cat_colors (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL UNIQUE
         )",
        [], // empty array for no parameters
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cats (
             id INTEGER PRIMARY KEY,
             name TEXT NOT NULL,
             color_id INTEGER NOT NULL REFERENCES cat_colors(id)
         )",
        [], // empty array for no parameters
    )?;

    Ok(())
}