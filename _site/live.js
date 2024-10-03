import init,{compile} from './pkg/dentritic_calculus.js'

async function run(){
    await init();
    let src_input = document.getElementById("source");

    let program = null;
    let comp_status_ok = document.getElementById("comp_status_ok");
    let comp_status_err = document.getElementById("comp_status_error");
    comp_status_ok.style.display='none';
    comp_status_err.style.display='none';
    let comp_assembly = document.getElementById("compiled_assembly");

    let output_div = document.getElementById("output");

    let output = null;

    document.getElementById("btn_compile").onclick = function(){
        let source = src_input.value;

        try {
            program = compile(source);

            comp_status_ok.style.display='block';
            comp_status_err.style.display='none';

            comp_assembly.innerText = program.as_assembly();
            
        } catch(error){
            comp_status_ok.style.display='none';
            comp_status_err.style.display='block';
            program = null;

            comp_status_err.innerText = error;
            comp_assembly.innerText = "";
        }

        output_div.innerText = "";
    }

    document.getElementById("btn_run").onclick = function(){
        if (!program) return;
        try {
            output = program.run_on_zero();
            output_div.innerText = output.to_string();
        } catch (error) {
            output_div.innerText = error;
        }
        
    }
}

run();