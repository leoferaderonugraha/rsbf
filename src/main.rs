use std::fs;

fn main() {
    let root_path = "scripts";
    let file_name = "hello.bf";
    let path = format!("{}/{}", root_path, file_name);

    let code = fs::read_to_string(path)
        .expect("Something went wrong reading the file")
        .chars()
        .collect::<Vec<char>>();

    let mut cells = vec![0u8; 30000];
    let mut cell_pointer = cells.len() / 2;

    let mut pc = 0;
    let mut ch;

    while pc < code.len() {
        ch = code[pc];
        match ch {
            '>' => {
                cell_pointer += 1;
            }
            '<' => {
                cell_pointer -= 1;
            }
            '+' => {
                cells[cell_pointer] += 1;
            }
            '-' => {
                cells[cell_pointer] -= 1;
            }
            '.' => {
                print!("{}", cells[cell_pointer] as char);
            }
            ',' => {
                todo!();
            }
            '[' => {
                if cells[cell_pointer] == 0 {
                    let mut loop_counter = 1;
                    let mut loop_index = 1;
                    while loop_counter > 0 {
                        let c = code[pc + loop_index];
                        match c {
                            '[' => {
                                loop_counter += 1;
                            }
                            ']' => {
                                loop_counter += 1;
                            }
                            _ => (),
                        }
                        loop_index += 1;
                    }
                    pc += loop_index;
                }
            }
            ']' => {
                if cells[cell_pointer] != 0 {
                    let mut loop_counter = 1;
                    let mut loop_index = 1;
                    while loop_counter > 0 {
                        let c = code[pc - loop_index];
                        match c {
                            '[' => {
                                loop_counter -= 1;
                            }
                            ']' => {
                                loop_counter += 1;
                            }
                            _ => (),
                        }
                        loop_index += 1;
                    }
                    pc -= loop_index;
                }
            }
            _ => (),
        }
        pc += 1;
    }
}
