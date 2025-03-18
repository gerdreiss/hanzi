use diesel::prelude::*;

use crate::schema::languages;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Phrase {
    pub(crate) id: i32,
    pub(crate) lang_id: i32,
    pub(crate) original: String,
    pub(crate) translation: String,
    pub(crate) romanization: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(super) struct NewPhrase {
    pub(super) lang_id: i32,
    pub(super) original: String,
    pub(super) translation: String,
    pub(super) romanization: Option<String>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::languages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Phrase))]
pub(super) struct Language {
    pub(super) id: i32,
    pub(super) name: String,
    pub(super) iso: String,
}

#[derive(Insertable)]
#[diesel(table_name = languages)]
pub(super) struct NewLanguage {
    pub(super) name: String,
    pub(super) iso: String,
}
