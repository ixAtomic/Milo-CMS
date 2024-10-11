import { Collection, Field } from "../types/models";
import React, { useEffect, useState } from 'react';
import { Box, Button, Container, List, ListItemButton, ListItemText, Typography } from '@mui/material';
import Grid from '@mui/material/Grid2';
import { DataGrid, GridColDef, GridRowParams } from '@mui/x-data-grid';
import { useNavigate } from 'react-router-dom';

export const Collections: React.FC = () => {
  const [collections, setCollections] = useState<Collection[]>([]);
  const [selectedCollection, setSelectedCollection] = useState<Collection | null>(null);
  const [fields, setFields] = useState<Field[]>([]);
  const navigate = useNavigate();
  
  useEffect(() => {
    getAllCollections().then((collections) => {
      setSelectedCollection(collections[0]);
      setCollections(collections);
    });
  }, []);

  useEffect(() => {
    if(selectedCollection)
    {
      getCollectionFields(selectedCollection.id).then((fields) => setFields(fields));
    }
  }, [selectedCollection]);

  const columns: GridColDef[] = [
    { field: 'id', headerName: 'ID', width: 150 },
    { field: 'collection', headerName: 'Collection', width: 150 },
    { field: 'name', headerName: 'Name', width: 300 },
    { field: 'default', headerName: 'Default', width: 200 },
    { field: 'nullable', headerName: 'Nullable', width: 150, type: 'boolean' },
    { field: 'is_unique', headerName: 'Unique', width: 150, type: 'boolean' },
    { field: 'admin.readonly', headerName: 'Read-Only', width: 150, type: 'boolean' },
    { field: 'admin.required', headerName: 'Required', width: 150, type: 'boolean' },
    { field: 'admin.hidden', headerName: 'Hidden', width: 150, type: 'boolean' },
    { field: 'admin.note', headerName: 'Note', width: 300 },
    { field: 'relation.relation_table', headerName: 'Relation Table', width: 200 },
    { field: 'relation.relation_field', headerName: 'Relation Field', width: 200 },
    { field: 'relation.delete_event', headerName: 'Delete Event', width: 150 },
  ];

  const directToFieldPage = (params: GridRowParams<Field>) => { 
    navigate(`/fields/${params.row.id}`);
  }

  return (
    <Container maxWidth={false} sx={{padding: 0, height: '100%'}}>
      <Grid sx={{height: '100%'}} container spacing={8}>
        <Grid size={3}>
          <Typography variant='h3'>Collections</Typography>
          <List>
            {collections.map((collection) => (
              <ListItemButton onClick={() => setSelectedCollection(collection)} key={collection.id}>
                <ListItemText primary={collection.name} />
              </ListItemButton>
            ))}
          </List>
        </Grid>
        <Grid sx={{ display: 'flex', flexDirection: 'column' }} size={8}>
          <Typography sx={{mb: 2}} variant='h3'>{selectedCollection?.name}</Typography>
          <Box sx={{display: 'flex'}}>
            <Button onClick={() => navigate('/fields/create')} sx={{ width: '12rem', height: '2rem' }} variant="contained" color="success">Add Field</Button>
          </Box>
          <DataGrid onRowClick={(params) => directToFieldPage(params)} sx={{ border: 0 }} rows={fields} columns={columns} />
        </Grid>
      </Grid>
    </Container>
  );
}

const getCollectionFields = async (collectionId: number): Promise<Field[]> => {
  try{
    const response = await fetch(`http://localhost:8000/collections/${collectionId}/fields`, {
      method: 'GET'
    }); 

    if(response.ok)
    {
      const fields: Field[] = await response.json();
      return fields;
    }
    else {
      return [];
    }
  }
  catch{
    return [];
  }
}

const getAllCollections = async (): Promise<Collection[]> => {
    try{
      const response = await fetch(`http://localhost:8000/collections`, {
        method: 'GET'
      }); 

      if(response.ok)
      {
        const collections: Collection[] = await response.json();
        return collections;
      }
      else {
        return [];
      }
    }
    catch{
      return [];
    }
}