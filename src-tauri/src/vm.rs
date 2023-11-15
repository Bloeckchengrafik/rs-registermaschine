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
    pub label: Option<String>,
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

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub struct Diagnostics {
    pub line: u32,
    pub file: String,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum CompileError {
    InvalidInstruction {
        file: String,
        line: u32,
    },
    ParamError {
        file: String,
        line: u32,
    },
    LabelError {
        file: String,
        line: u32,
    },
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum ExecutionError {
    EndMarkerMissing,
    NotImplemented,
    DivThroughZero,
    AccessingReg0
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
pub enum ExecutionResult {
    End{
        line: Diagnostics,
        register: Vec<u32>,
        accumulator: u32,
    },
    Executed {
        line: Diagnostics,
        register: Vec<u32>,
        accumulator: u32,
    },
}

macro_rules! executed {
    ($file:expr, $line:expr, $register:expr, $accumulator:expr) => {
        ExecutionResult::Executed {
            line: Diagnostics {
                line: $line,
                file: $file,
            },
            register: $register,
            accumulator: $accumulator,
        }
    };
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
        if size as usize > self.memory.len() {
            self.memory.resize(size as usize, 0);
        }
    }

    fn compute_ptr_type(&self, arg: String, line_nr: u32, file_name: String) -> Result<PtrType, CompileError> {
        let mut mut_arg = arg.clone();
        return if arg.clone().starts_with("*") {
            mut_arg.remove(0);
            match mut_arg.parse::<i32>() {
                Ok(i) => Ok(PtrType::Pointer(i)),
                Err(_) => Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() }),
            }
        } else if arg.starts_with("#") {
            mut_arg.remove(0);
            match mut_arg.parse::<i32>() {
                Ok(i) => Ok(PtrType::Immediate(i)),
                Err(_) => Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() }),
            }
        } else {
            match mut_arg.parse::<i32>() {
                Ok(i) => Ok(PtrType::Register(i)),
                Err(_) => Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() }),
            }
        }
    }

    fn compute_store_type(&self, arg: String, line_nr: u32, file_name: String) -> Result<RefPtrType, CompileError> {
        let mut mut_arg = arg.clone();
        return if arg.clone().starts_with("*") {
            mut_arg.remove(0);
            if let Ok(i) = mut_arg.parse::<i32>() {
                Ok(RefPtrType::Pointer(i))
            } else {
                Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() })
            }
        } else {
            match mut_arg.parse::<i32>() {
                Ok(i) => Ok(RefPtrType::Register(i)),
                Err(_) => Err(CompileError::ParamError { file: file_name.clone(), line: line_nr.clone() }),
            }
        }
    }

    fn compute_label(&self, arg: String, line_nr: u32, file_name: String) -> Result<String, CompileError> {
        // Check if label exists
        return if self.labels.contains(&arg) {
            Ok(arg)
        } else {
            Err(CompileError::LabelError { file: file_name.clone(), line: line_nr.clone() })
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

            _ => Err(CompileError::InvalidInstruction { file: file_name.clone(), line: line_nr.clone() }),
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
                let mut label_in_line = None;
                // Process labels
                if tokens.clone().next().unwrap().ends_with(":") {
                    let label = tokens.next().unwrap();
                    label_in_line = Some(label[..label.len() - 1].to_owned());
                }

                // check if a comment is present
                if let Some(token) = tokens.clone().next() {
                    if token.starts_with("#") {
                        line_number += 1;
                        continue;
                    }
                }

                if let Some(token) = tokens.next() {
                    let mut line = String::new();

                    line.push_str(token);

                    let mut argument = None;
                    if let Some(arg) = tokens.next() {
                        line.push_str(" ");
                        // Process defines
                        if let Some(value) = self.defines.get(arg) {
                            line.push_str(value);
                            argument = Some(value.clone().to_owned())
                        } else {
                            line.push_str(arg);
                            argument = Some(arg.to_owned())
                        }
                    }

                    let instruction = self.compile(
                        token.to_owned(),
                        argument,
                        line_number.clone(),
                        code.to_str().unwrap().to_owned(),
                    )?;

                    self.lines.push(Line {
                        line,
                        line_number,
                        file_name: code.to_str().unwrap().to_owned(),
                        instruction: Some(instruction),
                        label: label_in_line,
                    });
                } else if let Some(label) = label_in_line {
                    self.lines.push(Line {
                        line: label.clone(),
                        line_number,
                        file_name: code.to_str().unwrap().to_owned(),
                        instruction: None,
                        label: Some(label),
                    });
                }
            }

            line_number += 1;
        }

        Ok(())
    }
    fn resolve_label(&mut self, my_label: String) -> Result<u32, ExecutionError> {
        let mut line_number = 0;
        for line in self.lines.iter() {
            if let Some(label) = &line.label {
                if label == &my_label {
                    break;
                }
            }

            line_number += 1;
        }

        return Ok(line_number);
    }

    pub fn step(&mut self) -> Result<ExecutionResult, ExecutionError> {
        // Check for a pointer overrun
        if self.line_ptr >= self.lines.len() as u32 {
            return Ok(ExecutionResult::End {
                line: Diagnostics {
                    line: self.lines[self.line_ptr as usize - 1].line_number.clone(),
                    file: self.lines[self.line_ptr as usize - 1].file_name.clone(),
                },
                register: self.memory.clone(),
                accumulator: self.accumulator,
            });
        }

        // Skip empty lines & labels
        while self.lines[self.line_ptr as usize].instruction.is_none() {
            self.line_ptr += 1;
        }

        let line = &mut self.lines.clone()[self.line_ptr as usize];
        let diagnostic_line = line.clone();
        let instruction = line.instruction.as_ref().unwrap();


        return match instruction {
            Instruction::Add(ptr) => {
                let value: u32 = self.resolve_ptr(ptr)?;

                self.accumulator = self.accumulator + value;
                self.line_ptr += 1;
                Ok(executed!(diagnostic_line.file_name.clone(), diagnostic_line.line_number.clone(), self.memory.clone(), self.accumulator))
            }

            Instruction::Sub(ptr) => {
                let value: u32 = self.resolve_ptr(ptr)?;

                if value <= self.accumulator {
                    self.accumulator = self.accumulator - value;
                }
                self.line_ptr += 1;
                Ok(executed!(line.file_name.clone(), line.line_number.clone(), self.memory.clone(), self.accumulator))
            }

            Instruction::Mul(ptr) => {
                let value: u32 = self.resolve_ptr(ptr)?;

                self.accumulator = self.accumulator * value;
                self.line_ptr += 1;
                Ok(executed!(line.file_name.clone(), line.line_number.clone(), self.memory.clone(), self.accumulator))
            }

            Instruction::Div(ptr) => {
                let value: u32 = self.resolve_ptr(ptr)?;
                if value == 0 {
                    return Err(ExecutionError::DivThroughZero);
                }

                self.accumulator = self.accumulator / value;
                self.line_ptr += 1;
                Ok(executed!(line.file_name.clone(), line.line_number.clone(), self.memory.clone(), self.accumulator))
            }

            Instruction::Goto(label) => {
                let label_line_number = self.resolve_label(label.clone())?;
                self.line_ptr = label_line_number;
                Ok(executed!(line.file_name.clone(), line.line_number.clone(), self.memory.clone(), self.accumulator))
            }

            Instruction::JumpIfZero(label) => {
                if self.accumulator == 0 {
                    let label_line_number = self.resolve_label(label.clone())?;
                    self.line_ptr = label_line_number;
                } else {
                    self.line_ptr += 1;
                }

                Ok(executed!(line.file_name.clone(), line.line_number.clone(), self.memory.clone(), self.accumulator))
            }

            Instruction::JumpIfNotZero(label) => {
                if self.accumulator != 0 {
                    let label_line_number = self.resolve_label(label.clone())?;
                    self.line_ptr = label_line_number;
                } else {
                    self.line_ptr += 1;
                }

                Ok(executed!(line.file_name.clone(), line.line_number.clone(), self.memory.clone(), self.accumulator))
            }

            Instruction::Load(ptr) => {
                let value = self.resolve_ptr(ptr);
                self.accumulator = value?;
                self.line_ptr += 1;
                Ok(executed!(line.file_name.clone(), line.line_number.clone(), self.memory.clone(), self.accumulator))
            }

            Instruction::Store(ptr) => {
                let value = self.accumulator;
                match ptr {
                    RefPtrType::Register(i) => {
                        self.resize_memory(i.clone() as u32 + 1);
                        self.memory[i.clone() as usize-1] = value
                    },
                    RefPtrType::Pointer(i) => {
                        self.resize_memory(i.clone() as u32 + 1);
                        let pos = self.memory[i.clone() as usize-1] as usize;
                        self.resize_memory(pos as u32 + 1);
                        self.memory[pos-1] = value
                    },
                }

                self.line_ptr += 1;
                Ok(executed!(line.file_name.clone(), line.line_number.clone(), self.memory.clone(), self.accumulator))
            }

            Instruction::End() => {
                Ok(ExecutionResult::End {
                    line: Diagnostics {
                        line: line.line_number.clone(),
                        file: line.file_name.clone(),
                    },
                    register: self.memory.clone(),
                    accumulator: self.accumulator,
                })
            }
        };
    }

    fn resolve_ptr(&mut self, ptr: &PtrType) -> Result<u32, ExecutionError> {
        let value: u32 = match ptr {
            PtrType::Immediate(i) => i.clone() as u32,
            PtrType::Register(i) => {
                self.resize_memory(i.clone() as u32 + 1);
                if i.clone() == 0 {
                    return Err(ExecutionError::AccessingReg0);
                }
                self.memory[i.clone() as usize - 1]
            },
            PtrType::Pointer(i) => {
                self.resize_memory(i.clone() as u32 + 1);
                if i.clone() == 0 {
                    return Err(ExecutionError::AccessingReg0);
                }
                let pos = self.memory[i.clone() as usize - 1] as usize;
                if pos == 0 {
                    return Err(ExecutionError::AccessingReg0);
                }
                self.resize_memory(pos as u32 + 1);
                println!("Resolved pointer {} to {}", i.clone(), pos - 1);
                self.memory[pos - 1]
            },
        };
        Ok(value)
    }
}
