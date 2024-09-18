use std::{fmt::Display, str::FromStr};

use super::dendrons::Dendron;
use itertools::Itertools;
use num_bigint::{BigUint, ParseBigIntError};
use pest_derive::Parser;
use pest::{iterators::Pair, Parser};

#[derive(Debug)]
pub enum Error{
    PestError(pest::error::Error<Rule>),
    IntParsingError(ParseBigIntError)
}

impl Display for Error{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Error::PestError(pe) => write!(f,"Pest parsing error: {}",pe),
            Error::IntParsingError(pbie) => write!(f,"{}",pbie) 
        }
    }
}

impl From<pest::error::Error<Rule>> for Error{
    fn from(value: pest::error::Error<Rule>) -> Self {
        Error::PestError(value)
    }
}
impl From<ParseBigIntError> for Error{
    fn from(value: ParseBigIntError) -> Self {
        Error::IntParsingError(value)
    }
}


#[derive(Parser)]
#[grammar = "dc_grammar.pest"]
struct DCParser;

#[derive(Debug)]
pub struct CodeBlock(Vec<Instruction>);
impl CodeBlock{
    pub fn instructions(&self) -> &Vec<Instruction>{
        &self.0
    }
}
impl Display for CodeBlock{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0.iter().join(", "))
    }
}

#[derive(Debug)]
pub enum Instruction{
    Add(Dendron),
    Divsub{
        divisor : Dendron,
        replacement : Dendron
    },
    Prune(CodeBlock)
}

impl Display for Instruction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Instruction::Add(k) => write!(f,"+= {}",k),
            Instruction::Divsub { divisor, replacement } => write!(f,"/ {} > {}",divisor,replacement),
            Instruction::Prune(block) => write!(f,"{{{}}}", block)
        }
    }
}

fn parse_literal(pair : Pair<Rule>) -> Result<Dendron,Error>{
    let dend = match pair.as_rule(){
        Rule::nat => {
            let istr = pair.as_str();
            let integer = BigUint::from_str(istr)?;
            Dendron::from_int(integer)
        },
        Rule::monomial =>{
            let mut pair = pair.into_inner();
            let multiplier = parse_literal(pair.next().unwrap())?;
            let exp = parse_literal(pair.next().unwrap())?;
            Dendron::mul(multiplier,exp)
        },
        Rule::exp => {
            let inner = pair.into_inner().next().unwrap();
            Dendron::exp(parse_literal(inner)?)
        },
        Rule::literal => {
            let terms : Result<Vec<Dendron>,Error> = pair.into_inner()
            .map(parse_literal).collect();
            Dendron::sum(terms?)
        }
        _ => panic!("Literal parse error {:?}",pair.as_rule())
    };

    Ok(dend)
}

fn parse_instruction(pair : Pair<Rule>) -> Result<Instruction,Error>{
    
    let instr = match pair.as_rule(){
        Rule::instruction => {
            let instr_pair = pair.into_inner().next().unwrap();
            match instr_pair.as_rule(){
                Rule::add => Instruction::Add(
                    parse_literal(instr_pair.into_inner().next().unwrap())?
                ),
                Rule::divsub => {
                    let mut instr_pair = instr_pair.into_inner();
                    Instruction::Divsub { divisor: 
                        parse_literal(instr_pair.next().unwrap())?, 
                        replacement: 
                        parse_literal(instr_pair.next().unwrap())? }
                },
                Rule::prune => Instruction::Prune(
                    parse_block(instr_pair.into_inner().next().unwrap())?
                ),
                _ => unreachable!()
            }
        },
        _ => unreachable!()
    };
    Ok(instr)
}
fn parse_block(pair : Pair<Rule>) -> Result<CodeBlock,Error>{
    match pair.as_rule() {
        Rule::block => {},
        _ => unreachable!()
    };
    let crap : Result<Vec<Instruction>,Error> = pair
        .into_inner().into_iter()
        .map(parse_instruction).collect();
    Ok(CodeBlock(crap?))

}

pub fn parse_literal_source(src : &str) -> Result<Dendron,Error>{
    let mut pairs = DCParser::parse(Rule::literal, src)?;
    assert_eq!(pairs.len(),1);
    let pair = pairs.next().unwrap();
    parse_literal(pair)
}

pub fn parse_dc_source(src : &str) -> Result<CodeBlock,Error>{
    let mut pairs = DCParser::parse(Rule::program, src)?;
    // println!("{:?}",pairs);
    assert_eq!(pairs.len(),1);
    let pair = pairs.next().unwrap();

    parse_block(pair)
}