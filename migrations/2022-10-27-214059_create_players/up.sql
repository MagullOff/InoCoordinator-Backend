CREATE TABLE players (
  id uuid NOT NULL PRIMARY KEY,
  event_id uuid NOT NULL,
  access_code INT NOT NULL,
  name VARCHAR NOT NULL,
  CONSTRAINT fk_players FOREIGN KEY(event_id) REFERENCES events(id)
);
