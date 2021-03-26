
import editor from './editor.js'
import init, { greeting, run_test } from '../pkg/koweb.js';
async function run() {
    await init();
    greeting();
    var value = editor.getValue();
    console.log(value);
    var run = document.getElementById("run");
    run.onclick = run_test(value);

}
run();


//i want to get the cmds from the editor and turn that into an cmd iterator
//in rust
//see how i can access wasm stuff now