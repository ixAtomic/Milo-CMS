select 
	f.id as field_id
	,f.collection
	,f.relationship_id
	,f.admin_configuration_id
	,f.name
	,f.default_value
	,f.nullable
	,f.is_unique
	,ac.readonly as "readonly?"
	,ac.required as "required?"
	,ac.hidden as "hidden?"
	,ac.note as "note?"
	,r.relation_table as "relation_table?"
	,r.relation_field as "relation_field?"
	,r.delete_event as "delete_event?"
from fields f
left join admin_configurations ac on ac.id = f.admin_configuration_id
left join relationships r on r.id = f.relationship_id
where f.id = $1