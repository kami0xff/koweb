/* tslint:disable */
/* eslint-disable */
/**
* @param {string} make_text_js
* @returns {any}
*/
export function get_graph_rust(make_text_js: string): any;
/**
* @param {string} string_js
* @returns {string}
*/
export function get_string_js(string_js: string): string;
/**
*/
export function increment_test(): void;
/**
* @param {string} cmds_from_js
* @param {boolean} eta
* @param {boolean} no_scope
* @param {boolean} no_infer
* @param {boolean} no_check
*/
export function run_test(cmds_from_js: string, eta: boolean, no_scope: boolean, no_infer: boolean, no_check: boolean): void;
/**
* @param {any} programs
* @param {string} module_to_run
* @param {boolean} eta
* @param {boolean} no_scope
* @param {boolean} no_infer
* @param {boolean} no_check
* @returns {any}
*/
export function run_multiple(programs: any, module_to_run: string, eta: boolean, no_scope: boolean, no_infer: boolean, no_check: boolean): any;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly get_graph_rust: (a: number, b: number) => number;
  readonly get_string_js: (a: number, b: number, c: number) => void;
  readonly increment_test: () => void;
  readonly run_test: (a: number, b: number, c: number, d: number, e: number, f: number) => void;
  readonly run_multiple: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly _dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hdae24a6a5ca08f7a: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly wasm_bindgen__convert__closures__invoke2_mut__h003c824777088503: (a: number, b: number, c: number, d: number) => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
