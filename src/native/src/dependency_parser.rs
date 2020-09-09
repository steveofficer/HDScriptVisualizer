use std::collections::HashSet;
use crate::hd_script_parser;

use hd_script_parser::*;

pub fn parse(script: &Script) -> HashSet<String> {
    let mut result = HashSet::new();
    output_statements(&script.body, &mut result);
    result
}

fn output_statements(statements: &Vec<Statement>, uses: &mut HashSet<String>) {
    for statement in statements.iter().filter(|s| match s { Statement::Comment(_) => false, _ => true }) {
        output_statement(statement, uses);
    }
}

fn output_statement(statement: &Statement, uses: &mut HashSet<String>) {
    match statement {
        Statement::If(if_statement) => output_if_statement(if_statement, uses),
        Statement::Instruction(Instruction::Script(inst)) => output_script_instruction(inst, uses),
        Statement::Instruction(Instruction::Display(inst)) => output_display_instruction(inst, uses),
        Statement::Loop(LoopStatement::While(while_loop)) => {
            output_expression(&while_loop.condition, uses);
            output_statements(&while_loop.body, uses);
        },
        Statement::Loop(LoopStatement::Repeat(repeat_loop)) => {
            output_variable_reference(&repeat_loop.dialog, uses);
            output_statements(&repeat_loop.body, uses);
        },
        Statement::Return(return_value) => {
            output_expression(return_value, uses);
        },
        _ => ()
    };
}

fn output_script_instruction(instr: &ScriptInstruction, uses: &mut HashSet<String>) {
    match instr {
        ScriptInstruction::Increment(variable) => {
            output_variable_reference(variable, uses);
        },
        ScriptInstruction::Ascend(_) => (),
        ScriptInstruction::Assemble(_) => (),
        ScriptInstruction::Decrement(variable) => {
            output_variable_reference(variable, uses);
        },
        ScriptInstruction::Default(_, _) => (),
        ScriptInstruction::Descend(_) => (),
        ScriptInstruction::Erase(variable) => {
            output_variable_reference(variable, uses);
        },
        ScriptInstruction::Filter => (),
        ScriptInstruction::Format => (),
        ScriptInstruction::Other => (),
        ScriptInstruction::Quit => (),
        ScriptInstruction::Selection => (),
        ScriptInstruction::Set(variable, expression) => {
            output_variable_reference(variable, uses);
            output_expression(expression, uses);
        },
        ScriptInstruction::Add(variable, expression) => {
            output_variable_reference(variable, uses);
            output_expression(expression, uses);
        },
        ScriptInstruction::Unanswered => (),
        ScriptInstruction::Union => (),
        ScriptInstruction::Value => (),
        ScriptInstruction::Zero => ()
    }
}

fn output_display_instruction(instr: &DisplayInstruction, uses: &mut HashSet<String>) {
    match instr {
        DisplayInstruction::Ask(variable) => { uses.insert((*variable).to_owned()); },
        _ => ()
    }
}

fn output_variable_reference(variable: &VariableReference, uses: &mut HashSet<String>) {
    uses.insert(variable.name.to_owned());

    match &variable.indexer {
        Some(i) => {
            
            for (idx, e) in i.args.iter().enumerate() {
                output_expression(e, uses);
            }
        },
        _ => ()
    }
}

fn output_expression(expression: &Expression, uses: &mut HashSet<String>) {
    match expression {
        Expression::Variable(variable) => output_variable_reference(variable, uses),
        
        Expression::Literal(LiteralExpression::Text(t)) => {
            // process dot codes
            ()
        },
        Expression::Binary(ex) => output_binary_expression(ex, uses),
        Expression::Unary(ex) => output_unary_expression(ex, uses),
        Expression::FunctionCall(call) => {
            uses.insert(call.name.to_owned());
            for (idx, e) in call.args.iter().enumerate() {
                output_expression(e, uses);
            }
        },
        _ => ()
    }
}

fn output_unary_expression(unary_expression: &UnaryExpression, uses: &mut HashSet<String>) {
    let expression = &unary_expression.expression;

    output_expression(expression, uses);
}

fn output_binary_expression(binary_expression: &BinaryExpression, uses: &mut HashSet<String>) {
    let left = &binary_expression.left;
    let right = &binary_expression.right;
    
    match right[..] {
        [] => {
            output_expression(left, uses);
        },
        _ => {
            output_expression(left, uses);
            for (op, expr) in right.iter() {
                output_expression(expr, uses);
            }
        }
    }
}

fn output_if_statement(if_statement: &IfStatement, uses: &mut HashSet<String>) {
    output_expression(&if_statement.condition, uses);
    output_statements(&if_statement.then_body, uses);

    match &if_statement.else_body[..] {
        [] => (),
        [x] => {
            output_statement(&x, uses);
        },
        _ => {
            output_statements(&if_statement.else_body, uses);
        }
    }
}