use crate::repos::{CaptureRepo, EventRepo, PlayerRepo, PointRepo};
use uuid::Uuid;
use crate::types::stats::{PlayerStats, EventStats, PlayerPointStats, PointStat};
use crate::Errors;
use diesel::PgConnection;

pub fn get_player_stats(player_id: Uuid, conn: &PgConnection) -> Result<PlayerStats, Errors> {
    let player = PlayerRepo::get_by_id(player_id, conn)?;
    let captures = CaptureRepo::get_by_player(player_id, conn)?;
    let player_stats = captures
        .into_iter()
        .map(|capture| {
            let point = PointRepo::get_by_id(capture.point_id, conn).unwrap();
            PlayerPointStats {
                name: point.name,
                date: capture.date
            }
        })
        .collect::<Vec<PlayerPointStats>>();
        
    Ok(PlayerStats{
        name: player.name,
        point_stats: player_stats
    })
}

pub fn get_event_stats(event_id: Uuid, conn: &PgConnection) -> Result<EventStats, Errors> {
    let event = EventRepo::get_by_id(event_id, conn)?;
    let points = PointRepo::get_by_event(event_id, conn)?;
    let player_amount = PlayerRepo::get_by_event(event_id, conn)?.len() as i32;
    let point_stats = points
        .into_iter()
        .map(|point| {
            let pass_list = CaptureRepo::get_by_point(point.id, conn).unwrap();
            let name_pass_list = pass_list
                .into_iter()
                .map(|capture| {
                    PlayerRepo::get_by_id(capture.player_id, conn).unwrap().name
                })
                .collect::<Vec<String>>();
            PointStat {
                name: point.name,
                pass_list: name_pass_list
            }
        })
        .collect::<Vec<PointStat>>();

    Ok(EventStats {
        name: event.name,
        player_amount,
        point_stats
    })
}