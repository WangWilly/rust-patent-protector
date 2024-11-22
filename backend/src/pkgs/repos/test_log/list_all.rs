use diesel::prelude::*;

use crate::pkgs::db_helper::{DbPool, DbResult};
use crate::pkgs::models::test_log::TestLog;
use crate::pkgs::schema::test_logs::dsl::*;

////////////////////////////////////////////////////////////////////////////////

pub fn list_all(db: &DbPool) -> DbResult<Vec<TestLog>> {
    test_logs.load::<TestLog>(&mut db.get().unwrap())
}
