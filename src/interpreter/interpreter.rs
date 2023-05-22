use std::collections::HashMap;

use crate::{
    interpreter::{
        r#type::Type,
        value::{function::Function, int::Int, Value},
    },
    parser::{
        line::{
            binary_op::Operation, bracketed_identifier::BracketedIdentifier,
            expression::Expression, identifier::Identifier,
            indented_expression::IndentedExpression,
        },
        program::Program,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: Identifier,
    value: Value,
}

impl Variable {
    pub fn set_value(&mut self, value: Value) {
        self.value = value;
    }
}

#[derive(Default)]
pub struct InterpreterState {
    variables: HashMap<Identifier, Variable>,
}

impl InterpreterState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self, program: Program) {
        let lines = program
            .expressions
            .into_iter()
            .enumerate()
            .map(|(i, l)| (i + 1, l))
            .collect::<Vec<_>>();

        self.eval_lines(&lines);
    }

    fn eval_lines(&mut self, lines: &[(usize, IndentedExpression)]) -> Option<Value> {
        let come_froms = lines
            .iter()
            .filter_map(|(idx, e)| match &e.expr {
                Expression::ComeFrom(c) => Some((c.line_number.0 as usize, idx + 1)),
                _ => None,
            })
            .collect::<HashMap<_, _>>();

        let mut value = None;

        // Sort lines
        let lines = {
            let mut v = lines.iter().collect::<Vec<_>>();
            v.sort_by_key(|(l, _)| l);
            v
        };

        let mut current_idx = 0;

        while let Some((line_number, expr)) = lines.get(current_idx) {
            // If the next line in `lines` is the actual next line (i.e. the line number is
            // `line_number + 1`), and the line is indented, then this line should be interpreted
            // as a conditional.
            if let Some((next_line_number, next_expr)) = lines.get(current_idx + 1)
                && *next_line_number == line_number + 1
                && next_expr.indent_depth > expr.indent_depth
            {
                if self.eval_conditional((*line_number, &expr.expr)) == Int(1) {
                    // Conditional evaluated to true, so go to the next line
                    current_idx += 1;
                } else {
                    // Conditional evaluated to false, skip over the indented block
                    match lines
                        .iter()
                        // Skip over all lines up to and including the current one
                        .skip(current_idx + 1)
                        .position(|(_, e)| e.indent_depth <= expr.indent_depth)
                        // Add back on the `current_idx + 1` lines that we skipped
                        .map(|l| l + current_idx + 1)
                    {
                        Some(new_idx) => {
                            current_idx = new_idx;
                            continue;
                        }
                        None => break,
                    }
                }
            } else {
                // Not a conditional, so just evaluate the expression normally.
                value = Some(self.eval_expression((*line_number, &expr.expr)));

                // Check for `come from` jumps
                if let Some(&to) = come_froms.get(line_number) {
                    // Check if the line `to` is contained in `lines`, and jump there.
                    if let Some(new_idx) = lines.iter().position(|(l, _)| *l == to) {
                        current_idx = new_idx;
                    } else {
                        break;
                    }
                } else {
                    // Otherwise, just go to the next line
                    current_idx += 1;
                }
            }
        }

        value
    }

    fn eval_expression(&mut self, (line_number, expr): (usize, &Expression)) -> Value {
        match expr {
            Expression::Equality(eq) => {
                let Some(ident) = self.resolve_bracketed_identifier(&eq.lhs) else {
                    // The LHS of the equality is invalid. Set the inner identifier to 127.
                    self.set_variable_or_create(eq.lhs.identifier.clone(), Value::Int(Int(127)));
                    return Value::Int(Int(127));
                };

                // If ident is a function variable, just append the line and return immediately
                if let Some(v) = self.variables.get_mut(&ident) {
                    if v.value == Value::Uninitialized(Type::Function) {
                        v.value = Value::Function(Function::default());
                    }

                    if let Value::Function(f) = &mut v.value {
                        let a = (line_number, *eq.rhs.clone());
                        f.lines.push(a);
                        return Value::Int(Int(0));
                    }
                }

                // Eval the RHS
                let mut rhs = self.eval_expression((line_number, &eq.rhs.expr));

                if let Some(var) = self.variables.get_mut(&ident) {
                    // We already dealt with the case of `var` being a function, so we can just set
                    // the value here
                    rhs.cast(var.value.r#type());
                    var.set_value(rhs.clone());

                    rhs
                } else {
                    // Create a variable with RHS as the type name
                    let var_type = match &eq.rhs.expr {
                        Expression::Identifier(ident) => Type::from(ident.0.as_ref()),
                        _ => todo!(),
                    };
                    let value = Value::Uninitialized(var_type);
                    self.create_variable(ident, value.clone());
                    value
                }
            }
            Expression::ComeFrom(_) => Value::Int(Int(0)),
            Expression::Print(p) => {
                let value = self.eval_expression((line_number, &p.0));
                println!("{value}");
                value
            }
            Expression::BinaryOp(op) => {
                let lhs = self.eval_expression((line_number, &op.lhs));
                let rhs = self.eval_expression((line_number, &op.rhs));
                match op.op {
                    Operation::Add => lhs + rhs,
                    Operation::Sub => lhs - rhs,
                    Operation::Mul => lhs * rhs,
                    Operation::Div => lhs / rhs,
                    Operation::ModularDiv => lhs.modular_div(rhs),
                    Operation::Mod => lhs % rhs,
                }
            }
            Expression::Identifier(ident) => {
                if let Some(var) = self.variables.get(ident) {
                    if let Value::Function(f) = &var.value {
                        // Variable is a function, so call the function
                        self.call_function(&f.clone())
                    } else {
                        // Not a function, so return the value of the variable
                        var.value.clone()
                    }
                } else {
                    Value::Uninitialized(Type::Int)
                }
            }
            Expression::Literal(lit) => lit.into(),
            Expression::None => Value::Uninitialized(Type::Int),
        }
    }

    pub fn eval_conditional(&mut self, (line_number, expr): (usize, &Expression)) -> Int {
        match expr {
            Expression::Equality(eq) => {
                let rhs = self.eval_expression((line_number, &eq.rhs.expr));

                // Value of the LHS variable, or an uninitialized int if it's not defined
                let mut lhs = self
                    .resolve_bracketed_identifier(&eq.lhs)
                    .and_then(|ident| self.variables.get(&ident))
                    .map(|v| &v.value)
                    .unwrap_or_else(|| &Value::Uninitialized(Type::Int))
                    .to_owned();
                lhs.cast(rhs.r#type());

                Int((lhs == rhs) as u8)
            }
            Expression::BinaryOp(_) => todo!(),
            Expression::Literal(_) => todo!(),
            Expression::Identifier(_) => todo!(),
            Expression::ComeFrom(_) | Expression::Print(_) | Expression::None => Int(0),
        }
    }

    pub fn call_function(&mut self, function: &Function) -> Value {
        self.eval_lines(&function.lines)
            .unwrap_or(Value::Int(Int(127)))
    }

    pub fn create_variable(&mut self, name: Identifier, value: Value) {
        self.variables
            .insert(name.clone(), Variable { name, value });
    }

    pub fn set_variable_or_create(&mut self, name: Identifier, value: Value) {
        if let Some(var) = self.variables.get_mut(&name) {
            var.set_value(value);
        } else {
            self.create_variable(name, value);
        }
    }

    pub fn resolve_bracketed_identifier(
        &self,
        bracketed: &BracketedIdentifier,
    ) -> Option<Identifier> {
        // (x) = y means set the variable whose name is the value of x, to y.
        // So if LHS is in brackets, `eval` it that many times.
        let mut ident = bracketed.identifier.clone();
        for _ in 0..bracketed.num_brackets {
            if let Some(var) = self.variables.get(&ident) {
                ident = Identifier(var.value.to_string());
            } else {
                return None;
            }
        }

        Some(ident)
    }
}
