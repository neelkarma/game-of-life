use grid::Grid;
use macroquad::prelude::*;

mod grid;

#[macroquad::main("Life")]
async fn main() {
    let mut grid = Grid::random(screen_width() as usize, screen_height() as usize);
    let mut dimensions = (screen_width(), screen_height());
    loop {
        // Reset grid if window was resized
        if (screen_width(), screen_height()) != dimensions {
            grid = Grid::random(screen_width() as usize, screen_height() as usize);
            dimensions = (screen_width(), screen_height());
        }

        // Render grid
        for y in 0..grid.height {
            for x in 0..grid.width {
                if grid.get_cell(x, y) {
                    draw_rectangle(x as f32, y as f32, 1., 1., WHITE)
                } else {
                    draw_rectangle(x as f32, y as f32, 1., 1., BLACK)
                }
            }
        }

        // Update grid and await next frame
        grid.update();
        next_frame().await;
    }
}
