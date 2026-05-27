use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

const TILE_SIZE: i32 = 64;

// ============================================================================
// DEFINITIONS
// ============================================================================

#[derive(Clone, Debug)]
struct WallDefinition {
    texture: String,
}

#[derive(Clone, Debug)]
struct SectorDefinition {
    floor: String,
    ceiling: String,
}

#[derive(Clone, Debug)]
struct SectorTile {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct WallSegment {

    x1: i32,
    y1: i32,

    x2: i32,
    y2: i32,

    texture: String,

    wall_type: String,
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {

    let args: Vec<String> =
        env::args().collect();

    if args.len() != 4 {

        eprintln!(
            "Usage:\nconvert ascii2map source.txt map01.txt"
        );

        return;
    }

    let command =
        &args[1];

    let source =
        &args[2];

    let target =
        &args[3];

    match command.as_str() {

        "ascii2map" => {

            convert_ascii_to_map(
                source,
                target,
            );
        }

        _ => {

            eprintln!(
                "Unknown command: {}",
                command,
            );
        }
    }
}

// ============================================================================
// CONVERTER
// ============================================================================

fn convert_ascii_to_map(
    source: &str,
    target: &str,
) {

    println!(
        "Converting {} -> {}",
        source,
        target,
    );

    let content =
        fs::read_to_string(source)
            .expect("Failed to read source file");

    let mut wall_defs:
        HashMap<char, WallDefinition>
        =
        HashMap::new();

    let mut sector_defs:
        HashMap<char, SectorDefinition>
        =
        HashMap::new();

    let mut map_lines =
        Vec::new();

    let mut in_key =
        false;

    let mut in_map =
        false;

    for raw_line in content.lines() {

        let line =
            raw_line.trim();

        if line.is_empty() {
            continue;
        }

        if line == "KEY" {

            in_key = true;
            in_map = false;

            continue;
        }

        if line == "MAP" {

            in_key = false;
            in_map = true;

            continue;
        }

        // ====================================================================
        // KEY
        // ====================================================================

        if in_key {

            let parts: Vec<&str> =
                line.split('=')
                    .collect();

            if parts.len() != 2 {
                continue;
            }

            let symbol =
                parts[0]
                    .trim()
                    .chars()
                    .next()
                    .unwrap();

            let rhs =
                parts[1]
                    .trim();

            let rhs_parts: Vec<&str> =
                rhs.split_whitespace()
                    .collect();

            if rhs_parts.is_empty() {
                continue;
            }

            // WALL

            if rhs_parts[0] == "wall" {

                wall_defs.insert(

                    symbol,

                    WallDefinition {

                        texture:
                            rhs_parts[1]
                                .to_string(),
                    }
                );
            }

            // SECTOR

            else if rhs_parts[0] == "sector" {

                sector_defs.insert(

                    symbol,

                    SectorDefinition {

                        floor:
                            rhs_parts[1]
                                .to_string(),

                        ceiling:
                            rhs_parts[2]
                                .to_string(),
                    }
                );
            }
        }

        // ====================================================================
        // MAP
        // ====================================================================

        else if in_map {

            map_lines.push(
                line.to_string()
            );
        }
    }

    // ========================================================================
    // GRID
    // ========================================================================

    let height =
        map_lines.len();

    let width =
        map_lines[0]
            .chars()
            .count();

    let mut grid =
        vec![
            vec![' '; width];
            height
        ];

    let mut spawn_x =
        0usize;

    let mut spawn_y =
        0usize;

    for (y, line)
        in map_lines.iter().enumerate()
    {

        for (x, ch)
            in line.chars().enumerate()
        {

            grid[y][x] = ch;

            if ch == 'P' {

                spawn_x = x;
                spawn_y = y;

                grid[y][x] = '.';
            }
        }
    }

    // ========================================================================
    // FLOOD FILL SECTORS
    // ========================================================================

    let mut visited =
        vec![
            vec![false; width];
            height
        ];

    let mut sectors =
        Vec::new();

    let mut sector_index =
        0usize;

    for y in 0..height {

        for x in 0..width {

            let ch =
                grid[y][x];

            if visited[y][x] {
                continue;
            }

            if !sector_defs.contains_key(&ch) {
                continue;
            }

            let sector_symbol =
                ch;

            let mut queue =
                VecDeque::new();

            let mut tiles =
                Vec::new();

            queue.push_back((x, y));

            visited[y][x] = true;

            while let Some((cx, cy))
                =
                queue.pop_front()
            {

                tiles.push(

                    SectorTile {
                        x: cx,
                        y: cy,
                    }
                );

                let neighbors = [

                    (cx as i32 + 1, cy as i32),
                    (cx as i32 - 1, cy as i32),

                    (cx as i32, cy as i32 + 1),
                    (cx as i32, cy as i32 - 1),
                ];

                for (nx, ny)
                    in neighbors
                {

                    if nx < 0
                        || ny < 0
                    {
                        continue;
                    }

                    let nx =
                        nx as usize;

                    let ny =
                        ny as usize;

                    if nx >= width
                        || ny >= height
                    {
                        continue;
                    }

                    if visited[ny][nx] {
                        continue;
                    }

                    if grid[ny][nx]
                        != sector_symbol
                    {
                        continue;
                    }

                    visited[ny][nx] = true;

                    queue.push_back((nx, ny));
                }
            }

            sectors.push((
                sector_index,
                sector_symbol,
                tiles,
            ));

            sector_index += 1;
        }
    }

    println!(
        "Discovered {} sectors",
        sectors.len(),
    );

    // ========================================================================
    // SECTOR LOOKUP
    // ========================================================================

    let mut sector_lookup:
        HashMap<(usize, usize), usize>
        =
        HashMap::new();

    for (
        sector_id,
        _sector_symbol,
        tiles,
    )
        in &sectors
    {

        for tile in tiles {

            sector_lookup.insert(
                (tile.x, tile.y),
                *sector_id,
            );
        }
    }

    // ========================================================================
    // OUTPUT
    // ========================================================================

    let mut output =
        String::new();

    for (
        sector_id,
        sector_symbol,
        tiles,
    )
        in &sectors
    {

        let sector_def =
            sector_defs
                .get(sector_symbol)
                .unwrap();

        output.push_str(

            &format!(
                "sector sector_{}\n\n",
                sector_id,
            )
        );

        output.push_str(

            &format!(
                "floor {}\n",
                sector_def.floor,
            )
        );

        output.push_str(

            &format!(
                "ceiling {}\n\n",
                sector_def.ceiling,
            )
        );

        let tile_set:
            HashSet<(usize, usize)>
            =
            tiles
                .iter()
                .map(|t| (t.x, t.y))
                .collect();

        let mut raw_walls:
            Vec<WallSegment>
            =
            Vec::new();

        for tile in tiles {

            let tx =
                tile.x as i32;

            let ty =
                tile.y as i32;

            let world_x =
                tx * TILE_SIZE;

            let world_y =
                ty * TILE_SIZE;

            extract_wall(
                &grid,
                &tile_set,
                &wall_defs,
                &sector_lookup,
                *sector_id,
                tx,
                ty - 1,
                world_x,
                world_y,
                world_x + TILE_SIZE,
                world_y,
                &mut raw_walls,
            );

            extract_wall(
                &grid,
                &tile_set,
                &wall_defs,
                &sector_lookup,
                *sector_id,
                tx,
                ty + 1,
                world_x + TILE_SIZE,
                world_y + TILE_SIZE,
                world_x,
                world_y + TILE_SIZE,
                &mut raw_walls,
            );

            extract_wall(
                &grid,
                &tile_set,
                &wall_defs,
                &sector_lookup,
                *sector_id,
                tx - 1,
                ty,
                world_x,
                world_y + TILE_SIZE,
                world_x,
                world_y,
                &mut raw_walls,
            );

            extract_wall(
                &grid,
                &tile_set,
                &wall_defs,
                &sector_lookup,
                *sector_id,
                tx + 1,
                ty,
                world_x + TILE_SIZE,
                world_y,
                world_x + TILE_SIZE,
                world_y + TILE_SIZE,
                &mut raw_walls,
            );
        }

        let merged_walls =
            merge_walls(raw_walls);

        for wall in merged_walls {

            output.push_str(

                &format!(
                    "{} {} {} {} {} {} {}\n",

                    wall.wall_type,

                    wall.x1,
                    wall.y1,

                    wall.x2,
                    wall.y2,

                    wall.texture,

                    if wall.wall_type == "wall" {
                        "solid"
                    }
                    else {
                        ""
                    }
                )
            );
        }

        output.push_str("\n");
    }

    output.push_str(

        &format!(
            "spawn {} {} 0\n",

            spawn_x as i32 * TILE_SIZE
                + TILE_SIZE / 2,

            spawn_y as i32 * TILE_SIZE
                + TILE_SIZE / 2,
        )
    );

    fs::write(
        target,
        output,
    )
    .expect("Failed to write map");

    println!("Conversion complete.");
}

// ============================================================================
// EXTRACT WALL
// ============================================================================

fn extract_wall(

    grid: &Vec<Vec<char>>,
    tile_set: &HashSet<(usize, usize)>,
    wall_defs: &HashMap<char, WallDefinition>,
    sector_lookup: &HashMap<(usize, usize), usize>,

    current_sector_id: usize,

    nx: i32,
    ny: i32,

    x1: i32,
    y1: i32,

    x2: i32,
    y2: i32,

    raw_walls: &mut Vec<WallSegment>,
) {

    let width =
        grid[0].len() as i32;

    let height =
        grid.len() as i32;

    if nx < 0
        || ny < 0
        || nx >= width
        || ny >= height
    {
        return;
    }

    let nxu =
        nx as usize;

    let nyu =
        ny as usize;

    if tile_set.contains(&(nxu, nyu)) {
        return;
    }

    let neighbor =
        grid[nyu][nxu];

    // ========================================================================
    // SOLID WALL
    // ========================================================================

    if let Some(def)
        =
        wall_defs.get(&neighbor)
    {

        raw_walls.push(

            WallSegment {

                x1,
                y1,

                x2,
                y2,

                texture:
                    def.texture.clone(),

                wall_type:
                    "wall".to_string(),
            }
        );

        return;
    }

    // ========================================================================
    // PORTAL
    // ========================================================================

    if let Some(other_sector_id)
        =
        sector_lookup
            .get(&(nxu, nyu))
    {

        if *other_sector_id
            != current_sector_id
        {

            raw_walls.push(

                WallSegment {

                    x1,
                    y1,

                    x2,
                    y2,

                    texture:
                        format!(
                            "sector_{}",
                            other_sector_id,
                        ),

                    wall_type:
                        "portal".to_string(),
                }
            );
        }
    }
}

// ============================================================================
// MERGE WALLS
// ============================================================================

fn merge_walls(
    mut walls: Vec<WallSegment>
)
-> Vec<WallSegment> {

    let mut changed =
        true;

    while changed {

        changed = false;

        let mut merged =
            Vec::new();

        let mut used =
            vec![false; walls.len()];

        for i in 0..walls.len() {

            if used[i] {
                continue;
            }

            let mut current =
                walls[i].clone();

            for j in (i + 1)..walls.len() {

                if used[j] {
                    continue;
                }

                let other =
                    &walls[j];

                if current.texture
                    != other.texture
                {
                    continue;
                }

                if current.wall_type
                    != other.wall_type
                {
                    continue;
                }

                // ============================================================
                // HORIZONTAL
                // ============================================================

                if current.y1 == current.y2
                    &&
                   other.y1 == other.y2
                    &&
                   current.y1 == other.y1
                {

                    if current.x2 == other.x1 {

                        current.x2 =
                            other.x2;

                        used[j] = true;

                        changed = true;
                    }
                }

                // ============================================================
                // VERTICAL
                // ============================================================

                else if current.x1 == current.x2
                    &&
                        other.x1 == other.x2
                    &&
                        current.x1 == other.x1
                {

                    if current.y2 == other.y1 {

                        current.y2 =
                            other.y2;

                        used[j] = true;

                        changed = true;
                    }
                }
            }

            merged.push(current);
        }

        walls = merged;
    }

    walls
}