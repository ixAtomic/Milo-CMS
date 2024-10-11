export interface Collection {
    id: number;
    name: string;
    singleton: boolean;
}

export interface Field {
    id: number;
    collection: number;
    name: string;
    default?: string;
    nullable: boolean;
    is_unique: boolean;
    admin: AdminConfiguration;
    relation?: Relationship;
}

export interface AdminConfiguration {
    readonly: boolean;
    required: boolean;
    hidden: boolean;
    note: string;
}

export interface Relationship {
    relation_table: string;
    relation_field: string;
    delete_event: DeleteEvent;
}

export type DeleteEvent = 'Cascade' | 'Nullify' | 'Orphan';


