use ggez::{Context, GameResult};
use crate::{point, Coord, Map, Renderer};

//PUBLIC

pub fn draw_map_full(ctx: &mut Context, renderer: &mut Renderer, xy: (f32, f32), cell_size: f32, map: &Map, cols: usize, rows: usize, open_nodes: &Vec<Coord>, closed_nodes: &Vec<Coord>) -> GameResult<()> {
    draw_map_grid(ctx, renderer, xy, cell_size, cols, rows)?;
    draw_map_costs(ctx, renderer, xy, cell_size, cols, rows, map)?;
    draw_map_nodes(ctx, renderer, xy, cell_size, open_nodes, closed_nodes)?;
    draw_map_start_end(ctx, renderer, xy, cell_size, map.start, &map.targets)?;

    Ok(())
}

pub fn draw_map_with_costs(ctx: &mut Context, renderer: &mut Renderer, xy: (f32, f32), cell_size: f32, map: &Map, cols: usize, rows: usize) -> GameResult<()> {
    draw_map_grid(ctx, renderer, xy, cell_size, cols, rows)?;
    draw_map_costs(ctx, renderer, xy, cell_size, cols, rows, map)?;
    draw_map_start_end(ctx, renderer, xy, cell_size, map.start, &map.targets)?;

    Ok(())
}

pub fn draw_map_with_costs_nodes(ctx: &mut Context, renderer: &mut Renderer, xy: (f32, f32), cell_size: f32, map: &Map, cols: usize, rows: usize, open_nodes: &Vec<Coord>, closed_nodes: &Vec<Coord>) -> GameResult<()> {
    draw_map_grid(ctx, renderer, xy, cell_size, cols, rows)?;
    draw_map_costs(ctx, renderer, xy, cell_size, cols, rows, map)?;
    draw_map_nodes(ctx, renderer, xy, cell_size, open_nodes, closed_nodes)?;
    draw_map_start_end(ctx, renderer, xy, cell_size, map.start, &map.targets)?;

    Ok(())
}

pub fn draw_map_with_costs_path(ctx: &mut Context, renderer: &mut Renderer, xy: (f32, f32), cell_size: f32, map: &Map, cols: usize, rows: usize, path: &Vec<Coord>) -> GameResult<()> {
    draw_map_grid(ctx, renderer, xy, cell_size, cols, rows)?;
    draw_map_costs(ctx, renderer, xy, cell_size, cols, rows, map)?;
    draw_map_path(ctx, renderer, xy, cell_size, cols, rows, path)?;
    draw_map_start_end(ctx, renderer, xy, cell_size, map.start, &map.targets)?;

    Ok(())
}

//PRIVATE

fn draw_map_grid(ctx: &mut Context, renderer: &mut Renderer, xy: (f32, f32), cell_size: f32, cols: usize, rows: usize) -> GameResult<()> {
    let grid_mesh = renderer.make_grid_mesh(ctx, cell_size, cols, rows, 160)?;
    renderer.draw_mesh(ctx, grid_mesh.as_ref(), point(xy.0, xy.1));

    Ok(())
}

fn draw_map_costs(ctx: &mut Context, renderer: &mut Renderer, xy: (f32, f32), cell_size: f32, cols: usize, rows: usize, map: &Map) -> GameResult<()> {
    let square_mesh = renderer.make_square_mesh(ctx, cell_size, true, 2.)?;
    for map_x in 0..cols {
        for map_y in 0..rows {
            let cost = map.cost[map_x][map_y];
            if cost < 0 {
                renderer.draw_mesh(ctx, square_mesh.as_ref(), point(xy.0 + (map_x as f32 * cell_size), xy.1 + (map_y as f32 * cell_size)));
            } else if cost > 0 {
                let cost_perc = cost as f32 / 10.;
                let color = (1., 1., 1., cost_perc);
                renderer.draw_coloured_mesh(ctx, square_mesh.as_ref(), point(xy.0 + (map_x as f32 * cell_size), xy.1 + (map_y as f32 * cell_size)), color.into());
            }
        }
    }
    Ok(())
}

fn draw_map_nodes(ctx: &mut Context, renderer: &mut Renderer, xy: (f32, f32), cell_size: f32, open_nodes: &Vec<Coord>, closed_nodes: &Vec<Coord>) -> GameResult<()> {
    let square_mesh = renderer.make_square_mesh(ctx, cell_size, true, 2.)?;
    let open_color = (0.5, 0.5, 0.7, 0.8).into();
    let closed_color = (0.3, 0.3, 0.5, 0.8).into();

    for open in open_nodes {
        renderer.draw_coloured_mesh(ctx, square_mesh.as_ref(), point(xy.0 + (open.x as f32 * cell_size), xy.1 + (open.y as f32 * cell_size)), open_color);
    }
    for closed in closed_nodes {
        renderer.draw_coloured_mesh(ctx, square_mesh.as_ref(), point(xy.0 + (closed.x as f32 * cell_size), xy.1 + (closed.y as f32 * cell_size)), closed_color);
    }
    Ok(())
}

fn draw_map_start_end(ctx: &mut Context, renderer: &mut Renderer, xy: (f32, f32), cell_size: f32, start: Coord, targets: &Vec<Coord>) -> GameResult<()> {
    if (cell_size < 30.) {
        let square_mesh = renderer.make_square_mesh(ctx, cell_size, true, 2.)?;
        renderer.draw_coloured_mesh(ctx, square_mesh.as_ref(), point(xy.0 + (start.x as f32 * cell_size),xy.1 + (start.y as f32 * cell_size)), (0.5, 1., 0.5, 1.).into());
        for target in targets {
            renderer.draw_coloured_mesh(ctx, square_mesh.as_ref(), point(xy.0 + (target.x as f32 * cell_size),xy.1 + (target.y as f32 * cell_size)), (1., 0.5, 0.5, 1.).into());
        }
    } else {
        renderer.draw_text(ctx, String::from("S"), point(xy.0 + (start.x as f32 * cell_size) + 14., xy.1 + (start.y as f32 * cell_size) + 5.), (1., 0., 1., 1.).into());
        for target in targets {
            renderer.draw_text(ctx, String::from("E"), point(xy.0 + (target.x as f32 * cell_size) + 14., xy.1 + (target.y as f32 * cell_size) + 5.), (1., 0., 1., 1.).into());
        }
    }
    Ok(())
}

fn draw_map_path(ctx: &mut Context, renderer: &mut Renderer, xy: (f32, f32), cell_size: f32, cols: usize, rows: usize, path: &Vec<Coord>) -> GameResult<()> {
    let square_mesh = renderer.make_square_mesh(ctx, cell_size, true, 2.)?;
    let path_color = (0.5, 1.0, 0.5, 0.9).into();
    for step in path {
        renderer.draw_coloured_mesh(ctx, square_mesh.as_ref(), point(xy.0 + (step.x as f32 * cell_size), xy.1 + (step.y as f32 * cell_size)), path_color);
    }
    Ok(())
}