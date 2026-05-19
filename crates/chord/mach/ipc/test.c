/* Test C file for verifying the build.rs C compilation pipeline.
 *
 * This file is a placeholder for real Mach IPC code and will be removed
 * once the actual `mach/` source tree is populated.
 */

/**
 * Returns a constant test value.
 *
 * Called from Rust via `extern "C" fn chord_test() -> i32` to verify
 * that C functions are linked into the final ELF.
 */

/* Prototype to satisfy -Wmissing-prototypes. */
int chord_test(void);

int chord_test(void) {
    return 42;
}
