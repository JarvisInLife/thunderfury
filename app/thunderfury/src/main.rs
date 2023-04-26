// main.rs

use entity::{prelude::*, *};
use futures::executor::block_on;
use sea_orm::{Database, DbErr, ActiveValue, EntityTrait, ActiveModelTrait};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "sqlite:test.db?mode=rwc";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let happy_bakery = bakery::ActiveModel {
        name: ActiveValue::Set("Happy Bakery".to_owned()),
        profit_margin: ActiveValue::Set(0.0),
        ..Default::default()
    };
    let res = Bakery::insert(happy_bakery).exec(&db).await?;

    let sad_bakery = bakery::ActiveModel {
        id: ActiveValue::Set(res.last_insert_id),
        name: ActiveValue::Set("Sad Bakery".to_owned()),
        profit_margin: ActiveValue::NotSet,
    };
    sad_bakery.update(&db).await?;

    let john = chef::ActiveModel {
        name: ActiveValue::Set("John".to_owned()),
        bakery_id: ActiveValue::Set(res.last_insert_id), // a foreign key
        ..Default::default()
    };
    Chef::insert(john).exec(&db).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}
