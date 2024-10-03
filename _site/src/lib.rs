mod parsing;
mod dendrons;
mod interpreter;
pub use dendrons::Dendron;
pub use parsing::{
    CodeBlock,parse_dc_source,parse_literal_source,
    CompilationError};
pub use interpreter::{Interpreter,ExecutionError};

#[cfg(target_arch = "wasm32")]    
pub mod wasm32{
    use wasm_bindgen::prelude::*;

    use super::*;

    impl From<CompilationError> for JsValue{
        fn from(value: CompilationError) -> Self {
            JsValue::from_str(&format!("{}",value))
        }
    }

    impl From<ExecutionError> for JsValue{
        fn from(value: ExecutionError) -> Self {
            JsValue::from_str(&format!("{}",value))
        }
    }

    #[wasm_bindgen]
    pub struct DendriticProgram{
        code_block : CodeBlock,
    }

    #[wasm_bindgen]
    impl DendriticProgram{
        pub fn as_assembly(&self) -> String{
            format!("{}",self.code_block)
        }

        pub fn run_on_zero(&self) -> Result<DendronValue,ExecutionError>{
            let mut interp = Interpreter::new();
            interp.set_xi(Dendron::zero());

            interp.execute(&self.code_block)?;

            Ok(DendronValue(interp.get_xi().clone()))
        }
    }

    #[wasm_bindgen]
    pub fn compile(source : String) -> Result<DendriticProgram,CompilationError>{
        Ok(DendriticProgram{
            code_block : parse_dc_source(&source)?
        })
    }


    #[wasm_bindgen]
    pub struct DendronValue(Dendron);

    #[wasm_bindgen]
    impl DendronValue{
        pub fn to_string(&self) -> String{
            format!("{}",self.0)
        }
    }
}