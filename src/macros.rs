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

#[macro_export]
macro_rules! set_at {
    ($self:ident, $line:expr, $col:expr, $val:expr) => {
        $self.board[$self.line_map[$line]][$col] = $val
    };
}

#[macro_export]
macro_rules! get_at {
    ($self:ident, $line:expr, $col:expr) => {
        if $line < 0 || $line >= BOARD_HEIGHT as i32 || $col < 0 || $col >= BOARD_WIDTH as i32 {
            Piece::O
        } else {
            $self.board.board[$line as usize][$col as usize]
        }
    };
}

#[macro_export]
macro_rules! get_color {
    ($piece:expr) => {
        match $piece {
            Piece::T => Some(crate::T_COLOR),
            Piece::I => Some(crate::I_COLOR),
            Piece::J => Some(crate::J_COLOR),
            Piece::L => Some(crate::L_COLOR),
            Piece::S => Some(crate::S_COLOR),
            Piece::Z => Some(crate::Z_COLOR),
            Piece::O => Some(crate::O_COLOR),
            Piece::None => None,
        }
    };
}

#[macro_export]
macro_rules! softdrop {
    ($self:ident, $game:expr, $timestamp:expr) => {
        match $self.sdf {
            100 => $game.softdrop_instant($timestamp),
            _ => $game.softdrop_start($self.sdf),
        }
    };
}

#[macro_export]
macro_rules! transmute_active {
    ($self:ident, $line:expr, $col:expr, $rot:expr) => {
        $self.set_piece_at(
            $self.piece_active,
            $self.rot_active,
            Piece::None,
            $self.line_active,
            $self.col_active,
        );
        $self.line_active = $line;
        $self.col_active = $col;
        $self.rot_active = $rot;
        $self.set_piece_at(
            $self.piece_active,
            $self.rot_active,
            $self.piece_active,
            $self.line_active,
            $self.col_active,
        );
    };
}

#[macro_export]
macro_rules! get_deltas {
    ($shape:expr, $shape_rot:expr) => {
        match $shape {
            Piece::T => match $shape_rot {
                Rotation::Spawn => ((0, -1), (0, 1), (-1, 0)),
                Rotation::Left => ((-1, 0), (1, 0), (0, -1)),
                Rotation::Right => ((-1, 0), (1, 0), (0, 1)),
                Rotation::Flip => ((0, -1), (0, 1), (1, 0)),
            },
            Piece::I => match $shape_rot {
                Rotation::Spawn => ((0, -1), (0, 1), (0, 2)),
                Rotation::Left => ((-2, 0), (-1, 0), (1, 0)),
                Rotation::Right => ((-1, 0), (1, 0), (2, 0)),
                Rotation::Flip => ((0, -2), (0, -1), (0, 1)),
            },
            Piece::J => match $shape_rot {
                Rotation::Spawn => ((0, -1), (0, 1), (-1, -1)),
                Rotation::Left => ((-1, 0), (1, 0), (1, -1)),
                Rotation::Right => ((-1, 0), (1, 0), (-1, 1)),
                Rotation::Flip => ((0, -1), (0, 1), (1, 1)),
            },
            Piece::L => match $shape_rot {
                Rotation::Spawn => ((0, -1), (0, 1), (-1, 1)),
                Rotation::Left => ((-1, 0), (1, 0), (-1, -1)),
                Rotation::Right => ((-1, 0), (1, 0), (1, 1)),
                Rotation::Flip => ((0, -1), (0, 1), (1, -1)),
            },
            Piece::S => match $shape_rot {
                Rotation::Spawn => ((-1, 0), (-1, 1), (0, -1)),
                Rotation::Left => ((-1, -1), (0, -1), (1, 0)),
                Rotation::Right => ((0, 1), (1, 1), (-1, 0)),
                Rotation::Flip => ((0, 1), (1, 0), (1, -1)),
            },
            Piece::Z => match $shape_rot {
                Rotation::Spawn => ((-1, 0), (-1, -1), (0, 1)),
                Rotation::Left => ((1, -1), (0, -1), (-1, 0)),
                Rotation::Right => ((0, 1), (-1, 1), (1, 0)),
                Rotation::Flip => ((0, -1), (1, 0), (1, 1)),
            },
            Piece::O => ((0, 1), (1, 0), (1, 1)),
            Piece::None => ((0, 0), (0, 0), (0, 0)),
        }
    };
}
