INSERT INTO collection (name, singleton)
VALUES ($1, $2)
RETURNING id;