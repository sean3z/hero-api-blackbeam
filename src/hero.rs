use rocket::{State};
use mysql;

#[derive(Serialize, Deserialize, Debug)]
pub struct Hero {
    pub id: Option<i32>,
    pub name: String,
    pub identity: String,
    pub hometown: String,
    pub age: i32
}

impl Hero {
    pub fn create(hero: Hero, pool: State<mysql::Pool>) -> Hero {
        let params = params!{
            "name" => &hero.name,
            "identity" => &hero.identity,
            "hometown" => &hero.hometown,
            "age" => &hero.age,
        };

        let insert = "INSERT INTO heroes (name, identity, hometown, age) 
            VALUES (:name, :identity, :hometown, :age)";

        pool.prep_exec(insert, params);

        pool.prep_exec("SELECT id, name, identity, hometown, age FROM heroes ORDER BY id DESC LIMIT 1", ())            
        .map(|mut result| {
            let row = result.next();

            let (id, name, identity, hometown, age) = mysql::from_row(row.unwrap().unwrap());

            Hero {
                id: id,
                name: name,
                identity: identity,
                hometown: hometown,
                age: age
            }
        }).unwrap()
    }
}