#![allow(
    unused_imports,
    unused_variables,
    dead_code,
    unused_mut,
    unused_must_use
    )
]

#![feature(plugin)]
#![plugin(clippy)]

extern crate postgres;
extern crate type_printer;
extern crate csv;

mod database_creator;
mod database_cleaner;
mod database_dumper;
mod database_querier;
mod database_seeder;
mod config;
mod tests;
mod control_tower;
use postgres::{Connection, Error, FromSql, SslMode};
use postgres::Result as PgResult;
use std::fmt;

fn main() {
    control_tower::title();

    let conn = match config::database_connection() {
        Some(conn) => conn,
        None => {
            println!("Looks like we got a nil connection");
            return;
        }
    };

    // control_tower::clear(&conn);
    // control_tower::drop_tables(&conn);
    // control_tower::create_tables(&conn);
    // control_tower::seed_database(&conn);
    // control_tower::copy_database(&conn);
    // control_tower::blue_angels(&conn);
    time_to_try_to_summon_the_ghost_of_oo();
}

fn time_to_try_to_summon_the_ghost_of_oo() {
    let watch1 = Watch::new("Millguass".to_owned());
    let id: i32 = watch1.save();
    println!("New Watch: {} : {}", id, watch1);
}

struct Watch {
    name: String
}

impl Watch {
     fn new(name: String) -> Watch{
         Watch {
             name: name
         }
     }

     fn save(&self) -> i32 {
         let conn = config::database_connection().unwrap();

         let stmt = conn.prepare("INSERT INTO watches (name) VALUES ($1) RETURNING id")
             .ok()
             .expect("could not prepare to insert into watches");

         let mut rows = stmt.query(&[&self.name]).ok().expect("cold not query");

         let row = rows.iter().next().unwrap();
         let id: i32 = row.get("id");
         id
     }
}

impl fmt::Debug for Watch{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl fmt::Display for Watch{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Drop for Watch {
    fn drop(&mut self) {
        // println!("Watch {} deallocated", self);
    }
}
