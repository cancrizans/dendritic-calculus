use std::{fmt::Display, str::FromStr};

use super::dendrons::Dendron;
use itertools::Itertools;
use num_bigint::{BigUint, ParseBigIntError};
use pest_derive::Parser;
use pest::{iterators::Pair, Parser};

#[derive(Debug)]
pub enum CompilationError{
    PestError(pest::error::Error<Rule>),
    IntParsingError(ParseBigIntError),
    DivisionByZero,
}

impl Display for CompilationError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            CompilationError::PestError(pe) => write!(f,"Pest parsing error: {}",pe),
            CompilationError::IntParsingError(pbie) => write!(f,"{}",pbie) ,
            CompilationError::DivisionByZero => write!(f,"Division by zero-pattern."),
        }
    }
}

impl From<pest::error::Error<Rule>> for CompilationError{
    fn from(value: pest::error::Error<Rule>) -> Self {
        CompilationError::PestError(value)
    }
}
impl From<ParseBigIntError> for CompilationError{
    fn from(value: ParseBigIntError) -> Self {
        CompilationError::IntParsingError(value)
    }
}

trait Optimization{
    fn optimize(self) -> Self;
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

    pub fn as_tex(&self) -> String{
        format!("{}",
            self.as_tex_inner(0)
        )
    }
    pub fn as_tex_inner(&self, indent : usize) -> String{
        let prefix : String = (0..indent*4).map(|_|' ').collect();
        self.instructions().iter().map(|i|
                format!(r"{}* {}",prefix,i.as_tex(indent))
            ).join("\n")
    }
}
impl Display for CodeBlock{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.0.iter().join(", "))
    }
}
impl Optimization for CodeBlock{
    fn optimize(self) -> Self {
        CodeBlock(
            self.0.into_iter().map(|i|i.optimize()).collect()
        )
    }
}


#[derive(Debug)]
pub enum Instruction{
    Add(Dendron),
    Divsub{
        divisor : Dendron,
        replacement : Dendron
    },
    Prune(CodeBlock),
    MoveFinite(Dendron),
    InfLoopNZ,
}

impl Instruction{
    fn as_tex(&self, indent : usize) -> String{
        match self{
            Instruction::Add(k) => format!(r"$\xi \leftarrow \xi + {}$",k.as_tex()),
            Instruction::Divsub { divisor, replacement } => 
                if divisor.is_one(){
                    format!(r"$\xi \leftarrow (\xi \cdot {})$",replacement.as_tex())
                } else {
                    format!(r"$\xi \leftarrow \xi \Big/ \left({} \rightarrow {}\right) $", divisor.as_tex(),replacement.as_tex())
                },
            Instruction::InfLoopNZ => r"if $\xi = \alpha + 1$, infinite loop".to_string(),
            Instruction::MoveFinite(dest) => if dest.is_zero(){
                format!(r"$\xi \leftarrow \lfloor \xi \rfloor$")
            }else {
                format!(r"\xi \leftarrow \lfloor \xi \rfloor + \operatorname{{finite}}(\xi) \cdot \left({}\right)$",dest.as_tex())
            },
            Instruction::Prune(block) => {
                
                format!("while $\\xi = \\alpha + 1$: do $\\xi \\leftarrow \\alpha$ \n{}",
                    block.as_tex_inner(indent+1)
                )
            }

        }
    }
}

impl Display for Instruction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Instruction::Add(k) => write!(f,"+= {}",k),
            Instruction::Divsub { divisor, replacement } => write!(f,"/ {} > {}",divisor,replacement),
            Instruction::Prune(block) => write!(f,"{{{}}}", block),
            Instruction::MoveFinite(destination) => write!(f,"mvf({})",destination),
            Instruction::InfLoopNZ => write!(f,"lnz"),
        }
    }
}

fn optimize_prune(inner : CodeBlock) -> Instruction{
    use Instruction as I;
    match inner.instructions().len(){
        0 => I::MoveFinite(Dendron::zero()),

        1 => match &inner.instructions()[0]{
            I::Add(k) => {
                match k.clone().try_decrement(){

                    Some(..) => I::InfLoopNZ,
                    None => I::MoveFinite(k.clone())
                }


            }//,
            _ => I::Prune(inner.optimize())
        },
        _ => I::Prune(inner.optimize())
    }
}

impl Optimization for Instruction{
    fn optimize(self) -> Self {
        use Instruction as I;
        match self{
            I::Prune(block) => {
                optimize_prune(block)
            }
            _ => self,
        }
    }
}

fn parse_literal(pair : Pair<Rule>) -> Result<Dendron,CompilationError>{
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
            if let Some(inner) = pair.into_inner().next(){
                Dendron::exp(parse_literal(inner)?)
            } else {
                Dendron::exp(Dendron::zero())
            }

        },
        Rule::literal => {
            let terms : Result<Vec<Dendron>,CompilationError> = pair.into_inner()
            .map(parse_literal).collect();
            Dendron::sum(terms?)
        }
        _ => panic!("Literal parse error {:?}",pair.as_rule())
    };

    Ok(dend)
}

fn parse_instruction(pair : Pair<Rule>) -> Result<Instruction,CompilationError>{
    
    let instr = match pair.as_rule(){
        Rule::instruction => {
            let instr_pair = pair.into_inner().next().unwrap();
            match instr_pair.as_rule(){
                Rule::add => Instruction::Add(
                    parse_literal(instr_pair.into_inner().next().unwrap())?
                ),
                Rule::divsub => {
                    let mut instr_pair = instr_pair.into_inner();
                    let divisor = parse_literal(instr_pair.next().unwrap())?;
                    if divisor.is_zero() {
                        return Err(CompilationError::DivisionByZero)
                    }
                    Instruction::Divsub { divisor, 
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
fn parse_block(pair : Pair<Rule>) -> Result<CodeBlock,CompilationError>{
    match pair.as_rule() {
        Rule::block => {},
        _ => unreachable!()
    };
    let crap : Result<Vec<Instruction>,CompilationError> = pair
        .into_inner().into_iter()
        .map(parse_instruction).collect();
    Ok(CodeBlock(crap?))

}

pub fn parse_literal_source(src : &str) -> Result<Dendron,CompilationError>{
    let mut pairs = DCParser::parse(Rule::literal, src)?;
    assert_eq!(pairs.len(),1);
    let pair = pairs.next().unwrap();
    parse_literal(pair)
}

pub fn parse_dc_source(src : &str) -> Result<CodeBlock,CompilationError>{
    let mut pairs = DCParser::parse(Rule::program, src)?;
    // println!("{:?}",pairs);
    assert_eq!(pairs.len(),1);
    let pair = pairs.next().unwrap();

    Ok(parse_block(pair)?.optimize())
}