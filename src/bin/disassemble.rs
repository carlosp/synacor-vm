use synacor_vm::{Instruction, Number, Register, VM};

const LAST_CODE_POSITION: u16 = 6089;

trait AssemblerFmt {
    fn asm(&self) -> String;
}

impl AssemblerFmt for Register {
    fn asm(&self) -> String {
        format!("r{}", self)
    }
}

impl AssemblerFmt for Number {
    fn asm(&self) -> String {
        match &self {
            Number::Literal(x) => x.to_string(),
            Number::Register(x) => x.asm()
        }
    }
}

impl AssemblerFmt for Instruction {
    fn asm(&self) -> String {
        match &self {
            Instruction::Add(a, b, c)                => format!("{} = {} + {}", a.asm(), b.asm(), c.asm()),
            Instruction::BitwiseAnd(a, b, c)         => format!("{} = {} & {}", a.asm(), b.asm(), c.asm()),
            Instruction::BitwiseNot(a, b)            => format!("{} = not {}", a.asm(), b.asm()),
            Instruction::BitwiseOr(a, b, c)          => format!("{} = {} | {}", a.asm(), b.asm(), c.asm()),
            Instruction::CompareEquals(a, b, c)      => format!("{} = {} == {}", a.asm(), b.asm(), c.asm()),
            Instruction::CompareGreaterThan(a, b, c) => format!("{} = {} > {}", a.asm(), b.asm(), c.asm()),
            Instruction::FunctionCall(a)             => format!("call {}", a.asm()),
            Instruction::FunctionReturn              => format!("ret"),
            Instruction::Halt                        => format!("halt"),
            Instruction::Jump(a)                     => format!("jmp {}", a.asm()),
            Instruction::JumpIfFalse(a, b)           => format!("jmp {} if not {}", b.asm(), a.asm()),
            Instruction::JumpIfTrue(a, b)            => format!("jmp {} if {}", b.asm(), a.asm()),
            Instruction::MemoryRead(a, b)            => format!("{} = m[{}]", a.asm(), b.asm()),
            Instruction::MemoryWrite(a, b)           => format!("m[{}] = {}", a.asm(), b.asm()),
            Instruction::Mod(a, b, c)                => format!("{} = {} % {}", a.asm(), b.asm(), c.asm()),
            Instruction::Multiply(a, b, c)           => format!("{} = {} * {}", a.asm(), b.asm(), c.asm()),
            Instruction::NoOp                        => format!("noop"),
            Instruction::Pop(a)                      => format!("pop into {}", a.asm()),
            Instruction::PrintChar(a)                => format!("write {}", a.asm()),
            Instruction::Push(a)                     => format!("push {}", a.asm()),
            Instruction::ReadChar(a)                 => format!("read into {}", a.asm()),
            Instruction::SetRegister(a, b)           => format!("{} = {}", a.asm(), b.asm()),
            Instruction::Unknown(opcode)             => format!("unknown opcode {}", opcode)
        }
    }
}


fn main() {
    let mut vm = VM::new();

    vm.load_binary("files/challenge.bin");

    while vm.dbg_get_pc() <= LAST_CODE_POSITION {
        let pc = vm.dbg_get_pc();
        let instruction = vm.next_instruction();

        println!("{}: {}", pc, instruction.asm());
    }
}
