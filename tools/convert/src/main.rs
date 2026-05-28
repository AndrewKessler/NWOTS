use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

const TILE_SIZE: i32 = 64;
const DP_EPSILON: f32 = 64.0;

// ============================================================================
// DATA TYPES
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Clone, Debug, PartialEq)]
enum EdgeKind {
    Wall(String),
    Portal(usize),
}

#[derive(Clone, Debug)]
struct Edge {
    start: Point,
    end: Point,
    kind: EdgeKind,
}

#[derive(Clone, Debug)]
struct Sector {
    id: usize,
    floor: String,
    ceiling: String,
    tiles: Vec<SectorTile>,
}

// ============================================================================
// MAIN
// ============================================================================

fn main() {

    let args: Vec<String> =
        env::args().collect();

    if args.len() != 4 {

        eprintln!(
            "Usage:\n\
             cargo run -- ascii2map source.txt map01.txt"
        );

        return;
    }

    match args[1].as_str() {

        "ascii2map" => {

            convert_ascii_to_map(
                &args[2],
                &args[3],
            );
        }

        _ => {
            eprintln!("Unknown command");
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

    let content =
        fs::read_to_string(source)
            .expect("Failed to read source");

    let (
        wall_defs,
        sector_defs,
        map_lines,
    ) = parse_ascii_file(&content);

    let (
        grid,
        spawn_x,
        spawn_y,
    ) = build_grid(&map_lines);

    let sectors =
        flood_fill_sectors(
            &grid,
            &sector_defs,
        );

    let sector_lookup =
        build_sector_lookup(&sectors);

    let mut output =
        String::new();

    for sector in &sectors {

        output.push_str(
            &format!(
                "sector sector_{}\n\n",
                sector.id
            )
        );

        output.push_str(
            &format!(
                "floor {}\n",
                sector.floor
            )
        );

        output.push_str(
            &format!(
                "ceiling {}\n\n",
                sector.ceiling
            )
        );

        let mut edges =
            extract_sector_edges(
                &grid,
                sector,
                &wall_defs,
                &sector_lookup,
            );

        normalize_edges(
            &mut edges
        );

        let contours =
            trace_contours(edges);

        let mut final_edges =
            Vec::new();

        for contour in contours {

            let simplified =
                simplify_contour_dp(
                    contour,
                    DP_EPSILON,
                );

            final_edges.extend(
                simplified
            );
        }

        normalize_edges(
            &mut final_edges
        );

        let merged =
            global_merge_collinear(
                final_edges
            );

        write_edges(
            merged,
            &mut output,
        );

        output.push('\n');
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
// ASCII PARSER
// ============================================================================

fn parse_ascii_file(
    content: &str,
)
-> (
    HashMap<char, WallDefinition>,
    HashMap<char, SectorDefinition>,
    Vec<String>,
)
{

    let mut wall_defs =
        HashMap::new();

    let mut sector_defs =
        HashMap::new();

    let mut map_lines =
        Vec::new();

    let mut in_key = false;
    let mut in_map = false;

    for raw_line in content.lines() {

        let line =
            raw_line.trim_end();

        if line.is_empty() {
            continue;
        }

        if line == "KEY" {
            in_key = true;
            continue;
        }

        if line == "MAP" {
            in_key = false;
            in_map = true;
            continue;
        }

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

        else if in_map {

            map_lines.push(
                line.to_string()
            );
        }
    }

    (
        wall_defs,
        sector_defs,
        map_lines,
    )
}

// ============================================================================
// GRID
// ============================================================================

fn build_grid(
    map_lines: &Vec<String>,
)
-> (
    Vec<Vec<char>>,
    usize,
    usize,
)
{

    let height =
        map_lines.len();

    let width =
        map_lines
            .iter()
            .map(|l| l.len())
            .max()
            .unwrap();

    let mut grid =
        vec![
            vec![' '; width];
            height
        ];

    let mut spawn_x = 0;
    let mut spawn_y = 0;

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

    (
        grid,
        spawn_x,
        spawn_y,
    )
}

// ============================================================================
// FLOOD FILL
// ============================================================================

fn flood_fill_sectors(
    grid: &Vec<Vec<char>>,
    sector_defs: &HashMap<char, SectorDefinition>,
)
-> Vec<Sector>
{

    let height =
        grid.len();

    let width =
        grid[0].len();

    let mut visited =
        vec![
            vec![false; width];
            height
        ];

    let mut sectors =
        Vec::new();

    let mut sector_id =
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

                    if nx < 0 || ny < 0 {
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

                    if grid[ny][nx] != ch {
                        continue;
                    }

                    visited[ny][nx] = true;

                    queue.push_back((nx, ny));
                }
            }

            let def =
                sector_defs
                    .get(&ch)
                    .unwrap();

            sectors.push(
                Sector {

                    id: sector_id,

                    floor:
                        def.floor.clone(),

                    ceiling:
                        def.ceiling.clone(),

                    tiles,
                }
            );

            sector_id += 1;
        }
    }

    sectors
}

// ============================================================================
// LOOKUP
// ============================================================================

fn build_sector_lookup(
    sectors: &Vec<Sector>,
)
-> HashMap<(usize, usize), usize>
{

    let mut lookup =
        HashMap::new();

    for sector in sectors {

        for tile in &sector.tiles {

            lookup.insert(
                (tile.x, tile.y),
                sector.id,
            );
        }
    }

    lookup
}

// ============================================================================
// EDGE EXTRACTION
// ============================================================================

fn extract_sector_edges(
    grid: &Vec<Vec<char>>,
    sector: &Sector,
    wall_defs: &HashMap<char, WallDefinition>,
    sector_lookup: &HashMap<(usize, usize), usize>,
)
-> Vec<Edge>
{

    let mut edges =
        Vec::new();

    let tile_set:
        HashSet<(usize, usize)>
        =
        sector.tiles
            .iter()
            .map(|t| (t.x, t.y))
            .collect();

    for tile in &sector.tiles {

        let x =
            tile.x as i32;

        let y =
            tile.y as i32;

        let wx =
            x * TILE_SIZE;

        let wy =
            y * TILE_SIZE;

        add_edge(
            &mut edges,
            grid,
            &tile_set,
            wall_defs,
            sector_lookup,
            sector.id,

            x,
            y - 1,

            Point { x: wx, y: wy },

            Point {
                x: wx + TILE_SIZE,
                y: wy,
            },
        );

        add_edge(
            &mut edges,
            grid,
            &tile_set,
            wall_defs,
            sector_lookup,
            sector.id,

            x,
            y + 1,

            Point {
                x: wx + TILE_SIZE,
                y: wy + TILE_SIZE,
            },

            Point {
                x: wx,
                y: wy + TILE_SIZE,
            },
        );

        add_edge(
            &mut edges,
            grid,
            &tile_set,
            wall_defs,
            sector_lookup,
            sector.id,

            x - 1,
            y,

            Point {
                x: wx,
                y: wy + TILE_SIZE,
            },

            Point {
                x: wx,
                y: wy,
            },
        );

        add_edge(
            &mut edges,
            grid,
            &tile_set,
            wall_defs,
            sector_lookup,
            sector.id,

            x + 1,
            y,

            Point {
                x: wx + TILE_SIZE,
                y: wy,
            },

            Point {
                x: wx + TILE_SIZE,
                y: wy + TILE_SIZE,
            },
        );
    }

    edges
}

// ============================================================================
// ADD EDGE
// ============================================================================

fn add_edge(
    edges: &mut Vec<Edge>,
    grid: &Vec<Vec<char>>,
    tile_set: &HashSet<(usize, usize)>,
    wall_defs: &HashMap<char, WallDefinition>,
    sector_lookup: &HashMap<(usize, usize), usize>,
    sector_id: usize,

    nx: i32,
    ny: i32,

    start: Point,
    end: Point,
)
{

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

    if let Some(def)
        =
        wall_defs.get(&neighbor)
    {

        edges.push(
            Edge {

                start,
                end,

                kind:
                    EdgeKind::Wall(
                        def.texture.clone()
                    ),
            }
        );
    }

    else if let Some(other_sector)
        =
        sector_lookup.get(&(nxu, nyu))
    {

        if *other_sector != sector_id {

            edges.push(
                Edge {

                    start,
                    end,

                    kind:
                        EdgeKind::Portal(
                            *other_sector
                        ),
                }
            );
        }
    }
}

// ============================================================================
// NORMALIZE
// ============================================================================

fn normalize_edges(
    edges: &mut Vec<Edge>
)
{

    for edge in edges.iter_mut() {

        if edge.start.x > edge.end.x {

            std::mem::swap(
                &mut edge.start,
                &mut edge.end,
            );
        }

        else if edge.start.x == edge.end.x {

            if edge.start.y > edge.end.y {

                std::mem::swap(
                    &mut edge.start,
                    &mut edge.end,
                );
            }
        }
    }
}

// ============================================================================
// CONTOUR WALKER
// ============================================================================

fn trace_contours(
    edges: Vec<Edge>,
)
-> Vec<Vec<Edge>>
{

    let mut adjacency:
        HashMap<Point, Vec<usize>>
        =
        HashMap::new();

    for (i, edge)
        in edges.iter().enumerate()
    {

        adjacency
            .entry(edge.start)
            .or_default()
            .push(i);

        adjacency
            .entry(edge.end)
            .or_default()
            .push(i);
    }

    let mut visited =
        vec![false; edges.len()];

    let mut contours:
        Vec<Vec<Edge>>
        =
        Vec::new();

    for start_index in 0..edges.len() {

        if visited[start_index] {
            continue;
        }

        let mut contour:
            Vec<Edge>
            =
            Vec::new();

        let mut current =
            start_index;

        loop {

            if visited[current] {
                break;
            }

            visited[current] = true;

            let mut edge =
                edges[current]
                    .clone();

            if let Some(last)
                = contour.last()
            {

                if edge.start
                    != last.end
                {

                    std::mem::swap(
                        &mut edge.start,
                        &mut edge.end,
                    );
                }
            }

            contour.push(
                edge.clone()
            );

            let next_candidates =
                adjacency
                    .get(&edge.end);

            let Some(candidates)
                = next_candidates
            else {
                break;
            };

            let mut found =
                None;

            for candidate in candidates {

                if visited[*candidate] {
                    continue;
                }

                let next =
                    &edges[*candidate];

                if next.kind
                    != edge.kind
                {
                    continue;
                }

                if next.start == edge.end
                    ||
                   next.end == edge.end
                {
                    found =
                        Some(*candidate);

                    break;
                }
            }

            let Some(next_index)
                = found
            else {
                break;
            };

            current =
                next_index;
        }

        contours.push(contour);
    }

    contours
}

// ============================================================================
// DOUGLAS PEUCKER
// ============================================================================

fn simplify_contour_dp(
    contour: Vec<Edge>,
    epsilon: f32,
)
-> Vec<Edge>
{

    if contour.len() <= 2 {
        return contour;
    }

    let kind =
        contour[0]
            .kind
            .clone();

    let mut points =
        Vec::new();

    points.push(
        contour[0]
            .start
    );

    for edge in &contour {
        points.push(edge.end);
    }

    let simplified =
        douglas_peucker(
            &points,
            epsilon,
        );

    let mut result =
        Vec::new();

    for i in 0..simplified.len() - 1 {

        result.push(
            Edge {

                start:
                    simplified[i],

                end:
                    simplified[i + 1],

                kind:
                    kind.clone(),
            }
        );
    }

    result
}

fn douglas_peucker(
    points: &[Point],
    epsilon: f32,
)
-> Vec<Point>
{

    if points.len() < 3 {
        return points.to_vec();
    }

    let first =
        points[0];

    let last =
        points[points.len() - 1];

    let mut max_distance =
        0.0;

    let mut index =
        0usize;

    for i in 1..points.len() - 1 {

        let distance =
            perpendicular_distance(
                points[i],
                first,
                last,
            );

        if distance > max_distance {

            max_distance =
                distance;

            index = i;
        }
    }

    if max_distance > epsilon {

        let mut left =
            douglas_peucker(
                &points[..=index],
                epsilon,
            );

        let right =
            douglas_peucker(
                &points[index..],
                epsilon,
            );

        left.pop();

        left.extend(right);

        left
    }

    else {

        vec![
            first,
            last,
        ]
    }
}

fn perpendicular_distance(
    point: Point,
    line_start: Point,
    line_end: Point,
)
-> f32
{

    let px =
        point.x as f32;

    let py =
        point.y as f32;

    let x1 =
        line_start.x as f32;

    let y1 =
        line_start.y as f32;

    let x2 =
        line_end.x as f32;

    let y2 =
        line_end.y as f32;

    let dx =
        x2 - x1;

    let dy =
        y2 - y1;

    if dx == 0.0
        &&
       dy == 0.0
    {
        return 0.0;
    }

    let numerator =
        (
            dy * px
            -
            dx * py
            +
            x2 * y1
            -
            y2 * x1
        )
        .abs();

    let denominator =
        (dx * dx + dy * dy)
            .sqrt();

    numerator / denominator
}

// ============================================================================
// GLOBAL COLLINEAR MERGE
// ============================================================================

fn global_merge_collinear(
    mut edges: Vec<Edge>,
)
-> Vec<Edge>
{

    let mut changed =
        true;

    while changed {

        changed = false;

        let mut used =
            vec![false; edges.len()];

        let mut result =
            Vec::new();

        for i in 0..edges.len() {

            if used[i] {
                continue;
            }

            let mut current =
                edges[i].clone();

            used[i] = true;

            loop {

                let mut merged =
                    false;

                for j in 0..edges.len() {

                    if used[j] {
                        continue;
                    }

                    let candidate =
                        &edges[j];

                    if candidate.kind
                        != current.kind
                    {
                        continue;
                    }

                    if current.end
                        != candidate.start
                    {
                        continue;
                    }

                    let dx1 =
                        current.end.x
                        -
                        current.start.x;

                    let dy1 =
                        current.end.y
                        -
                        current.start.y;

                    let dx2 =
                        candidate.end.x
                        -
                        candidate.start.x;

                    let dy2 =
                        candidate.end.y
                        -
                        candidate.start.y;

                    let cross =
                        dx1 * dy2
                        -
                        dy1 * dx2;

                    if cross != 0 {
                        continue;
                    }

                    current.end =
                        candidate.end;

                    used[j] = true;

                    changed = true;

                    merged = true;

                    break;
                }

                if !merged {
                    break;
                }
            }

            result.push(current);
        }

        edges = result;
    }

    edges
}

// ============================================================================
// WRITE
// ============================================================================

fn write_edges(
    edges: Vec<Edge>,
    output: &mut String,
)
{

    for edge in edges {

        match edge.kind {

            EdgeKind::Wall(texture) => {

                output.push_str(

                    &format!(
                        "wall {} {} {} {} {} solid\n",

                        edge.start.x,
                        edge.start.y,

                        edge.end.x,
                        edge.end.y,

                        texture,
                    )
                );
            }

            EdgeKind::Portal(sector) => {

                output.push_str(

                    &format!(
                        "portal {} {} {} {} sector_{}\n",

                        edge.start.x,
                        edge.start.y,

                        edge.end.x,
                        edge.end.y,

                        sector,
                    )
                );
            }
        }
    }
}
