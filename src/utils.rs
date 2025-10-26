pub trait MemoryMapped: Sized {
    #[inline(always)]
    fn from_addr(addr: u32) -> &'static mut Self {
        unsafe { &mut *(addr as *mut Self) }
    }
}

pub fn update_reg(reg: &mut u32, val: u32, mask: u32) {
    let tmp = *reg & (!mask);
    *reg = tmp | (val & mask);
}
