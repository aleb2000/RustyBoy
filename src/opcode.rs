trait OpcodeTrait {
    fn name(&self) -> &str;
    fn size(&self) -> usize;
}

// TODO: remove dead_code suppression
// Opcode names in camel case look hideous, hence the warning is suppressed
#[allow(non_camel_case_types, dead_code)]
#[derive(Copy, Clone)]
enum Opcode {

    // An 'a' after a parameter name means that the value is treated as an address to a memory location (pointer)
    // n = 8-bit immediate; nn = 16-bit immediate
    // An 'r' prefix means that the opcode has been redefined as a two-byte instruction (will probably have different effect?)
    // The pattern 'XX__{HEX}__' identifies opcodes of hex value {HEX} which were removed from the CPU


    // One-byte instruction codes

    NOP         = 0x00,      // No Operation
    LD_BC_nn    = 0x01,      // Load 16-bit immediate into BC
    LD_BCa_A    = 0x02,      // Save A to address pointed by BC
    INC_BC      = 0x03,      // Increment 16-bit BC
    INC_B       = 0x04,      // Increment B
    DEC_B       = 0x05,      // Decrement B
    LD_B_n      = 0x06,      // Load 8-bit immediate into B
    RLC_A       = 0x07,      // Rotate A left with carry
    LD_nna_SP   = 0x08,      // Save SP to given address
    ADD_HL_BC   = 0x09,      // Add 16-bit BC to HL
    LD_A_BCa    = 0x0A,      // Load A from address pointed to by BC
    DEC_BC      = 0x0B,      // Decrement 16-bit BC
    INC_C       = 0x0C,      // Increment C
    DEC_C       = 0x0D,      // Decrement C
    LD_C_n      = 0x0E,      // Load 8-bit immediate into C
    RRC_A       = 0x0F,      // Rotate A right with carry

    // 10
    STOP        = 0x10,     // Stop processor
    LD_DE_nn    = 0x11,     // Load 16-bit immediate into DE
    LD_DEa_A    = 0x12,     // Save A to address pointed by DE
    INC_DE      = 0x13,     // Increments 16-bit DE
    INC_D       = 0x14,     // Increment D
    DEC_D       = 0x15,     // Decrement D
    LD_D_n      = 0x16,     // Load 8-bit immediate into D
    RL_A        = 0x17,     // Rotate A left
    JR_n        = 0x18,     // Relative jump by signed immediate
    ADD_HL_DE   = 0x19,     // Add 16-bit DE to HL
    LD_A_DEa    = 0x1A,     // Load A from address pointed to by DE
    DEC_DE      = 0x1B,     // Decrement 16-bit DE
    INC_E       = 0x1C,     // Increment E
    DEC_E       = 0x1D,     // Decrement E
    LD_E_n      = 0x1E,     // Load 8-bit immediate into E
    RR_A        = 0x1F,     // Rotate A right

    // 20
    JR_NZ_n     = 0x20,     // Relative jump by signed immediate if last result was not zero
    LD_HL_nn    = 0x21,     // Load 16-bit immediate into HL
    LDI_HLa_A   = 0x22,     // Save A to address pointed by HL, and increment HL
    INC_HL      = 0x23,     // Increment 16-bit HL
    INC_H       = 0x24,     // Increment H
    DEC_H       = 0x25,     // Decrement H
    LD_H_n      = 0x26,     // Load 8-bit immediate into H
    DAA         = 0x27,     // Adjust A for BCD addition
    JR_Z_n      = 0x28,     // Relative jump by signed immediate if last result was zero
    ADD_HL_HL   = 0x29,     // Add 16-bit HL to HL
    LDI_A_HLa   = 0x2A,     // Load A from address pointed to by HL, and increment HL
    DEC_HL      = 0x2B,     // Decrement 16-bit HL
    INC_L       = 0x2C,     // Increment L
    DEC_L       = 0x2D,     // Decrement L
    LD_L_n      = 0x2E,     // Load 8-bit immediate into L
    CPL         = 0x2F,     // Complement (logical NOT) on A

    // 30
    JR_NC_n     = 0x30,     // Relative jump by signed immediate if last result caused no carry
    LD_SP_nn    = 0x31,     // Load 16-bit immediate into SP
    LDD_HLa_A   = 0x32,     // Save A to address pointed by HL, and decrement HL
    INC_SP      = 0x33,     // Increment 16-bit SP
    INC_HLa     = 0x34,     // Increment value pointed by HL
    DEC_HLa     = 0x35,     // Decrement value pointed by HL
    LD_HLa_n    = 0x36,     // Load 8-bit immediate into address pointed by HL
    SCF         = 0x37,     // Set carry flag
    JR_C_n      = 0x38,     // Relative jump by signed immediate if last result caused carry
    ADD_HL_SP   = 0x39,     // Add 16-bit SP to HL
    LDD_A_HLa   = 0x3A,     // Load A from address pointed to by HL, and decrement HL
    DEC_SP      = 0x3B,     // Decrement 16-bit SP
    INC_A       = 0x3C,     // Increment A
    DEC_A       = 0x3D,     // Decrement A
    LD_A_n      = 0x3E,     // Load 8-bit immediate into A
    CCF         = 0x3F,     // Clear carry flag

    // 40
    LD_B_B      = 0x40,     // Copy B to B
    LD_B_C      = 0x41,     // Copy C to B
    LD_B_D      = 0x42,     // Copy D to B
    LD_B_E      = 0x43,     // Copy E to B
    LD_B_H      = 0x44,     // Copy H to B
    LD_B_L      = 0x45,     // Copy L to B
    LD_B_HLa    = 0x46,     // Copy value pointed by HL to B
    LD_B_A      = 0x47,     // Copy A to B
    LD_C_B      = 0x48,     // Copy B to C
    LD_C_C      = 0x49,     // Copy C to C
    LD_C_D      = 0x4A,     // Copy D to C
    LD_C_E      = 0x4B,     // Copy E to C
    LD_C_H      = 0x4C,     // Copy H to C
    LD_C_L      = 0x4D,     // Copy L to C
    LD_C_HLa    = 0x4E,     // Copy value pointed by HL to C
    LD_C_A      = 0x4F,     // Copy A to C

    // 50
    LD_D_B      = 0x50,     // Copy B to D
    LD_D_C      = 0x51,     // Copy C to D
    LD_D_D      = 0x52,     // Copy D to D
    LD_D_E      = 0x53,     // Copy E to D
    LD_D_H      = 0x54,     // Copy H to D
    LD_D_L      = 0x55,     // Copy L to D
    LD_D_HLa    = 0x56,     // Copy value pointed to by HL to D
    LD_D_A      = 0x57,     // Copy A to D
    LD_E_B      = 0x58,     // Copy B to E
    LD_E_C      = 0x59,     // Copy C to E
    LD_E_D      = 0x5A,     // Copy D to E
    LD_E_E      = 0x5B,     // Copy E to E
    LD_E_H      = 0x5C,     // Copy H to E
    LD_E_L      = 0x5D,     // Copy L to E
    LD_E_HLa    = 0x5E,     // Copy value pointed to by HL to E
    LD_E_A      = 0x5F,     // Copy A to E

    // 60
    LD_H_B      = 0x60,     // Copy B to H
    LD_H_C      = 0x61,     // Copy C to H
    LD_H_D      = 0x62,     // Copy D to H
    LD_H_E      = 0x63,     // Copy E to H
    LD_H_H      = 0x64,     // Copy H to H
    LD_H_L      = 0x65,     // Copy L to H
    LD_H_HLa    = 0x66,     // Copy value pointed by HL to H
    LD_H_A      = 0x67,     // Copy A to H
    LD_L_B      = 0x68,     // Copy B to L
    LD_L_C      = 0x69,     // Copy C to L
    LD_L_D      = 0x6A,     // Copy D to L
    LD_L_E      = 0x6B,     // Copy E to L
    LD_L_H      = 0x6C,     // Copy H to L
    LD_L_L      = 0x6D,     // Copy L to L
    LD_L_HLa    = 0x6E,     // Copy value pointed by HL to L
    LD_L_A      = 0x6F,     // Copy A to L

    // 70
    LD_HLa_B    = 0x70,     // Copy B to address pointed by HL
    LD_HLa_C    = 0x71,     // Copy C to address pointed by HL
    LD_HLa_D    = 0x72,     // Copy D to address pointed by HL
    LD_HLa_E    = 0x73,     // Copy E to address pointed by HL
    LD_HLa_H    = 0x74,     // Copy H to address pointed by HL
    LD_HLa_L    = 0x75,     // Copy L to address pointed by HL
    HALT        = 0x76,     // Halt processor
    LD_HLa_A    = 0x77,     // Copy A to address pointed by HL
    LD_A_B      = 0x78,     // Copy B to A
    LD_A_C      = 0x79,     // Copy C to A
    LD_A_D      = 0x7A,     // Copy D to A
    LD_A_E      = 0x7B,     // Copy E to A
    LD_A_H      = 0x7C,     // Copy H to A
    LD_A_L      = 0x7D,     // Copy L to A
    LD_A_HLa    = 0x7E,     // Copy value pointed by HL to A
    LD_A_A      = 0x7F,     // Copy A to A

    // 80
    ADD_A_B     = 0x80,     // Add B to A
    ADD_A_C     = 0x81,     // Add C to A
    ADD_A_D     = 0x82,     // Add D to A
    ADD_A_E     = 0x83,     // Add E to A
    ADD_A_H     = 0x84,     // Add H to A
    ADD_A_L     = 0x85,     // Add L to A
    ADD_A_HLa   = 0x86,     // Add value pointed by HL to A
    ADD_A_A     = 0x87,     // Add A to A
    ADC_A_B     = 0x88,     // Add B and carry flag to A
    ADC_A_C     = 0x89,     // Add C and carry flag to A
    ADC_A_D     = 0x8A,     // Add D and carry flag to A
    ADC_A_E     = 0x8B,     // Add E and carry flag to A
    ADC_A_H     = 0x8C,     // Add H and carry flag to A
    ADC_A_L     = 0x8D,     // Add L and carry flag to A
    ADC_A_HLa   = 0x8E,     // Add value pointed by HL and carry flag to A
    ADC_A_A     = 0x8F,     // Add A and carry flag to A

    // 90
    SUB_A_B     = 0x90,     // Subtract B from A
    SUB_A_C     = 0x91,     // Subtract C from A
    SUB_A_D     = 0x92,     // Subtract D from A
    SUB_A_E     = 0x93,     // Subtract E from A
    SUB_A_H     = 0x94,     // Subtract H from A
    SUB_A_L     = 0x95,     // Subtract L from A
    SUB_A_HLa   = 0x96,     // Subtract value pointed by HL from A
    SUB_A_A     = 0x97,     // Subtract A from A
    SBC_A_B     = 0x98,     // Subtract B and carry flag from A
    SBC_A_C     = 0x99,     // Subtract C and carry flag from A
    SBC_A_D     = 0x9A,     // Subtract D and carry flag from A
    SBC_A_E     = 0x9B,     // Subtract E and carry flag from A
    SBC_A_H     = 0x9C,     // Subtract H and carry flag from A
    SBC_A_L     = 0x9D,     // Subtract L and carry flag from A
    SBC_A_HLa   = 0x9E,     // Subtract value pointed by HL and carry flag from A
    SBC_A_A     = 0x9F,     // Subtract A and carry flag from A

    // A0
    AND_B       = 0xA0,     // Logical AND B against A
    AND_C       = 0xA1,     // Logical AND C against A
    AND_D       = 0xA2,     // Logical AND D against A
    AND_E       = 0xA3,     // Logical AND E against A
    AND_H       = 0xA4,     // Logical AND H against A
    AND_L       = 0xA5,     // Logical AND L against A
    AND_HLa     = 0xA6,     // Logical AND value pointed by HL against A
    AND_A       = 0xA7,     // Logical AND A against A
    XOR_B       = 0xA8,     // Logical XOR B against A
    XOR_C       = 0xA9,     // Logical XOR C against A
    XOR_D       = 0xAA,     // Logical XOR D against A
    XOR_E       = 0xAB,     // Logical XOR E against A
    XOR_H       = 0xAC,     // Logical XOR H against A
    XOR_L       = 0xAD,     // Logical XOR L against A
    XOR_HLa     = 0xAE,     // Logical XOR value pointed by HL against A
    XOR_A       = 0xAF,     // Logical XOR A against A

    // B0
    OR_B        = 0xB0,     // Logical OR B against A
    OR_C        = 0xB1,     // Logical OR C against A
    OR_D        = 0xB2,     // Logical OR D against A
    OR_E        = 0xB3,     // Logical OR E against A
    OR_H        = 0xB4,     // Logical OR H against A
    OR_L        = 0xB5,     // Logical OR L against A
    OR_HLa      = 0xB6,     // Logical OR value pointed by HL against A
    OR_A        = 0xB7,     // Logical OR A against A
    CP_B        = 0xB8,     // Compare B against A
    CP_C        = 0xB9,     // Compare C against A
    CP_D        = 0xBA,     // Compare D against A
    CP_E        = 0xBB,     // Compare E against A
    CP_H        = 0xBC,     // Compare H against A
    CP_L        = 0xBD,     // Compare L against A
    CP_HLa      = 0xBE,     // Compare value pointed by HL against A
    CP_A        = 0xBF,     // Compare A against A

    // C0
    RET_NZ      = 0xC0,     // Return if last result was not zero
    POP_BC      = 0xC1,     // Pop 16-bit value from stack into BC
    JP_NZ_nn    = 0xC2,     // Absolute jump to 16-bit location if last result was not zero
    JP_nn       = 0xC3,     // Absolute jump to 16-bit location
    CALL_NZ_nn  = 0xC4,     // Call routine at 16-bit location if last result was not zero
    PUSH_BC     = 0xC5,     // Push 16-bit BC onto stack
    ADD_A_n     = 0xC6,     // Add 8-bit immediate to A
    RST_0       = 0xC7,     // Call routine at address 0000h
    RET_Z       = 0xC8,     // Return if last result was zero
    RET         = 0xC9,     // Return to calling routine
    JP_Z_nn     = 0xCA,     // Absolute jump to 16-bit location if last result was zero
    EXT_OPS     = 0xCB,     // Extended operations (two-byte instruction code)
    CALL_Z_nn   = 0xCC,     // Call routine at 16-bit location if last result was zero
    CALL_nn     = 0xCD,     // Call routine at 16-bit location
    ADC_A_n     = 0xCE,     // Add 8-bit immediate and carry to A
    RST_8       = 0xCF,     // Call routine at address 0008h

    // D0
    RET_NC      = 0xD0,     // Return if last result caused no carry
    POP_DE      = 0xD1,     // Pop 16-bit value from stack into DE
    JP_NC_nn    = 0xD2,     // Absolute jump to 16-bit location if last result caused no carry
    XX__D3__    = 0xD3,     // Operation removed from this CPU
    CALL_NC_nn  = 0xD4,     // Call routine at 16-bit location if last result caused no carry
    PUSH_DE     = 0xD5,     // Push 16-bit DE onto stack
    SUB_A_n     = 0xD6,     // Subtract 8-bit immediate from A
    RST_10      = 0xD7,     // Call routine at address 0010h
    RET_C       = 0xD8,     // Return if last result caused carry
    RETI        = 0xD9,     // Enable interrupts and return to calling routine
    JP_C_nn     = 0xDA,     // Absolute jump to 16-bit location if last result caused carry
    XX__DB__    = 0xDB,     // Operation removed in this CPU
    CALL_C_nn   = 0xDC,     // Call routine at 16-bit location if last result caused carry
    XX__DD__    = 0xDD,     // Operation removed in this CPU
    SBC_A_n     = 0xDE,     // Subtract 8-bit immediate and carry from A
    RST_18      = 0xDF,     // Call routine at address 0018h

    // E0
    LDH_na_A    = 0xE0,     // Save A at address pointed to by (FF00h + 8-bit immediate)
    POP_HL      = 0xE1,     // Pop 16-bit value from stack into HL
    LDH_Ca_A    = 0xE2,     // Save A at address pointed to by (FF00h + C)
    XX__E3__    = 0xE3,     // Operation removed in this CPU
    XX__E4__    = 0xE4,     // Operation removed in this CPU
    PUSH_HL     = 0xE5,     // Push 16-bit HL onto stack
    AND_n       = 0xE6,     // Logical AND 8-bit immediate against A
    RST_20      = 0xE7,     // Call routine at address 0020h
    ADD_SP_d    = 0xE8,     // Add 8-bit immediate to SP
    JP_HLa      = 0xE9,     // Jump to 16-bit value pointed by HL
    LD_nna_A    = 0xEA,     // Save A at given 16-bit address
    XX__EB__    = 0xEB,     // Operation removed in this CPU
    XX__EC__    = 0xEC,     // Operation removed in this CPU
    XX__ED__    = 0xED,     // Operation removed in this CPU
    XOR_n       = 0xEE,     // Logical XOR 8-bit immediate against A
    RST_28      = 0xEF,     // Call routine at address 0028h

    // F0
    LDH_A_na    = 0xF0,     // Load A from address pointed to by (FF00h + 8-bit immediate)
    POP_AF      = 0xF1,     // Pop 16-bit value from stack into AF
    XX__F2__    = 0xF2,    // Operation removed in this CPU
    DI          = 0xF3,     // Disable interrupts
    XX__F4__    = 0xF4,     // Operation removed in this CPU
    PUSH_AF     = 0xF5,     // Push 16-bit AF onto the stack
    OR_n        = 0xF6,     // Logical Or 8-bit immediate against A
    RST_30      = 0xF7,     // Call routine at address 0030h
    LDHL_SP_d   = 0xF8,     // Add signed 8-bit immediate to SP and save result in HL
    LD_SP_HL    = 0xF9,     // Copy HL to SP
    LD_A_nna    = 0xFA,     // Load A from given 16-bit address
    EI          = 0xFB,     // Enable interrupts
    XX__FC__    = 0xFC,     // Operation removed in this CPU
    XX__FD__    = 0xFD,     // Operation removed in this CPU
    CP_n        = 0xFE,     // Compare 8-bit immediate against A
    RST_38      = 0xFF      // Call routine at address 0038h
}

// TODO: remove dead_code suppression
#[allow(non_camel_case_types, dead_code)]
#[derive(Copy, Clone)]
enum OpcodeExt {
    // Two-byte instruction codes

    // 00
    RLC_B		= 0x00,		// Rotate B left with carry
    RLC_C		= 0x01,		// Rotate C left with carry
    RLC_D		= 0x02,		// Rotate D left with carry
    RLC_E		= 0x03,		// Rotate E left with carry
    RLC_H		= 0x04,		// Rotate H left with carry
    RLC_L		= 0x05,		// Rotate L left with carry
    RLC_HLa		= 0x06,		// Rotate value pointed by HL left with carry
    rRLC_A		= 0x07,		// Rotate A left with carry
    RRC_B		= 0x08,		// Rotate B right with carry
    RRC_C		= 0x09,		// Rotate C right with carry
    RRC_D		= 0x0A,		// Rotate D right with carry
    RRC_E		= 0x0B,		// Rotate E right with carry
    RRC_H		= 0x0C,		// Rotate H right with carry
    RRC_L		= 0x0D,		// Rotate L right with carry
    RRC_HLa		= 0x0E,		// Rotate value pointed by HL right with carry
    rRRC_A		= 0x0F,		// Rotate A right with carry

    // 10
    RL_B		= 0x10,		// Rotate B left
    RL_C		= 0x11,		// Rotate C left
    RL_D		= 0x12,		// Rotate D left
    RL_E		= 0x13,		// Rotate E left
    RL_H		= 0x14,		// Rotate H left
    RL_L		= 0x15,		// Rotate L left
    RL_HLa		= 0x16,		// Rotate value pointed by HL left
    rRL_A		= 0x17,		// Rotate A left
    RR_B		= 0x18,		// Rotate B right
    RR_C		= 0x19,		// Rotate C right
    RR_D		= 0x1A,		// Rotate D right
    RR_E		= 0x1B,		// Rotate E right
    RR_H		= 0x1C,		// Rotate H right
    RR_L		= 0x1D,		// Rotate L right
    RR_HLa		= 0x1E,		// Rotate value pointed by HL right
    rRR_A		= 0x1F,		// Rotate A right

    // 20
    SLA_B		= 0x20,		// Shift B left preserving sign
    SLA_C		= 0x21,		// Shift C left preserving sign
    SLA_D		= 0x22,		// Shift D left preserving sign
    SLA_E		= 0x23,		// Shift E left preserving sign
    SLA_H		= 0x24,		// Shift H left preserving sign
    SLA_L		= 0x25,		// Shift L left preserving sign
    SLA_HLa		= 0x26,		// Shift value pointed by HL left preserving sign
    SLA_A		= 0x27,		// Shift A left preserving sign
    SRA_B		= 0x28,		// Shift B right preserving sign
    SRA_C		= 0x29,		// Shift C right preserving sign
    SRA_D		= 0x2A,		// Shift D right preserving sign
    SRA_E		= 0x2B,		// Shift E right preserving sign
    SRA_H		= 0x2C,		// Shift H right preserving sign
    SRA_L		= 0x2D,		// Shift L right preserving sign
    SRA_HLa		= 0x2E,		// Shift value pointed by HL right preserving sign
    SRA_A		= 0x2F,		// Shift A right preserving sign

    // 30
    SWAP_B		= 0x30,		// Swap nybbles in B
    SWAP_C		= 0x31,		// Swap nybbles in C
    SWAP_D		= 0x32,		// Swap nybbles in D
    SWAP_E		= 0x33,		// Swap nybbles in E
    SWAP_H		= 0x34,		// Swap nybbles in H
    SWAP_L		= 0x35,		// Swap nybbles in L
    SWAP_HLa	= 0x36,		// Swap nybbles in value pointed by HL
    SWAP_A		= 0x37,		// Swap nybbles in A
    SRL_B		= 0x38,		// Shift B right
    SRL_C		= 0x39,		// Shift C right
    SRL_D		= 0x3A,		// Shift D right
    SRL_E		= 0x3B,		// Shift E right
    SRL_H		= 0x3C,		// Shift H right
    SRL_L		= 0x3D,		// Shift L right
    SRL_HLa		= 0x3E,		// Shift value pointed by HL right
    SRL_A		= 0x3F,		// Shift A right

    // 40
    BIT_0_B		= 0x40,		// Test bit 0 of B
    BIT_0_C		= 0x41,		// Test bit 0 of C
    BIT_0_D		= 0x42,		// Test bit 0 of D
    BIT_0_E		= 0x43,		// Test bit 0 of E
    BIT_0_H		= 0x44,		// Test bit 0 of H
    BIT_0_L		= 0x45,		// Test bit 0 of L
    BIT_0_HLa	= 0x46,		// Test bit 0 of value pointed by HL
    BIT_0_A		= 0x47,		// Test bit 0 of A
    BIT_1_B		= 0x48,		// Test bit 1 of B
    BIT_1_C		= 0x49,		// Test bit 1 of C
    BIT_1_D		= 0x4A,		// Test bit 1 of D
    BIT_1_E		= 0x4B,		// Test bit 1 of E
    BIT_1_H		= 0x4C,		// Test bit 1 of H
    BIT_1_L		= 0x4D,		// Test bit 1 of L
    BIT_1_HLa	= 0x4E,		// Test bit 1 of value pointed by HL
    BIT_1_A		= 0x4F,		// Test bit 1 of A

    // 50
    BIT_2_B		= 0x50,		// Test bit 2 of B
    BIT_2_C		= 0x51,		// Test bit 2 of C
    BIT_2_D		= 0x52,		// Test bit 2 of D
    BIT_2_E		= 0x53,		// Test bit 2 of E
    BIT_2_H		= 0x54,		// Test bit 2 of H
    BIT_2_L		= 0x55,		// Test bit 2 of L
    BIT_2_HLa	= 0x56,		// Test bit 2 of value pointed by HL
    BIT_2_A		= 0x57,		// Test bit 2 of A
    BIT_3_B		= 0x58,		// Test bit 3 of B
    BIT_3_C		= 0x59,		// Test bit 3 of C
    BIT_3_D		= 0x5A,		// Test bit 3 of D
    BIT_3_E		= 0x5B,		// Test bit 3 of E
    BIT_3_H		= 0x5C,		// Test bit 3 of H
    BIT_3_L		= 0x5D,		// Test bit 3 of L
    BIT_3_HLa	= 0x5E,		// Test bit 3 of value pointed by HL
    BIT_3_A		= 0x5F,		// Test bit 3 of A

    // 60
    BIT_4_B		= 0x60,		// Test bit 4 of B
    BIT_4_C		= 0x61,		// Test bit 4 of C
    BIT_4_D		= 0x62,		// Test bit 4 of D
    BIT_4_E		= 0x63,		// Test bit 4 of E
    BIT_4_H		= 0x64,		// Test bit 4 of H
    BIT_4_L		= 0x65,		// Test bit 4 of L
    BIT_4_HLa	= 0x66,		// Test bit 4 of value pointed by HL
    BIT_4_A		= 0x67,		// Test bit 4 of A
    BIT_5_B		= 0x68,		// Test bit 5 of B
    BIT_5_C		= 0x69,		// Test bit 5 of C
    BIT_5_D		= 0x6A,		// Test bit 5 of D
    BIT_5_E		= 0x6B,		// Test bit 5 of E
    BIT_5_H		= 0x6C,		// Test bit 5 of H
    BIT_5_L		= 0x6D,		// Test bit 5 of L
    BIT_5_HLa	= 0x6E,		// Test bit 5 of value pointed by HL
    BIT_5_A		= 0x6F,		// Test bit 5 of A

    // 70
    BIT_6_B		= 0x70,		// Test bit 6 of B
    BIT_6_C		= 0x71,		// Test bit 6 of C
    BIT_6_D		= 0x72,		// Test bit 6 of D
    BIT_6_E		= 0x73,		// Test bit 6 of E
    BIT_6_H		= 0x74,		// Test bit 6 of H
    BIT_6_L		= 0x75,		// Test bit 6 of L
    BIT_6_HLa	= 0x76,		// Test bit 6 of value pointed by HL
    BIT_6_A		= 0x77,		// Test bit 6 of A
    BIT_7_B		= 0x78,		// Test bit 7 of B
    BIT_7_C		= 0x79,		// Test bit 7 of C
    BIT_7_D		= 0x7A,		// Test bit 7 of D
    BIT_7_E		= 0x7B,		// Test bit 7 of E
    BIT_7_H		= 0x7C,		// Test bit 7 of H
    BIT_7_L		= 0x7D,		// Test bit 7 of L
    BIT_7_HLa	= 0x7E,		// Test bit 7 of value pointed by HL
    BIT_7_A		= 0x7F,		// Test bit 7 of A

    // 80
    RES_0_B		= 0x80,		// Clear (reset) bit 0 of B
    RES_0_C		= 0x81,		// Clear (reset) bit 0 of C
    RES_0_D		= 0x82,		// Clear (reset) bit 0 of D
    RES_0_E		= 0x83,		// Clear (reset) bit 0 of E
    RES_0_H		= 0x84,		// Clear (reset) bit 0 of H
    RES_0_L		= 0x85,		// Clear (reset) bit 0 of L
    RES_0_HLa	= 0x86,		// Clear (reset) bit 0 of value pointed by HL
    RES_0_A		= 0x87,		// Clear (reset) bit 0 of A
    RES_1_B		= 0x88,		// Clear (reset) bit 1 of B
    RES_1_C		= 0x89,		// Clear (reset) bit 1 of C
    RES_1_D		= 0x8A,		// Clear (reset) bit 1 of D
    RES_1_E		= 0x8B,		// Clear (reset) bit 1 of E
    RES_1_H		= 0x8C,		// Clear (reset) bit 1 of H
    RES_1_L		= 0x8D,		// Clear (reset) bit 1 of L
    RES_1_HLa	= 0x8E,		// Clear (reset) bit 1 of value pointed by HL
    RES_1_A		= 0x8F,		// Clear (reset) bit 1 of A

    // 90
    RES_2_B		= 0x90,		// Clear (reset) bit 2 of B
    RES_2_C		= 0x91,		// Clear (reset) bit 2 of C
    RES_2_D		= 0x92,		// Clear (reset) bit 2 of D
    RES_2_E		= 0x93,		// Clear (reset) bit 2 of E
    RES_2_H		= 0x94,		// Clear (reset) bit 2 of H
    RES_2_L		= 0x95,		// Clear (reset) bit 2 of L
    RES_2_HLa	= 0x96,		// Clear (reset) bit 2 of value pointed by HL
    RES_2_A		= 0x97,		// Clear (reset) bit 2 of A
    RES_3_B		= 0x98,		// Clear (reset) bit 3 of B
    RES_3_C		= 0x99,		// Clear (reset) bit 3 of C
    RES_3_D		= 0x9A,		// Clear (reset) bit 3 of D
    RES_3_E		= 0x9B,		// Clear (reset) bit 3 of E
    RES_3_H		= 0x9C,		// Clear (reset) bit 3 of H
    RES_3_L		= 0x9D,		// Clear (reset) bit 3 of L
    RES_3_HLa	= 0x9E,		// Clear (reset) bit 3 of value pointed by HL
    RES_3_A		= 0x9F,		// Clear (reset) bit 3 of A

    // A0
    RES_4_B		= 0xA0,		// Clear (reset) bit 4 of B
    RES_4_C		= 0xA1,		// Clear (reset) bit 4 of C
    RES_4_D		= 0xA2,		// Clear (reset) bit 4 of D
    RES_4_E		= 0xA3,		// Clear (reset) bit 4 of E
    RES_4_H		= 0xA4,		// Clear (reset) bit 4 of H
    RES_4_L		= 0xA5,		// Clear (reset) bit 4 of L
    RES_4_HLa	= 0xA6,		// Clear (reset) bit 4 of value pointed by HL
    RES_4_A		= 0xA7,		// Clear (reset) bit 4 of A
    RES_5_B		= 0xA8,		// Clear (reset) bit 5 of B
    RES_5_C		= 0xA9,		// Clear (reset) bit 5 of C
    RES_5_D		= 0xAA,		// Clear (reset) bit 5 of D
    RES_5_E		= 0xAB,		// Clear (reset) bit 5 of E
    RES_5_H		= 0xAC,		// Clear (reset) bit 5 of H
    RES_5_L		= 0xAD,		// Clear (reset) bit 5 of L
    RES_5_HLa	= 0xAE,		// Clear (reset) bit 5 of value pointed by HL
    RES_5_A		= 0xAF,		// Clear (reset) bit 5 of A

    // B0
    RES_6_B		= 0xB0,		// Clear (reset) bit 6 of B
    RES_6_C		= 0xB1,		// Clear (reset) bit 6 of C
    RES_6_D		= 0xB2,		// Clear (reset) bit 6 of D
    RES_6_E		= 0xB3,		// Clear (reset) bit 6 of E
    RES_6_H		= 0xB4,		// Clear (reset) bit 6 of H
    RES_6_L		= 0xB5,		// Clear (reset) bit 6 of L
    RES_6_HLa	= 0xB6,		// Clear (reset) bit 6 of value pointed by HL
    RES_6_A		= 0xB7,		// Clear (reset) bit 6 of A
    RES_7_B		= 0xB8,		// Clear (reset) bit 7 of B
    RES_7_C		= 0xB9,		// Clear (reset) bit 7 of C
    RES_7_D		= 0xBA,		// Clear (reset) bit 7 of D
    RES_7_E		= 0xBB,		// Clear (reset) bit 7 of E
    RES_7_H		= 0xBC,		// Clear (reset) bit 7 of H
    RES_7_L		= 0xBD,		// Clear (reset) bit 7 of L
    RES_7_HLa	= 0xBE,		// Clear (reset) bit 7 of value pointed by HL
    RES_7_A		= 0xBF,		// Clear (reset) bit 7 of A

    // C0
    SET_0_B		= 0xC0,		// Set bit 0 of B
    SET_0_C		= 0xC1,		// Set bit 0 of C
    SET_0_D		= 0xC2,		// Set bit 0 of D
    SET_0_E		= 0xC3,		// Set bit 0 of E
    SET_0_H		= 0xC4,		// Set bit 0 of H
    SET_0_L		= 0xC5,		// Set bit 0 of L
    SET_0_HLa	= 0xC6,		// Set bit 0 of value pointed by HL
    SET_0_A		= 0xC7,		// Set bit 0 of A
    SET_1_B		= 0xC8,		// Set bit 1 of B
    SET_1_C		= 0xC9,		// Set bit 1 of C
    SET_1_D		= 0xCA,		// Set bit 1 of D
    SET_1_E		= 0xCB,		// Set bit 1 of E
    SET_1_H		= 0xCC,		// Set bit 1 of H
    SET_1_L		= 0xCD,		// Set bit 1 of L
    SET_1_HLa	= 0xCE,		// Set bit 1 of value pointed by HL
    SET_1_A		= 0xCF,		// Set bit 1 of A

    // D0
    SET_2_B		= 0xD0,		// Set bit 2 of B
    SET_2_C		= 0xD1,		// Set bit 2 of C
    SET_2_D		= 0xD2,		// Set bit 2 of D
    SET_2_E		= 0xD3,		// Set bit 2 of E
    SET_2_H		= 0xD4,		// Set bit 2 of H
    SET_2_L		= 0xD5,		// Set bit 2 of L
    SET_2_HLa	= 0xD6,		// Set bit 2 of value pointed by HL
    SET_2_A		= 0xD7,		// Set bit 2 of A
    SET_3_B		= 0xD8,		// Set bit 3 of B
    SET_3_C		= 0xD9,		// Set bit 3 of C
    SET_3_D		= 0xDA,		// Set bit 3 of D
    SET_3_E		= 0xDB,		// Set bit 3 of E
    SET_3_H		= 0xDC,		// Set bit 3 of H
    SET_3_L		= 0xDD,		// Set bit 3 of L
    SET_3_HLa	= 0xDE,		// Set bit 3 of value pointed by HL
    SET_3_A		= 0xDF,		// Set bit 3 of A

    // E0
    SET_4_B		= 0xE0,		// Set bit 4 of B
    SET_4_C		= 0xE1,		// Set bit 4 of C
    SET_4_D		= 0xE2,		// Set bit 4 of D
    SET_4_E		= 0xE3,		// Set bit 4 of E
    SET_4_H		= 0xE4,		// Set bit 4 of H
    SET_4_L		= 0xE5,		// Set bit 4 of L
    SET_4_HLa	= 0xE6,		// Set bit 4 of value pointed by HL
    SET_4_A		= 0xE7,		// Set bit 4 of A
    SET_5_B		= 0xE8,		// Set bit 5 of B
    SET_5_C		= 0xE9,		// Set bit 5 of C
    SET_5_D		= 0xEA,		// Set bit 5 of D
    SET_5_E		= 0xEB,		// Set bit 5 of E
    SET_5_H		= 0xEC,		// Set bit 5 of H
    SET_5_L		= 0xED,		// Set bit 5 of L
    SET_5_HLa	= 0xEE,		// Set bit 5 of value pointed by HL
    SET_5_A		= 0xEF,		// Set bit 5 of A

    // F0
    SET_6_B		= 0xF0,		// Set bit 6 of B
    SET_6_C		= 0xF1,		// Set bit 6 of C
    SET_6_D		= 0xF2,		// Set bit 6 of D
    SET_6_E		= 0xF3,		// Set bit 6 of E
    SET_6_H		= 0xF4,		// Set bit 6 of H
    SET_6_L		= 0xF5,		// Set bit 6 of L
    SET_6_HLa	= 0xF6,		// Set bit 6 of value pointed by HL
    SET_6_A		= 0xF7,		// Set bit 6 of A
    SET_7_B		= 0xF8,		// Set bit 7 of B
    SET_7_C		= 0xF9,		// Set bit 7 of C
    SET_7_D		= 0xFA,		// Set bit 7 of D
    SET_7_E		= 0xFB,		// Set bit 7 of E
    SET_7_H		= 0xFC,		// Set bit 7 of H
    SET_7_L		= 0xFD,		// Set bit 7 of L
    SET_7_HLa	= 0xFE,		// Set bit 7 of value pointed by HL
    SET_7_A		= 0xFF		// Set bit 7 of A
}

const OPCODE_NAMES: &'static [&'static str] = &[
//  0x0         0x1         0x2             0x3         0x4             0x5         0x6             0x7         0x8             0x9             0xA             0xB         0xC             0xD         0xE             0xF
	"NOP",  	"LD_BC_nn",	"LD_BCa_A",	    "INC_BC",	"INC_B",	    "DEC_B",	"LD_B_n",   	"RLC_A",	"LD_nna_SP",	"ADD_HL_BC",	"LD_A_BCa",	    "DEC_BC",	"INC_C",    	"DEC_C",	"LD_C_n",	    "RRC_A",
	"STOP", 	"LD_DE_nn",	"LD_DEa_A",	    "INC_DE",	"INC_D",	    "DEC_D",	"LD_D_n",   	"RL_A",	    "JR_n",	        "ADD_HL_DE",	"LD_A_DEa",	    "DEC_DE",	"INC_E",    	"DEC_E",	"LD_E_n",	    "RR_A",
	"JR_NZ_n",	"LD_HL_nn",	"LDI_HLa_A",	"INC_HL",	"INC_H",	    "DEC_H",	"LD_H_n",   	"DAA",	    "JR_Z_n",   	"ADD_HL_HL",	"LDI_A_HLa",	"DEC_HL",	"INC_L",    	"DEC_L",	"LD_L_n",	    "CPL",
	"JR_NC_n",	"LD_SP_nn",	"LDD_HLa_A",	"INC_SP",	"INC_HLa",	    "DEC_HLa",	"LD_HLa_n", 	"SCF",	    "JR_C_n",   	"ADD_HL_SP",	"LDD_A_HLa",	"DEC_SP",	"INC_A",    	"DEC_A",	"LD_A_n",	    "CCF",
	"LD_B_B",	"LD_B_C",	"LD_B_D",	    "LD_B_E",	"LD_B_H",	    "LD_B_L",	"LD_B_HLa", 	"LD_B_A",	"LD_C_B",   	"LD_C_C",   	"LD_C_D",   	"LD_C_E",	"LD_C_H",   	"LD_C_L",	"LD_C_HLa",	    "LD_C_A",
	"LD_D_B",	"LD_D_C",	"LD_D_D",	    "LD_D_E",	"LD_D_H",	    "LD_D_L",	"LD_D_HLa", 	"LD_D_A",	"LD_E_B",   	"LD_E_C",   	"LD_E_D",   	"LD_E_E",	"LD_E_H",   	"LD_E_L",	"LD_E_HLa",	    "LD_E_A",
	"LD_H_B",	"LD_H_C",	"LD_H_D",	    "LD_H_E",	"LD_H_H",	    "LD_H_L",	"LD_H_HLa", 	"LD_H_A",	"LD_L_B",   	"LD_L_C",   	"LD_L_D",   	"LD_L_E",	"LD_L_H",   	"LD_L_L",	"LD_L_HLa",	    "LD_L_A",
	"LD_HLa_B",	"LD_HLa_C",	"LD_HLa_D",	    "LD_HLa_E",	"LD_HLa_H",	    "LD_HLa_L",	"HALT",	        "LD_HLa_A",	"LD_A_B",   	"LD_A_C",   	"LD_A_D",   	"LD_A_E",	"LD_A_H",   	"LD_A_L",	"LD_A_HLa",	    "LD_A_A",
	"ADD_A_B",	"ADD_A_C",	"ADD_A_D",	    "ADD_A_E",	"ADD_A_H",	    "ADD_A_L",	"ADD_A_HLa",	"ADD_A_A",	"ADC_A_B",  	"ADC_A_C",  	"ADC_A_D",  	"ADC_A_E",	"ADC_A_H",  	"ADC_A_L",	"ADC_A_HLa",	"ADC_A_A",
	"SUB_A_B",	"SUB_A_C",	"SUB_A_D",	    "SUB_A_E",	"SUB_A_H",	    "SUB_A_L",	"SUB_A_HLa",	"SUB_A_A",	"SBC_A_B",  	"SBC_A_C",  	"SBC_A_D",  	"SBC_A_E",	"SBC_A_H",  	"SBC_A_L",	"SBC_A_HLa",	"SBC_A_A",
	"AND_B",	"AND_C",	"AND_D",	    "AND_E",	"AND_H",	    "AND_L",	"AND_HLa",  	"AND_A",	"XOR_B",    	"XOR_C",    	"XOR_D",    	"XOR_E",	"XOR_H",    	"XOR_L",	"XOR_HLa",	    "XOR_A",
	"OR_B", 	"OR_C",	    "OR_D",	        "OR_E",	    "OR_H",	        "OR_L",	    "OR_HLa",   	"OR_A",	    "CP_B",	        "CP_C", 	    "CP_D", 	    "CP_E",	    "CP_H",	        "CP_L",	    "CP_HLa",	    "CP_A",
	"RET_NZ",	"POP_BC",	"JP_NZ_nn",	    "JP_nn",	"CALL_NZ_nn",	"PUSH_BC",	"ADD_A_n",  	"RST_0",	"RET_Z",	    "RET",  	    "JP_Z_nn",  	"EXT_OPS",	"CALL_Z_nn",	"CALL_nn",	"ADC_A_n",  	"RST_8",
	"RET_NC",	"POP_DE",	"JP_NC_nn",	    "XX__D3__",	"CALL_NC_nn",	"PUSH_DE",	"SUB_A_n",  	"RST_10",	"RET_C",	    "RETI", 	    "JP_C_nn",  	"XX__DB__",	"CALL_C_nn",	"XX__DD__",	"SBC_A_n",  	"RST_18",
	"LDH_na_A",	"POP_HL",	"LDH_Ca_A",	    "XX__E3__",	"XX__E4__",	    "PUSH_HL",	"AND_n",    	"RST_20",	"ADD_SP_d",	    "JP_HLa",   	"LD_nna_A", 	"XX__EB__",	"XX__EC__",	    "XX__ED__",	"XOR_n",    	"RST_28",
	"LDH_A_na",	"POP_AF",	"XX__F2__",	    "DI",	    "XX__F4__",	    "PUSH_AF",	"OR_n",	        "RST_30",	"LDHL_SP_d",	"LD_SP_HL", 	"LD_A_nna", 	"EI",	    "XX__FC__",	    "XX__FD__",	"CP_n",	        "RST_38",
	"RLC_B",	"RLC_C",	"RLC_D",	    "RLC_E",	"RLC_H",	    "RLC_L",	"RLC_HLa",	    "rRLC_A",	"RRC_B",	    "RRC_C",    	"RRC_D",    	"RRC_E",	"RRC_H",	    "RRC_L",	"RRC_HLa",  	"rRRC_A",
	"RL_B", 	"RL_C",	    "RL_D",	        "RL_E",	    "RL_H",	        "RL_L",	    "RL_HLa",	    "rRL_A",	"RR_B",	        "RR_C", 	    "RR_D",     	"RR_E",	    "RR_H",	        "RR_L",	    "RR_HLa",   	"rRR_A",
	"SLA_B",	"SLA_C",	"SLA_D",	    "SLA_E",	"SLA_H",	    "SLA_L",	"SLA_HLa",	    "SLA_A",	"SRA_B",	    "SRA_C",    	"SRA_D",    	"SRA_E",	"SRA_H",    	"SRA_L",	"SRA_HLa",  	"SRA_A",
	"SWAP_B",	"SWAP_C",	"SWAP_D",	    "SWAP_E",	"SWAP_H",	    "SWAP_L",	"SWAP_HLa",	    "SWAP_A",	"SRL_B",	    "SRL_C",    	"SRL_D",    	"SRL_E",	"SRL_H",    	"SRL_L",	"SRL_HLa",  	"SRL_A",
	"BIT_0_B",	"BIT_0_C",	"BIT_0_D",	    "BIT_0_E",	"BIT_0_H",	    "BIT_0_L",	"BIT_0_HLa",	"BIT_0_A",	"BIT_1_B",	    "BIT_1_C",  	"BIT_1_D",  	"BIT_1_E",	"BIT_1_H",  	"BIT_1_L",	"BIT_1_HLa",	"BIT_1_A",
	"BIT_2_B",	"BIT_2_C",	"BIT_2_D",	    "BIT_2_E",	"BIT_2_H",	    "BIT_2_L",	"BIT_2_HLa",	"BIT_2_A",	"BIT_3_B",	    "BIT_3_C",  	"BIT_3_D",  	"BIT_3_E",	"BIT_3_H",  	"BIT_3_L",	"BIT_3_HLa",	"BIT_3_A",
	"BIT_4_B",	"BIT_4_C",	"BIT_4_D",	    "BIT_4_E",	"BIT_4_H",	    "BIT_4_L",	"BIT_4_HLa",	"BIT_4_A",	"BIT_5_B",	    "BIT_5_C",  	"BIT_5_D",  	"BIT_5_E",	"BIT_5_H",  	"BIT_5_L",	"BIT_5_HLa",	"BIT_5_A",
	"BIT_6_B",	"BIT_6_C",	"BIT_6_D",	    "BIT_6_E",	"BIT_6_H",  	"BIT_6_L",	"BIT_6_HLa",	"BIT_6_A",	"BIT_7_B",	    "BIT_7_C",  	"BIT_7_D",  	"BIT_7_E",	"BIT_7_H",  	"BIT_7_L",	"BIT_7_HLa",	"BIT_7_A",
	"RES_0_B",	"RES_0_C",	"RES_0_D",	    "RES_0_E",	"RES_0_H",  	"RES_0_L",	"RES_0_HLa",	"RES_0_A",	"RES_1_B",	    "RES_1_C",  	"RES_1_D",  	"RES_1_E",	"RES_1_H",  	"RES_1_L",	"RES_1_HLa",	"RES_1_A",
	"RES_2_B",	"RES_2_C",	"RES_2_D",	    "RES_2_E",	"RES_2_H",  	"RES_2_L",	"RES_2_HLa",	"RES_2_A",	"RES_3_B",	    "RES_3_C",  	"RES_3_D",  	"RES_3_E",	"RES_3_H",  	"RES_3_L",	"RES_3_HLa",	"RES_3_A",
	"RES_4_B",	"RES_4_C",	"RES_4_D",	    "RES_4_E",	"RES_4_H",  	"RES_4_L",	"RES_4_HLa",	"RES_4_A",	"RES_5_B",	    "RES_5_C",  	"RES_5_D",  	"RES_5_E",	"RES_5_H",  	"RES_5_L",	"RES_5_HLa",	"RES_5_A",
	"RES_6_B",	"RES_6_C",	"RES_6_D",	    "RES_6_E",	"RES_6_H",  	"RES_6_L",	"RES_6_HLa",	"RES_6_A",	"RES_7_B",	    "RES_7_C",  	"RES_7_D",  	"RES_7_E",	"RES_7_H",  	"RES_7_L",	"RES_7_HLa",	"RES_7_A",
	"SET_0_B",	"SET_0_C",	"SET_0_D",	    "SET_0_E",	"SET_0_H",  	"SET_0_L",	"SET_0_HLa",	"SET_0_A",	"SET_1_B",	    "SET_1_C",  	"SET_1_D",  	"SET_1_E",	"SET_1_H",  	"SET_1_L",	"SET_1_HLa",	"SET_1_A",
	"SET_2_B",	"SET_2_C",	"SET_2_D",	    "SET_2_E",	"SET_2_H",  	"SET_2_L",	"SET_2_HLa",	"SET_2_A",	"SET_3_B",	    "SET_3_C",  	"SET_3_D",  	"SET_3_E",	"SET_3_H",  	"SET_3_L",	"SET_3_HLa",	"SET_3_A",
	"SET_4_B",	"SET_4_C",	"SET_4_D",	    "SET_4_E",	"SET_4_H",  	"SET_4_L",	"SET_4_HLa",	"SET_4_A",	"SET_5_B",	    "SET_5_C",  	"SET_5_D",  	"SET_5_E",	"SET_5_H",  	"SET_5_L",	"SET_5_HLa",	"SET_5_A",
	"SET_6_B",	"SET_6_C",	"SET_6_D",	    "SET_6_E",	"SET_6_H",  	"SET_6_L",	"SET_6_HLa",	"SET_6_A",	"SET_7_B",	    "SET_7_C",  	"SET_7_D",  	"SET_7_E",	"SET_7_H",  	"SET_7_L",	"SET_7_HLa",	"SET_7_A",
];


/*
    Currently the size of instruction 0xCB (EXT_OPS) is set to 0, while all extended opcodes are set to 2 bytes.
    I am still not sure whether this is a good idea or not.
    This note is written so that I don't forget about this detail.
*/
const OPCODE_SIZES: [usize; 512] = [
//  0  1  2  3  4  5  6  7  8  9  A  B  C  D  E  F
    1, 3, 1, 1, 1, 1, 2, 1, 3, 1, 1, 1, 1, 1, 2, 1, 
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, 
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, 
    2, 3, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1, 1, 1, 2, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 
    1, 1, 3, 3, 3, 1, 2, 1, 1, 1, 3, 0, 3, 3, 2, 1, 
    1, 1, 3, 0, 3, 1, 2, 1, 1, 1, 3, 0, 3, 0, 2, 1, 
    2, 1, 1, 0, 0, 1, 2, 1, 2, 1, 3, 0, 0, 0, 2, 1, 
    2, 1, 1, 1, 0, 1, 2, 1, 2, 1, 3, 1, 0, 0, 2, 1, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 
    2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2
];

impl OpcodeTrait for Opcode {
    fn name(&self) -> &str {
        let idx = *self as usize;
        OPCODE_NAMES[idx]
    }

    fn size(&self) -> usize {
        let idx = *self as usize;
        OPCODE_SIZES[idx]
    }
}

impl OpcodeTrait for OpcodeExt {
    fn name(&self) -> &str {
        let idx = *self as usize;
        OPCODE_NAMES[0xFF + 1 + idx]
    }

    fn size(&self) -> usize {
        let idx = *self as usize;
        OPCODE_SIZES[0xFF + 1 + idx]
    }
}
