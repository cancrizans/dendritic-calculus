

use std::fmt::Display;

use super::dendrons::Dendron;
use super::parsing::{CodeBlock,Instruction};

#[derive(Debug)]
pub enum ExecutionError{
    InfiniteLoop
}

impl Display for ExecutionError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            ExecutionError::InfiniteLoop => write!(f,"Infinite loop detected.")
        }
    }
}

pub struct Interpreter{
    xi : Dendron
}

impl Interpreter{
    pub fn new()->Interpreter{
        Interpreter{
            xi : Dendron::zero()
        }
    }

    pub fn set_xi(&mut self, xi : Dendron){
        self.xi = xi;
    }
    pub fn get_xi(&self) -> &Dendron{
        &self.xi
    }

    pub fn execute(&mut self, block : &CodeBlock)->Result<(),ExecutionError>{
        block.instructions().iter()
        .try_for_each(|i|self.execute_instruction(i))
    }

    fn execute_instruction(&mut self, instruction : &Instruction) -> Result<(),ExecutionError>{
        match instruction{
            Instruction::Add(k) 
                => self.xi.add_assign(k),
            Instruction::Divsub { divisor, replacement } 
                => self.xi.divsub(divisor, replacement),
            Instruction::Prune(block) => {
                while let Some(()) = self.xi.try_decrement(){
                    self.execute(block)?
                }
            },
            Instruction::MoveFinite(destination) => {
                self.xi.move_finite(destination)
            },
            Instruction::InfLoopNZ => if let Some(()) = self.xi.try_decrement(){
                Err(ExecutionError::InfiniteLoop)?
            } else {()}
        };
        Ok(())
    }
}