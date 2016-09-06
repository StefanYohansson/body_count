use connection;

pub mod itep_rn;

pub fn request() {
    let result = itep_rn::request();
    let conn = connection::db_conn();

    for dead in &result {
        let query = conn.query("SELECT register_date, deceased_date, name, gender, age, address,
        place, cause_death, city, source FROM deads
        WHERE name = $1 AND register_date = $2 AND place = $3",
        &[&dead.name, &dead.register_date, &dead.place]).unwrap();
        if query.len() > 0 {
            continue;
        }
        conn.execute("INSERT INTO deads (register_date, deceased_date, name, gender, age, address,
        place, cause_death, city) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        &[
            &dead.register_date,
            &dead.deceased_date,
            &dead.name,
            &dead.gender,
            &dead.age,
            &dead.address,
            &dead.place,
            &dead.cause_death,
            &dead.city,
            &dead.source
        ]);
    }
}
