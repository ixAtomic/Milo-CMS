use std::str::FromStr;

use rocket::shield::Feature;
use sqlx::postgres::{PgPoolOptions, PgRow};
use sqlx::Row;
use sqlx::{FromRow, Pool};

use crate::application::traits::collection_trait::CollectionTrait;
use crate::application::traits::field_trait::FieldTrait;
use crate::application::traits::queryable::Queryable;
use crate::domain::models::collection::{Collection, RCollection};
use crate::domain::models::field::{AdminConfiguration, DeleteEvent, Field, Relationship};

pub struct PostgresAccess {
    pool: Pool<sqlx::Postgres>,
}

impl Queryable for PostgresAccess {
    type ConnType = sqlx::Postgres;

    async fn new(connection: &str) -> Result<Self, sqlx::Error>
    where
        Self: Sized,
    {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(connection)
            .await?;

        Ok(Self { pool })
    }
}

impl CollectionTrait for PostgresAccess {
    async fn get_collections(&self) -> Result<Vec<Collection>, sqlx::Error> {
        sqlx::query("SELECT id, name, singleton FROM collection")
            .map(|row: PgRow| Collection {
                id: row.get("id"),
                name: row.get("name"),
                singleton: row.get("singleton"),
            })
            .fetch_all(&self.pool)
            .await
    }

    async fn get_collection_by_id(&self, _id: i32) -> Result<Collection, sqlx::Error> {
        sqlx::query_as!(
            Collection,
            "SELECT id, name, singleton FROM collection WHERE id = $1",
            _id,
        )
        .fetch_one(&self.pool)
        .await
    }

    async fn create_collection(&self, collection: RCollection) -> Result<i32, sqlx::Error> {
        let result = sqlx::query_file!(
            "queries/postgres/insert_collection.sql",
            collection.name,
            collection.singleton
        )
        .fetch_one(&self.pool)
        .await?;

        // Access the id field from the returned result
        let inserted_id = result.id;

        Ok(inserted_id)
    }
}

impl FieldTrait for PostgresAccess {
    async fn get_field(&self, _id: i32) -> Result<Field, sqlx::Error> {
        sqlx::query_file!("queries/postgres/get_fields.sql", _id)
            .map(|row| Field {
                id: row.field_id,
                collection: row.collection,
                name: row.name,
                default: row.default_value,
                nullable: row.nullable,
                is_unique: row.is_unique,
                admin: if row.admin_configuration_id.is_some() {
                    AdminConfiguration {
                        readonly: row.readonly.expect("readonly should exist here"),
                        required: row.readonly.expect("required should exist here"),
                        hidden: row.readonly.expect("hidden should exist here"),
                        note: row.note.expect("note should be a string"),
                    }
                } else {
                    //default admin configuration
                    AdminConfiguration {
                        readonly: false,
                        required: false,
                        hidden: false,
                        note: "".to_string(),
                    }
                },
                relation: if row.relationship_id.is_some() {
                    Some(Relationship {
                        relation_field: row
                            .relation_field
                            .expect("if there is a relationship_id then this should always exist"),
                        relation_table: row
                            .relation_table
                            .expect("if there is a relationship_id then this should always exist"),
                        delete_event: DeleteEvent::from_str(
                            row.delete_event
                                .expect(
                                    "if there is a relationship_id then this should always exist",
                                )
                                .as_str(),
                        )
                        .expect("valid"),
                    })
                } else {
                    None
                },
            })
            .fetch_one(&self.pool)
            .await
    }

    async fn get_fields_by_collection(
        &self,
        _collection_id: i32,
    ) -> Result<Vec<Field>, sqlx::Error> {
        sqlx::query_file!("queries/postgres/fields_by_collection.sql", _collection_id)
            .map(|row| Field {
                id: row.field_id,
                collection: row.collection,
                name: row.name,
                default: row.default_value,
                nullable: row.nullable,
                is_unique: row.is_unique,
                admin: if row.admin_configuration_id.is_some() {
                    AdminConfiguration {
                        readonly: row.readonly.expect("readonly should exist here"),
                        required: row.readonly.expect("required should exist here"),
                        hidden: row.readonly.expect("hidden should exist here"),
                        note: row.note.expect("note should be a string"),
                    }
                } else {
                    //default admin configuration
                    AdminConfiguration {
                        readonly: false,
                        required: false,
                        hidden: false,
                        note: "".to_string(),
                    }
                },
                relation: if row.relationship_id.is_some() {
                    Some(Relationship {
                        relation_field: row
                            .relation_field
                            .expect("if there is a relationship_id then this should always exist"),
                        relation_table: row
                            .relation_table
                            .expect("if there is a relationship_id then this should always exist"),
                        delete_event: DeleteEvent::from_str(
                            row.delete_event
                                .expect(
                                    "if there is a relationship_id then this should always exist",
                                )
                                .as_str(),
                        )
                        .expect("valid"),
                    })
                } else {
                    None
                },
            })
            .fetch_all(&self.pool)
            .await
    }
}
