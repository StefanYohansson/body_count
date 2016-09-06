use postgres;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TheDead {
    pub id: i32,
    pub register_date: String,
    pub deceased_date: String,
    pub name: String,
    pub gender: String,
    pub age: String,
    pub address: String,
    pub place: String,
    pub cause_death: String,
    pub city: String,
    pub source: String
}

impl TheDead {
    pub fn latest(conn: &postgres::Connection) -> Vec<TheDead> {
        let deads = conn.query("SELECT id, register_date, deceased_date, name, gender, age, address,
        place, cause_death, city, source FROM deads ORDER BY register_date DESC",
        &[]).unwrap().iter().fold(<Vec<TheDead>>::new(), |mut acc, row| {
            let dead = TheDead {
                id: row.get(0),
                register_date: row.get(1),
                deceased_date: row.get(2),
                name: row.get(3),
                gender: row.get(4),
                age: row.get(5),
                address: row.get(6),
                place: row.get(7),
                cause_death: row.get(8),
                city: row.get(9),
                source: row.get(10)
            };
            acc.push(dead);
            return acc;
        });
        return deads;
    }
}
