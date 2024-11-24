use diesel::prelude::*;

use crate::pkgs::db_helper::{DbPool, DbResult};
use crate::pkgs::models::assessments::*;
use crate::pkgs::schema::assessments::dsl::*;

////////////////////////////////////////////////////////////////////////////////

pub fn get_by_id(db: &DbPool, aid: i32) -> DbResult<Assessment> {
    assessments
        .find(aid)
        .first::<Assessment>(&mut db.get().unwrap())
}

////////////////////////////////////////////////////////////////////////////////
