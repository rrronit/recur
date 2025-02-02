const STACK_CAPACITY: usize = 1024;

#[derive(Clone, Copy)]
struct Word {
    value: i32,
}
struct VM {
    data: [Word; STACK_CAPACITY],
    size: usize,

    program: Vec<Instruction>,
    instruction_pointer: Word,
    is_halted: bool,
}

impl VM {
    fn new() -> VM {
        VM {
            data: [Word { value: 0 }; STACK_CAPACITY],
            instruction_pointer: Word { value: 0 },
            program: vec![],
            size: 0,
            is_halted: false,
        }
    }

    fn push(&mut self, word: Word) -> Option<()> {
        //check if stack is full
        if self.size == STACK_CAPACITY {
            return None;
        }
        self.data[self.size] = word;
        self.size += 1;
        Some(())
    }

    fn pop(&mut self) -> Option<Word> {
        //check if stack is empty
        if self.size == 0 {
            return None;
        }
        self.size -= 1;
        Some(self.data[self.size])
    }

    fn execute(&mut self) -> Trap {
        if self.instruction_pointer.value as usize >= self.program.len() {
            return Trap::TrapIllegalAccess;
        }

        let ip = self.instruction_pointer.value as usize;
        let inst = self.program[ip];
        match inst.inst_type {
            InstructionType::Push => {
                self.instruction_pointer.value += 1;
                if self.push(inst.operand).is_none() {
                    return Trap::TrapStackOverflow;
                }
                Trap::NoTrap
            }
            InstructionType::Plus => {
                self.instruction_pointer.value += 1;

                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(a), Some(b)) => {
                        let result = Word {
                            value: a.value + b.value,
                        };
                        if self.push(result).is_none() {
                            return Trap::TrapStackOverflow;
                        }
                        Trap::NoTrap
                    }
                    (None, _) => Trap::TrapStackUnderflow,
                    (_, None) => Trap::TrapStackUnderflow,
                }
            }
            InstructionType::Pop => {
                self.instruction_pointer.value += 1;
                if self.pop().is_none() {
                    return Trap::TrapStackUnderflow;
                }
                Trap::NoTrap
            }

            InstructionType::Dup => {
                self.instruction_pointer.value += 1;
                let value = inst.operand;

                if (value.value as usize) >= self.size {
                    return Trap::TrapIllegalAccess;
                }
                if self
                    .push(self.data[self.size - 1 - value.value as usize])
                    .is_none()
                {
                    return Trap::TrapStackOverflow;
                }

                Trap::NoTrap
            }

            InstructionType::Minus => {
                self.instruction_pointer.value += 1;

                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(a), Some(b)) => {
                        let result = Word {
                            value: a.value - b.value,
                        };
                        if self.push(result).is_none() {
                            return Trap::TrapStackOverflow;
                        }
                        Trap::NoTrap
                    }
                    (None, _) => Trap::TrapStackUnderflow,
                    (_, None) => Trap::TrapStackUnderflow,
                }
            }
            InstructionType::Mult => {
                self.instruction_pointer.value += 1;

                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(a), Some(b)) => {
                        let result = Word {
                            value: a.value * b.value,
                        };
                        if self.push(result).is_none() {
                            return Trap::TrapStackOverflow;
                        }
                        Trap::NoTrap
                    }
                    (None, _) => Trap::TrapStackUnderflow,
                    (_, None) => Trap::TrapStackUnderflow,
                }
            }
            InstructionType::Div => {
                self.instruction_pointer.value += 1;

                let a = self.pop();
                let b = self.pop();
                match (a, b) {
                    (Some(a), Some(b)) => {
                        if b.value == 0 {
                            return Trap::TrapDivisionByZero;
                        }

                        let result = Word {
                            value: a.value / b.value,
                        };
                        if self.push(result).is_none() {
                            return Trap::TrapStackOverflow;
                        }
                        Trap::NoTrap
                    }
                    (None, _) => Trap::TrapStackUnderflow,
                    (_, None) => Trap::TrapStackUnderflow,
                }
            }

            InstructionType::JMP => {
                self.instruction_pointer = inst.operand;
                Trap::NoTrap
            }

            InstructionType::JMP_IF => {
                let condition = self.pop();
                match condition {
                    Some(condition) => {
                        if condition.value != 0 {
                            self.instruction_pointer = inst.operand;
                        } else {
                            self.instruction_pointer.value += 1;
                        }
                        Trap::NoTrap
                    }
                    None => Trap::TrapStackUnderflow,
                }
            }

            InstructionType::JMP_EQ => {
                if self.size < 2 {
                    return Trap::TrapStackUnderflow;
                }
                let a = self.data[self.size - 1];
                let b = self.data[self.size - 2];

                if a.value == b.value {
                    self.instruction_pointer = inst.operand;
                } else {
                    self.instruction_pointer.value += 1;
                }
                self.pop();
                Trap::NoTrap
            }

            InstructionType::Halt => {
                self.is_halted = true;
                Trap::NoTrap
            }
        }
    }

    fn dump(&self) {
        println!("Stack dump");
        match self.size {
            0 => println!("Empty"),
            _ => {
                for i in 0..self.size {
                    println!("{}: {}", i, self.data[i].value);
                }
            }
        }
        println!();
    }
}

#[derive(Clone, Copy)]
enum InstructionType {
    Push,
    Pop,
    Dup,
    Plus,
    Minus,
    Mult,
    Div,
    JMP,
    JMP_IF,
    JMP_EQ,
    Halt,
}

#[derive(Clone, Copy)]
struct Instruction {
    inst_type: InstructionType,
    operand: Word,
}

impl Instruction {
    fn new(inst_type: InstructionType, operand: Word) -> Instruction {
        Instruction { inst_type, operand }
    }
}

enum Trap {
    TrapStackOverflow,
    TrapStackUnderflow,
    NoTrap,
    TrapDivisionByZero,
    TrapIllegalAccess,
}

fn main() {
    let mut vm = VM::new();
    let insts = vec![
        Instruction::new(InstructionType::Push, Word { value: 0 }), // a
        Instruction::new(InstructionType::Push, Word { value: 1 }), // b
        Instruction::new(InstructionType::Dup, Word { value: 1 }),  // a
        Instruction::new(InstructionType::Dup, Word { value: 1 }),  // b
        Instruction::new(InstructionType::Plus, Word { value: 0 }), // a
        Instruction::new(InstructionType::JMP, Word { value: 2 }), // b
    ];


    


   
    vm.program = insts;
    let mut number_of_inst_to_execute: u128 = 69;

    while !vm.is_halted && number_of_inst_to_execute != 0 {
        number_of_inst_to_execute -= 1;
        let trap = vm.execute();
        match trap {
            Trap::TrapStackOverflow => {
                println!("Stack overflow");
                break;
            }
            Trap::TrapStackUnderflow => {
                println!("Stack underflow");
                break;
            }
            Trap::TrapDivisionByZero => {
                println!("Division by zero");
                break;
            }
            Trap::TrapIllegalAccess => {
                println!("Illegal access");
                break;
            }
            Trap::NoTrap => (),
        }
        vm.dump();
    }
}
