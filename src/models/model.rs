use diesel::result::Error;
use diesel::pg::PgConnection;

pub trait Model<T> {
    fn find(db: &PgConnection, id: i32) -> Result<T, Error>;
    fn all(fb: &PgConnection) -> Result<Vec<T>, Error>;
    fn update(self: Self, db: &PgConnection) -> Result<T, Error>;
    fn delete(self: Self, db: &PgConnection) -> Result<T, Error>;
}

pub trait NewModel<T> {
    fn save(self: Self, db: &PgConnection) -> Result<T, Error>;
}
