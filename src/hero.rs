use diesel;
use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use crate::schema::heroes;

#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
#[table_name = "heroes"]
pub struct Hero {
    pub id: i32,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {
    pub fn create(hero: Hero, connection: &MysqlConnection) -> Hero {
        diesel::insert_into(heroes::table)
            .values(&hero)
            .execute(connection)
            .expect("Error creating new hero");

        heroes::table.order(heroes::id.desc()).first(connection).unwrap()
    }

    pub fn read(connection: &MysqlConnection) -> Vec<Hero> {
        heroes::table.order(heroes::id).load::<Hero>(connection).unwrap()
    }

    pub fn update(id: i32, hero: Hero, connection: &MysqlConnection) -> bool {
        diesel::update(heroes::table.find(id)).set(hero).execute(connection).is_ok()
    }

    pub fn delete(id: i32, connection: &MysqlConnection) -> bool {
        diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
    }
}
