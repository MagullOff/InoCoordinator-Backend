CREATE TABLE events (
  id uuid NOT NULL PRIMARY KEY,
  organizer_id uuid NOT NULL,
  name VARCHAR NOT NULL,
  CONSTRAINT fk_events FOREIGN KEY(organizer_id) REFERENCES organizers(id)
);
