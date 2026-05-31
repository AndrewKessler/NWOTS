use glam::Vec2;

use crate::world::{
    Sector,
    Wall,
};

pub fn raycast_wall(
    origin: Vec2,
    dir: Vec2,
    wall: &Wall,
) -> Option<(f32, Vec2)> {

    let x1 = wall.start.x;
    let y1 = wall.start.y;

    let x2 = wall.end.x;
    let y2 = wall.end.y;

    let x3 = origin.x;
    let y3 = origin.y;

    let x4 = origin.x + dir.x;
    let y4 = origin.y + dir.y;

    let denominator =
        (x1 - x2) * (y3 - y4)
            -
        (y1 - y2) * (x3 - x4);

    if denominator.abs() < 0.0001 {
        return None;
    }

    let t =
        ((x1 - x3) * (y3 - y4)
            -
        (y1 - y3) * (x3 - x4))
            / denominator;

    let u =
        -(
            (x1 - x2) * (y1 - y3)
                -
            (y1 - y2) * (x1 - x3)
        )
            / denominator;

    if t >= 0.0
        &&
        t <= 1.0
        &&
        u > 0.0
    {

        let hit_point =
            origin + dir * u;

        let distance =
            (hit_point - origin)
                .length();

        Some((
            distance,
            hit_point,
        ))
    }

    else {

        None
    }
}

pub fn point_in_sector(
    point: Vec2,
    sector: &Sector,
) -> bool {

    let mut inside =
        false;

    let walls =
        &sector.walls;

    let count =
        walls.len();

    for i in 0..count {

        let a =
            walls[i].start;

        let b =
            walls[i].end;

        let intersect =
            ((a.y > point.y)
                !=
             (b.y > point.y))
            &&
            (
                point.x
                    <
                (b.x - a.x)
                    * (point.y - a.y)
                    / ((b.y - a.y) + 0.0001)
                    + a.x
            );

        if intersect {
            inside = !inside;
        }
    }

    inside
}