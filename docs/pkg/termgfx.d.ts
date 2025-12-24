/* tslint:disable */
/* eslint-disable */

export function init(): void;

/**
 * Render an ASCII banner (HTML output)
 */
export function render_banner(text: string): string;

/**
 * Render a styled box with message (HTML output)
 */
export function render_box(message: string, style: string, border: string): string;

/**
 * Render a progress bar (HTML output)
 */
export function render_progress(percent: number, style: string): string;

/**
 * Render a sparkline (plain text - no colors needed)
 */
export function render_sparkline(data: string): string;

/**
 * Render a table (HTML output)
 */
export function render_table(headers: string, rows: string, border: string): string;

/**
 * Render a tree structure from JSON (HTML output)
 */
export function render_tree(json: string): string;

/**
 * Typewriter effect - returns array of partial strings for animation
 */
export function typewriter_frames(message: string): any[];

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly render_banner: (a: number, b: number, c: number) => void;
  readonly render_box: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly render_progress: (a: number, b: number, c: number, d: number) => void;
  readonly render_sparkline: (a: number, b: number, c: number) => void;
  readonly render_table: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly render_tree: (a: number, b: number, c: number) => void;
  readonly typewriter_frames: (a: number, b: number, c: number) => void;
  readonly init: () => void;
  readonly __wbindgen_export: (a: number, b: number, c: number) => void;
  readonly __wbindgen_export2: (a: number, b: number) => number;
  readonly __wbindgen_export3: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
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
