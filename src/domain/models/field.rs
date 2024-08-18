use rocket::serde::Serialize;
use sqlx::postgres::PgRow;
use sqlx::{FromRow, Row};
use std::str::FromStr;

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Field {
    pub id: i32,
    pub collection: i32,
    pub name: String,
    pub default: Option<String>,
    pub nullable: bool,
    pub is_unique: bool,
    pub admin: AdminConfiguration,
    pub relation: Option<Relationship>,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct AdminConfiguration {
    pub readonly: bool, // Admin panel field is not editable
    pub required: bool, // if true value will be forced to be set
    pub hidden: bool,   // if true value will not display on the Admin panel
    pub note: String,   //Helpful not to display for more field information
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Relationship {
    pub relation_table: String,
    pub relation_field: String,
    pub delete_event: DeleteEvent,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum DeleteEvent {
    Cascade,
    Nullify,
    Orphan,
}

impl FromStr for DeleteEvent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "cascade" => Ok(DeleteEvent::Cascade),
            "nullify" => Ok(DeleteEvent::Nullify),
            "orphan" => Ok(DeleteEvent::Orphan),
            _ => Err(()),
        }
    }
}

impl<'r> FromRow<'r, PgRow> for DeleteEvent {
    fn from_row(row: &'r PgRow) -> Result<Self, sqlx::Error> {
        let value: &str = row.get("delete_event");
        match value.to_lowercase().as_str() {
            "cascade" => Ok(DeleteEvent::Cascade),
            "nullify" => Ok(DeleteEvent::Nullify),
            "orphan" => Ok(DeleteEvent::Orphan),
            _ => Err(sqlx::Error::ColumnDecode {
                index: "DeleteEvent".to_owned(),
                source: Box::new(sqlx::Error::Decode(
                    format!("Unknown variant: {}", value).into(),
                )),
            }),
        }
    }
}
