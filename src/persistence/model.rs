use diesel::prelude::*;

use crate::persistence::schema;

#[derive(Selectable, Queryable)]
#[diesel(table_name = schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Phrase {
    pub(crate) id: i32,
    pub(crate) original: String,
    pub(crate) pinyin: String,
    pub(crate) translation: String,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = schema::phrases)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(super) struct NewPhrase {
    pub(super) original: String,
    pub(crate) pinyin: String,
    pub(super) translation: String,
}

#[derive(Selectable, Queryable)]
#[diesel(table_name = schema::settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(crate) struct Setting {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) value: String,
}

#[derive(AsChangeset, Insertable)]
#[diesel(table_name = schema::settings)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub(super) struct NewSetting {
    pub(crate) name: String,
    pub(crate) value: String,
}
