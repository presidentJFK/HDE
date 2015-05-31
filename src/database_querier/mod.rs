use postgres::Connection;
use type_printer;

pub fn select_all_company_names(conn: &Connection) -> Vec<String> {
    let stmt = match conn.prepare("SELECT * FROM companies") {
        Ok(stmt) => stmt,
        Err(e) => {
            return vec![];
        }
    };

    let result = stmt.query(&[]).ok().expect("dang it");
    let mapped_result = result.iter().map(|i| {
        let name: String = i.get("name");
        name
    }).collect::<Vec<String>>();

    mapped_result
}

pub fn print_all_companies(conn: &Connection) {
    let stmt = match conn.prepare("SELECT * FROM companies") {
        Ok(stmt) => stmt,
        Err(e) => {
            return;
        }
    };

    let result = stmt.query(&[]).ok().expect("dang it");

    println!("\n\tCompanies:\n");

    // TODO: I need to handle the fancy characters being read from the DB
    for row in result {
        let id: i32 = row.get("id");
        let name: String = row.get("name");
        let year_founded: i16 = row.get("year_founded");
        println!("Name: {:?}", name);
        println!("Year Founded: {:?}", year_founded);
        println!("\n-------------------------------------\n");
    }
}

pub fn print_all_watches(conn: &Connection) {
    let stmt = match conn.prepare("SELECT * FROM watches") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("There was an Error: {:?}", e);
            return;
        }
    };

    let result = stmt.query(&[]).ok().expect("dang it");

    for row in result {
        let id: i32              = row.get("id");
        let name: String         = row.get("name");
        let reference: String = row.get("reference");
        let year: i16            = row.get("year");
        println!("Reference: {}", reference);
        println!("Name: {}", name);
        println!("Year: {}", year);
        println!("\n-------------------------------------\n");
    }

}

pub fn print_all_movements(conn: &Connection) {
    let stmt = match conn.prepare("SELECT * FROM movements") {
        Ok(stmt) => stmt,
        Err(e) => {
            println!("There was an error: {:?}", e);
            return;
        }
    };

    let result = stmt.query(&[]).ok().expect("dang it");

    for row in result {
        let id: i32 = row.get("id");
        let name: String = row.get("name");

        println!("Caliber: {:?}", name);
    }
}


fn company_count(conn: &Connection) -> i32 {
    // I selected just the id, because I know selecting all
    // the companies' columns is more expensive, because it
    // has to grab all that data, and do an extra query to find
    // out what all the columns are
    //
    // if I am too dumb to figure out to use count(*)
    // because I don't know how to annotate the type
    // of something as the Rust Postgres Library's type Int8
    //
    // so I'll and minimize the shame and pain, but not really.
    //
    // also I should benchmark, could be interesting!
    //
    // and I would like this project to have a full test and
    // benchmark suite!
    let select_all_companies = conn.prepare("SELECT id FROM companies");

    // so first what does this return?
    pp(&select_all_companies);
    // I am guessing a Result
    // core::result::Result<postgres::Statement, postgres::error::Error>

    // Awesome! so this is a result
    // so if there was an error preparing that statement what do we want to do

    // well if we prepared the statement wrong,
    // I want this method to return an -1

    // I am doing this, because it is simple, and for now,
    // it means an error was generated by me
    // that and a descriptive error message,
    // really gets me to where I need to go

    // also this is random, but match, unwraps!
    // I always forget that, and need to remember
    // other times rust is doing things like that
    // ala print! and println! dereferncing
    let stmt = match select_all_companies {
      Ok(stmt) => stmt,
      Err(e) => {
        println!("\n\tError: match select_all_companies");
        return -1;
      }
    };

    let rows_result = stmt.query(&[]);
    pp(&rows_result);
    // core::result::Result<postgres::Rows, postgres::error::Error>

    let rows = match rows_result {
      Ok(rows) => rows,
      Err(e) => {
        println!("\t\nError: match rows_result");
        return -1;
      }
    };

    let count = rows.iter().len();

    count as i32
}

fn watches_by_company(conn: &Connection, company_name: String) -> i32 {
    // So I need to find all the watches for a company

    // first lets get the company id for the company

    let find_company = conn.prepare("SELECT * FROM companies WHERE name = $1")
        .ok()
        .expect("could not prepare to select company");


    let mut rows = find_company.query(&[&company_name])
        .ok()
        .expect("could not select company");

    // if we unwrap this and its none, we want to return 0

    let company = match rows.iter().next() {
        Some(company) => company,
        None => {
            return 0;
        }
    };

    let company_id: i32 = company.get("id");

    let find_watches_for_company_query = conn
        .prepare("SELECT * FROM watches WHERE company_id = $1")
        .ok()
        .expect("could not prepare to find the watches for said company");

    let mut rows = find_watches_for_company_query.query(&[&company_id])
        .ok()
        .expect("could not find watches");

    let watch_count = rows.iter().len();
    watch_count as i32
}


#[cfg(test)]
mod tests {
    use super::company_count;
    use postgres::Connection;
    use config;
    use database_cleaner;
    use database_cleaner::clear_companies;
    use super::watches_by_company;

    // #[test]
    fn it_returns_0_when_the_db_is_cleaned() {
        let conn = before_each();

        assert_eq!(company_count(&conn), 0);
    }

    // #[test]
    fn i_can_create_a_company_and_then_assert_it_exists() {
        let conn = before_each();

        conn.execute("INSERT INTO companies (name) VALUES ('Arnold and Son')", &[])
            .ok()
            .expect("darn I though I had it");

        assert_eq!(company_count(&conn), 1);
    }

    // So now lets tdd some useful functions for quering watches!

    // so first by company

    // also I want to try and use Structs and impl to make
    // to make it all ORMy ... because you know, I'm a ruby programming
    // running to any and all comforts
    // #[test]
    fn watches_by_company_returns_the_number_watches_for_a_company() {
        let conn = before_each();

        let rolex_count = watches_by_company(&conn, "Rolex".to_string());
        assert_eq!(rolex_count, 0);
    }

    // #[test]
    fn watches_by_company_2() {
        let conn = before_each();

        conn.execute("INSERT INTO companies (name) VALUES ('Rolex')", &[])
            .ok()
            .expect("could not insert into companies");

        let rolex_count = watches_by_company(&conn, "Rolex".to_string());
        assert_eq!(rolex_count, 0);
        before_each();
    }

    #[test]
    fn watches_by_company_3() {
        let conn = before_each();

        conn.execute("INSERT INTO companies (id, name) VALUES (1, 'Rolex')", &[])
            .ok()
            .expect("could not insert into companies");

        conn.execute("INSERT INTO watches (name, company_id) VALUES ('Aquatimer', 1)", &[])
            .ok()
            .expect("could not insert into watches");

        let rolex_count = watches_by_company(&conn, "Rolex".to_string());
        assert_eq!(rolex_count, 1);
        before_each();
    }

    fn before_each() -> Connection{
        let conn = config::database_connection().unwrap();
        database_cleaner::clear_companies(&conn);
        conn
    }
}

fn fpp<T>(strang: &T) {
  println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
  type_printer::print_type_of(strang);
  println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
}

fn pp<T>(strang: &T) {
  // Can I look for an environment variable here?
  // println!("\n~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
  // type_printer::print_type_of(strang);
  // println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~\n");
}
