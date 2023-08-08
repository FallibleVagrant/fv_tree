use std::fmt;

use common::Point;

pub struct Canvas {
    canvas: Vec<Vec<char>>,
    //Change where the origin is located within the vec,
    //for the sake of simplicity, cannot be negative,
    //and only adjusts for negative values, since we have a lot of positive space anyways.
    x_offset: usize,
    y_offset: usize,
}

impl Canvas {
    pub fn new() -> Canvas {
        let canvas: Vec<Vec<char>> = Vec::new();

        return Canvas {canvas, x_offset: 0, y_offset: 0};
    }

    //The point is translated into the vec's index by the canvas offset.
    //Say it is negative, the offset will translate it to positive or zero.
    //Useful for accessing where it will actually be placed in the vec.
    //Keep in mind this is only for one axis.
    //
    //Also might want to run is_point_within_offset before using this. Doesn't work well when points are
    //still negative after offset.
    fn calc_vec_index(point: i32, offset: usize) -> usize {
        let i: usize;
        if point.is_positive() {
            i = (point as usize) + offset;
        }
        else {
            i = offset - (point.abs() as usize);
        }

        return i;
    }

    fn resize_x_axis_if_needed(&mut self, target_point: Point) {
        let y_index: usize = Canvas::calc_vec_index(target_point.y, self.y_offset);
        let row: &mut Vec<char> = &mut self.canvas[y_index];
        let target_x = target_point.x;

        //Check if point has overshot past row.len().
        if target_x >= 0 && (target_x as usize) + self.x_offset + 1 > row.len() {
            row.resize((target_x as usize) + self.x_offset + 1, ' ');
        }

        //Check if point has undershot below row[0], into the negative indices.
        //If so, adjust the offset, and insert blanks into non-empty rows, or the row to be
        //inserted into.
        if target_x.is_negative() && (target_x.abs() as usize) > self.x_offset {
            let additional_offset: usize = (target_x.abs() as usize) - self.x_offset;
            //Modify the offset.
            self.x_offset += additional_offset;

            for (i, mut row) in self.canvas.iter_mut().enumerate() {
                if !row.is_empty() || i == y_index {
                    row.reserve(additional_offset);
                    //self.canvas[additional_offset..].clone_from_slice(&self.canvas[..additional_offset]);

                    for _j in 0..additional_offset {
                        //self.canvas[_j] = Vec::new();

                        //TODO: revise for efficiency.
                            row.insert(0, ' ');
                    }
                }
            }
        }

        //Borrow again.
        let row: &mut Vec<char> = &mut self.canvas[y_index];

        //Check if point HASN'T undershot row[0] into the negative indices, but still needs space
        //allocated within the row for it.
        if target_x.is_negative() && self.x_offset - (target_x.abs() as usize) + 1 > row.len() {
            row.resize(self.x_offset - (target_x.abs() as usize) + 1, ' ');
        }
    }

    fn resize_y_axis_if_needed(&mut self, target_point: Point) {
        let col: &mut Vec<Vec<char>> = &mut self.canvas;
        //Don't actually need target_point.x, include the entire Point struct for consistency.
        let target_y = target_point.y;

        //Check if point has overshot past col.len().
        if target_y >= 0 && (target_y as usize) + self.y_offset + 1 > col.len() {
            col.resize((target_y as usize) + self.y_offset + 1, Vec::new());
        }

        //Check if point has undershot below col[0], into the negative indices.
        //If so, adjust the offset and insert blank rows.
        if target_y.is_negative() && (target_y.abs() as usize) > self.y_offset {
            let additional_offset: usize = (target_y.abs() as usize) - self.y_offset;
            //Modify the offset.
            self.y_offset += additional_offset;

            col.reserve(additional_offset);
            //self.canvas[additional_offset..].clone_from_slice(&self.canvas[..additional_offset]);

            for _i in 0..additional_offset {
                //self.canvas[_i] = Vec::new();

                //TODO: revise for efficiency.
                col.insert(0, Vec::new());
            }
        }

        //Check if point HASN'T undershot col[0] into the negative indices, but still needs space
        //allocated within the col for it.
        //UNNECESSARY, col will always have a Vec::new() in it.
        /*if target_y.is_negative() && self.y_offset - (target_y.abs() as usize) + 1 > col.len() {
            col.resize(self.y_offset - (target_y.abs() as usize) + 1, Vec::new());
        }*/
    }

    ///Put characters into the canvas.
    ///#Panics
    ///Panics if a point contains i32::MIN
    pub fn put(&mut self, point: Point, c: char) {
        self.resize_y_axis_if_needed(point);

        let y_index: usize = Canvas::calc_vec_index(point.y, self.y_offset);
        
        self.resize_x_axis_if_needed(point);

        let x_index: usize = Canvas::calc_vec_index(point.x, self.x_offset);
        let row: &mut Vec<char> = &mut self.canvas[y_index];

        row[x_index] = c;
    }

    //Check if point >= 0 when offset is added,
    //i.e., point values aren't negative for calc_vec_index.
    fn is_point_within_offset(&self, point: Point) -> bool {
        return ((point.x.is_negative() && point.x.abs() as usize > self.x_offset)
                && (point.y.is_negative() && point.y.abs() as usize > self.y_offset))

                || (point.x >= 0 && point.y >= 0);
    }

    ///Check if a point in the canvas is blank.
    ///Returns true if the point is off-canvas.
    pub fn is_blank_point(&self, point: Point) -> bool {
        //Check we don't get negative values for calc_vec_index.
        if !self.is_point_within_offset(point) {
            return false;
        }

        let x_index: usize = Canvas::calc_vec_index(point.x, self.x_offset);
        let y_index: usize = Canvas::calc_vec_index(point.y, self.y_offset);

        let row: &Vec<char>;
        match self.canvas.get(y_index) {
            Some(r) => row = r,
            None => return true,
        }

        let c: char;
        match row.get(x_index) {
            Some(r) => c = *r,
            None => return true,
        }

        return c == ' ';
    }

    ///Check if a point in the canvas is the specified char.
    pub fn is_char_point(&self, point: Point, c: char) -> bool {
        //Check we don't get negative values for calc_vec_index.
        if !self.is_point_within_offset(point) {
            return false;
        }

        let x_index: usize = Canvas::calc_vec_index(point.x, self.x_offset);
        let y_index: usize = Canvas::calc_vec_index(point.y, self.y_offset);

        let row: &Vec<char>;
        match self.canvas.get(y_index) {
            Some(r) => row = r,
            None => return false,
        }

        let canvas_c: char;
        match row.get(x_index) {
            Some(r) => canvas_c = *r,
            None => return false,
        }

        return canvas_c == c;
    }
}

impl fmt::Display for Canvas {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for row in self.canvas.iter().rev() {
            let row: String = row.iter().collect();
            output.push_str(&(row + "\n"));
        }

        write!(f, "{}", output)
    }
}

struct LayeredCanvas {
    layers: Vec<Canvas>,
}

impl LayeredCanvas {
    pub fn into_canvas() -> Canvas {
        //TODO: placeholder.
        Canvas::new()
    }
}

#[cfg(test)]
mod text_canvas_tests {
    use super::*;

    #[test]
    fn print_nothing() {
        let canvas = Canvas::new();
        let output = format!("{}", canvas);
        assert_eq!("", output);
    }

    #[test]
    fn print_onething() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: 0, y: 0}, 'x');
        let output = format!("{}", canvas);
        assert_eq!("x\n", output);
    }

    #[test]
    fn print_positive() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: 1, y: 1}, 'x');
        let output = format!("{}", canvas);
        assert_eq!(" x\n\n", output);
    }

    #[test]
    fn print_negative() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: -1, y: 0}, 'x');
        let output = format!("{}", canvas);
        assert_eq!("x\n", output);

        let mut canvas = Canvas::new();
        canvas.put(Point {x: 0, y: -1}, 'x');
        let output = format!("{}", canvas);
        assert_eq!("x\n", output);
    }

    #[test]
    fn print_twothing() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: 0, y: 0}, 'x');
        canvas.put(Point {x: 2, y: 0}, 'x');
        let output = format!("{}", canvas);
        assert_eq!("x x\n", output);
    }

    #[test]
    fn print_negative_then_positive_x() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: -1, y: 0}, 'x');
        canvas.put(Point {x: 1, y: 0}, 'x');
        let output = format!("{}", canvas);
        assert_eq!("x x\n", output);
    }

    #[test]
    fn print_positive_then_negative_x() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: 1, y: 0}, 'x');
        canvas.put(Point {x: -1, y: 0}, 'x');
        let output = format!("{}", canvas);
        assert_eq!("x x\n", output);
    }

    #[test]
    fn print_negative_then_positive_y() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: 0, y: -1}, 'x');
        canvas.put(Point {x: 0, y: 1}, 'x');
        let output = format!("{}", canvas);
        assert_eq!("x\n\nx\n", output);
    }

    #[test]
    fn print_positive_then_negative_y() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: 0, y: 1}, 'x');
        canvas.put(Point {x: 0, y: -1}, 'x');
        let output = format!("{}", canvas);
        assert_eq!("x\n\nx\n", output);
    }

    #[test]
    fn print_something() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: -1, y: 0}, '-');
        canvas.put(Point {x: 1, y: 0}, '-');
        canvas.put(Point {x: 0, y: 0}, '_');
        canvas.put(Point {x: 1, y: 1}, '^');
        canvas.put(Point {x: -1, y: 1}, '^');
        canvas.put(Point {x: 0, y: -1}, 'v');
        let output = format!("{}", canvas);
        assert_eq!("^ ^\n-_-\n v\n", output);
    }

    #[test]
    fn up_then_left() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: 0, y: 0}, '|');
        canvas.put(Point {x: -1, y: 1}, '.');
        let output = format!("{}", canvas);
        assert_eq!(".\n |\n", output);
    }

    #[test]
    fn offset_across_rows() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: -1, y: 1}, 'x');
        canvas.put(Point {x: -1, y: 2}, 'y');
        canvas.put(Point {x: -1, y: 3}, 'z');
        let output = format!("{}", canvas);
        assert_eq!("z\ny\nx\n\n", output);
    }

    #[test]
    fn char_is_present_check() {
        let mut canvas = Canvas::new();
        canvas.put(Point {x: 2, y: 3}, 'x');
        let output = canvas.is_char_point(Point {x: 2, y: 3}, 'x');
        assert!(output);
    }
}
