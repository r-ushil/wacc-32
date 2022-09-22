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
