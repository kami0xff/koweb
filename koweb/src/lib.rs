use wasm_bindgen::prelude::*;

//parses strings into commands which are then either
use colosseum::unsync::Arena;
// use kontroli::error::SignatureError;
use kontroli::rc::{Intro, Rule, Signature, Typing};
use kontroli::scope::{Command, Symbols};

// use kontroli::error::Error;
use kontroli::error::SymbolsError;

use byte_unit::{Byte, ByteError};
use kocheck::{parse, seq, Error, Event, Opt};

use log::{info, trace, warn, Level};

//not sure what wee alloc does
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

extern crate console_error_panic_hook;
use std::panic;

//i should get the the automatic code format and debugger going as well today
fn init_console_wasm_debug() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    // ...
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

}

//https://github.com/Deducteam/lambdapi/tree/master/editors/vscode

#[wasm_bindgen]
pub fn greeting() {
    alert("debug code is run");
}

fn produce_from_js(
    cmds_from_js: &'static str,
    opt: &Opt,
) -> impl Iterator<Item = Result<Event, Error>> {
    let module = std::iter::once(Ok(Event::Module(vec!["js".to_string()])));
    let cmds = parse(cmds_from_js.as_bytes(), opt).map(|cmd| cmd.map(Event::Command));
    module.chain(cmds)
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn write_to_webpage(event: &kocheck::Event) {
    //i need to match here on the cocheck enum ??
    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().expect("should have a document on window");
    let body = document.body().expect("document should have a body");

    // let val = document.get_element_by_id("console").unwrap();
    let line = document.create_element("div").unwrap();
    let val = document.create_element("p").unwrap();
    let lambda_span = document.create_element("span").unwrap();
    let prompt_span = document.create_element("span").unwrap();
    lambda_span.set_class_name("lambda");
    lambda_span.set_text_content(Some("λ"));
    prompt_span.set_class_name("prompt");
    prompt_span.set_text_content(Some("> "));

    lambda_span.append_child(&prompt_span);
    val.set_class_name("line");
    val.set_text_content(Some(format!("{}", event).as_str()));

    line.append_child(&lambda_span);
    line.append_child(&val);

    //now it needs to go in the ouput
    let output = document.get_element_by_id("output").unwrap();
    output.append_child(&line).unwrap();
    // body.append_child(&line).unwrap();
}

//temporary i'll use the definition from bin maybe
pub fn flatten_nested_results<O, I, T, E>(outer: O) -> impl Iterator<Item = Result<T, E>>
where
    O: Iterator<Item = Result<I, E>>,
    I: Iterator<Item = Result<T, E>>,
{
    outer.flat_map(|inner_result| {
        let (v, r) = match inner_result {
            Ok(v) => (Some(v), None),
            Err(e) => (None, Some(Err(e))),
        };
        v.into_iter().flatten().chain(r)
    })
}

fn print_iterator<I>(iter: &mut I)
where
    I: Iterator<Item = Result<Event, Error>>,
{
    for element in iter {
        match element {
            Result::Ok(Event) => log(format!("{}", Event).as_str()),

            Result::Err(Error) => log(format!("something went wrong : {:?}", Error).as_str()),
        }
    }
}

static mut test: i32 = 0;

#[wasm_bindgen()]
pub fn increment() {
    unsafe {
        alert(format!("test : {}", test).as_str());
        test += 1;
    }
}

// pub fn lazy() {
// goal is to get only a piece of the string if there is nothing left to get
// then it returns what ? False
// }

#[wasm_bindgen]
pub fn run_test(
    cmds_from_js: String,
    eta: bool,
    no_scope: bool,
    no_infer: bool,
    no_check: bool,
) -> Result<(), JsValue> {
    // wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));
    console_log::init_with_level(Level::Trace);
    init_console_wasm_debug();
    alert(cmds_from_js.as_str());

    info!("testing the info part");
    // https://stackoverflow.com/questions/19846078/how-to-read-from-chromes-console-in-javascript
    //CAREFULLLLL when something goes wrong in the code i get unreachable in the browser console i need to find a way to get good error messages
    //essayer de virer le static lifetime =)
    //essayer de passer de maniere async le code dans le text editor (lazy)
    //verification de fichier passer par le file system ou url
    //run from : ----
    //error
    // -> String ??? -> (peut etre async lazy reading)???
    //regarder le parse buffer

    //typing reduce convertible
    //something i wrong and i reach unreachable how can i fix that the logging works tho which is really cool

    // env_logger::from_env(Env::default().filter_or("LOG", "warn")).init();
    // log::debug!("hello");
    // Introduce symbol {|Pure.Appt|const|}

    let static_cmds_str = string_to_static_str(cmds_from_js);

    let opt = Opt {
        eta,
        no_scope,
        no_infer,
        no_check,
        buffer: Byte::from_str("64MB").unwrap(),
        channel_capacity: None,
        jobs: None,
        files: vec![],
    };

    let iter = produce_from_js(static_cmds_str, &opt);

    let mut iter = Box::new(iter).inspect(|r| r.iter().for_each(|event| write_to_webpage(event)));

    seq::consume(iter, &opt).expect("something went wrong in the consume");

    //i want to be able to get error messages and line numbers
    //then i want to be able to run it with the run button
    //make the loading bar thing
    //calling parse on my string when passing it in parse .as_bytes()
    // https://stackoverflow.com/questions/32674905/pass-string-to-function-taking-read-trait

    Ok(())
}
