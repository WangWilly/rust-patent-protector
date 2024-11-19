use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::result::Error;

use crate::pkgs::schema::test_logs::dsl::*;
use crate::pkgs::models::test_log::{TestLog, NewTestLog};

pub fn create(db: &Pool<ConnectionManager<PgConnection>>) -> Result<TestLog, Error> {
    let record = NewTestLog {
        log: "test log",
    };

    diesel::insert_into(test_logs)
        .values(&record)
        .get_result::<TestLog>(&mut db.get().unwrap())
}
