/* tslint:disable */
/* eslint-disable */
export function main(): void;
export class TruthTable {
  private constructor();
  free(): void;
}
export class WasmEngine {
  free(): void;
  constructor();
  eval(input: string, assignments: any): boolean;
  check_tautology(input: string): boolean;
  check_contradiction(input: string): boolean;
  check_contingency(input: string): boolean;
  compute_truth_table(input: string): TruthTable;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_wasmengine_free: (a: number, b: number) => void;
  readonly __wbg_truthtable_free: (a: number, b: number) => void;
  readonly wasmengine_new: () => number;
  readonly wasmengine_eval: (a: number, b: number, c: number, d: any) => [number, number, number];
  readonly wasmengine_check_tautology: (a: number, b: number, c: number) => [number, number, number];
  readonly wasmengine_check_contradiction: (a: number, b: number, c: number) => [number, number, number];
  readonly wasmengine_check_contingency: (a: number, b: number, c: number) => [number, number, number];
  readonly wasmengine_compute_truth_table: (a: number, b: number, c: number) => [number, number, number];
  readonly main: () => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
