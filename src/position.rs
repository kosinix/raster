//!  A module for computing position on an image.

// crates


// from rust


// from external crate


// from local crate
use error::RasterResult;

#[derive(Debug)]
pub enum PositionType {
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
    position: PositionType,
    offset_x: i32,
    offset_y: i32
}

impl Position {
    pub fn new(position: PositionType, offset_x: i32, offset_y: i32) -> Position {
        Position {
            position: position,
            offset_x: offset_x,
            offset_y: offset_y
        }
    }

    /// Get X and Y position based on parameters.
    pub fn get_x_y(&self, canvas_width: i32, canvas_height: i32, image_width:i32, image_height:i32) -> RasterResult<(i32, i32)> {
        let offset_x = self.offset_x;
        let offset_y = self.offset_y;

        match self.position {
            PositionType::TopLeft => {
                Ok((offset_x, offset_y))
            },
            PositionType::TopCenter => {
                let x = ((canvas_width / 2) - (image_width / 2)) + offset_x;
                Ok((x, offset_y))
            },
            PositionType::TopRight => {
                let x = (canvas_width - image_width) + offset_x;
                Ok((x, offset_y))
            },
            PositionType::CenterLeft => {
                let y = ((canvas_height / 2) - (image_height / 2)) + offset_x;
                Ok((offset_x, y))
            },
            PositionType::Center => {
                let x = ((canvas_width / 2) - (image_width / 2)) + offset_x;
                let y = ((canvas_height / 2) - (image_height / 2)) + offset_y;
                Ok((x, y))
            },
            PositionType::CenterRight => {
                let x = (canvas_width - image_width) + offset_x;
                let y = ((canvas_height / 2) - (image_height / 2)) + offset_y;
                Ok((x, y))
            },
            PositionType::BottomLeft => {
                let y = (canvas_height - image_height) + offset_y;
                Ok((offset_x, y))
            },
            PositionType::BottomCenter => {
                let x = ((canvas_width / 2) - (image_width / 2)) + offset_x;
                let y = (canvas_height - image_height) + offset_y;
                Ok((x, y))
            },
            PositionType::BottomRight => {
                let x = (canvas_width - image_width) + offset_y;
                let y = (canvas_height - image_height) + offset_y;
                Ok((x, y))
            }
        }
    }
}
