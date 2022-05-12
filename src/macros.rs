#[macro_export]
macro_rules! texture {
    ($self:ident) => {
        $self
            .texture_creator
            .create_texture_target(
                sdl2::pixels::PixelFormatEnum::RGBA32,
                4 + BOARD_WIDTH as u32 * TILE_SIZE,
                4 + BOARD_HEIGHT as u32 * TILE_SIZE,
            )
            .map_err(|e| e.to_string())?
    };
}
