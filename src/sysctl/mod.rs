
#[repr(C)]
pub struct SYSCTL_SOCLOCK_Regs {
    reserved0: [u32; 8],
    IIDX: u32, /* !< (@ 0x00001020) SYSCTL interrupt index */
    reserved1: u32,
    IMASK: u32, /* !< (@ 0x00001028) SYSCTL interrupt mask */
    reserved2: u32,
    RIS: u32, /* !< (@ 0x00001030) SYSCTL raw interrupt status */
    reserved3: u32,
    MIS: u32, /* !< (@ 0x00001038) SYSCTL masked interrupt status */
    reserved4: u32,
    ISET: u32, /* !< (@ 0x00001040) SYSCTL interrupt set */
    reserved5: u32,
    ICLR: u32, /* !< (@ 0x00001048) SYSCTL interrupt clear */
    reserved6: u32,
    NMIIIDX: u32, /* !< (@ 0x00001050) NMI interrupt index */
    reserved7: [u32; 3],
    NMIRIS: u32, /* !< (@ 0x00001060) NMI raw interrupt status */
    reserved8: [u32; 3],
    NMIISET: u32, /* !< (@ 0x00001070) NMI interrupt set */
    reserved9: u32,
    NMIICLR: u32, /* !< (@ 0x00001078) NMI interrupt clear */
    reserved10: [u32; 33],
    SYSOSCCFG: u32,   /* !< (@ 0x00001100) SYSOSC configuration */
    MCLKCFG: u32,     /* !< (@ 0x00001104) Main clock (MCLK) configuration */
    pub HSCLKEN: u32,     /* !< (@ 0x00001108) High-speed clock (HSCLK) source enable/disable */
    HSCLKCFG: u32,    /* !< (@ 0x0000110C) High-speed clock (HSCLK) source selection */
    HFCLKCLKCFG: u32, /* !< (@ 0x00001110) High-frequency clock (HFCLK) configuration */
    LFCLKCFG: u32,    /* !< (@ 0x00001114) Low frequency crystal oscillator (LFXT)
                      configuration */
    reserved11: [u32; 8],
    GENCLKCFG: u32, /* !< (@ 0x00001138) General clock configuration */
    GENCLKEN: u32,  /* !< (@ 0x0000113C) General clock enable control */
    PMODECFG: u32,  /* !< (@ 0x00001140) Power mode configuration */
    reserved12: [u32; 3],
    FCC: u32, /* !< (@ 0x00001150) Frequency clock counter (FCC) count */
    reserved13: [u32; 7],
    SYSOSCTRIMUSER: u32, /* !< (@ 0x00001170) SYSOSC user-specified trim */
    reserved14: u32,
    SRAMBOUNDARY: u32, /* !< (@ 0x00001178) SRAM Write Boundary */
    reserved15: u32,
    SYSTEMCFG: u32, /* !< (@ 0x00001180) System configuration */
    reserved16: [u32; 31],
    WRITELOCK: u32,  /* !< (@ 0x00001200) SYSCTL register write lockout */
    CLKSTATUS: u32,  /* !< (@ 0x00001204) Clock module (CKM) status */
    SYSSTATUS: u32,  /* !< (@ 0x00001208) System status information */
    DEDERRADDR: u32, /* !< (@ 0x0000120C) Memory DED Address */
    reserved17: [u32; 4],
    RSTCAUSE: u32, /* !< (@ 0x00001220) Reset cause */
    reserved18: [u32; 55],
    RESETLEVEL: u32, /* !< (@ 0x00001300) Reset level for application-triggered reset
                     command */
    RESETCMD: u32, /* !< (@ 0x00001304) Execute an application-triggered reset command */
    pub BORTHRESHOLD: u32, /* !< (@ 0x00001308) BOR threshold selection */
    BORCLRCMD: u32, /* !< (@ 0x0000130C) Set the BOR threshold */
    SYSOSCFCLCTL: u32, /* !< (@ 0x00001310) SYSOSC frequency correction loop (FCL) ROSC enable */
    LFXTCTL: u32,  /* !< (@ 0x00001314) LFXT and LFCLK control */
    EXLFCTL: u32,  /* !< (@ 0x00001318) LFCLK_IN and LFCLK control */
    SHDNIOREL: u32, /* !< (@ 0x0000131C) SHUTDOWN IO release control */
    EXRSTPIN: u32, /* !< (@ 0x00001320) Disable the reset function of the NRST pin */
    SYSSTATUSCLR: u32, /* !< (@ 0x00001324) Clear sticky bits of SYSSTATUS */
    SWDCFG: u32,   /* !< (@ 0x00001328) Disable the SWD function on the SWD pins */
    FCCCMD: u32,   /* !< (@ 0x0000132C) Frequency clock counter start capture */
    reserved19: [u32; 52],
    SHUTDNSTORE0: u32, /* !< (@ 0x00001400) Shutdown storage memory (byte 0) */
    SHUTDNSTORE1: u32, /* !< (@ 0x00001404) Shutdown storage memory (byte 1) */
    SHUTDNSTORE2: u32, /* !< (@ 0x00001408) Shutdown storage memory (byte 2) */
    SHUTDNSTORE3: u32, /* !< (@ 0x0000140C) Shutdown storage memory (byte 3) */
}

#[repr(C)]
struct SYSCTL_SECCFG_Regs {
    FWEPROTMAIN: u32, /* !< (@ 0x00003000) 1 Sector Write-Erase per bit starting at address
                      0x0 of flash */
    reserved0: [u32; 5],
    FRXPROTMAINSTART: u32, /* !< (@ 0x00003018) Flash RX Protection Start Address */
    FRXPROTMAINEND: u32,   /* !< (@ 0x0000301C) Flash RX Protection End Address */
    FIPPROTMAINSTART: u32, /* !< (@ 0x00003020) Flash IP Protection Start Address */
    FIPPROTMAINEND: u32,   /* !< (@ 0x00003024) Flash IP Protection End Address */
    reserved1: [u32; 4],
    FLBANKSWPPOLICY: u32, /* !< (@ 0x00003038) Flash Bank Swap Policy */
    FLBANKSWP: u32,       /* !< (@ 0x0000303C) Flash MAIN bank address swap */
    reserved2: u32,
    FWENABLE: u32,  /* !< (@ 0x00003044) Security Firewall Enable Register */
    SECSTATUS: u32, /* !< (@ 0x00003048) Security Configuration  status */
    reserved3: [u32; 5],
    INITDONE: u32, /* !< (@ 0x00003060) INITCODE PASS */
}

#[repr(C)]
pub struct SYSCTL_Regs {
    reserved0: [u32; 1024],
    pub SOCLOCK: SYSCTL_SOCLOCK_Regs, /* !< (@ 0x00001000) SYSCTL SOCLOCK Region */
    reserved1: [u32; 1788],
    SECCFG: SYSCTL_SECCFG_Regs, /* !< (@ 0x00003000) SYSCTL SECCFG Region */
}
