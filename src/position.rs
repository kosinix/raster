//!  A module for computing position on an image.

// crates


// from rust


// from external crate


pub struct Position<'a> {
    position: &'a str,
    offset_x: i32,
    offset_y: i32
}

impl<'a> Position<'a> {
    pub fn new(position: &'a str, offset_x: i32, offset_y: i32) -> Position {
        Position {
            position: position,
            offset_x: offset_x,
            offset_y: offset_y
        }
    }

    pub fn get_x_y(&self, canvas_width: i32, canvas_height: i32, image_width:i32, image_height:i32) -> (i32, i32) {
        let offset_x = self.offset_x;
        let offset_y = self.offset_y;

        match self.position {
            "top-left" => {
                (offset_x, offset_y)
            },
            "top-center" => {
                let x = ((canvas_width / 2) - (image_width / 2)) + offset_x;
                (x, offset_y)
            },
            "top-right" => {
                let x = (canvas_width - image_width) + offset_x;
                (x, offset_y)
            },
            "center-left" => {
                let y = ((canvas_height / 2) - (image_height / 2)) + offset_x;
                (offset_x, y)
            },
            "center-right" => {
                let x = (canvas_width - image_width) + offset_x;
                let y = ((canvas_height / 2) - (image_height / 2)) + offset_y;
                (x, y)
            },
            "bottom-left" => {
                let y = (canvas_height - image_height) + offset_y;
                (offset_x, y)
            },
            "bottom-center" => {
                let x = ((canvas_width / 2) - (image_width / 2)) + offset_x;
                let y = (canvas_height - image_height) + offset_y;
                (x, y)
            },
            "bottom-right" => {
                let x = (canvas_width - image_width) + offset_y;
                let y = (canvas_height - image_height) + offset_y;
                (x, y)
            },
            "center" => {
                let x = ((canvas_width / 2) - (image_width / 2)) + offset_x;
                let y = ((canvas_height / 2) - (image_height / 2)) + offset_y;
                (x, y)
            },
            _ => {
                panic!(format!("Invalid position {}.", self.position)) // TODO: Proper error handling
            }
        }
    }
}