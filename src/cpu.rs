use crate::rom_file::RomFile;
use crate::memory::*;

use log::{debug, info, error, warn};

// https://github.com/ltriant/nes/blob/master/src/cpu.rs
// https://emudev.de/nes-emulator/opcodes-and-addressing-modes-the-6502/

const STACK_INIT: u8                = 0xfd;

const FLAG_CARRY: u8                = 0b00000001;
const FLAG_ZERO: u8                 = 0b00000010;
const FLAG_INTERRUPT_DISABLE: u8    = 0b00000100;
const FLAG_DECIMAL: u8              = 0b00001000;
const FLAG_B: u8                    = 0b00010000;
const FLAG_U: u8                    = 0b00100000;
const FLAG_OVERFLOW: u8             = 0b01000000;
const FLAG_NEGATIVE: u8             = 0b10000000;

pub enum Instruction {
    ALR,
    ANC,
    AND,
    ARR,
    ASL,
    AXS,
    BCC,
    BEQ,
    BIT,
    BMI,
    BPL,
    BRK,
    BVC,
    BVS,
    CLC,
    CLI,
    CMP,
    DCP,
    DEC,
    DEX,
    EOR,
    INC,
    ISC,
    JMP,
    JSR,
    LAX,
    LDX,
    LDY,
    LSR,
    NOP,
    ORA,
    PHA,
    PHP,
    PLA,
    PLP,
    RLA,
    ROL,
    ROR,
    RRA,
    RTI,
    RTS,
    SEC,
    SED,
    SEI,
    SHX,
    SHY,
    SRE,
    STA,
    STP,
    SLO,
    TAS,
    TSX,
}

pub struct Cpu {
    // region Registers
    /// Accumulator
    a: u8,
    /// X index
    x: u8,
    /// Y index
    y: u8,
    /// Program Counter
    pc: u16,
    /// Stack pointer
    s: u8,
    /// Status register
    p: u8,
    // endregion

    /// Cycles count
    cycles: u32
}

impl Cpu {
    pub fn new(mem: &Memory) -> Cpu {
        let mut cpu = Cpu {
            a: 0,
            x: 0,
            y: 0,
            pc: 0,
            s: 0,
            p: 0,
            cycles: 0
        };
        cpu.reset(mem);

        return cpu;
    }

    pub fn reset(&mut self, memory: &Memory) {
        let lo = memory.read(0xFFFC) as u16;
        let hi = memory.read(0xFFFD) as u16;
        let addr = (hi << 8) | lo;
        self.pc = addr;

        self.p = 0x24;

        self.s = STACK_INIT;
        self.a = 0;
        self.x = 0;
        self.y = 0;

        // self.interrupt = None;
        // self.stall = None;
        self.cycles = 0;
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let opcode = memory.read(self.pc);
        debug!("A: 0x{0:02x}, X: 0x{1:02x}, Y: 0x{2:02x}", self.a, self.x, self.y);
        debug!("PC: 0x{0:02x}, OpCode: 0x{1:02x}", self.pc, opcode);
        self.execute_opcode(opcode, memory);
    }

    pub fn execute_opcode(&mut self, opcode: u8, memory: &mut Memory) {
        match opcode {
            0x00 => {
                self.brk(memory);
                self.cycles += 7;
            },
            0x10 => {   // BPL REL
                self.bpl(memory.get_relative(self.pc));
                self.pc += 2;
            }
            0x18 => {
                self.clc();
                self.pc += 1;
                self.cycles += 2;
            }
            0x19 => {   // ORA ABY
                self.ora(memory,memory.get_absolute_y(self.pc, self.y));
                self.pc += 1;
            }
            0x1a => {   // NOP
                self.nop();
                self.pc += 1;
            }
            0x20 => {   // JSR ABS
                self.jsr(memory, memory.get_absolute(self.pc));
            }
            0x29 => {   // AND IMM
                self.and(memory, memory.get_zeropage(self.pc));
                self.pc += 1;
            }
            0x2d => {   // AND ABS
                self.and(memory, memory.get_absolute(self.pc));
                self.pc += 1;
            }
            0x31 => {   // AND INY
                self.and(memory, memory.get_indirect_y(self.pc, self.y));
                self.pc += 2;
            }
            0x35 => {   // AND ZPX
                self.and(memory, memory.get_zeropage_x(self.pc, self.x));
                self.pc += 1;
            }
            0x39 => {   // AND ABY
                self.and(memory, memory.get_absolute_y(self.pc, self.y));
                self.pc += 1;
            }
            0x3d => {   // AND ABX
                self.and(memory, memory.get_absolute_x(self.pc, self.x));
                self.pc += 1;
            }
            0x58 => {   // CLI IMP
                self.cli();
                self.pc += 1;
            }
            0x78 => {   // SEI IMP
                self.sei();
                self.pc += 1;
            }
            0x85 => {   // STA ZP
                self.sta(memory, memory.get_zeropage(self.pc));
                self.pc += 2;
            }
            0x86 => {   // STX ZP
                self.stx(memory, memory.get_zeropage(self.pc));
                self.pc += 2;
            }
            0x88 => {   // DEY IMP
                self.dey();
                self.pc += 1;
            }
            0x8d => {   // STA ABS
                self.sta(memory, memory.get_absolute(self.pc));
                self.pc += 3;
            }
            0x91 => {   // STA INY
                self.sta(memory, memory.get_indirect_y(self.pc, self.y));
                self.pc += 2;
            }
            0x9a => {   // TXS IMP
                self.txs();
                self.pc += 1;
            }
            0xa2 => {   // LDX IMM
                self.ldx(memory, memory.get_immediate(self.pc));
                self.pc += 2;
            }
            0xa9 => {   // LDA IMM
                self.lda(memory, memory.get_immediate(self.pc));
                self.pc += 2;
            }
            0xa0 => {   // LDY IMM
                self.ldy(memory, memory.get_immediate(self.pc));
                self.pc += 2;
            }
            0xad => {   // LDA ABS
                self.lda(memory, memory.get_absolute(self.pc));
                self.pc += 3;
                self.cycles += 4
            }
            0xb0 => {   // BCS REL
                self.bcs(memory.get_relative(self.pc));
                self.pc += 2;
            }
            0xb8 => {   // CLV IMP
                self.clv();
                self.pc += 1;
            }
            0xbd => {   // LDA ABX
                self.lda(memory, memory.get_absolute_x(self.pc, self.x));
                self.pc += 3;
            }
            0xc0 => {   // CPY IMM
                self.cpy(memory, memory.get_immediate(self.pc));
                self.pc += 2;
                self.cycles += 2;
            }
            0xc9 => {   // CMP IMM
                self.cmp(memory, memory.get_immediate(self.pc));
                self.pc += 2;
                self.cycles += 2;
            }
            0xca => {   // DEX IMP
                self.dex();
                self.pc += 1;
            }
            0xd0 => {   // BNE REL
                self.bne(memory.get_relative(self.pc));
                self.pc += 2;
            }
            0xd8 => {   // CLD IMP
                self.cld();
                self.pc += 1;
            }
            0xe0 => {   // CPX IMM
                self.cpx(memory, memory.get_immediate(self.pc));
                self.pc += 2
            }
            0xf0 => {   // BEQ REL
                self.beq(memory.get_relative(self.pc));
                self.pc += 2;
            }
            _ => println!("Unexpected OP code (0x{:02x})", opcode)
        }
    }

    fn add_branch_cycles(&mut self, address: u16) {
        self.cycles += 1;

        if (self.pc & 0xff00) != (address & 0xff00) {
            self.cycles += 1;
        }
    }

    /// Update the zero and negative flags according to the value of the register A
    fn update_sz(&mut self, val: u8) {
        self.set_status(FLAG_ZERO, val == 0);
        self.set_status(FLAG_NEGATIVE, val & 0x80 != 0);
    }

    // region Flag control
    fn get_status(&self, flag: u8) -> bool {
        self.p & flag != 0
    }

    fn set_status(&mut self, flag: u8, status: bool) {
        if status {
            self.p = self.p | flag
        } else {
            self.p = self.p & !flag
        }
    }

    fn get_carry(&self) -> u8 {
        if self.get_status(FLAG_CARRY) { 1 } else { 0 }
    }

    fn set_carry(&mut self, carry: bool) {
        self.set_status(FLAG_CARRY, carry)
    }
    // endregion

    // region Stack control
    fn stack_push8(&mut self, memory: &mut Memory, val: u8) {
        let address = 0x0100 | (self.s as u16);
        memory.write(address, val);

        let n = self.s.wrapping_sub(1);
        self.s = n;
    }

    fn stack_pop8(&mut self, memory: &Memory) -> u8 {
        let n = self.s.wrapping_add(1);
        self.s = n;

        // The stack page exists from 0x0100 to 0x01FF
        let address = 0x0100 | (self.s as u16);
        return memory.read(address);
    }

    fn stack_push16(&mut self, memory: &mut Memory, val: u16) {
        let hi = (val >> 8) as u8;
        self.stack_push8(memory, hi);

        let lo = (val & 0x00ff) as u8;
        self.stack_push8(memory, lo);
    }

    fn stack_pop16(&mut self, memory: &Memory) -> u16 {
        let lo = self.stack_pop8(memory) as u16;
        let hi = self.stack_pop8(memory) as u16;
        (hi << 8) | lo
    }
    // endregion

    // region Operations
    fn adc(&mut self, memory: &Memory, address: u16) {
        self.add(memory.read(address));
    }

    fn and(&mut self, memory: &Memory, address: u16) {
        let val = memory.read(address);
        self.a &= val;
        self.update_sz(self.a);
    }

    fn asl(&mut self, memory: &mut Memory, address: u16, addressing_mode: AddressingMode) {
        let val = match addressing_mode {
            AddressingMode::Accumulator => self.a,
            _ => memory.read(address),
        };

        self.set_carry(val & 0x80 != 0);

        let n = (val << 1) & 0xff;

        match addressing_mode {
            AddressingMode::Accumulator => { self.a = n; },
            _ => { memory.write(address, n); }
        };

        self.update_sz(n);
    }

    fn bcs(&mut self, address: u16) {
        if self.get_status(FLAG_CARRY) {
            self.add_branch_cycles( address);
            self.pc = address;
        }
    }

    fn beq(&mut self, address: u16) {
        if self.get_status(FLAG_ZERO) {
            self.add_branch_cycles(address);
            self.pc = address;
        }
    }

    fn bit(&mut self, memory: &Memory, address: u16) {
        let val = memory.read(address);
        self.set_status(FLAG_NEGATIVE, val & 0x80 != 0);
        self.set_status(FLAG_OVERFLOW, (val >> 0x06 & 0x01) == 1);
        let f = self.a & val;
        self.set_status(FLAG_ZERO, f == 0)
    }

    fn bne(&mut self, address: u16) {
        if !self.get_status(FLAG_ZERO) {
            self.add_branch_cycles(address);
            self.pc = address;
        }
    }

    fn bpl(&mut self, address: u16) {
        if !self.get_status(FLAG_NEGATIVE) {
            // self.add_branch_cycles(pc, addr);
            self.pc = address;
        }
    }

    fn brk(&mut self, memory: &mut Memory) {
        let pc = self.pc + 1;
        self.stack_push16( memory, pc);

        self.set_status(FLAG_B, true);

        let flags = self.p | 0x10;
        self.stack_push8(memory, flags);

        self.set_status(FLAG_INTERRUPT_DISABLE, true);

        let lo = memory.read(0xFFFE) as u16;
        let hi = memory.read(0xFFFF) as u16;
        self.pc = (hi << 8) | lo;
    }

    fn add(&mut self, val: u8) {
        let sum = self.a + val + self.get_carry();
        self.set_status(FLAG_OVERFLOW, (!(self.a ^ val) & (self.a ^ sum) & 0x80) > 0);
        self.a = sum;
        self.update_sz(self.a);
    }

    fn clc(&mut self) {
        self.set_status(FLAG_CARRY, false)
    }

    fn cld(&mut self) {
        self.set_status(FLAG_DECIMAL, false)
    }

    fn cli(&mut self) {
        self.set_status(FLAG_INTERRUPT_DISABLE, false)
    }

    fn clv(&mut self) {
        self.set_status(FLAG_OVERFLOW, false)
    }

    fn cmp(&mut self, memory: &Memory, address: u16) {
        let val = memory.read(address);
        let n = self.a.wrapping_sub(val);
        self.set_carry(self.a >= val);
        self.update_sz(n);
    }

    fn cpx(&mut self, memory: &Memory, address: u16) {
        let val = memory.read(address);
        let n = self.x.wrapping_sub(val);
        self.update_sz(n);
        self.set_carry(self.x >= val);
    }

    fn cpy(&mut self, memory: &Memory, address: u16) {
        let val = memory.read(address);
        let n = self.y.wrapping_sub(val);
        self.update_sz(n);
        self.set_carry(self.y >= val);
    }

    fn dex(&mut self) {
        let n = self.x.wrapping_sub(1);
        self.x = n;
        self.update_sz(n);
    }

    fn dey(&mut self) {
        let n = self.y.wrapping_sub(1);
        self.y = n;
        self.update_sz(n);
    }

    fn jsr(&mut self, memory: &mut Memory, address: u16) {
        let ret_address = self.pc;
        self.stack_push16(memory, ret_address);
        self.pc = address;
    }

    fn lda(&mut self, memory: &Memory, address: u16) {
        let val = memory.read(address);
        self.a = val;
        self.update_sz(val)
    }

    fn ldx(&mut self, memory: &Memory, address: u16) {
        let val = memory.read(address);
        self.x = val;
        self.update_sz(val)
    }

    fn ldy(&mut self, memory: &Memory, address: u16) {
        let val = memory.read(address);
        self.y = val;
        self.update_sz(val)
    }

    fn nop(&mut self) {}

    fn ora(&mut self, memory: &Memory, address: u16) {
        let val = memory.read(address);
        let na = self.a | val;
        self.a = na;
        self.update_sz(na);
    }

    fn sbc(&mut self, memory: &Memory, address: u16) {
        return self.add(!memory.read(address));
    }

    fn sec(&mut self) {
        self.set_carry(true);
    }

    fn sed(&mut self) {
        self.set_status(FLAG_DECIMAL, true);
    }

    fn sei(&mut self) {
        self.set_status(FLAG_INTERRUPT_DISABLE, true);
    }

    fn sta(&mut self, memory: &mut Memory, addr: u16) {
        memory.write(addr, self.a);
    }

    fn stx(&mut self, memory: &mut Memory, addr: u16) {
        memory.write(addr, self.x);
    }

    fn sty(&mut self, memory: &mut Memory, addr: u16) {
        memory.write(addr, self.y);
    }

    fn tax(&mut self) {
        let n = self.a;
        self.x = n;
        self.update_sz(n);
    }

    fn tay(&mut self) {
        let n = self.a;
        self.y = n;
        self.update_sz(n);
    }

    fn tsx(&mut self) {
        let s = self.s;
        self.update_sz(s);
        self.x = s;
    }

    fn txa(&mut self) {
        let n = self.x;
        self.a = n;
        self.update_sz(n);
    }

    fn txs(&mut self) {
        self.s = self.x;
    }

    fn tya(&mut self) {
        let n = self.y;
        self.a = n;
        self.update_sz(n);
    }

    // endregion
}
