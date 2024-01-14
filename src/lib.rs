use std::{collections::{HashSet, VecDeque}, fs};

const MAX_SIZE: usize = 32768;

pub type Literal = u16;
pub type Register = usize;

pub enum Number {
    Literal(Literal),
    Register(Register)
}

impl Number {
    pub fn from(value: u16) -> Self {
        match value {
            0 ..= 32767     => Number::Literal(value),
            32768 ..= 32775 => Number::Register(value as usize - 32768),
            _               => panic!("Invalid number {}", value)
        }
    }
}

pub enum Instruction {
    Add(Register, Number, Number),
    BitwiseAnd(Register, Number, Number),
    BitwiseNot(Register, Number),
    BitwiseOr(Register, Number, Number),
    FunctionCall(Number),
    FunctionReturn,
    CompareEquals(Register, Number, Number),
    CompareGreaterThan(Register, Number, Number),
    Halt,
    Jump(Number),
    JumpIfFalse(Number, Number),
    JumpIfTrue(Number, Number),
    MemoryRead(Register, Number),
    MemoryWrite(Number, Number),
    Mod(Register, Number, Number),
    Multiply(Register, Number, Number),
    NoOp,
    Pop(Register),
    PrintChar(Number),
    Push(Number),
    ReadChar(Register),
    SetRegister(Register, Number),
    Unknown(Literal)
}

pub struct VM {
    pc: u16,
    breakpoints: HashSet<u16>,
    input_buf: VecDeque<u16>,
    interrupted: bool,
    output_enabled: bool,
    memory: [u16; MAX_SIZE],
    registers: [u16; 8],
    stack: Vec<u16>
}

impl VM {
    pub fn new() -> Self {
        Self {
            pc: 0,
            breakpoints: HashSet::new(),
            input_buf: VecDeque::new(),
            interrupted: false,
            output_enabled: true,
            memory: [0; MAX_SIZE],
            registers: [0; 8],
            stack: vec![]
        }
    }

    pub fn dbg_add_breakpoint(&mut self, position: u16) {
        self.breakpoints.insert(position);
    }

    pub fn dbg_get_memory(&self) -> &[u16] {
        &self.memory
    }

    pub fn dbg_get_pc(&self) -> u16 {
        self.pc
    }

    pub fn dbg_set_memory(&mut self, position: usize, value: u16) {
        self.memory[position] = value;
    }

    pub fn dbg_set_output_enabled(&mut self, enabled: bool) {
        self.output_enabled = enabled;
    }

    pub fn dbg_set_register(&mut self, register: usize, value: u16) {
        self.registers[register] = value;
    }

    pub fn load_binary(&mut self, file_path: &str) {
        fs::read(file_path)
            .unwrap_or_else(|_| panic!("{} should be a readable file", file_path))
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .enumerate()
            .for_each(|(idx, value)| self.memory[idx] = value);
    }

    pub fn input_command(&mut self, command: &str) {
        self.input_buf.extend(command.chars().map(|c| c as u16));
    }

    pub fn next_instruction(&mut self) -> Instruction {
        match self.next_word() {
            0  => Instruction::Halt,
            1  => Instruction::SetRegister       (self.next_register(), self.next_number()),
            2  => Instruction::Push              (self.next_number()),
            3  => Instruction::Pop               (self.next_register()),
            4  => Instruction::CompareEquals     (self.next_register(), self.next_number(), self.next_number()),
            5  => Instruction::CompareGreaterThan(self.next_register(), self.next_number(), self.next_number()),
            6  => Instruction::Jump              (self.next_number()),
            7  => Instruction::JumpIfTrue        (self.next_number(),   self.next_number()),
            8  => Instruction::JumpIfFalse       (self.next_number(),   self.next_number()),
            9  => Instruction::Add               (self.next_register(), self.next_number(), self.next_number()),
            10 => Instruction::Multiply          (self.next_register(), self.next_number(), self.next_number()),
            11 => Instruction::Mod               (self.next_register(), self.next_number(), self.next_number()),
            12 => Instruction::BitwiseAnd        (self.next_register(), self.next_number(), self.next_number()),
            13 => Instruction::BitwiseOr         (self.next_register(), self.next_number(), self.next_number()),
            14 => Instruction::BitwiseNot        (self.next_register(), self.next_number()),
            15 => Instruction::MemoryRead        (self.next_register(), self.next_number()),
            16 => Instruction::MemoryWrite       (self.next_number(),   self.next_number()),
            17 => Instruction::FunctionCall      (self.next_number()),
            18 => Instruction::FunctionReturn,
            19 => Instruction::PrintChar         (self.next_number()),
            20 => Instruction::ReadChar          (self.next_register()),
            21 => Instruction::NoOp,
            op => Instruction::Unknown(op)
        }
    }

    pub fn run(&mut self) {
        self.interrupted = false;

        while !self.interrupted {
            match &self.next_instruction() {
                Instruction::Add(a, b, c)                => self.perform_add(a, b, c),
                Instruction::BitwiseAnd(a, b, c)         => self.perform_bitwise_and(a, b, c),
                Instruction::BitwiseNot(a, b)            => self.perform_bitwise_not(a, b),
                Instruction::BitwiseOr(a, b, c)          => self.perform_bitwise_or(a, b, c),
                Instruction::CompareEquals(a, b, c)      => self.perform_compare_equals(a, b, c),
                Instruction::CompareGreaterThan(a, b, c) => self.perform_compare_greater_than(a, b, c),
                Instruction::FunctionCall(a)             => self.perform_function_call(a),
                Instruction::FunctionReturn              => self.perform_function_return(),
                Instruction::Halt                        => break,
                Instruction::Jump(a)                     => self.perform_jump(a),
                Instruction::JumpIfFalse(a, b)           => self.perform_jump_if_false(a, b),
                Instruction::JumpIfTrue(a, b)            => self.perform_jump_if_true(a, b),
                Instruction::MemoryRead(a, b)            => self.perform_memory_read(a, b),
                Instruction::MemoryWrite(a, b)           => self.perform_memory_write(a, b),
                Instruction::Mod(a, b, c)                => self.perform_mod(a, b, c),
                Instruction::Multiply(a, b, c)           => self.perform_multiply(a, b, c),
                Instruction::NoOp                        => (),
                Instruction::Pop(a)                      => self.perform_pop(a),
                Instruction::PrintChar(a)                => self.perform_print_char(a),
                Instruction::Push(a)                     => self.perform_push(a),
                Instruction::ReadChar(a)                 => self.perform_read_char(a),
                Instruction::SetRegister(a, b)           => self.perform_set_register(a, b),
                Instruction::Unknown(opcode)             => panic!("Invalid opcode {}", opcode)
            }

            self.interrupted |= self.breakpoints.contains(&self.pc);
        }
    }

    fn next_number(&mut self) -> Number {
        Number::from(self.next_word())
    }

    fn next_register(&mut self) -> Register {
        Register::from(self.next_word() - 32768)
    }

    fn next_word(&mut self) -> u16 {
        let word = self.memory[self.pc as usize];

        self.pc += 1;
        word
    }

    fn perform_add(&mut self, a: &Register, b: &Number, c: &Number) {
        self.registers[*a] = (self.resolve_number(b) + self.resolve_number(c)) % MAX_SIZE as u16;
    }

    fn perform_bitwise_and(&mut self, a: &Register, b: &Number, c: &Number) {
        self.registers[*a] = self.resolve_number(b) & self.resolve_number(c);
    }

    fn perform_bitwise_not(&mut self, a: &Register, b: &Number) {
        self.registers[*a] = (!self.resolve_number(b)) % MAX_SIZE as u16;
    }

    fn perform_bitwise_or(&mut self, a: &Register, b: &Number, c: &Number) {
        self.registers[*a] = self.resolve_number(b) | self.resolve_number(c);
    }

    fn perform_compare_equals(&mut self, a: &Register, b: &Number, c: &Number) {
        self.registers[*a] = (self.resolve_number(b) == self.resolve_number(c)).into()
    }

    fn perform_compare_greater_than(&mut self, a: &Register, b: &Number, c: &Number) {
        self.registers[*a] = (self.resolve_number(b) > self.resolve_number(c)).into()
    }

    fn perform_function_call(&mut self, a: &Number) {
        self.stack.push(self.pc);
        self.perform_jump(a);
    }

    fn perform_function_return(&mut self) {
        self.pc = self.stack.pop().unwrap();
    }

    fn perform_jump(&mut self, a: &Number) {
        self.pc = self.resolve_number(a);
    }

    fn perform_jump_if_false(&mut self, a: &Number, b: &Number) {
        if self.resolve_number(a) == 0 {
            self.perform_jump(b)
        }
    }

    fn perform_jump_if_true(&mut self, a: &Number, b: &Number) {
        if self.resolve_number(a) > 0 {
            self.perform_jump(b)
        }
    }

    fn perform_memory_read(&mut self, a: &Register, b: &Number) {
        self.registers[*a] = self.memory[self.resolve_number(b) as usize];
    }

    fn perform_memory_write(&mut self, a: &Number, b: &Number) {
        self.memory[self.resolve_number(a) as usize] = self.resolve_number(b);
    }

    fn perform_mod(&mut self, a: &Register, b: &Number, c: &Number) {
        self.registers[*a] = self.resolve_number(b) % self.resolve_number(c);
    }

    fn perform_multiply(&mut self, a: &Register, b: &Number, c: &Number) {
        self.registers[*a] = (self.resolve_number(b) * self.resolve_number(c)) % MAX_SIZE as u16;
    }

    fn perform_pop(&mut self, a: &Register) {
        self.registers[*a] = self.stack.pop().unwrap();
    }

    fn perform_print_char(&self, a: &Number) {
        if self.output_enabled {
            print!("{}", self.resolve_number(a) as u8 as char);
        }
    }

    fn perform_push(&mut self, a: &Number) {
        self.stack.push(self.resolve_number(a));
    }

    fn perform_read_char(&mut self, a: &Register) {
        match self.input_buf.pop_front() {
            None => {
                self.interrupted = true;
                self.pc -= 2;
            }
            Some(c) => {
                self.registers[*a] = c;
            }
        }
    }

    fn perform_set_register(&mut self, a: &Register, b: &Number) {
        self.registers[*a] = self.resolve_number(b)
    }

    fn resolve_number(&self, number: &Number) -> Literal {
        match number {
            Number::Literal(x)  => *x,
            Number::Register(x) => self.registers[*x]
        }
    }
}
