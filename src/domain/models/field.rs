pub struct Field {
    id: i32,
    collection: i32,
    name: String,
    default: String,
    nullable: bool,
    is_unique: bool,
    admin: AdminConfiguration,
    relation: Relationship,
}

pub struct AdminConfiguration {
    readonly: bool, // Admin panel field is not editable
    required: bool, // if true value will be forced to be set
    hidden: bool,   // if true value will not display on the Admin panel
    note: String,   //Helpful not to display for more field information
}

pub struct Relationship {
    relation_table: String,
    relation_field: String,
    delete_event: DeleteEvent,
}

pub enum DeleteEvent {
    Cascade,
    Nullify,
    Orphan,
}
