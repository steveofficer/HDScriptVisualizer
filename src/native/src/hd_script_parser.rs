use nom::{
    branch::alt,
    number::complete::float,
    bytes::complete::tag,
    character::complete::{ anychar, char, line_ending, multispace0, multispace1, not_line_ending, one_of, none_of },
    combinator::{ map, opt, peek, recognize, value, rest },
    error::{ context, make_error, ErrorKind },
    multi::{ many0, many_till, separated_list },
    sequence::{ delimited, preceded, separated_pair, tuple, terminated },
    Err, 
    IResult
};

use std::iter::FromIterator;

#[derive(Debug, PartialEq, Clone)]
pub enum LimitExpression<'a> {
    Number(f32),
    Variable(&'a str),
    Function(FunctionCall<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DisplayInstruction<'a> {
    Ask(&'a str),
    Gray(&'a str),
    GrayAll,
    Hide(&'a str),
    HideAll,
    Show(&'a str),
    ShowAll,
    Ungray(&'a str),
    UngrayAll,
    Require(&'a str),
    RequireAll,
    Limit(LimitExpression<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ScriptInstruction<'a> {
    Erase(VariableReference<'a>),
    Increment(VariableReference<'a>),
    Decrement(VariableReference<'a>),
    Quit,
    Ascend(&'a str),
    Descend(&'a str),
    Filter,
    Format,
    Set(VariableReference<'a>, Expression<'a>),
    Add(VariableReference<'a>, Expression<'a>),
    Default(VariableReference<'a>, Expression<'a>),
    Other,
    Selection,
    Unanswered,
    Union,
    Value,
    Zero,
    Assemble(VariableReference<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction<'a> {
    Display(DisplayInstruction<'a>),
    Script(ScriptInstruction<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionCall<'a> {
    pub name: &'a str,
    pub args: Vec<Expression<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperation {
    And,
    Or,
    Subtract,
    Add,
    Multiply,
    Divide,
    GT,
    LT,
    EQ,
    NE,
    LTE,
    GTE,
    Contains,
    StartsWith,
    EndsWith,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperation {
    Not,
    Negate,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryExpression<'a> {
    pub left: Expression<'a>,
    pub right: Vec<(BinaryOperation, Expression<'a>)>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpression<'a> {
    pub operation: UnaryOperation,
    pub expression: Expression<'a>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralExpression {
    Number(f32),
    Text(String),
    Boolean(bool),
    List,
    Record,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VariableReference<'a> {
    pub name: &'a str,
    pub indexer: Option<FunctionCall<'a>>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression<'a> {
    Variable(VariableReference<'a>),
    Literal(LiteralExpression),
    Binary(Box<BinaryExpression<'a>>),
    Unary(Box<UnaryExpression<'a>>),
    FunctionCall(FunctionCall<'a>),
    Days(Box<Expression<'a>>),
    Months(Box<Expression<'a>>),
    Years(Box<Expression<'a>>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement<'a> {
    pub condition: Expression<'a>,
    pub then_body: Vec<Statement<'a>>,
    pub else_body: Vec<Statement<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CommentStatement<'a> {
    pub comment: &'a str,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileStatement<'a> {
    pub condition: Expression<'a>,
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct RepeatStatement<'a> {
    pub dialog: VariableReference<'a>,
    pub body: Vec<Statement<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum LoopStatement<'a> {
    While(WhileStatement<'a>),
    Repeat(RepeatStatement<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'a> {
    Comment(CommentStatement<'a>),
    If(IfStatement<'a>),
    Loop(LoopStatement<'a>),
    Instruction(Instruction<'a>),
    Return(Expression<'a>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Script<'a> {
    pub body: Vec<Statement<'a>>,
}

fn parse_boolean(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("TRUE")), value(false, tag("FALSE"))))(input)
}

fn parse_binary_operator(input: &str) -> IResult<&str, BinaryOperation> {
    terminated(
        alt((
            value(BinaryOperation::And, tag("AND")),
            value(BinaryOperation::Or, tag("OR")),
            value(BinaryOperation::LTE, tag("<=")),
            value(BinaryOperation::GTE, tag(">=")),
            value(BinaryOperation::Multiply, tag("*")),
            value(BinaryOperation::Divide, tuple((tag("/"), peek(none_of("/"))))),
            value(BinaryOperation::Subtract, tag("-")),
            value(BinaryOperation::Add, tag("+")),
            value(BinaryOperation::GT, tag(">")),
            value(BinaryOperation::LT, tag("<")),
            value(BinaryOperation::EQ, tag("=")),
            value(BinaryOperation::NE, tag("!=")),
            value(BinaryOperation::Contains, tag("CONTAINS")),
            value(BinaryOperation::StartsWith, tag("STARTS WITH")),
            value(BinaryOperation::EndsWith, tag("ENDS WITH")),
        )),
        multispace0,
    )(input)
}

fn parenthesized<'a, R, T>(g: T, opening: char, closing: char) -> impl Fn(&'a str) -> IResult<&'a str, R>
where T: Fn(&'a str) -> IResult<&'a str, R> {
    delimited(
        terminated(char(opening), multispace0), 
        g, 
        delimited(multispace0, char(closing), multispace0)
    )
}

fn parse_not_expression(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(
            delimited(multispace0, alt((tag("!"), tag("NOT"))), multispace0),
            parse_entry_expression
        ),
        |e| Expression::Unary(Box::new(UnaryExpression { operation: UnaryOperation::Not, expression: e }))
    )(input)
}

fn parse_negation_expression(input: &str) -> IResult<&str, Expression> {
    map(
        preceded(tag("-"), parse_expression_atom), 
        |e| Expression::Unary(Box::new(UnaryExpression { operation: UnaryOperation::Negate, expression: e }))
    )(input)
}

// This uses the precedence climbing algorithm to create an AST where the binary expression is properly nested based
//  on the precedence of the operators joining the atoms of the expression
fn parse_operator_precedence(current_level: u8, input: &str) -> IResult<&str, (BinaryOperation, Expression)> {
    // Look ahead to see if the next token is a binary operation.
    let (input, op) = peek(parse_binary_operator)(input)?;

    let next_precedence_level = match op {
        BinaryOperation::Or => 1,
        BinaryOperation::And => 2,
        BinaryOperation::EQ
        | BinaryOperation::GT
        | BinaryOperation::GTE
        | BinaryOperation::LT
        | BinaryOperation::LTE
        | BinaryOperation::NE => 3,
        BinaryOperation::Add | BinaryOperation::Subtract => 4,
        BinaryOperation::Multiply | BinaryOperation::Divide => 5,
        _ => 6
    };

    if next_precedence_level < current_level {
        Err(Err::Error(make_error(input, ErrorKind::IsA)))
    } else {
        let (input, op) = terminated(parse_binary_operator, multispace0)(input)?;
        let (input, right) = parse_expression(input, next_precedence_level + 1)?;
        Ok((input, (op, right)))
    }
}

fn parse_string_literal(input: &str) -> IResult<&str, String> {
    fn p(input: &str) -> IResult<&str, char> {
        let (i, c) = anychar(input)?;

        // If the current char is a " then we need to see if it is an embedded " or the end of the literal
        if c == '"' {
            // If this fails, the parser goes into Err and we know we got to the end of the string
            let (i, _) = char('"')(i)?;
            // We got here so we know the next char is a ", so this is an embedded "
            Ok((i, c))
        } else {
            // Just a normal char
            Ok((i, c))
        }
    };

    let (input, result) = delimited(tag("\""), many0(p), tag("\""))(input)?;

    Ok((input, String::from_iter(result)))
}

fn parse_variable_reference(input: &str) -> IResult<&str, VariableReference> {
    fn reserved_word(input: &str) -> IResult<&str, ()> {
        peek(
            alt((
                value((), peek(line_ending)),
                value((), one_of("[](),\"")),
                value((), preceded(multispace1, parse_boolean)),
                value((), preceded(multispace1, parse_binary_operator)),
                value((), preceded(multispace1, tag("IF"))),
                value((), preceded(multispace1, tag("//"))),
                value((), preceded(multispace1, tag("NOT"))),
                value((), preceded(multispace1, tag("ELSE"))),
                value((), preceded(multispace1, tag("END IF"))),
                value((), preceded(multispace1, tag("REPEAT"))),
                value((), preceded(multispace1, tag("WHILE"))),
                value((), preceded(multispace1, tag("TO"))),
            )),
        )(input)
    }

    let (input, reference) = recognize(many_till(anychar, reserved_word))(input)?;

    let (input, indexer) = opt(parse_indexer)(input)?;

    Ok((input, VariableReference { name: reference, indexer }))
}

pub fn parse_expression_atom(input: &str) -> IResult<&str, Expression> {
    let (input, expression) = terminated(
        alt((
            parenthesized(parse_entry_expression, '(', ')'),
            map(float, |d| Expression::Literal(LiteralExpression::Number(d))),
            map(parse_boolean, |b| Expression::Literal(LiteralExpression::Boolean(b))),
            // String literal
            map(parse_string_literal, |s| Expression::Literal(LiteralExpression::Text(s))),
            // Unary operators
            parse_not_expression,
            parse_negation_expression,
            map(parse_function_call, Expression::FunctionCall),
            map(parse_variable_reference, |v| Expression::Variable(v)),
        )),
        multispace0
    )(input)?;

    let (input, suffix) = terminated(opt(alt((tag("DAYS"), tag("MONTHS"), tag("YEARS"), tag("DAY"), tag("MONTH"), tag("YEAR")))), multispace0)(input)?;

    let effective_expression =
        match suffix {
            Some("DAYS") | Some("DAY") => Expression::Days(Box::new(expression)),
            Some("MONTHS") | Some("MONTH") => Expression::Months(Box::new(expression)),
            Some("YEARS") | Some("YEAR") => Expression::Years(Box::new(expression)),
            _ => expression
        };

    Ok((input, effective_expression))
}

fn parse_expression(input: &str, precedence_level: u8) -> IResult<&str, Expression> {
    let (input, left) = terminated(parse_expression_atom, multispace0)(input)?;

    let (input, right_acc) = context(
        "Right expressions",
        terminated(many0(|input| parse_operator_precedence(precedence_level, input)), multispace0),
    )(input)?;

    if right_acc.len() > 0 {
        Ok((
            input,
            Expression::Binary(Box::new(BinaryExpression {
                left: left,
                right: right_acc,
            })),
        ))
    } else {
        Ok((input, left))
    }
}

pub fn parse_entry_expression(input: &str) -> IResult<&str, Expression> {
    parse_expression(input, 1)
}

fn parse_limit_expression(input: &str) -> IResult<&str, LimitExpression> {
    alt((
        map(float, |n| LimitExpression::Number(n)),
        map(not_line_ending, |v| LimitExpression::Variable(v)),
    ))(input)
}

fn remaining_text(input: &str) -> IResult<&str, &str> {
    preceded(multispace1, not_line_ending)(input)
}

fn parse_display_instruction(input: &str) -> IResult<&str, Instruction> {
    terminated(
        alt((
            map(preceded(tag("ASK"), remaining_text), |v| Instruction::Display(DisplayInstruction::Ask(v))),    
            value(Instruction::Display(DisplayInstruction::GrayAll), tag("GRAY ALL")),
            map(preceded(tag("GRAY"), remaining_text), |v| Instruction::Display(DisplayInstruction::Gray(v))),
            value(Instruction::Display(DisplayInstruction::UngrayAll), tag("UNGRAY ALL")),
            map(preceded(tag("UNGRAY"), remaining_text), |v| Instruction::Display(DisplayInstruction::Ungray(v))),
            value(Instruction::Display(DisplayInstruction::ShowAll), tag("SHOW ALL")),
            map(preceded(tag("SHOW"), remaining_text), |v| Instruction::Display(DisplayInstruction::Show(v))),
            value(Instruction::Display(DisplayInstruction::HideAll), tag("HIDE ALL")),
            map(preceded(tag("HIDE"), remaining_text), |v| Instruction::Display(DisplayInstruction::Hide(v))),
            value(Instruction::Display(DisplayInstruction::RequireAll), tag("REQUIRE ALL")),
            map(preceded(tag("REQUIRE"), remaining_text), |v| Instruction::Display(DisplayInstruction::Require(v))),
            map(preceded(tag("LIMIT"), preceded(multispace1, parse_limit_expression)), |v| Instruction::Display(DisplayInstruction::Limit(v)))
        )), 
        multispace0
    )(input)
}

pub fn parse_if_statement(input: &str) -> IResult<&str, IfStatement> {
    fn else_if(input: &str) -> IResult<&str, Statement> {
        // The difference here is that we only peek END IF so that we terminate up the stack
        // and allow the topmost parser to consume END IF
        let (input, condition) =
            preceded(tag(" IF"), preceded(multispace0, parse_entry_expression))(input)?;

        let (input, (then_block, terminator)) = preceded(
            multispace0,
            many_till(
                parse_statement,
                preceded(multispace0, alt((peek(tag("END IF")), tag("ELSE")))),
            ),
        )(input)?;

        let (input, else_block) = match terminator {
            "END IF" => (input, vec![]),
            _ => {
                let (input, (else_block, _)) = many_till(
                    alt((else_if, preceded(multispace0, parse_statement))),
                    preceded(multispace0, peek(tag("END IF"))),
                )(input)?;
                (input, else_block)
            }
        };

        Ok((
            input,
            Statement::If(IfStatement {
                condition: condition,
                then_body: then_block,
                else_body: else_block,
            }),
        ))
    }

    let (input, condition) = context(
        "IF",
        preceded(tag("IF"), preceded(multispace0, parse_entry_expression)),
    )(input)?;

    let (input, (then_block, terminator)) = context(
        "THEN",
        preceded(
            multispace0,
            many_till(
                parse_statement,
                preceded(multispace0, alt((tag("END IF"), tag("ELSE")))),
            ),
        ),
    )(input)?;

    // Parse statements\instructions until we get ELSE or END IF
    // ELSE IF creates an if statement that doesn't require an END IF
    // Could it be treated as an Else with a single statement?
    // Else could be defined as str -> Statement|Statement[]. Where Statement & IfStatement => no END IF
    let (input, else_block) = match terminator {
        "END IF" => (input, vec![]),
        _ => {
            let (input, (else_block, _)) = many_till(
                alt((else_if, preceded(multispace0, parse_statement))),
                preceded(multispace0, tag("END IF")),
            )(input)?;
            (input, else_block)
        }
    };

    Ok((
        input,
        IfStatement {
            condition: condition,
            then_body: then_block,
            else_body: else_block,
        },
    ))
}

pub fn parse_while_statement(input: &str) -> IResult<&str, LoopStatement> {
    let (input, _) = peek(tag("WHILE"))(input)?;

    let (input, (condition, (body, _))) = tuple((
        preceded(tag("WHILE "), parse_entry_expression),
        many_till(parse_statement, preceded(multispace0, tag("END WHILE"))),
    ))(input)?;

    Ok((
        input,
        LoopStatement::While(WhileStatement {
            condition: condition,
            body: body,
        }),
    ))
}

pub fn parse_repeat_statement(input: &str) -> IResult<&str, LoopStatement> {
    let (input, (dialog, (body, _))) = 
        terminated(
            tuple((
                preceded(tag("REPEAT "), parse_variable_reference),
                many_till(parse_statement, preceded(multispace0, tag("END REPEAT")))
            )), 
            multispace0
        )(input)?;

    Ok((
        input,
        LoopStatement::Repeat(RepeatStatement {
            dialog: dialog,
            body: body,
        }),
    ))
}

pub fn parse_return_statement(input: &str) -> IResult<&str, Expression> {
    parse_entry_expression(input)
}

pub fn parse_comment_statement(input: &str) -> IResult<&str, CommentStatement> {
    let (input, comment) = preceded(tag("//"), not_line_ending)(input)?;
    Ok((input, CommentStatement { comment: comment }))
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((
        map(parse_increment_instruction, Instruction::Script),
        map(parse_decrement_instruction, Instruction::Script),
        map(parse_quit_instruction, Instruction::Script),
        map(parse_erase_instruction, Instruction::Script),
        map(parse_set_instruction, Instruction::Script),
        map(parse_add_instruction, Instruction::Script),
        map(parse_default_instruction, Instruction::Script),
        map(parse_assemble_instruction, Instruction::Script),
        parse_display_instruction,
    ))(input)
}

pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    preceded(
        multispace0,
        alt((
            map(context("Comment", parse_comment_statement), Statement::Comment),
            map(context("If statement", parse_if_statement), Statement::If),
            map(context("While statement", parse_while_statement), Statement::Loop),
            map(context("Repeat statement", parse_repeat_statement), Statement::Loop),
            map(context("Instruction", parse_instruction), Statement::Instruction),
            map(context("Return statment", parse_return_statement), Statement::Return),
        )),
    )(input)
}

pub fn parse_quit_instruction(input: &str) -> IResult<&str, ScriptInstruction> {
    map(preceded(tag("QUIT"), rest), |_| ScriptInstruction::Quit)(input)
}


pub fn parse_increment_instruction(input: &str) -> IResult<&str, ScriptInstruction> {
    map(preceded(tag("INCREMENT "), parse_variable_reference), ScriptInstruction::Increment)(input)
}

pub fn parse_decrement_instruction(input: &str) -> IResult<&str, ScriptInstruction> {
    map(preceded(tag("DECREMENT "), parse_variable_reference), ScriptInstruction::Decrement)(input)
}

pub fn parse_assemble_instruction(input: &str) -> IResult<&str, ScriptInstruction> {
    map(preceded(tag("ASSEMBLE "), parse_variable_reference), ScriptInstruction::Assemble)(input)
}

pub fn parse_set_instruction(input: &str) -> IResult<&str, ScriptInstruction> {
    map(
        preceded(
            tag("SET "),
            separated_pair(
                parse_variable_reference,
                preceded(multispace0, tag("TO ")),
                parse_entry_expression,
            ),
        ),
        |(v, r)| ScriptInstruction::Set(v, r),
    )(input)
}

pub fn parse_erase_instruction(input: &str) -> IResult<&str, ScriptInstruction> {
    map(preceded(tag("ERASE "), parse_variable_reference), |v| ScriptInstruction::Erase(v))(input)
}

pub fn parse_add_instruction(input: &str) -> IResult<&str, ScriptInstruction> {
    map(
        preceded(
            tag("ADD "),
            separated_pair(
                parse_entry_expression,
                preceded(multispace0, tag("TO ")),
                parse_variable_reference,
            ),
        ),
        |(e, v)| ScriptInstruction::Add(v, e),
    )(input)
}

pub fn parse_default_instruction(input: &str) -> IResult<&str, ScriptInstruction> {
    map(
        preceded(
            tag("DEFAULT "),
            separated_pair(
                parse_variable_reference,
                delimited(multispace0, tag("TO"), multispace1),
                parse_entry_expression,
            ),
        ),
        |(v, r)| ScriptInstruction::Default(v, r),
    )(input)
}
pub fn parse_function_call(input: &str) -> IResult<&str, FunctionCall> {
    let (input, name) = parse_variable_reference(input)?;

    let (input, args) = parenthesized(
        separated_list(
            tag(","),
            delimited(multispace0, parse_entry_expression, multispace0),
        ),
        '(',
        ')',
    )(input)?;
    Ok((
        input,
        FunctionCall {
            name: name.name,
            args: args,
        },
    ))
}

pub fn parse_indexer(input: &str) -> IResult<&str, FunctionCall> {
    let (input, args) = parenthesized(
        separated_list(
            tag(","),
            delimited(multispace0, parse_entry_expression, multispace0),
        ),
        '[',
        ']',
    )(input)?;

    Ok((
        input,
        FunctionCall {
            name: "",
            args: args,
        },
    ))
}

pub fn parse(input: &str) -> IResult<&str, Script> {
    let (input, body) = many0(parse_statement)(input)?;
    Ok((input, Script { body: body }))
}
