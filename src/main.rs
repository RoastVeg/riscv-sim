use std::{env, fs::File, io::{self, Read}};



struct Cpu {
    regs: [u64; 32],
    pc: u64,
    dram: Vec<u8>,
}

impl Cpu {
    fn new(code: Vec<u8>) -> Self {
	let mut regs = [0; 32];
	regs[2] = 1024 * 1024 * 128;
	Self {
	    regs,
	    pc: 0,
	    dram: code,
	}
    }
    
    fn fetch(&self) -> u32 {
	let index = self.pc as usize;
	return (self.dram[index] as u32)
	    | ((self.dram[index + 1] as u32) << 8)
	    | ((self.dram[index + 2] as u32) << 16)
	    | ((self.dram[index + 3] as u32) << 24)
    }

    fn execute(&mut self, inst: u32) {
	let opcode = inst & 0x7f;
	let rd = ((inst >> 7) & 0x1f) as usize;
	let rs1 = ((inst >> 15) & 0x1f) as usize;
	match opcode {
	    0x13 => {
		// addi
		let imm = ((inst & 0xfff00000) as i32 as i64 >> 20) as u64;
		self.regs[rd] = self.regs[rs1].wrapping_add(imm);
	    }
	    0x33 => {
		// add
		let rs2 = ((inst >> 20) & 0x1f) as usize;
		self.regs[rd] = self.regs[rs1].wrapping_add(self.regs[rs2]);
	    }
	    _ => {
		panic!("not implemented");
	    }
	}
    }
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
	panic!("usage: riscv <filename>");
    }
    let mut file = File::open(&args[1])?;
    let mut code = Vec::new();
    file.read_to_end(&mut code)?;
    
    let mut cpu = Cpu::new(code);

    while cpu.pc < cpu.dram.len() as u64 {
	let inst = cpu.fetch();
	cpu.pc = cpu.pc + 4;
	cpu.execute(inst);
    }

    Ok(())
}
