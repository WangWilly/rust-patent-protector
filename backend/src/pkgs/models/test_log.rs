use diesel::prelude::*;
use std::time::SystemTime;

use crate::pkgs::schema::test_logs;

#[derive(Queryable)]
pub struct TestLog {
    pub id: i32,
    pub log: String,
    pub created_at: Option<SystemTime>,
}

#[derive(Insertable)]
#[diesel(table_name = test_logs)]
pub struct NewTestLog<'a> {
    pub log: &'a str,
}
