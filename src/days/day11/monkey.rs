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

type Number = u64;

#[derive(Debug)]
pub struct Monkey {
    items: Vec<Number>,
    operand: Number,
    operator: Operator,
    divisor: u32,
    true_pass: u32,
    false_pass: u32,
}

impl Monkey {
    pub fn from_definition(definition: &str) -> Monkey {
        let mut lines = definition.lines();
        lines.next();
        let items = lines.next().unwrap()[18..]
            .split(", ")
            .map(|item| item.parse::<Number>().unwrap())
            .collect();
        let operation = lines.next().unwrap()[23..].split_once(' ').unwrap();
        let operand = operation.1.parse::<Number>().unwrap_or_default();
        let operator = operation.0.into();
        let test = lines.next().unwrap()[21..].parse().unwrap_or_default();
        let true_pass = lines.next().unwrap()[29..].parse().unwrap_or_default();
        let false_pass = lines.next().unwrap()[30..].parse().unwrap_or_default();
        Monkey {
            items,
            operand,
            operator,
            divisor: test,
            true_pass,
            false_pass,
        }
    }

    pub fn remove_item(&mut self, item: Number) {
        if let Some(position) = self.items.iter().position(|i| *i == item) {
            self.items.remove(position);
        }
    }

    pub fn get_items(&self) -> Vec<Number> {
        self.items.clone()
    }

    pub fn operate(&self, item: Number) -> Number {
        let operand = if self.operand > 0 { self.operand } else { item };
        match self.operator {
            Operator::Mult => item * operand,
            Operator::Add => item + operand,
        }
    }

    pub fn calculate_new_worry_level(&self, item: Number, div: u32, gcd: u32) -> Number {
        (self.operate(item) / div as Number) % gcd as Number
    }

    pub fn calculate_recipient(&self, item: Number, div: u32, gcd: u32) -> u32 {
        let worry_level = self.calculate_new_worry_level(item, div, gcd);
        if worry_level % self.divisor as Number == 0 {
            self.true_pass
        } else {
            self.false_pass
        }
    }

    pub fn add_item(&mut self, item: Number) {
        self.items.push(item);
    }

    pub fn get_divisor(&self) -> u32 {
        self.divisor
    }
}
