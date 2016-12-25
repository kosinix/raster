//!  A module for computing position on an image.

// crates


// from rust


// from external crate


// from local crate
use error::RasterResult;

/// An enum for the various modes that can be used for positioning.
#[derive(Debug)]
pub enum PositionMode {
    TopLeft,
    TopCenter,
    TopRight,
    CenterLeft,
    Center,
    CenterRight,
    BottomLeft,
    BottomCenter,
    BottomRight
}

/// Struct for computing position on an image.
pub struct Position {
    position: PositionMode,
    offset_x: i32,
    offset_y: i32
}

impl Position {
    pub fn new(position: PositionMode, offset_x: i32, offset_y: i32) -> Position {
        Position {
            position: position,
            offset_x: offset_x,
            offset_y: offset_y
        }
    }

    /// Get X and Y position based on parameters.
    // Will this ever fail?
    pub fn get_x_y(&self, canvas_width: i32, canvas_height: i32, image_width:i32, image_height:i32) -> RasterResult<(i32, i32)> {
        let offset_x = self.offset_x;
        let offset_y = self.offset_y;

        Ok(match self.position {
            PositionMode::TopLeft => (offset_x, offset_y),
            PositionMode::TopCenter => {
                let x = ((canvas_width / 2) - (image_width / 2)) + offset_x;
                (x, offset_y)
            },
            PositionMode::TopRight => {
                let x = (canvas_width - image_width) + offset_x;
                (x, offset_y)
            },
            PositionMode::CenterLeft => {
                let y = ((canvas_height / 2) - (image_height / 2)) + offset_x;
                (offset_x, y)
            },
            PositionMode::Center => {
                let x = ((canvas_width / 2) - (image_width / 2)) + offset_x;
                let y = ((canvas_height / 2) - (image_height / 2)) + offset_y;
                (x, y)
            },
            PositionMode::CenterRight => {
                let x = (canvas_width - image_width) + offset_x;
                let y = ((canvas_height / 2) - (image_height / 2)) + offset_y;
                (x, y)
            },
            PositionMode::BottomLeft => {
                let y = (canvas_height - image_height) + offset_y;
                (offset_x, y)
            },
            PositionMode::BottomCenter => {
                let x = ((canvas_width / 2) - (image_width / 2)) + offset_x;
                let y = (canvas_height - image_height) + offset_y;
                (x, y)
            },
            PositionMode::BottomRight => {
                let x = (canvas_width - image_width) + offset_y;
                let y = (canvas_height - image_height) + offset_y;
                (x, y)
            }
        })
    }
}
