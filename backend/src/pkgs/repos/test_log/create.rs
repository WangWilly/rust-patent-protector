use diesel::prelude::*;

use crate::pkgs::db_helper::{DbPool, DbResult};
use crate::pkgs::models::test_log::{NewTestLog, TestLog};
use crate::pkgs::schema::test_logs::dsl::*;

////////////////////////////////////////////////////////////////////////////////

pub fn create(db: &DbPool) -> DbResult<TestLog> {
    let record = NewTestLog { log: "test log" };

    diesel::insert_into(test_logs)
        .values(&record)
        .get_result::<TestLog>(&mut db.get().unwrap())
}

////////////////////////////////////////////////////////////////////////////////
