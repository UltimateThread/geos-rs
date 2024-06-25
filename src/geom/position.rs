pub struct Position {}

impl Position {
    /** Specifies that a location is <i>on</i> a component */
    pub const ON: i32 = 0;

    /** Specifies that a location is to the <i>left</i> of a component */
    pub const LEFT: i32 = 1;

    /** Specifies that a location is to the <i>right</i> of a component */
    pub const RIGHT: i32 = 2;

    /**
     * Returns LEFT if the position is RIGHT, RIGHT if the position is LEFT, or the position
     * otherwise.
     */
    pub fn opposite(position: i32) -> i32 {
        if position == Position::LEFT {
            return Position::RIGHT;
        }
        if position == Position::RIGHT {
            return Position::LEFT;
        }
        return position;
    }
}
