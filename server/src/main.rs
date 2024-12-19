mod db;
use db::*;
use diesel::prelude::*;
use dotenvy::dotenv;
use models::system::User;

fn main() {
    dotenv().ok();

    use db::system_schema::system::users::dsl::*;
    let mut connection = db::connection::establish_connection();
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(&mut connection)
        .expect("Error loading users");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.as_json());
        println!("----------\n");
    }
}
