use std::io::stdin;
use synacor_vm::VM;

fn main() {
    let mut vm = VM::new();

    vm.load_binary("files/challenge.bin");

    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).ok();

        match input.split_whitespace().collect::<Vec<&str>>()[..] {
            ["$", "add_breakpoint", position]      => vm.dbg_add_breakpoint(position.parse().unwrap()),
            ["$", "set_memory", position, value]   => vm.dbg_set_memory(position.parse().unwrap(), value.parse().unwrap()),
            ["$", "set_register", register, value] => vm.dbg_set_register(register.parse().unwrap(), value.parse().unwrap()),
            ["$", "continue"]                      => vm.run(),
            ["$", "exit"]                          => return,
            _ => {
                vm.input_command(&input);
                vm.run();
            }
        }
    }
}
