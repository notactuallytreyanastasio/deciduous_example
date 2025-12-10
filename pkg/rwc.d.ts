/* tslint:disable */
/* eslint-disable */

export class WcResult {
  private constructor();
  free(): void;
  [Symbol.dispose](): void;
  lines: number;
  words: number;
  bytes: number;
}

/**
 * Get all counts at once (lines, words, bytes).
 * Returns a JS object with lines, words, and bytes fields.
 */
export function count_all(text: string): WcResult;

/**
 * Count the number of bytes in the given text.
 * This matches the behavior of `wc -c`.
 */
export function count_bytes(text: string): number;

/**
 * Count the number of lines in the given text.
 * A line is defined by the presence of a newline character.
 * This matches the behavior of `wc -l`.
 */
export function count_lines(text: string): number;

/**
 * Count the number of words in the given text.
 * Words are separated by whitespace (spaces, tabs, newlines).
 * This matches the behavior of `wc -w`.
 */
export function count_words(text: string): number;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_get_wcresult_bytes: (a: number) => number;
  readonly __wbg_get_wcresult_lines: (a: number) => number;
  readonly __wbg_get_wcresult_words: (a: number) => number;
  readonly __wbg_set_wcresult_bytes: (a: number, b: number) => void;
  readonly __wbg_set_wcresult_lines: (a: number, b: number) => void;
  readonly __wbg_set_wcresult_words: (a: number, b: number) => void;
  readonly __wbg_wcresult_free: (a: number, b: number) => void;
  readonly count_all: (a: number, b: number) => number;
  readonly count_bytes: (a: number, b: number) => number;
  readonly count_lines: (a: number, b: number) => number;
  readonly count_words: (a: number, b: number) => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
