CREATE TABLE points (
  id uuid NOT NULL PRIMARY KEY,
  event_id uuid NOT NULL,
  code VARCHAR NOT NULL,
  name VARCHAR NOT NULL,
  CONSTRAINT fk_points FOREIGN KEY(event_id) REFERENCES events(id)
);
