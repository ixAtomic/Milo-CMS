import React, { useState } from 'react';
import { TextField, Checkbox, FormControlLabel, Button, Box, Typography } from '@mui/material';
import { Field, AdminConfiguration, Relationship } from '../types/models';

const initialAdminConfig: AdminConfiguration = {
  readonly: false,
  required: false,
  hidden: false,
  note: '',
};

const initialRelationship: Relationship = {
  relation_table: '',
  relation_field: '',
  delete_event: 'Cascade',
};

export const CreateField: React.FC = () => {
  const [field, setField] = useState<Field>({
    id: 0,
    collection: 0,
    name: '',
    default: '',
    nullable: false,
    is_unique: false,
    admin: initialAdminConfig,
    relation: initialRelationship,
  });

  const handleChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setField({ ...field, [name]: value });
  };

  const handleCheckboxChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, checked } = e.target;
    setField({ ...field, [name]: checked });
  };

  const handleAdminChange = (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const { name, value } = e.target;
    setField({ ...field, admin: { ...field.admin, [name]: value } });
  };

  const handleAdminCheckboxChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const { name, checked } = e.target;
    setField({ ...field, admin: { ...field.admin, [name]: checked } });
  };

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    console.log(field);
    // Add your form submission logic here
  };

  return (
    <Box component="form" onSubmit={handleSubmit} sx={{ display: 'flex', flexDirection: 'column', gap: 2 }}>
      <Typography variant="h4">Create Field</Typography>
      <TextField label="ID" name="id" type="number" value={field.id} onChange={handleChange} required />
      <TextField label="Collection" name="collection" type="number" value={field.collection} onChange={handleChange} required />
      <TextField label="Name" name="name" value={field.name} onChange={handleChange} required />
      <TextField label="Default" name="default" value={field.default} onChange={handleChange} />
      <FormControlLabel
        control={<Checkbox checked={field.nullable} onChange={handleCheckboxChange} name="nullable" />}
        label="Nullable"
      />
      <FormControlLabel
        control={<Checkbox checked={field.is_unique} onChange={handleCheckboxChange} name="is_unique" />}
        label="Unique"
      />
      <Typography variant="h6">Admin Configuration</Typography>
      <FormControlLabel
        control={<Checkbox checked={field.admin.readonly} onChange={handleAdminCheckboxChange} name="readonly" />}
        label="Read-Only"
      />
      <FormControlLabel
        control={<Checkbox checked={field.admin.required} onChange={handleAdminCheckboxChange} name="required" />}
        label="Required"
      />
      <FormControlLabel
        control={<Checkbox checked={field.admin.hidden} onChange={handleAdminCheckboxChange} name="hidden" />}
        label="Hidden"
      />
      <TextField label="Note" name="note" value={field.admin.note} onChange={handleAdminChange} />
      <Button type="submit" variant="contained" color="primary">
        Create Field
      </Button>
    </Box>
  );
};