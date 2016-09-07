use connection;

pub mod itep_rn;

pub fn request() {
    let result = itep_rn::request();
    let conn = connection::db_conn();

    for dead in &result {
        // @TODO(snotr): Move this to a model filter fn
        let query = conn.query("SELECT register_date, deceased_date, name, gender, age, address,
        place, cause_death, city, source FROM deads
        WHERE name = $1 AND register_date = $2 AND place = $3",
        &[&dead.name, &dead.register_date, &dead.place]).unwrap();
        if query.len() > 0 {
            continue;
        }
        dead.insert(&conn);
    }
}
