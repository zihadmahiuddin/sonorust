use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, alphanumeric1, multispace0},
    combinator::{eof, map, recognize},
    error::ParseError,
    multi::{many0, separated_list0},
    number::float,
    sequence::delimited,
};

use sonorust_ir::IRValue;

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
pub enum Expr {
    Float(IRValue),
    Ident(String),
    Array(Vec<Expr>),
}

#[derive(Debug, PartialEq)]
#[cfg_attr(feature = "wasm", derive(tsify::Tsify))]
pub enum Statement {
    Assign {
        target: String,
        opcode: Option<String>,
        args: Vec<Expr>,
    },
    Return(String),
}

fn ws<'a, F, O, E>(inner: F) -> impl Parser<&'a str, Output = O, Error = E>
where
    F: Parser<&'a str, Output = O, Error = E>,
    E: ParseError<&'a str>,
{
    delimited(multispace0, inner, multispace0)
}

fn parse_ident(input: &str) -> IResult<&str, String> {
    map(
        recognize((
            alt((alpha1, tag("_"))),
            many0(alt((alphanumeric1, tag("_")))),
        )),
        String::from,
    )
    .parse(input)
}

fn parse_expr(input: &str) -> IResult<&str, Expr> {
    alt((
        map(float(), Expr::Float),
        map(parse_ident, Expr::Ident),
        parse_array,
    ))
    .parse(input)
}

fn parse_array(input: &str) -> IResult<&str, Expr> {
    map(
        delimited(
            ws(tag("[")),
            separated_list0(ws(tag(",")), parse_expr),
            ws(tag("]")),
        ),
        Expr::Array,
    )
    .parse(input)
}

fn parse_assignment(input: &str) -> IResult<&str, Statement> {
    let (input, target) = ws(parse_ident).parse(input)?;
    let (input, _) = ws(tag("=")).parse(input)?;

    let (input, (opcode, args)) = alt((
        map(
            (
                ws(parse_ident),
                delimited(
                    ws(tag("(")),
                    separated_list0(ws(tag(",")), parse_expr),
                    ws(tag(")")),
                ),
            ),
            |(op, args)| (Some(op), args),
        ),
        map(parse_expr, |expr| (None, vec![expr])),
    ))
    .parse(input)?;

    let (input, _) = ws(tag(";")).parse(input)?;

    Ok((
        input,
        Statement::Assign {
            target,
            opcode,
            args,
        },
    ))
}

fn parse_return(input: &str) -> IResult<&str, Statement> {
    let (input, _) = ws(tag("return")).parse(input)?;
    let (input, target) = ws(parse_ident).parse(input)?;
    let (input, _) = ws(tag(";")).parse(input)?;

    Ok((input, Statement::Return(target)))
}

pub fn parse_statement(input: &str) -> IResult<&str, Statement> {
    alt((parse_return, parse_assignment)).parse(input)
}

pub fn parse_script(input: &str) -> IResult<&str, Vec<Statement>> {
    let (input, stmts) = many0(ws(parse_statement)).parse(input)?;
    let (input, _) = multispace0(input)?;
    let (input, _) = eof(input)?;
    Ok((input, stmts))
}
