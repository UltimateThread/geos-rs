pub struct Location {}

impl Location {
    /**
     * The location value for the interior of a geometry.
     * Also, DE-9IM row index of the interior of the first geometry and column index of
     *  the interior of the second geometry.
     */
    pub const INTERIOR: i32 = 0;
    /**
     * The location value for the boundary of a geometry.
     * Also, DE-9IM row index of the boundary of the first geometry and column index of
     *  the boundary of the second geometry.
     */
    pub const BOUNDARY: i32 = 1;
    /**
     * The location value for the exterior of a geometry.
     * Also, DE-9IM row index of the exterior of the first geometry and column index of
     *  the exterior of the second geometry.
     */
    pub const EXTERIOR: i32 = 2;

    /**
     *  Used for uninitialized location values.
     */
    pub const NONE: i32 = -1;

    /**
     *  Converts the location value to a location symbol, for example, <code>EXTERIOR =&gt; 'e'</code>
     *  .
     *
     *@param  locationValue  either EXTERIOR, BOUNDARY, INTERIOR or NONE
     *@return                either 'e', 'b', 'i' or '-'
     */
    pub fn to_location_symbol(location_value: i32) -> Option<char> {
        match location_value {
            Location::EXTERIOR => return Some('e'),
            Location::BOUNDARY => return Some('b'),
            Location::INTERIOR => return Some('i'),
            Location::NONE => return Some('-'),
            _ => return None,
        }
    }
}
