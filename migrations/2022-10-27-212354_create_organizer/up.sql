CREATE TABLE organizers (
  id uuid NOT NULL PRIMARY KEY,
  name VARCHAR NOT NULL,
  access_code INT NOT NULL,
  email VARCHAR NOT NULL
);
