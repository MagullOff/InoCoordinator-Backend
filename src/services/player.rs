use crate::repos::PlayerRepo;
use crate::types::player::NewPlayer;
use crate::types::Player;
use crate::Errors;
use diesel::PgConnection;
use rand::Rng;
use uuid::Uuid;

pub fn add_player(new_player: NewPlayer, conn: &PgConnection) -> Result<Player, Errors> {
    let mut rng = rand::thread_rng();
    let player = Player {
        id: Uuid::new_v4(),
        name: new_player.name,
        event_id: new_player.event_id,
        access_code: ((rng.gen::<f64>() * 0.9 + 0.1) * 10000000.0) as i32,
    };
    PlayerRepo::insert(player, conn)
}

pub fn get_by_id(id: Uuid, conn: &PgConnection) -> Result<Player, Errors> {
    PlayerRepo::get_by_id(id, conn)
}

pub fn get_by_code(code: i32, conn: &PgConnection) -> Result<Player, Errors> {
    PlayerRepo::get_by_code(code, conn)
}

pub fn get_by_event(event_id: Uuid, conn: &PgConnection) -> Result<Vec<Player>, Errors> {
    PlayerRepo::get_by_event(event_id, conn)
}
