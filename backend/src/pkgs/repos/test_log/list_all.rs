use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;

use crate::pkgs::models::test_log::TestLog;
use crate::pkgs::schema::test_logs::dsl::*;

pub fn list_all(db: &Pool<ConnectionManager<PgConnection>>) -> Result<Vec<TestLog>, Error> {
    test_logs.load::<TestLog>(&mut db.get().unwrap())
}
