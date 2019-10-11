use crate::instructions::{INSTRUCTIONS, LOCATION};

use termion::{color, style};

use std::io;

#[derive(Debug)]
pub struct Node {
    acc: i16,
    bak: i16,
    pointer: usize,
    instructions: Vec<INSTRUCTIONS>,
}

const NULL_VALUE: i16 = 0;

impl Node {
    pub fn new() -> Node {
        Node {
            acc: 0,
            bak: 0,
            pointer: 0,
            instructions: Vec::new(),
        }
    }

    fn swp(&mut self) {
        let temp_bak = self.bak;
        self.bak = self.acc;
        self.acc = temp_bak;
    }

    fn jmp(&mut self, jmp_point: usize) {
        if self.instructions.len() < jmp_point {
            println!(
                "Out of scope, cannot access {} :: {}",
                jmp_point, self.pointer
            );
        } else {
            self.pointer = jmp_point;
        }
    }

    fn jez(&mut self, jmp_point: usize) {
        if self.instructions.len() < jmp_point {
            println!(
                "Out of scope, cannot access {} :: {}",
                jmp_point, self.pointer
            );
        } else if self.acc == 0 {
            self.pointer = jmp_point
        }
    }

    fn jlz(&mut self, jmp_point: usize) {
        if self.instructions.len() < jmp_point {
            println!(
                "Out of scope, cannot access {} :: {}",
                jmp_point, self.pointer
            );
        } else if self.acc < 0 {
            self.pointer = jmp_point
        }
    }

    fn jgz(&mut self, jmp_point: usize) {
        if self.instructions.len() < jmp_point {
            println!(
                "Out of scope, cannot access {} :: {}",
                jmp_point, self.pointer
            );
        } else if self.acc > 0 {
            self.pointer = jmp_point
        }
    }

    fn add(acc: &mut i16, value: &i16) {
        if *acc + value >= std::i16::MAX {
            println!("Overflow");
        } else {
            *acc += value;
        }
    }

    fn mov(src: i16, dest: &mut i16) {
        *dest = src;
    }
    // Parse and processing

    pub fn load(&mut self, raw_instructions: String) {
        for line in raw_instructions.lines() {
            let mut frag = line.split_whitespace();
            match frag.nth(0).unwrap() {
                "JMP" => match frag.next() {
                    Some(jmp_point) => {
                        let parsed_point = jmp_point.parse::<usize>();
                        match parsed_point {
                            Ok(point) => self.instructions.push(INSTRUCTIONS::JMP(point)),
                            Err(err) => println!("Jump point could not be reached"),
                        }
                    }
                    None => println!("No Jump Destination has been provided"),
                },
                "MOV" => match frag.next() {
                    Some(src) => {
                        let source = match src {
                            "ACC" => LOCATION::ACC,
                            "DISPLAY" => LOCATION::DISPLAY,
                            // ...
                            _ => match src.parse::<i16>() {
                                Ok(value) => LOCATION::VALUE(value),
                                Err(err) => {
                                    panic!("Error: Could not parse value source");
                                }
                            },
                        };

                        let dest = match frag.next() {
                            Some(dest) => match dest {
                                "ACC" => LOCATION::ACC,
                                "DISPLAY" => LOCATION::DISPLAY,
                                _ => {
                                    panic!("Error: Could not parse value SOURCE");
                                }
                            },
                            None => panic!("Error: Could not find MOV Dest"),
                        };

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
                    None => println!("No value has been provided"),
                },
                _ => println!("UNKNOWN INSTRUCTION"),
            }
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
                    },
                    INSTRUCTIONS::JMP(pointer) => {
                        self.pointer = *pointer;
                    }
                    _ => println!("PANIC ! Unknown Instruction"),
                }
            }
            None => println!("Out of scope"),
        }
    }

    pub fn run(&mut self) {
        for i in 0..10 {
            println!("{}{:?}", color::Fg(color::Red), self.instructions.get(self.pointer).unwrap());
            println!("{}{:?}", color::Fg(color::Green), self);
            print!("{}", color::Fg(color::White));
            self.process();
        }
    }
}
