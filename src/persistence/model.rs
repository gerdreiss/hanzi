use diesel::prelude::*;

use crate::schema::languages;

#[derive(Associations, Selectable, Queryable)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
#[diesel(belongs_to(Language))]
pub(crate) struct Phrase {
    pub(crate) id: i32,
    pub(crate) language_id: i32,
    pub(crate) text: String,
    pub(crate) translation: String,
    pub(crate) romanization: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(super) struct NewPhrase {
    pub(super) language_id: i32,
    pub(super) text: String,
    pub(super) translation: String,
    pub(super) romanization: Option<String>,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = crate::schema::languages)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Language {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) code: String,
}

#[derive(Insertable)]
#[diesel(table_name = languages)]
pub(super) struct NewLanguage {
    pub(super) name: String,
    pub(super) code: String,
}
