use crate::repos::{CaptureRepo, EventRepo, PlayerRepo, PointRepo};
use crate::types::stats::{EventStats, PlayerPointStats, PlayerStats};
use crate::Errors;
use diesel::PgConnection;
use uuid::Uuid;

pub fn get_player_stats(player_id: Uuid, conn: &PgConnection) -> Result<PlayerStats, Errors> {
    let player = PlayerRepo::get_by_id(player_id, conn)?;
    let player_captures = CaptureRepo::get_by_player(player_id, conn)?;
    let points = PointRepo::get_by_event(player.event_id, conn)?;
    let player_amout = PlayerRepo::get_by_event(player.event_id, conn)?.len();
    let capture_percentage = if points.len() > 0 {
        (player_captures.len() * 100 / points.len()) as i32
    } else {
        0
    };
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
    let players = PlayerRepo::get_by_event(event_id, conn)?;
    let points = PointRepo::get_by_event(event_id, conn)?;

    let player_stats = players
        .into_iter()
        .map(|p| {
            let player_captures = CaptureRepo::get_by_player(p.id, conn)?;
            Ok(if points.len()>0 {(player_captures.len() * 100 / points.len()) as i32} else {0})
        })
        .collect::<Result<Vec<i32>, Errors>>()?;

    Ok(EventStats {
        id: event_id,
        name: event.name,
        completion_amount: player_stats.iter().filter(|i| **i == 100).count() as i32,
        average_completion_amount: avg(&player_stats).unwrap_or(0),
    })
}

fn avg(vec: &Vec<i32>) -> Option<i32> {
    match vec.len() as i32 {
        0 => None,
        l => Some(vec.iter().sum::<i32>() / l),
    }
}
