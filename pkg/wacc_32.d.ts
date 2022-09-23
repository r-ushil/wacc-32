/* tslint:disable */
/* eslint-disable */
/**
* @param {string} program
* @param {boolean} analysis_only
* @returns {CompileResult}
*/
export function compile(program: string, analysis_only: boolean): CompileResult;
/**
*/
export class CompileResult {
  free(): void;
/**
*/
  readonly asm_output: string;
/**
*/
  readonly terminal_output: string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_compileresult_free: (a: number) => void;
  readonly compileresult_terminal_output: (a: number, b: number) => void;
  readonly compileresult_asm_output: (a: number, b: number) => void;
  readonly compile: (a: number, b: number, c: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
