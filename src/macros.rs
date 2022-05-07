#[macro_export]
macro_rules! texture {
    ( $texture_canvas:expr, $draw_color:expr, $tile_size:expr ) => {{
        for x in $tile_size * 0..$tile_size {
            for y in $tile_size * 0..$tile_size {
                $texture_canvas.set_draw_color($draw_color);
                $texture_canvas
                    .draw_point(Point::new(x as i32, y as i32))
                    .expect("could not draw point");
            }
        }
    }};
}
