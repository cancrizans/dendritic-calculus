/* tslint:disable */
/* eslint-disable */
/**
* @param {string} source
* @returns {DendriticProgram}
*/
export function compile(source: string): DendriticProgram;
/**
*/
export class DendriticProgram {
  free(): void;
/**
* @returns {string}
*/
  as_assembly(): string;
/**
* @returns {DendronValue}
*/
  run_on_zero(): DendronValue;
}
/**
*/
export class DendronValue {
  free(): void;
/**
* @returns {string}
*/
  to_string(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_dendriticprogram_free: (a: number, b: number) => void;
  readonly dendriticprogram_as_assembly: (a: number, b: number) => void;
  readonly dendriticprogram_run_on_zero: (a: number, b: number) => void;
  readonly compile: (a: number, b: number, c: number) => void;
  readonly __wbg_dendronvalue_free: (a: number, b: number) => void;
  readonly dendronvalue_to_string: (a: number, b: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
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
