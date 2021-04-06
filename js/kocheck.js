
// import {init_editor} from './editor.js';
import init, {run_test, greeting} from '../pkg/koweb.js';
// import * as wasm from '../pkg/koweb.js';

function remove_all_errors_dom () {
    document.querySelectorAll('.error').forEach(e => e.remove());
}

//when this is run all error tags should be cleared 
function get_string_from_url (context_id) {
    remove_all_errors_dom();
    const url = document.getElementById("url").value;
    if(url != ""){
        fetch(url).then(result => {
            result.text().then(string => console.log(string)).catch((err) => {
                console.log(err);
                display_error_onpage("Given Url is not valid",context_id)
            });
        }).catch((err) => {
            console.log(err);
            display_error_onpage("Given Url is not valid",context_id);
        });
    }
    else {
        display_error_onpage("Empty url field", context_id)
    }
}

function display_error_onpage (error_msg, context) {
    var error_msg_dom = document.createElement("p");
    var text = document.createTextNode(error_msg);
    error_msg_dom.classList.add("error")
    error_msg_dom.classList.add("bounce-in")
    error_msg_dom.appendChild(text);
    var element = document.getElementById(context);
    element.appendChild(error_msg_dom);
}

function print_options(){
    console.log(window.editor);
    console.log(document.querySelector("#eta").checked);
    console.log(document.querySelector("#no_scope").checked);
    console.log(document.querySelector("#no_infer").checked);
    console.log(document.querySelector("#no_check").checked);
}

//javascript and Webassembly share the same execution thread when you execute wasm within javascript the javascript halts and vice versa
//still some erors loadings idk 

console.log(window.editor);


async function run() {
    const wasm = await init();
    window.wasm = wasm;
    console.log(wasm);
    var testing = await window.editor.getValue();
    console.log("this is testing ::: ", testing);
    greeting();
    run_test(window.editor.getValue(), document.getElementById("eta").checked, document.getElementById("no_scope").checked , document.getElementById("no_infer").checked , document.getElementById("no_check").checked);
}

print_options();
console.log(window.wasm)


var run_url = document.getElementById("run_url").onclick = () => {
    get_string_from_url("url_operations");
} 

var run_button = document.getElementById("run");
run_button.onclick = async () => {
    await run();
}
