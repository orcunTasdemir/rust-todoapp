use sea_orm_migration::prelude::*;

#[async_std::main]
async fn main() {

    //Setting 'DATABASE_URL' environment variable
    let key = "DATABASE_URL";
    if std::env::var(key).is_err(){
        //getting database url from Rocket.toml if it is not set here
        let figment = rocket::Config::figment();
        let database_url: String = figment
        .extract_inner("database.todo.url")
        .expect("Cannot find database url in the Rocket.toml file!");
        std::env::set_var(key, database_url);
    }


    cli::run_cli(migration::Migrator).await;
}
