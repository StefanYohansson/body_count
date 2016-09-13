use postgres;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Source {
    pub id: i32,
    pub name: String
}

impl Source {
    pub fn all(conn: &postgres::Connection) -> Vec<Source> {
        let sources = conn.query("SELECT id, name FROM sources", &[]).unwrap().iter()
            .fold(<Vec<Source>>::new(), |mut acc, row| {
                let source = Source {
                    id: row.get(0),
                    name: row.get(1)
                };
                acc.push(source);
                return acc;
            });
        return sources;
    }
}
