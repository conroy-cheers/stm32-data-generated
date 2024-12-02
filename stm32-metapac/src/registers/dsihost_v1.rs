
                    use crate::metadata::ir::*;
                    pub(crate) static REGISTERS: IR = IR {
    blocks: &[
        Block {
            name: "Dsihost",
            extends: None,
            description: Some(
                "DSI Host.",
            ),
            items: &[
                BlockItem {
                    name: "vr",
                    description: Some(
                        "DSI Host Version Register.",
                    ),
                    array: None,
                    byte_offset: 0x0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cr",
                    description: Some(
                        "DSI Host Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ccr",
                    description: Some(
                        "DSI HOST Clock Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lvcidr",
                    description: Some(
                        "DSI Host LTDC VCID Register.",
                    ),
                    array: None,
                    byte_offset: 0xc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lvcidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcolcr",
                    description: Some(
                        "DSI Host LTDC Color Coding Register.",
                    ),
                    array: None,
                    byte_offset: 0x10,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcolcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpcr",
                    description: Some(
                        "DSI Host LTDC Polarity Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x14,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpmcr",
                    description: Some(
                        "DSI Host Low-Power mode Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x18,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pcr",
                    description: Some(
                        "DSI Host Protocol Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x2c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gvcidr",
                    description: Some(
                        "DSI Host Generic VCID Register.",
                    ),
                    array: None,
                    byte_offset: 0x30,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gvcidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "mcr",
                    description: Some(
                        "DSI Host mode Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x34,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Mcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vmcr",
                    description: Some(
                        "DSI Host Video mode Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x38,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vpcr",
                    description: Some(
                        "DSI Host Video Packet Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x3c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vccr",
                    description: Some(
                        "DSI Host Video Chunks Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x40,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vnpcr",
                    description: Some(
                        "DSI Host Video Null Packet Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x44,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vnpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vhsacr",
                    description: Some(
                        "DSI Host Video HSA Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x48,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vhsacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vhbpcr",
                    description: Some(
                        "DSI Host Video HBP Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x4c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vhbpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vlcr",
                    description: Some(
                        "DSI Host Video Line Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x50,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vlcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvsacr",
                    description: Some(
                        "DSI Host Video VSA Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x54,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvsacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvbpcr",
                    description: Some(
                        "DSI Host Video VBP Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x58,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvbpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvfpcr",
                    description: Some(
                        "DSI Host Video VFP Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x5c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvfpcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvacr",
                    description: Some(
                        "DSI Host Video VA Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x60,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvacr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lccr",
                    description: Some(
                        "DSI Host LTDC Command Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x64,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Lccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cmcr",
                    description: Some(
                        "DSI Host Command mode Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x68,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cmcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ghcr",
                    description: Some(
                        "DSI Host Generic Header Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x6c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ghcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpdr",
                    description: Some(
                        "DSI Host Generic Payload Data Register.",
                    ),
                    array: None,
                    byte_offset: 0x70,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpdr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "gpsr",
                    description: Some(
                        "DSI Host Generic Packet Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x74,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Gpsr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr0",
                    description: Some(
                        "DSI Host Timeout Counter Configuration Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x78,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr1",
                    description: Some(
                        "DSI Host Timeout Counter Configuration Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x7c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr2",
                    description: Some(
                        "DSI Host Timeout Counter Configuration Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x80,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr3",
                    description: Some(
                        "DSI Host Timeout Counter Configuration Register 3.",
                    ),
                    array: None,
                    byte_offset: 0x84,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr4",
                    description: Some(
                        "DSI Host Timeout Counter Configuration Register 4.",
                    ),
                    array: None,
                    byte_offset: 0x88,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "tccr5",
                    description: Some(
                        "DSI Host Timeout Counter Configuration Register 5.",
                    ),
                    array: None,
                    byte_offset: 0x8c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Tccr5",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "clcr",
                    description: Some(
                        "DSI Host Clock Lane Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x94,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Clcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "cltcr",
                    description: Some(
                        "DSI Host Clock Lane Timer Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x98,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Cltcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "dltcr",
                    description: Some(
                        "DSI Host Data Lane Timer Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x9c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Dltcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pctlr",
                    description: Some(
                        "DSI Host PHY Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xa0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pctlr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pconfr",
                    description: Some(
                        "DSI Host PHY Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xa4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pconfr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pucr",
                    description: Some(
                        "DSI Host PHY ULPS Control Register.",
                    ),
                    array: None,
                    byte_offset: 0xa8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pucr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "pttcr",
                    description: Some(
                        "DSI Host PHY TX Triggers Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0xac,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Pttcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "psr",
                    description: Some(
                        "DSI Host PHY Status Register.",
                    ),
                    array: None,
                    byte_offset: 0xb0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Psr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr0",
                    description: Some(
                        "DSI Host Interrupt & Status Register 0.",
                    ),
                    array: None,
                    byte_offset: 0xbc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "isr1",
                    description: Some(
                        "DSI Host Interrupt & Status Register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc0,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Isr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier0",
                    description: Some(
                        "DSI Host Interrupt Enable Register 0.",
                    ),
                    array: None,
                    byte_offset: 0xc4,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "ier1",
                    description: Some(
                        "DSI Host Interrupt Enable Register 1.",
                    ),
                    array: None,
                    byte_offset: 0xc8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Ier1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fir0",
                    description: Some(
                        "DSI Host Force Interrupt Register 0.",
                    ),
                    array: None,
                    byte_offset: 0xd8,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Fir0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "fir1",
                    description: Some(
                        "DSI Host Force Interrupt Register 1.",
                    ),
                    array: None,
                    byte_offset: 0xdc,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Write,
                            bit_size: 32,
                            fieldset: Some(
                                "Fir1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vscr",
                    description: Some(
                        "DSI Host Video Shadow Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x100,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Vscr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcvcidr",
                    description: Some(
                        "DSI Host LTDC Current VCID Register.",
                    ),
                    array: None,
                    byte_offset: 0x10c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcvcidr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lcccr",
                    description: Some(
                        "DSI Host LTDC Current Color Coding Register.",
                    ),
                    array: None,
                    byte_offset: 0x110,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Lcccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "lpmccr",
                    description: Some(
                        "DSI Host Low-Power mode Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x118,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Lpmccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vmccr",
                    description: Some(
                        "DSI Host Video mode Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x138,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vmccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vpccr",
                    description: Some(
                        "DSI Host Video Packet Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x13c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vcccr",
                    description: Some(
                        "DSI Host Video Chunks Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x140,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vcccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vnpccr",
                    description: Some(
                        "DSI Host Video Null Packet Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x144,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vnpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vhsaccr",
                    description: Some(
                        "DSI Host Video HSA Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x148,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vhsaccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vhbpccr",
                    description: Some(
                        "DSI Host Video HBP Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x14c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vhbpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vlccr",
                    description: Some(
                        "DSI Host Video Line Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x150,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vlccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvsaccr",
                    description: Some(
                        "DSI Host Video VSA Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x154,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvsaccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvbpccr",
                    description: Some(
                        "DSI Host Video VBP Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x158,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvbpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvfpccr",
                    description: Some(
                        "DSI Host Video VFP Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x15c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvfpccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "vvaccr",
                    description: Some(
                        "DSI Host Video VA Current Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x160,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Vvaccr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wcfgr",
                    description: Some(
                        "DSI Wrapper Configuration Register.",
                    ),
                    array: None,
                    byte_offset: 0x400,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wcfgr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wcr",
                    description: Some(
                        "DSI Wrapper Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x404,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wier",
                    description: Some(
                        "DSI Wrapper Interrupt Enable Register.",
                    ),
                    array: None,
                    byte_offset: 0x408,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wier",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wisr",
                    description: Some(
                        "DSI Wrapper Interrupt & Status Register.",
                    ),
                    array: None,
                    byte_offset: 0x40c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::Read,
                            bit_size: 32,
                            fieldset: Some(
                                "Wisr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wifcr",
                    description: Some(
                        "DSI Wrapper Interrupt Flag Clear Register.",
                    ),
                    array: None,
                    byte_offset: 0x410,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wifcr",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpcr0",
                    description: Some(
                        "DSI Wrapper PHY Configuration Register 0.",
                    ),
                    array: None,
                    byte_offset: 0x418,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpcr0",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpcr1",
                    description: Some(
                        "DSI Wrapper PHY Configuration Register 1.",
                    ),
                    array: None,
                    byte_offset: 0x41c,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpcr1",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpcr2",
                    description: Some(
                        "DSI Wrapper PHY Configuration Register 2.",
                    ),
                    array: None,
                    byte_offset: 0x420,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpcr2",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpcr3",
                    description: Some(
                        "DSI Wrapper PHY Configuration Register 3.",
                    ),
                    array: None,
                    byte_offset: 0x424,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpcr3",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wpcr4",
                    description: Some(
                        "DSI Wrapper PHY Configuration Register 4.",
                    ),
                    array: None,
                    byte_offset: 0x428,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wpcr4",
                            ),
                        },
                    ),
                },
                BlockItem {
                    name: "wrpcr",
                    description: Some(
                        "DSI Wrapper Regulator and PLL Control Register.",
                    ),
                    array: None,
                    byte_offset: 0x430,
                    inner: BlockItemInner::Register(
                        Register {
                            access: Access::ReadWrite,
                            bit_size: 32,
                            fieldset: Some(
                                "Wrpcr",
                            ),
                        },
                    ),
                },
            ],
        },
    ],
    fieldsets: &[
        FieldSet {
            name: "Ccr",
            extends: None,
            description: Some(
                "DSI HOST Clock Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "txeckdiv",
                    description: Some(
                        "TX Escape Clock Division.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tockdiv",
                    description: Some(
                        "Timeout Clock Division.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Clcr",
            extends: None,
            description: Some(
                "DSI Host Clock Lane Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dpcc",
                    description: Some(
                        "D-PHY Clock Control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "acr",
                    description: Some(
                        "Automatic Clock lane Control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cltcr",
            extends: None,
            description: Some(
                "DSI Host Clock Lane Timer Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lp2hs_time",
                    description: Some(
                        "Low-Power to High-Speed Time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hs2lp_time",
                    description: Some(
                        "High-Speed to Low-Power Time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cmcr",
            extends: None,
            description: Some(
                "DSI Host Command mode Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "teare",
                    description: Some(
                        "Tearing Effect Acknowledge Request Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "are",
                    description: Some(
                        "Acknowledge Request Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gsw0tx",
                    description: Some(
                        "Generic Short Write Zero parameters Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gsw1tx",
                    description: Some(
                        "Generic Short Write One parameters Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gsw2tx",
                    description: Some(
                        "Generic Short Write Two parameters Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gsr0tx",
                    description: Some(
                        "Generic Short Read Zero parameters Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gsr1tx",
                    description: Some(
                        "Generic Short Read One parameters Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gsr2tx",
                    description: Some(
                        "Generic Short Read Two parameters Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "glwtx",
                    description: Some(
                        "Generic Long Write Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsw0tx",
                    description: Some(
                        "DCS Short Write Zero parameter Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsw1tx",
                    description: Some(
                        "DCS Short Read One parameter Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsr0tx",
                    description: Some(
                        "DCS Short Read Zero parameter Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dlwtx",
                    description: Some(
                        "DCS Long Write Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "mrdps",
                    description: Some(
                        "Maximum Read Packet Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Cr",
            extends: None,
            description: Some(
                "DSI Host Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Dltcr",
            extends: None,
            description: Some(
                "DSI Host Data Lane Timer Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "mrd_time",
                    description: Some(
                        "Maximum Read Time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lp2hs_time",
                    description: Some(
                        "Low-Power To High-Speed Time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hs2lp_time",
                    description: Some(
                        "High-Speed To Low-Power Time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fir0",
            extends: None,
            description: Some(
                "DSI Host Force Interrupt Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "fae0",
                    description: Some(
                        "Force Acknowledge Error 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae1",
                    description: Some(
                        "Force Acknowledge Error 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae2",
                    description: Some(
                        "Force Acknowledge Error 2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae3",
                    description: Some(
                        "Force Acknowledge Error 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae4",
                    description: Some(
                        "Force Acknowledge Error 4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae5",
                    description: Some(
                        "Force Acknowledge Error 5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae6",
                    description: Some(
                        "Force Acknowledge Error 6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae7",
                    description: Some(
                        "Force Acknowledge Error 7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae8",
                    description: Some(
                        "Force Acknowledge Error 8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae9",
                    description: Some(
                        "Force Acknowledge Error 9.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae10",
                    description: Some(
                        "Force Acknowledge Error 10.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae11",
                    description: Some(
                        "Force Acknowledge Error 11.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae12",
                    description: Some(
                        "Force Acknowledge Error 12.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae13",
                    description: Some(
                        "Force Acknowledge Error 13.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae14",
                    description: Some(
                        "Force Acknowledge Error 14.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fae15",
                    description: Some(
                        "Force Acknowledge Error 15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpe0",
                    description: Some(
                        "Force PHY Error 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpe1",
                    description: Some(
                        "Force PHY Error 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpe2",
                    description: Some(
                        "Force PHY Error 2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpe3",
                    description: Some(
                        "Force PHY Error 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpe4",
                    description: Some(
                        "Force PHY Error 4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Fir1",
            extends: None,
            description: Some(
                "DSI Host Force Interrupt Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ftohstx",
                    description: Some(
                        "Force Timeout High-Speed Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ftolprx",
                    description: Some(
                        "Force Timeout Low-Power Reception.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "feccse",
                    description: Some(
                        "Force ECC Single-bit Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "feccme",
                    description: Some(
                        "Force ECC Multi-bit Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fcrce",
                    description: Some(
                        "Force CRC Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fpse",
                    description: Some(
                        "Force Packet Size Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "feotpe",
                    description: Some(
                        "Force EoTp Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flpwre",
                    description: Some(
                        "Force LTDC Payload Write Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fgcwre",
                    description: Some(
                        "Force Generic Command Write Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fgpwre",
                    description: Some(
                        "Force Generic Payload Write Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fgptxe",
                    description: Some(
                        "Force Generic Payload Transmit Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fgprde",
                    description: Some(
                        "Force Generic Payload Read Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fgprxe",
                    description: Some(
                        "Force Generic Payload Receive Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ghcr",
            extends: None,
            description: Some(
                "DSI Host Generic Header Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dt",
                    description: Some(
                        "Type.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vcid",
                    description: Some(
                        "Channel.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wclsb",
                    description: Some(
                        "WordCount LSB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "wcmsb",
                    description: Some(
                        "WordCount MSB.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gpdr",
            extends: None,
            description: Some(
                "DSI Host Generic Payload Data Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "data1",
                    description: Some(
                        "Payload Byte 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "data2",
                    description: Some(
                        "Payload Byte 2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "data3",
                    description: Some(
                        "Payload Byte 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "data4",
                    description: Some(
                        "Payload Byte 4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gpsr",
            extends: None,
            description: Some(
                "DSI Host Generic Packet Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmdfe",
                    description: Some(
                        "Command FIFO Empty.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cmdff",
                    description: Some(
                        "Command FIFO Full.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwrfe",
                    description: Some(
                        "Payload Write FIFO Empty.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pwrff",
                    description: Some(
                        "Payload Write FIFO Full.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prdfe",
                    description: Some(
                        "Payload Read FIFO Empty.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "prdff",
                    description: Some(
                        "Payload Read FIFO Full.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rcb",
                    description: Some(
                        "Read Command Busy.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Gvcidr",
            extends: None,
            description: Some(
                "DSI Host Generic VCID Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcid",
                    description: Some(
                        "Virtual Channel ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ier0",
            extends: None,
            description: Some(
                "DSI Host Interrupt Enable Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ae0ie",
                    description: Some(
                        "Acknowledge Error 0 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae1ie",
                    description: Some(
                        "Acknowledge Error 1 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae2ie",
                    description: Some(
                        "Acknowledge Error 2 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae3ie",
                    description: Some(
                        "Acknowledge Error 3 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae4ie",
                    description: Some(
                        "Acknowledge Error 4 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae5ie",
                    description: Some(
                        "Acknowledge Error 5 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae6ie",
                    description: Some(
                        "Acknowledge Error 6 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae7ie",
                    description: Some(
                        "Acknowledge Error 7 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae8ie",
                    description: Some(
                        "Acknowledge Error 8 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae9ie",
                    description: Some(
                        "Acknowledge Error 9 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae10ie",
                    description: Some(
                        "Acknowledge Error 10 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae11ie",
                    description: Some(
                        "Acknowledge Error 11 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae12ie",
                    description: Some(
                        "Acknowledge Error 12 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae13ie",
                    description: Some(
                        "Acknowledge Error 13 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae14ie",
                    description: Some(
                        "Acknowledge Error 14 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae15ie",
                    description: Some(
                        "Acknowledge Error 15 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe0ie",
                    description: Some(
                        "PHY Error 0 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe1ie",
                    description: Some(
                        "PHY Error 1 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe2ie",
                    description: Some(
                        "PHY Error 2 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe3ie",
                    description: Some(
                        "PHY Error 3 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe4ie",
                    description: Some(
                        "PHY Error 4 Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Ier1",
            extends: None,
            description: Some(
                "DSI Host Interrupt Enable Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tohstxie",
                    description: Some(
                        "Timeout High-Speed Transmission Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tolprxie",
                    description: Some(
                        "Timeout Low-Power Reception Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccseie",
                    description: Some(
                        "ECC Single-bit Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccmeie",
                    description: Some(
                        "ECC Multi-bit Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crceie",
                    description: Some(
                        "CRC Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pseie",
                    description: Some(
                        "Packet Size Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eotpeie",
                    description: Some(
                        "EoTp Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpwreie",
                    description: Some(
                        "LTDC Payload Write Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gcwreie",
                    description: Some(
                        "Generic Command Write Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpwreie",
                    description: Some(
                        "Generic Payload Write Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptxeie",
                    description: Some(
                        "Generic Payload Transmit Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gprdeie",
                    description: Some(
                        "Generic Payload Read Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gprxeie",
                    description: Some(
                        "Generic Payload Receive Error Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr0",
            extends: None,
            description: Some(
                "DSI Host Interrupt & Status Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ae0",
                    description: Some(
                        "Acknowledge Error 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae1",
                    description: Some(
                        "Acknowledge Error 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae2",
                    description: Some(
                        "Acknowledge Error 2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae3",
                    description: Some(
                        "Acknowledge Error 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae4",
                    description: Some(
                        "Acknowledge Error 4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae5",
                    description: Some(
                        "Acknowledge Error 5.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae6",
                    description: Some(
                        "Acknowledge Error 6.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae7",
                    description: Some(
                        "Acknowledge Error 7.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae8",
                    description: Some(
                        "Acknowledge Error 8.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae9",
                    description: Some(
                        "Acknowledge Error 9.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae10",
                    description: Some(
                        "Acknowledge Error 10.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae11",
                    description: Some(
                        "Acknowledge Error 11.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae12",
                    description: Some(
                        "Acknowledge Error 12.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae13",
                    description: Some(
                        "Acknowledge Error 13.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae14",
                    description: Some(
                        "Acknowledge Error 14.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ae15",
                    description: Some(
                        "Acknowledge Error 15.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe0",
                    description: Some(
                        "PHY Error 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe1",
                    description: Some(
                        "PHY Error 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 17,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe2",
                    description: Some(
                        "PHY Error 2.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe3",
                    description: Some(
                        "PHY Error 3.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pe4",
                    description: Some(
                        "PHY Error 4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Isr1",
            extends: None,
            description: Some(
                "DSI Host Interrupt & Status Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tohstx",
                    description: Some(
                        "Timeout High-Speed Transmission.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tolprx",
                    description: Some(
                        "Timeout Low-Power Reception.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccse",
                    description: Some(
                        "ECC Single-bit Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccme",
                    description: Some(
                        "ECC Multi-bit Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crce",
                    description: Some(
                        "CRC Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pse",
                    description: Some(
                        "Packet Size Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eotpe",
                    description: Some(
                        "EoTp Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpwre",
                    description: Some(
                        "LTDC Payload Write Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gcwre",
                    description: Some(
                        "Generic Command Write Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gpwre",
                    description: Some(
                        "Generic Payload Write Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gptxe",
                    description: Some(
                        "Generic Payload Transmit Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gprde",
                    description: Some(
                        "Generic Payload Read Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "gprxe",
                    description: Some(
                        "Generic Payload Receive Error.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lcccr",
            extends: None,
            description: Some(
                "DSI Host LTDC Current Color Coding Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "colc",
                    description: Some(
                        "Color Coding.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpe",
                    description: Some(
                        "Loosely Packed Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lccr",
            extends: None,
            description: Some(
                "DSI Host LTDC Command Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmdsize",
                    description: Some(
                        "Command Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lcolcr",
            extends: None,
            description: Some(
                "DSI Host LTDC Color Coding Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "colc",
                    description: Some(
                        "Color Coding.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpe",
                    description: Some(
                        "Loosely Packet Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lcvcidr",
            extends: None,
            description: Some(
                "DSI Host LTDC Current VCID Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcid",
                    description: Some(
                        "Virtual Channel ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lpcr",
            extends: None,
            description: Some(
                "DSI Host LTDC Polarity Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dep",
                    description: Some(
                        "Data Enable Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vsp",
                    description: Some(
                        "VSYNC Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsp",
                    description: Some(
                        "HSYNC Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lpmccr",
            extends: None,
            description: Some(
                "DSI Host Low-Power mode Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlpsize",
                    description: Some(
                        "VACT Largest Packet Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpsize",
                    description: Some(
                        "Largest Packet Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lpmcr",
            extends: None,
            description: Some(
                "DSI Host Low-Power mode Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vlpsize",
                    description: Some(
                        "VACT Largest Packet Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpsize",
                    description: Some(
                        "Largest Packet Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Lvcidr",
            extends: None,
            description: Some(
                "DSI Host LTDC VCID Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vcid",
                    description: Some(
                        "Virtual Channel ID.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Mcr",
            extends: None,
            description: Some(
                "DSI Host mode Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cmdm",
                    description: Some(
                        "Command mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pconfr",
            extends: None,
            description: Some(
                "DSI Host PHY Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "nl",
                    description: Some(
                        "Number of Lanes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sw_time",
                    description: Some(
                        "Stop Wait Time.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pcr",
            extends: None,
            description: Some(
                "DSI Host Protocol Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "ettxe",
                    description: Some(
                        "EoTp Transmission Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "etrxe",
                    description: Some(
                        "EoTp Reception Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "btae",
                    description: Some(
                        "Bus Turn Around Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "eccrxe",
                    description: Some(
                        "ECC Reception Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crcrxe",
                    description: Some(
                        "CRC Reception Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pctlr",
            extends: None,
            description: Some(
                "DSI Host PHY Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "den",
                    description: Some(
                        "Digital Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cke",
                    description: Some(
                        "Clock Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Psr",
            extends: None,
            description: Some(
                "DSI Host PHY Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pd",
                    description: Some(
                        "PHY Direction.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pssc",
                    description: Some(
                        "PHY Stop State Clock lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uanc",
                    description: Some(
                        "ULPS Active Not Clock lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pss0",
                    description: Some(
                        "PHY Stop State lane 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uan0",
                    description: Some(
                        "ULPS Active Not lane 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rue0",
                    description: Some(
                        "RX ULPS Escape lane 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pss1",
                    description: Some(
                        "PHY Stop State lane 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uan1",
                    description: Some(
                        "ULPS Active Not lane 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pttcr",
            extends: None,
            description: Some(
                "DSI Host PHY TX Triggers Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tx_trig",
                    description: Some(
                        "Transmission Trigger.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Pucr",
            extends: None,
            description: Some(
                "DSI Host PHY ULPS Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "urcl",
                    description: Some(
                        "ULPS Request on Clock Lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uecl",
                    description: Some(
                        "ULPS Exit on Clock Lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "urdl",
                    description: Some(
                        "ULPS Request on Data Lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "uedl",
                    description: Some(
                        "ULPS Exit on Data Lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tccr0",
            extends: None,
            description: Some(
                "DSI Host Timeout Counter Configuration Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lprx_tocnt",
                    description: Some(
                        "Low-power Reception Timeout Counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hstx_tocnt",
                    description: Some(
                        "High-Speed Transmission Timeout Counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tccr1",
            extends: None,
            description: Some(
                "DSI Host Timeout Counter Configuration Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsrd_tocnt",
                    description: Some(
                        "High-Speed Read Timeout Counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tccr2",
            extends: None,
            description: Some(
                "DSI Host Timeout Counter Configuration Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lprd_tocnt",
                    description: Some(
                        "Low-Power Read Timeout Counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tccr3",
            extends: None,
            description: Some(
                "DSI Host Timeout Counter Configuration Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hswr_tocnt",
                    description: Some(
                        "High-Speed Write Timeout Counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pm",
                    description: Some(
                        "Presp mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tccr4",
            extends: None,
            description: Some(
                "DSI Host Timeout Counter Configuration Register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "lswr_tocnt",
                    description: Some(
                        "Low-Power Write Timeout Counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Tccr5",
            extends: None,
            description: Some(
                "DSI Host Timeout Counter Configuration Register 5.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "bta_tocnt",
                    description: Some(
                        "Bus-Turn-Around Timeout Counter.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 16,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vcccr",
            extends: None,
            description: Some(
                "DSI Host Video Chunks Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "numc",
                    description: Some(
                        "Number of Chunks.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vccr",
            extends: None,
            description: Some(
                "DSI Host Video Chunks Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "numc",
                    description: Some(
                        "Number of Chunks.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vhbpccr",
            extends: None,
            description: Some(
                "DSI Host Video HBP Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hbp",
                    description: Some(
                        "Horizontal Back-Porch duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vhbpcr",
            extends: None,
            description: Some(
                "DSI Host Video HBP Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hbp",
                    description: Some(
                        "Horizontal Back-Porch duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vhsaccr",
            extends: None,
            description: Some(
                "DSI Host Video HSA Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsa",
                    description: Some(
                        "Horizontal Synchronism Active duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vhsacr",
            extends: None,
            description: Some(
                "DSI Host Video HSA Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hsa",
                    description: Some(
                        "Horizontal Synchronism Active duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 12,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vlccr",
            extends: None,
            description: Some(
                "DSI Host Video Line Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hline",
                    description: Some(
                        "Horizontal Line duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vlcr",
            extends: None,
            description: Some(
                "DSI Host Video Line Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hline",
                    description: Some(
                        "Horizontal Line duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 15,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vmccr",
            extends: None,
            description: Some(
                "DSI Host Video mode Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vmt",
                    description: Some(
                        "Video mode Type.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpvsae",
                    description: Some(
                        "Low-Power Vertical Sync time Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpvbpe",
                    description: Some(
                        "Low-power Vertical Back-Porch Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpvfpe",
                    description: Some(
                        "Low-power Vertical Front-Porch Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpvae",
                    description: Some(
                        "Low-Power Vertical Active Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lphbpe",
                    description: Some(
                        "Low-power Horizontal Back-Porch Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lphfe",
                    description: Some(
                        "Low-Power Horizontal Front-Porch Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fbtaae",
                    description: Some(
                        "Frame BTA Acknowledge Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpce",
                    description: Some(
                        "Low-Power Command Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vmcr",
            extends: None,
            description: Some(
                "DSI Host Video mode Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vmt",
                    description: Some(
                        "Video mode Type.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpvsae",
                    description: Some(
                        "Low-Power Vertical Sync Active Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpvbpe",
                    description: Some(
                        "Low-power Vertical Back-Porch Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpvfpe",
                    description: Some(
                        "Low-power Vertical Front-porch Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpvae",
                    description: Some(
                        "Low-Power Vertical Active Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lphbpe",
                    description: Some(
                        "Low-Power Horizontal Back-Porch Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lphfpe",
                    description: Some(
                        "Low-Power Horizontal Front-Porch Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "fbtaae",
                    description: Some(
                        "Frame Bus-Turn-Around Acknowledge Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpce",
                    description: Some(
                        "Low-Power Command Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 15,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pge",
                    description: Some(
                        "Pattern Generator Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgm",
                    description: Some(
                        "Pattern Generator mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pgo",
                    description: Some(
                        "Pattern Generator Orientation.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vnpccr",
            extends: None,
            description: Some(
                "DSI Host Video Null Packet Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "npsize",
                    description: Some(
                        "Null Packet Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vnpcr",
            extends: None,
            description: Some(
                "DSI Host Video Null Packet Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "npsize",
                    description: Some(
                        "Null Packet Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 13,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vpccr",
            extends: None,
            description: Some(
                "DSI Host Video Packet Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vpsize",
                    description: Some(
                        "Video Packet Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vpcr",
            extends: None,
            description: Some(
                "DSI Host Video Packet Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vpsize",
                    description: Some(
                        "Video Packet Size.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vr",
            extends: None,
            description: Some(
                "DSI Host Version Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "version",
                    description: Some(
                        "Version of the DSI Host.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 32,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vscr",
            extends: None,
            description: Some(
                "DSI Host Video Shadow Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "en",
                    description: Some(
                        "Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ur",
                    description: Some(
                        "Update Register.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvaccr",
            extends: None,
            description: Some(
                "DSI Host Video VA Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "va",
                    description: Some(
                        "Vertical Active duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvacr",
            extends: None,
            description: Some(
                "DSI Host Video VA Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "va",
                    description: Some(
                        "Vertical Active duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 14,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvbpccr",
            extends: None,
            description: Some(
                "DSI Host Video VBP Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbp",
                    description: Some(
                        "Vertical Back-Porch duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvbpcr",
            extends: None,
            description: Some(
                "DSI Host Video VBP Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vbp",
                    description: Some(
                        "Vertical Back-Porch duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvfpccr",
            extends: None,
            description: Some(
                "DSI Host Video VFP Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vfp",
                    description: Some(
                        "Vertical Front-Porch duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvfpcr",
            extends: None,
            description: Some(
                "DSI Host Video VFP Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vfp",
                    description: Some(
                        "Vertical Front-Porch duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvsaccr",
            extends: None,
            description: Some(
                "DSI Host Video VSA Current Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsa",
                    description: Some(
                        "Vertical Synchronism Active duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Vvsacr",
            extends: None,
            description: Some(
                "DSI Host Video VSA Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "vsa",
                    description: Some(
                        "Vertical Synchronism Active duration.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 10,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wcfgr",
            extends: None,
            description: Some(
                "DSI Wrapper Configuration Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "dsim",
                    description: Some(
                        "DSI Mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "colmux",
                    description: Some(
                        "Color Multiplexing.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 3,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tesrc",
                    description: Some(
                        "TE Source.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 4,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tepol",
                    description: Some(
                        "TE Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 5,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ar",
                    description: Some(
                        "Automatic Refresh.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "vspol",
                    description: Some(
                        "VSync Polarity.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wcr",
            extends: None,
            description: Some(
                "DSI Wrapper Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "colm",
                    description: Some(
                        "Color Mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "shtdn",
                    description: Some(
                        "Shutdown.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ltdcen",
                    description: Some(
                        "LTDC Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "dsien",
                    description: Some(
                        "DSI Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 3,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wier",
            extends: None,
            description: Some(
                "DSI Wrapper Interrupt Enable Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "teie",
                    description: Some(
                        "Tearing Effect Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erie",
                    description: Some(
                        "End of Refresh Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plllie",
                    description: Some(
                        "PLL Lock Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plluie",
                    description: Some(
                        "PLL Unlock Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrie",
                    description: Some(
                        "Regulator Ready Interrupt Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wifcr",
            extends: None,
            description: Some(
                "DSI Wrapper Interrupt Flag Clear Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "cteif",
                    description: Some(
                        "Clear Tearing Effect Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cerif",
                    description: Some(
                        "Clear End of Refresh Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cplllif",
                    description: Some(
                        "Clear PLL Lock Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cplluif",
                    description: Some(
                        "Clear PLL Unlock Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "crrif",
                    description: Some(
                        "Clear Regulator Ready Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wisr",
            extends: None,
            description: Some(
                "DSI Wrapper Interrupt & Status Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "teif",
                    description: Some(
                        "Tearing Effect Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "erif",
                    description: Some(
                        "End of Refresh Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 1,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "busy",
                    description: Some(
                        "Busy Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pllls",
                    description: Some(
                        "PLL Lock Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plllif",
                    description: Some(
                        "PLL Lock Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "plluif",
                    description: Some(
                        "PLL Unlock Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrs",
                    description: Some(
                        "Regulator Ready Status.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "rrif",
                    description: Some(
                        "Regulator Ready Interrupt Flag.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wpcr0",
            extends: None,
            description: Some(
                "DSI Wrapper PHY Configuration Register 0.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "uix4",
                    description: Some(
                        "Unit Interval multiplied by 4.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 6,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swcl",
                    description: Some(
                        "Swap Clock Lane pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swdl0",
                    description: Some(
                        "Swap Data Lane 0 pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 7,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "swdl1",
                    description: Some(
                        "Swap Data Lane 1 pins.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsicl",
                    description: Some(
                        "Invert Hight-Speed data signal on Clock Lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 9,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsidl0",
                    description: Some(
                        "Invert the Hight-Speed data signal on Data Lane 0.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 10,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hsidl1",
                    description: Some(
                        "Invert the High-Speed data signal on Data Lane 1.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ftxsmcl",
                    description: Some(
                        "Force in TX Stop Mode the Clock Lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ftxsmdl",
                    description: Some(
                        "Force in TX Stop Mode the Data Lanes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 13,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "cdoffdl",
                    description: Some(
                        "Contention Detection OFF on Data Lanes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 14,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tddl",
                    description: Some(
                        "Turn Disable Data Lanes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "pden",
                    description: Some(
                        "Pull-Down Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tclkprepen",
                    description: Some(
                        "custom time for tCLK-PREPARE Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 19,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tclkzeroen",
                    description: Some(
                        "custom time for tCLK-ZERO Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 20,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "thsprepen",
                    description: Some(
                        "custom time for tHS-PREPARE Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 21,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "thstrailen",
                    description: Some(
                        "custom time for tHS-TRAIL Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "thszeroen",
                    description: Some(
                        "custom time for tHS-ZERO Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 23,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tlpxden",
                    description: Some(
                        "custom time for tLPX for Data lanes Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "thsexiten",
                    description: Some(
                        "custom time for tHS-EXIT Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tlpxcen",
                    description: Some(
                        "custom time for tLPX for Clock lane Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 26,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tclkposten",
                    description: Some(
                        "custom time for tCLK-POST Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 27,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wpcr1",
            extends: None,
            description: Some(
                "DSI Wrapper PHY Configuration Register 1.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "hstxdcl",
                    description: Some(
                        "High-Speed Transmission Delay on Clock Lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hstxdll",
                    description: Some(
                        "High-Speed Transmission Delay on Data Lanes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpsrcl",
                    description: Some(
                        "Low-Power transmission Slew Rate Compensation on Clock Lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 6,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lpsrdl",
                    description: Some(
                        "Low-Power transmission Slew Rate Compensation on Data Lanes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "sdcc",
                    description: Some(
                        "SDD Control.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 12,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hstxsrccl",
                    description: Some(
                        "High-Speed Transmission Slew Rate Control on Clock Lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "hstxsrcdl",
                    description: Some(
                        "High-Speed Transmission Slew Rate Control on Data Lanes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 18,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "flprxlpm",
                    description: Some(
                        "Forces LP Receiver in Low-Power Mode.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 22,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "lprxft",
                    description: Some(
                        "Low-Power RX low-pass Filtering Tuning.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 25,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wpcr2",
            extends: None,
            description: Some(
                "DSI Wrapper PHY Configuration Register 2.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tclkprep",
                    description: Some(
                        "tCLK-PREPARE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tclkzeo",
                    description: Some(
                        "tCLK-ZERO.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "thsprep",
                    description: Some(
                        "tHS-PREPARE.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "thstrail",
                    description: Some(
                        "tHSTRAIL.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wpcr3",
            extends: None,
            description: Some(
                "DSI Wrapper PHY Configuration Register 3.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "thszero",
                    description: Some(
                        "tHS-ZERO.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tlpxd",
                    description: Some(
                        "tLPX for Data lanes.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 8,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "thsexit",
                    description: Some(
                        "tHSEXIT.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "tlpxc",
                    description: Some(
                        "tLPXC for Clock lane.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wpcr4",
            extends: None,
            description: Some(
                "DSI Wrapper PHY Configuration Register 4.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "tclkpost",
                    description: Some(
                        "tCLK-POST.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 8,
                    array: None,
                    enumm: None,
                },
            ],
        },
        FieldSet {
            name: "Wrpcr",
            extends: None,
            description: Some(
                "DSI Wrapper Regulator and PLL Control Register.",
            ),
            bit_size: 32,
            fields: &[
                Field {
                    name: "pllen",
                    description: Some(
                        "PLL Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 0,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "ndiv",
                    description: Some(
                        "PLL Loop Division Factor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 2,
                        },
                    ),
                    bit_size: 7,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "idf",
                    description: Some(
                        "PLL Input Division Factor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 11,
                        },
                    ),
                    bit_size: 4,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "odf",
                    description: Some(
                        "PLL Output Division Factor.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 16,
                        },
                    ),
                    bit_size: 2,
                    array: None,
                    enumm: None,
                },
                Field {
                    name: "regen",
                    description: Some(
                        "Regulator Enable.",
                    ),
                    bit_offset: BitOffset::Regular(
                        RegularBitOffset {
                            offset: 24,
                        },
                    ),
                    bit_size: 1,
                    array: None,
                    enumm: None,
                },
            ],
        },
    ],
    enums: &[],
};
                