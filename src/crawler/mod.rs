#[derive(Debug, Clone)]
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
    pub city: String
}

pub mod itep_rn;

pub fn request() {
    let result = itep_rn::request();
}
