// @generated automatically by Diesel CLI.

diesel::table! {
    captures (id) {
        id -> Uuid,
        player_id -> Uuid,
        point_id -> Uuid,
        date -> Date,
    }
}

diesel::table! {
    events (id) {
        id -> Uuid,
        organizer_id -> Uuid,
        name -> Varchar,
    }
}

diesel::table! {
    organizers (id) {
        id -> Uuid,
        name -> Varchar,
        access_code -> Int4,
        email -> Varchar,
    }
}

diesel::table! {
    players (id) {
        id -> Uuid,
        event_id -> Uuid,
        name -> Varchar,
        access_code -> Int4,
    }
}

diesel::table! {
    points (id) {
        id -> Uuid,
        event_id -> Uuid,
        code -> Varchar,
        name -> Varchar,
    }
}

diesel::joinable!(captures -> players (player_id));
diesel::joinable!(captures -> points (point_id));
diesel::joinable!(events -> organizers (organizer_id));
diesel::joinable!(players -> events (event_id));
diesel::joinable!(points -> events (event_id));

diesel::allow_tables_to_appear_in_same_query!(
    captures,
    events,
    organizers,
    players,
    points,
);
