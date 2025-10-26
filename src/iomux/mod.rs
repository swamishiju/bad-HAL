#[repr(C)]
pub struct IOMUX_SECCFG_Regs {
    reserved0: u32,
    pub PINCM: [u32; 251],
}

#[repr(C)]
pub struct IOMUX_Regs {
    pub SECCFG: IOMUX_SECCFG_Regs,
}


