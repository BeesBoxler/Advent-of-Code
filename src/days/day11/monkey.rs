use std::ops::Index;

#[derive(Debug)]
enum Operator {
    Mult,
    Add,
}

impl std::convert::From<&str> for Operator {
    fn from(operator: &str) -> Self {
        match operator {
            "*" => Operator::Mult,
            "+" => Operator::Add,
            op => panic!("ERROR: `{}` IS NOT A VALID OPERATOR", op),
        }
    }
}

#[derive(Debug)]
pub struct Monkey {
    items: Vec<u32>,
    operand: u32,
    operator: Operator,
    test: u32,
    true_pass: u32,
    false_pass: u32,
}

impl Monkey {
    pub fn from_definition(definition: &str) -> Monkey {
        let mut lines = definition.lines();
        lines.next();
        let items = lines.next().unwrap()[18..]
            .split(", ")
            .map(|item| item.parse::<u32>().unwrap())
            .collect();
        let operation = lines.next().unwrap()[23..].split_once(' ').unwrap();
        let operand = operation.1.parse::<u32>().unwrap_or_default();
        let operator = operation.0.into();
        let test = lines.next().unwrap()[21..].parse().unwrap_or_default();
        let true_pass = lines.next().unwrap()[29..].parse().unwrap_or_default();
        let false_pass = lines.next().unwrap()[30..].parse().unwrap_or_default();
        Monkey {
            items,
            operand,
            operator,
            test,
            true_pass,
            false_pass,
        }
    }

    pub fn remove_item(&mut self, item: u32) {
        if let Some(position) = self.items.iter().position(|i| *i == item) {
            self.items.remove(position);
        }
    }

    pub fn get_items(&self) -> Vec<u32> {
        self.items.clone()
    }

    pub fn operate(&self, item: u32) -> u32 {
        let operand = if self.operand > 0 { self.operand } else { item };
        match self.operator {
            Operator::Mult => item * operand,
            Operator::Add => item + operand,
        }
    }

    pub fn calculate_new_worry_level(&self, item: u32) -> u32 {
        self.operate(item) / 3
    }

    pub fn calculate_recipient(&self, item: u32) -> u32 {
        let worry_level = self.calculate_new_worry_level(item);
        if worry_level % self.test == 0 {
            self.true_pass
        } else {
            self.false_pass
        }
    }

    pub fn add_item(&mut self, item: u32) {
        self.items.push(item);
    }
}
