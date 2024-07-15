pub struct XorShiftRng {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}

impl XorShiftRng {
    pub fn new(seed: u64) -> XorShiftRng {
        let x = seed as u32;
        let y = (seed >> 32) as u32;
        let z = x ^ y;
        let w = y.wrapping_add(1);
        XorShiftRng { x, y, z, w }
    }

    pub fn gen_range(&mut self, range: std::ops::RangeInclusive<u32>) -> u32 {
        let (start, end) = range.into_inner();
        self.gen_range_start_end(start, end)
    }

    fn gen_range_start_end(&mut self, start: u32, end: u32) -> u32 {
        assert!(start <= end, "Invalid range");
        let range = end - start;
        start + self.gen_u32() % (range + 1)
    }

    fn gen_u32(&mut self) -> u32 {
        let t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = self.w ^ (self.w >> 19) ^ (t ^ (t >> 8));
        self.w
    }
}
