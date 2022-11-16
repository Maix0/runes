//! [`carrying_add`] and [`borrowing_sub`] are taken from the std, but since they aren't stablized
//! as of writting, I copied their source here.

pub fn carrying_add(this: u8, rhs: u8, carry: bool) -> (u8, bool) {
    let (a, b) = this.overflowing_add(rhs);
    let (c, d) = a.overflowing_add(carry as u8);
    (c, b || d)
}

pub fn borrowing_sub(this: u8, rhs: u8, borrow: bool) -> (u8, bool) {
    let (a, b) = this.overflowing_sub(rhs);
    let (c, d) = a.overflowing_sub(borrow as u8);
    (c, b || d)
}
