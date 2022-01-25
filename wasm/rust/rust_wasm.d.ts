/* tslint:disable */
/* eslint-disable */
/**
* @returns {string}
*/
export function randomGame(): string;
/**
* @param {string} board
* @returns {string}
*/
export function displayGame(board: string): string;
/**
* @param {string} board
* @param {string} moves
* @returns {string}
*/
export function evaluateMoves(board: string, moves: string): string;
/**
* @param {string} board
* @param {string} moves
* @returns {string}
*/
export function evaluateMovesWire(board: string, moves: string): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly randomGame: (a: number) => void;
  readonly displayGame: (a: number, b: number, c: number) => void;
  readonly evaluateMoves: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly evaluateMovesWire: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
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
