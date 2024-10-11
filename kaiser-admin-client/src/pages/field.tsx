import { Field } from '../types/models';
import { useLoaderData } from 'react-router-dom';

import React, { useEffect, useState } from 'react';

export const FieldPage: React.FC = () => { 
    const data = useLoaderData() as { id: number };
    const [field, setField] = useState<Field | null>(null);
    console.log(data.id);
    useEffect(() => {
        const fetchField = async () => {
            const fetchedField = await getField(data.id);
            setField(fetchedField);
        };

        fetchField();
    }, [data.id]);

    return (
        <div>
            {field ? (
                <div>
                    <h1>{field.name}</h1>
                    <p>{field.collection}</p>
                </div>
            ) : (
                <p>Loading...</p>
            )}
        </div>
    );
}

const getField = async (id: number): Promise<Field | null> => {
    try 
    {
        const response = await fetch(`http://localhost:8000/fields/${id}`, {
            method: 'GET'
        });

        if(response.ok)
        {
            const field: Field = await response.json();
            return field;
        }
        else 
        {
            return null;
        }
    }
    catch (e) 
    {
        console.error(e);
    }
    return null;
}

