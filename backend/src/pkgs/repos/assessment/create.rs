use diesel::prelude::*;

use crate::pkgs::db_helper::{DbPool, DbResult};
use crate::pkgs::models::assessments::*;
use crate::pkgs::schema::assessments::dsl::*;

////////////////////////////////////////////////////////////////////////////////

pub fn create(db: &DbPool, record: &NewAssessment) -> DbResult<Assessment> {
    diesel::insert_into(assessments)
        .values(record)
        .get_result::<Assessment>(&mut db.get().unwrap())
}

////////////////////////////////////////////////////////////////////////////////
