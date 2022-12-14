use crate::repos::{CaptureRepo, EventRepo, PlayerRepo, PointRepo};
use crate::types::stats::{EventStats, PlayerPointStats, PlayerStats, PointStat};
use crate::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub fn get_player_stats(player_id: Uuid, conn: &PgConnection) -> Result<PlayerStats, Errors> {
    let player = PlayerRepo::get_by_id(player_id, conn)?;
    let player_captures = CaptureRepo::get_by_player(player_id, conn)?;
    let points = PointRepo::get_by_event(player.event_id, conn)?;
    let player_amout = PlayerRepo::get_by_event(player.event_id, conn)?.len();
    let capture_percentage = (player_captures.len() * 100 / points.len()) as i32;
    let event = EventRepo::get_by_id(player.event_id, conn)?;

    let player_stats = points
        .into_iter()
        .map(|point| {
            let capture = player_captures.iter().find(|c| c.point_id == point.id);
            let capture_amount = CaptureRepo::get_by_point(point.id, conn)?.len();
            Ok(PlayerPointStats {
                capture_percentage: (capture_amount * 100 / player_amout) as i32,
                name: point.name.clone(),
                date: capture.map(|c| c.date),
            })
        })
        .collect::<Result<Vec<PlayerPointStats>, Errors>>()?;

    Ok(PlayerStats {
        capture_percentage,
        name: event.name,
        point_stats: player_stats,
    })
}

pub fn get_event_stats(event_id: Uuid, conn: &PgConnection) -> Result<EventStats, Errors> {
    let event = EventRepo::get_by_id(event_id, conn)?;
    let points = PointRepo::get_by_event(event_id, conn)?;
    let player_amount = PlayerRepo::get_by_event(event_id, conn)?.len() as i32;
    let point_stats = points
        .into_iter()
        .map(|point| {
            let pass_list = CaptureRepo::get_by_point(point.id, conn)?;
            let name_pass_list = pass_list
                .into_iter()
                .map(|capture| PlayerRepo::get_by_id(capture.player_id, conn).map(|c| c.name))
                .collect::<Result<Vec<String>, Errors>>()?;
            Ok(PointStat {
                name: point.name,
                pass_list: name_pass_list,
            })
        })
        .collect::<Result<Vec<PointStat>, Errors>>()?;

    Ok(EventStats {
        name: event.name,
        player_amount,
        point_stats,
    })
}
