use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Clone, PartialEq)]
pub enum PtrType {
    Immediate(i32),
    Register(i32),
    Pointer(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum RefPtrType {
    Register(i32),
    Pointer(i32),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Instruction {
    Load(PtrType),
    Store(RefPtrType),
    Add(PtrType),
    Sub(PtrType),
    Div(PtrType),
    Mul(PtrType),
    Goto(String),
    JumpIfZero(String),
    JumpIfNotZero(String),
    End(),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub line: String,
    pub instruction: Option<Instruction>,
    pub line_number: u32,
    pub file_name: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VirtualMachine {
    pub memory: Vec<u32>,
    pub accumulator: u32,
    pub lines: Vec<Line>,
    pub line_ptr: u32,
    pub defines: HashMap<String, String>,
    pub labels: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CompileError {
    InvalidInstruction,
    ParamError{
        file: String,
        line: u32
    },
    LabelError{
        file: String,
        line: u32
    },
}

impl VirtualMachine {
    pub fn new() -> VirtualMachine {
        VirtualMachine {
            memory: vec![0; 0],
            accumulator: 0,
            lines: Vec::new(),
            line_ptr: 0,
            defines: HashMap::new(),
            labels: Vec::new(),
        }
    }

    pub fn reuse(&mut self) {
        self.memory = vec![0; 0];
        self.accumulator = 0;
        self.lines = Vec::new();
        self.line_ptr = 0;
        self.defines = HashMap::new();
        self.labels = Vec::new();
    }

    fn resize_memory(&mut self, size: u32) {
        self.memory.resize(size as usize, 0);
    }

    fn compute_ptr_type(&self, arg: String, line_nr: u32, file_name: String) -> Result<PtrType, CompileError> {
        let mut mut_arg = arg.clone(); 
        if arg.clone().starts_with("*") {
            mut_arg.remove(0);
            return match mut_arg.parse::<i32>() {
                Ok(i) => Ok(PtrType::Pointer(i)),
                Err(_) => Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() }),
            }
        } else if arg.starts_with("#") {
            mut_arg.remove(0);
            return match mut_arg.parse::<i32>() {
                Ok(i) => Ok(PtrType::Immediate(i)),
                Err(_) => Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() }),
            }
        } else {
            return match mut_arg.parse::<i32>() {
                Ok(i) => Ok(PtrType::Register(i)),
                Err(_) => Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() }),
            }
        }
    }

    fn compute_store_type(&self, arg: String, line_nr: u32, file_name: String) -> Result<RefPtrType, CompileError> {
        let mut mut_arg = arg.clone(); 
        if arg.clone().starts_with("*") {
            mut_arg.remove(0);
            return if let Ok(i) = mut_arg.parse::<i32>() {
                Ok(RefPtrType::Pointer(i))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            }
        } else {
            return match mut_arg.parse::<i32>() {
                Ok(i) => Ok(RefPtrType::Register(i)),
                Err(_) => Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() }),
            }
        }
    }   

    fn compute_label(&self, arg: String, line_nr: u32, file_name: String) -> Result<String, CompileError> {
        // Check if label exists
        if self.labels.contains(&arg) {
            return Ok(arg);
        } else {
            return Err(CompileError::LabelError { file: file_name.clone(), line: line_nr.clone() });
        }
    }

    fn compile(
        &self,
        instr: String,
        arg: Option<String>,
        line_nr: u32,
        file_name: String,
    ) -> Result<Instruction, CompileError> {
        return match instr.to_ascii_lowercase().as_str() {
            "load" => if let Some(arg) = arg {
                let ptr_type = self.compute_ptr_type(arg, line_nr.clone(), file_name.clone())?;
                Ok(Instruction::Load(ptr_type))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },
            "store" => if let Some(arg) = arg {
                Ok(Instruction::Store(self.compute_store_type(arg, line_nr.clone(), file_name.clone())?))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },
            "add" => if let Some(arg) = arg {
                Ok(Instruction::Add(self.compute_ptr_type(arg, line_nr.clone(), file_name.clone())?))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },
            "sub" => if let Some(arg) = arg {
                Ok(Instruction::Sub(self.compute_ptr_type(arg, line_nr.clone(), file_name.clone())?))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },
            "mul" => if let Some(arg) = arg {
                Ok(Instruction::Mul(self.compute_ptr_type(arg, line_nr.clone(), file_name.clone())?))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },
            "div" => if let Some(arg) = arg {
                Ok(Instruction::Div(self.compute_ptr_type(arg, line_nr.clone(), file_name.clone())?))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },
            "jzero" => if let Some(arg) = arg {
                Ok(Instruction::JumpIfZero(self.compute_label(arg, line_nr.clone(), file_name.clone())?))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },
            "jnzero" => if let Some(arg) = arg {
                Ok(Instruction::JumpIfNotZero(self.compute_label(arg, line_nr.clone(), file_name.clone())?))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },
            "goto" => if let Some(arg) = arg {
                Ok(Instruction::Goto(self.compute_label(arg, line_nr.clone(), file_name.clone())?))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },
            "end" => if let None = arg {
                Ok(Instruction::End())
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            },

            _ => Err(CompileError::InvalidInstruction)
        };
    }

    pub fn load(&mut self, code: &PathBuf) -> Result<(), CompileError> {
        let mut line_number = 0;

        let file = std::fs::read_to_string(code).unwrap();

        // Load labels
        for line in file.lines() {
            let trimmed_line = line.trim();
            let mut tokens = trimmed_line.split_whitespace().peekable();
            // Process labels

            if tokens.peek().is_none() {
                continue;
            }

            if tokens.peek().unwrap().ends_with(":") {
                let label = tokens.next().unwrap();
                self.labels.push(label[..label.len() - 1].to_owned());
            }

            line_number += 1;
        }

        line_number = 0;

        for line in file.lines() {
            // Process compiler directives
            let trimmed_line = line.trim();

            if trimmed_line.starts_with("#define") {
                let mut tokens = trimmed_line.split_whitespace();
                tokens.next();
                let name = tokens.next().unwrap();
                let value = tokens.next().unwrap();
                self.defines.insert(name.to_owned(), value.to_owned());
            } else if trimmed_line.starts_with("#include") {
                let mut tokens = trimmed_line.split_whitespace();
                tokens.next();
                let name = tokens.next().unwrap();
                let mut path = code.clone();
                path.pop();
                path.push(name);
                self.load(&path)?;
            } else if !trimmed_line.is_empty() {
                // Code
                let mut tokens = trimmed_line.split_whitespace();

                // Process labels
                if tokens.clone().next().unwrap().ends_with(":") {
                    tokens.next();
                }

                if let Some(token) = tokens.next() {
                    let mut line = String::new();

                    line.push_str(token);

                    let mut argument = Option::None;
                    if let Some(arg) = tokens.next() {
                        line.push_str(" ");
                        // Process defines
                        if let Some(value) = self.defines.get(arg) {
                            line.push_str(value);
                            argument = Option::Some(value.clone().to_owned())
                        } else {
                            line.push_str(arg);
                            argument = Option::Some(arg.to_owned())
                        }
                    }

                    let instruction = self.compile(
                        token.to_owned(),
                        argument,
                        line_number.clone(),
                        code.to_str().unwrap().to_owned(),
                    )?;

                    self.lines.push(Line {
                        line: line,
                        line_number: line_number,
                        file_name: code.to_str().unwrap().to_owned(),
                        instruction: Some(instruction),
                    });
                }
            }

            line_number += 1;
        }

        Ok(())
    }
}
