#[repr(C)]
pub struct MMSignedPoint {
    pub x: cty::int32_t,
    pub y: cty::int32_t,
}

#[link(name = "robotjs", kind = "static")]
extern "C" {
    pub fn moveMouse(point: MMSignedPoint);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe {
            moveMouse(MMSignedPoint { x: 20, y: 20 });
        }
    }
}
