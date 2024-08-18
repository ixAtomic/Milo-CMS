select 
	f.id as field_id
	,f.collection
	,f.relationship_id
	,f.name
	,f.default_value
	,f.nullable
	,f.is_unique
	,ac.readonly
	,ac.required
	,ac.hidden
	,ac.note
	,r.relation_table as "relation_table?"
	,r.relation_field as "relation_field?"
	,r.delete_event as "delete_event?"
from collection c
join fields f on f.collection = c.id
left join admin_configurations ac on ac.id = f.admin_configuration_id
left join relationships r on r.id = f.relationship_id
where c.id = $1
