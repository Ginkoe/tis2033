use crate::instructions::{INSTRUCTIONS, LOCATION};
use std::collections::HashMap;

use termion::{color, style};

use std::io;

pub struct Node {
    acc: i16,
    bak: i16,
    pointer: usize,
    instructions: Vec<INSTRUCTIONS>,
    memory: [i16; 256],
    label_map: HashMap<String, usize>,
}

const NULL_VALUE: i16 = 0;

impl Node {
    pub fn new() -> Node {
        Node {
            acc: 0,
            bak: 0,
            pointer: 0,
            instructions: Vec::new(),
            memory: [0; 256],
            label_map: HashMap::new(),
        }
    }

    fn jmp(label_map: &HashMap<String, usize>, instructions: &Vec<INSTRUCTIONS>, label: &String, pointer: &mut usize) {
        let jump_point = match label_map.get(label) {
            Some(jp) => jp.clone(),
            None => panic!("Error: LABEL NOT FOUND")
        };

        if jump_point > instructions.len() - 1 {
            panic!("Error: OUT OF SCOPE")
        } else {
            *pointer = jump_point;
        }
    }

    fn swp(&mut self) {
        let temp_bak = self.bak;
        self.bak = self.acc;
        self.acc = temp_bak;
    }

    fn add(acc: &mut i16, value: &i16) {
        if *acc + value >= std::i16::MAX {
            println!("Overflow");
        } else {
            *acc += value;
        }
    }

    fn sub(acc: &mut i16, value: &i16) {
        if *acc - value <= std::i16::MIN {
            println!("Overflow");
        } else {
            *acc -= value;
        }
    }

    fn mov(src: i16, dest: &mut i16) {
        *dest = src;
    }
    // Parse and processing

    pub fn load(&mut self, raw_instructions: String) {
        let mut instruction_pos = 0;
        for line in raw_instructions.lines() {
            if &line[line.len() - 1..] == ":" {
                // TODO : ignoring whitespaces
                if line.contains(" ") {
                    panic!("Error : Label syntax wrong");
                }

                self.label_map
                    .insert(String::from(&line[0..line.len()-1]), instruction_pos);
                self.instructions.push(INSTRUCTIONS::IGNORE);
            } else {
                let mut frag = line.split_whitespace();
                match frag.nth(0).unwrap() {
                    "JMP" => match frag.next() {
                        Some(jmp_point) => {
                            if jmp_point.contains(" ") {
                                panic!("Error: Invalid Jump Space")
                            }

                            self.instructions.push(INSTRUCTIONS::JMP(String::from(jmp_point)));
                        }
                        None => panic!("Error: No Jump Destination has been provided"),
                    },
                    "JEZ" => match frag.next() {
                        Some(jmp_point) => {
                            if jmp_point.contains(" ") {
                                panic!("Error: Invalid Jump Space")
                            }

                            self.instructions.push(INSTRUCTIONS::JEZ(String::from(jmp_point)));
                        }
                        None => panic!("Error: No Jump Destination has been provided"),
                    },
                    "JGZ" => match frag.next() {
                        Some(jmp_point) => {
                            if jmp_point.contains(" ") {
                                panic!("Error: Invalid Jump Space")
                            }

                            self.instructions.push(INSTRUCTIONS::JGZ(String::from(jmp_point)));
                        }
                        None => panic!("Error: No Jump Destination has been provided"),
                    },
                    "JLZ" => match frag.next() {
                        Some(jmp_point) => {
                            if jmp_point.contains(" ") {
                                panic!("Error: Invalid Jump Space")
                            }

                            self.instructions.push(INSTRUCTIONS::JLZ(String::from(jmp_point)));
                        }
                        None => panic!("Error: No Jump Destination has been provided"),
                    },

                    "MOV" => match frag.next() {
                        Some(src) => {
                            let source = match src {
                                "ACC" => LOCATION::ACC,
                                "DISPLAY" => LOCATION::DISPLAY,
                                // ...
                                _ => match src.parse::<i16>() {
                                    Ok(value) => LOCATION::VALUE(value),
                                    Err(_) => {
                                        if &src[0..1] == "$" {
                                            match &src[1..].parse::<u8>() {
                                                Ok(value) => LOCATION::REGISTER(*value),
                                                Err(_) => panic!("Error: Unknown Register"),
                                            }
                                        } else {
                                            panic!("Error: Could not parse value source");
                                        }
                                    }
                                },
                            };

                            let dest = match frag.next() {
                                Some(raw_dest) => match raw_dest {
                                    "ACC" => LOCATION::ACC,
                                    "DISPLAY" => LOCATION::DISPLAY,
                                    _ => match raw_dest.parse::<i16>() {
                                        Ok(_) => panic!("Error: Cannot pass value to value"),
                                        Err(_) => {
                                            if &raw_dest[0..1] == "$" {
                                                println!("{}", &raw_dest[1..]);
                                                match &raw_dest[1..].parse::<u8>() {
                                                    Ok(value) => LOCATION::REGISTER(*value),
                                                    Err(_) => panic!("Error: Unknown register"),
                                                }
                                            } else {
                                                panic!("Error: Could not parse DEST")
                                            }
                                        }
                                    },
                                },
                                None => panic!("Error: Could not find MOV Dest"),
                            };
                            if source == dest {
                                panic!("Error: MOV from to same LOCATION")
                            }
                            self.instructions.push(INSTRUCTIONS::MOV(source, dest))
                        }

                        None => println!("MOV Source not found"),
                    },
                    "ADD" => match frag.next() {
                        Some(raw_value) => {
                            let parsed_value = raw_value.parse::<i16>();
                            match parsed_value {
                                Ok(value) => {
                                    self.instructions.push(INSTRUCTIONS::ADD(value));
                                }
                                Err(err) => println!("Incorect add value"),
                            }
                        }
                        None => panic!("No value has been provided"),
                    },
                    "SUB" => match frag.next() {
                        Some(raw_value) => {
                            let parsed_value = raw_value.parse::<i16>();
                            match parsed_value {
                                Ok(value) => {
                                    self.instructions.push(INSTRUCTIONS::SUB(value));
                                }
                                Err(err) => println!("Incorect add value"),
                            }
                        }
                        None => panic!("No value has been provided"),
                    },
                    _ => println!("UNKNOWN INSTRUCTION"),
                }
            }

            instruction_pos += 1;
        }
    }

    fn process(&mut self) {
        match &self.instructions.get(self.pointer) {
            Some(instruction) => {
                if self.pointer >= self.instructions.len() - 1 {
                    self.pointer = 0;
                } else {
                    self.pointer += 1;
                }
                match instruction {
                    INSTRUCTIONS::ADD(data) => Node::add(&mut self.acc, data),
                    INSTRUCTIONS::SUB(data) => Node::sub(&mut self.acc, data),
                    INSTRUCTIONS::MOV(src, dest) => {
                        let src_ref: i16;
                        src_ref = match src {
                            LOCATION::ACC => self.acc,
                            LOCATION::VALUE(v) => *v,
                            _ => NULL_VALUE,
                        };

                        match dest {
                            LOCATION::DISPLAY => println!("{}", src_ref),
                            LOCATION::ACC => Node::mov(src_ref, &mut self.acc),
                            _ => {}
                        }
                    }
                    INSTRUCTIONS::JMP(label) => {
                        Node::jmp(&self.label_map,&self.instructions, label, &mut self.pointer)
                    }
                    INSTRUCTIONS::JEZ(label) => {
                        if self.acc == 0 {
                            Node::jmp(&self.label_map,&self.instructions, label, &mut self.pointer)
                        }
                    }
                    INSTRUCTIONS::JGZ(label) => {
                        if self.acc > 0 {
                            Node::jmp(&self.label_map,&self.instructions, label, &mut self.pointer)                            
                        }
                    }
                    INSTRUCTIONS::JLZ(label) => {
                        if self.acc < 0 {
                            Node::jmp(&self.label_map,&self.instructions, label, &mut self.pointer)
                        }
                    },
                    INSTRUCTIONS::IGNORE => {
                        // Ignore
                    }
                    _ => println!("PANIC ! Unknown Instruction"),
                }
            }
            None => println!("Out of scope"),
        }
    }

    pub fn run(&mut self) {
        println!("{:?}", self.label_map);
        loop {
            self.process();
            std::thread::sleep(std::time::Duration::from_millis(200))
        }
    }
}
