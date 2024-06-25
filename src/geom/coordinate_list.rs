use super::coordinate::Coordinate;


#[derive(Clone)]
pub struct CoordinateList {
    vec: Vec<Coordinate>,
}

impl CoordinateList {
    /**
     * Constructs a new list without any coordinates
     */
    pub fn default() -> Self {
        Self { vec: vec![] }
    }

    /**
     * Constructs a new list from an array of Coordinates, allowing repeated points.
     * (I.e. this constructor produces a {@link CoordinateList} with exactly the same set of points
     * as the input array.)
     *
     * @param coord the initial coordinates
     */
    pub fn new(coord: Vec<Coordinate>) -> Self {
        let mut new = Self { vec: vec![] };
        new.add_coordinate_list_repeated(&coord, true);
        new
    }

    /**
     * Constructs a new list from an array of Coordinates,
     * allowing caller to specify if repeated points are to be removed.
     *
     * @param coord the array of coordinates to load into the list
     * @param allowRepeated if <code>false</code>, repeated points are removed
     */
    pub fn new_with_repeated(coord: Vec<Coordinate>, allow_repeated: bool) -> Self {
        let mut new = Self { vec: vec![] };
        new.add_coordinate_list_repeated(&coord, allow_repeated);
        new
    }

    pub fn get_coordinate(&self, i: usize) -> Option<Coordinate> {
        let coord = self.vec.get(i);
        match coord {
            Some(coord) => return Some(*coord),
            None => return None,
        }
    }

    /**
     * Adds a section of an array of coordinates to the list.
     * @param coord The coordinates
     * @param allowRepeated if set to false, repeated coordinates are collapsed
     * @param start the index to start from
     * @param end the index to add up to but not including
     * @return true (as by general collection contract)
     */
    pub fn add_coordinate_list_repeated_start_end(
        &mut self,
        coord_list: &Vec<Coordinate>,
        allow_repeated: bool,
        start: usize,
        end: usize,
    ) -> bool {
        let mut inc: i32 = 1;
        if start > end {
            inc = -1;
        }

        let mut i = start;
        while i != end {
            self.add_coordinate_repeated(coord_list[i], allow_repeated);
            i += inc as usize
        }
        return true;
    }

    /**
     * Adds an array of coordinates to the list.
     * @param coord The coordinates
     * @param allowRepeated if set to false, repeated coordinates are collapsed
     * @param direction if false, the array is added in reverse order
     * @return true (as by general collection contract)
     */
    pub fn add_coordinate_list_repeated_direction(
        &mut self,
        coord_list: &Vec<Coordinate>,
        allow_repeated: bool,
        direction: bool,
    ) -> bool {
        if direction {
            for i in 0..coord_list.len() {
                self.add_coordinate_repeated(coord_list[i], allow_repeated);
            }
        } else {
            let mut i = coord_list.len();
            while i >= 0 {
                self.add_coordinate_repeated(coord_list[i], allow_repeated);
                i -= 1;
            }
        }
        return true;
    }

    /**
     * Adds an array of coordinates to the list.
     * @param coord The coordinates
     * @param allowRepeated if set to false, repeated coordinates are collapsed
     * @return true (as by general collection contract)
     */
    pub fn add_coordinate_list_repeated(
        &mut self,
        coord_list: &Vec<Coordinate>,
        allow_repeated: bool,
    ) -> bool {
        self.add_coordinate_list_repeated_direction(coord_list, allow_repeated, true);
        return true;
    }

    /**
     * Adds a coordinate to the end of the list.
     *
     * @param coord The coordinates
     * @param allowRepeated if set to false, repeated coordinates are collapsed
     */
    pub fn add_coordinate_repeated(&mut self, coord: Coordinate, allow_repeated: bool) {
        // don't add duplicate coordinates
        if !allow_repeated {
            if self.vec.len() >= 1 {
                let index = self.vec.len() - 1;
                let last = self.get_coordinate(index);
                match last {
                    Some(last) => {
                        if last.equals_2d(&coord) {
                            return;
                        }
                    }
                    None => {}
                }
            }
        }
        self.vec.push(coord);
    }

    /**
     * Inserts the specified coordinate at the specified position in this list.
     *
     * @param i the position at which to insert
     * @param coord the coordinate to insert
     * @param allowRepeated if set to false, repeated coordinates are collapsed
     */
    pub fn add_index_coordinate_repeated(
        &mut self,
        i: usize,
        coord: Coordinate,
        allow_repeated: bool,
    ) {
        // don't add duplicate coordinates
        if !allow_repeated {
            let size = self.vec.len();
            if size > 0 {
                if i > 0 {
                    let prev = self.get_coordinate(i - 1);
                    match prev {
                        Some(prev) => {
                            if prev.equals_2d(&coord) {
                                return;
                            }
                        }
                        None => {}
                    }
                }
                if i < size {
                    let next = self.get_coordinate(i);
                    match next {
                        Some(next) => {
                            if next.equals_2d(&coord) {
                                return;
                            }
                        }
                        None => {}
                    }
                }
            }
        }
        self.vec.insert(i, coord);
    }

    /** Add an array of coordinates
     * @param coll The coordinates
     * @param allowRepeated if set to false, repeated coordinates are collapsed
     * @return true (as by general collection contract)
     */
    pub fn add_all(&mut self, coll: Vec<Coordinate>, allow_repeated: bool) -> bool {
        let mut is_changed = false;
        for coordinate in coll.iter() {
            self.add_coordinate_repeated(*coordinate, allow_repeated);
            is_changed = true;
        }
        return is_changed;
    }

    /**
     * Ensure this coordList is a ring, by adding the start point if necessary
     */
    pub fn close_ring(&mut self) {
        if self.vec.len() > 0 {
            let duplicate = self.get_coordinate(0);
            match duplicate {
                Some(duplicate) => {
                    self.add_coordinate_repeated(duplicate, false);
                }
                None => {}
            }
        }
    }

    /** Returns the Coordinates in this collection.
     *
     * @return the coordinates
     */
    pub fn to_coordinate_array(&self) -> Vec<Coordinate> {
        return self.vec.to_vec();
    }

    /**
     * Creates an array containing the coordinates in this list,
     * oriented in the given direction (forward or reverse).
     *
     * @param isForward true if the direction is forward, false for reverse
     * @return an oriented array of coordinates
     */
    pub fn to_coordinate_array_forward(&self, is_forward: bool) -> Vec<Coordinate> {
        if is_forward {
            return self.vec.to_vec();
        }
        // construct reversed array
        let size = self.vec.len();
        let mut pts: Vec<Coordinate> = vec![Coordinate::default(); size];
        for i in 0..size {
            let coord = self.get_coordinate(size - i - 1);
            match coord {
                Some(coord) => pts[i] = coord,
                None => {}
            }
        }
        return pts;
    }
}
