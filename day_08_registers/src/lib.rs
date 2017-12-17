use std::collections::HashMap;

pub trait ProcessorTrait {
    fn new() -> Processor;
    fn process_instruction(&mut self, instruction: &str);
    fn highest_value_in_register(&self) -> i32;
}

#[derive(Debug, PartialEq)]
enum Operator {
    Empty,
    Inc,
    Dec,
}

#[derive(Debug, PartialEq, Clone)]
enum Conditional {
    Empty,
    Equal,
    NotEqual,
    Greater,
    Lesser,
    GreaterEqual,
    LesserEqual,
}

struct Instruction {
    operand_lhs: String,
    operand_rhs: i32,
    operator: Operator,
    conditional: Conditional,
    conditional_lhs: String,
    conditional_rhs: i32,
}

impl Instruction {
    fn new() -> Instruction {
        Instruction {
            operand_lhs: String::new(),
            operand_rhs: 0,
            operator: Operator::Empty,
            conditional: Conditional::Empty,
            conditional_lhs: String::new(),
            conditional_rhs: 0,
        }
    }

    fn parse_instruction_string(&mut self, instruction: &str) {
        let instuction_vector: Vec<&str> = instruction.split(' ').collect();
        assert_eq!(instuction_vector[3], "if");

        self.operand_lhs = String::from(instuction_vector[0]);
        self.operand_rhs = instuction_vector[2].parse().unwrap();
        self.parse_operator_str(instuction_vector[1]);

        self.conditional_lhs = String::from(instuction_vector[4]);
        self.conditional_rhs = instuction_vector[6].parse().unwrap();
        self.parse_conditional_str(instuction_vector[5]);
    }

    fn parse_operator_str(&mut self, operator_string: &str) {
        match operator_string {
            "inc" => self.operator = Operator::Inc,
            "dec" => self.operator = Operator::Dec,
            _ => self.operator = Operator::Empty,
        }
    }

    fn parse_conditional_str(&mut self, conditional_string: &str) {
        match conditional_string {
            "==" => self.conditional = Conditional::Equal,
            "!=" => self.conditional = Conditional::NotEqual,
            ">" => self.conditional = Conditional::Greater,
            "<" => self.conditional = Conditional::Lesser,
            ">=" => self.conditional = Conditional::GreaterEqual,
            "<=" => self.conditional = Conditional::LesserEqual,
            _ => self.conditional = Conditional::Empty,
        }
    }
}

pub struct Processor {
    instruction: Instruction,
    register: HashMap<String, i32>,
}

impl ProcessorTrait for Processor {
    fn new() -> Processor {
        Processor {
            instruction: Instruction::new(),
            register: HashMap::new(),
        }
    }

    fn process_instruction(&mut self, instruction: &str) {
        self.feed(instruction);
        self.process();
    }

    fn highest_value_in_register(&self) -> i32 {
        let mut max_value: i32 = 0;
        for val in self.register.values() {
            if val > &max_value {
                max_value = *val;
            }
        }
        max_value
    }
}

impl Processor {
    fn feed(&mut self, instruction: &str) {
        self.instruction.parse_instruction_string(instruction);
    }

    fn with_feed(instruction: &str) -> Processor {
        let mut cpu = Processor::new();
        cpu.feed(instruction);
        cpu
    }

    fn process(&mut self) {
        self.check_var_in_register();
        if self.check_conditional() {
            self.compute_instruction();
        }
    }

    fn check_var_in_register(&mut self) {
        let var_container = [
            self.instruction.conditional_lhs.clone(),
            self.instruction.operand_lhs.clone(),
        ];

        for var in var_container.iter() {
            if !self.register.contains_key(var) {
                self.init_var_in_register(var);
            }
        }
    }

    fn compute_instruction(&mut self) {
        let a_name = self.instruction.operand_lhs.clone();
        let a_val = self.register.get_mut(&a_name).unwrap();
        let b_val = self.instruction.operand_rhs;
        match self.instruction.operator {
            Operator::Inc => *a_val += b_val,
            Operator::Dec => *a_val -= b_val,
            Operator::Empty => println!("Operator was Empty"),
        }
    }

    fn init_var_in_register(&mut self, variable: &String) {
        self.register.insert(variable.clone(), 0);
    }

    fn check_conditional(&self) -> bool {
        let a = self.instruction.conditional_lhs.clone();
        let a = self.register.get(&a).unwrap();
        let b = self.instruction.conditional_rhs;
        let cond = self.instruction.conditional.clone();
        self.compare(*a, b, cond)
    }

    fn compare(&self, a: i32, b: i32, cond: Conditional) -> bool {
        match cond {
            Conditional::Empty => return false,
            Conditional::Equal => return a == b,
            Conditional::NotEqual => return a != b,
            Conditional::Greater => return a > b,
            Conditional::Lesser => return a < b,
            Conditional::GreaterEqual => return a >= b,
            Conditional::LesserEqual => return a <= b,
        }
    }
}

#[cfg(test)]
mod processor_test {
    use super::*;

    #[test]
    fn can_parse_input_instruction() {
        let cpu = Processor::with_feed("a inc 1 if a == 0");

        let operand_lhs = "a";
        assert_eq!(cpu.instruction.operand_lhs, "a");

        let operand_rhs = 1;
        assert_eq!(cpu.instruction.operand_rhs, operand_rhs);

        let operator = Operator::Inc;
        assert_eq!(cpu.instruction.operator, operator);

        let conditional_lhs = "a";
        assert_eq!(cpu.instruction.conditional_lhs, conditional_lhs);

        let conditional_rhs = 0;
        assert_eq!(cpu.instruction.conditional_rhs, conditional_rhs);

        let conditional = Conditional::Equal;
        assert_eq!(cpu.instruction.conditional, conditional);
    }

    #[test]
    fn can_increment_value_in_register() {
        let mut cpu = Processor::with_feed("a inc 1 if a == 0");
        cpu.process();
        assert_eq!(cpu.register.get("a"), Some(&1));
    }
}
