CREATE TABLE captures (
  id uuid NOT NULL PRIMARY KEY,
  player_id uuid NOT NULL,
  point_id uuid NOT NULL,
  date DATE NOT NULL,
  CONSTRAINT fk_captures_players FOREIGN KEY(player_id) REFERENCES players(id),
  CONSTRAINT fk_captures_points FOREIGN KEY(point_id) REFERENCES points(id)
);
