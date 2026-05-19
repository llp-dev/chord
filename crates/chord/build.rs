//! Build script: compiles all `.c` files recursively found under `mach/`
//! into a static archive (`libchord_c.a`) linked into the `chord` seL4
//! root task ELF.
//!
//! # Compiler selection
//!
//! Defaults to system `clang` with `--target=x86_64-unknown-none-elf`.
//! Override by setting the environment variable `CC_x86_64_unknown_none`
//! (e.g., `CC_x86_64_unknown_none=x86_64-elf-gcc`).
//!
//! # Rebuild detection
//!
//! - `cargo::rerun-if-changed=mach/` catches directory-structure changes
//!   (files added/removed/renamed).
//! - Per-file `rerun-if-changed` watches each `.c` and `.h` for content
//!   edits.

use std::path::Path;

fn main() {
    // Rebuild when the directory structure changes (add/remove/rename files)
    println!("cargo::rerun-if-changed=mach/");

    let mut build = cc::Build::new();
    // Enable verbose output so every clang invocation is printed
    build.cargo_debug(true);

    // --- Compiler: default to clang with bare-metal target ---
    // Environment variable CC_x86_64_unknown_none overrides this.
    build.compiler("clang");
    build.flag("--target=x86_64-unknown-none-elf");

    // --- Target (for cc's internal platform detection) ---
    build.target("x86_64-unknown-none");

    // Disable default flags that assume a hosted OS
    build.no_default_flags(true);

    // --- Freestanding environment ---
    build.flag("-ffreestanding");
    build.flag("-fno-builtin");

    // --- Code model and stack (match target spec) ---
    build.flag("-mcmodel=kernel");
    build.flag("-mno-red-zone");

    // --- No SIMD / FPU (kernel must not touch FPU/SIMD casually) ---
    build.flag("-mno-mmx");
    build.flag("-mno-sse");
    build.flag("-mno-sse2");
    build.flag("-mno-sse3");
    build.flag("-mno-ssse3");
    build.flag("-mno-sse4.1");
    build.flag("-mno-sse4.2");
    build.flag("-mno-avx");
    build.flag("-mno-avx2");
    build.flag("-mno-3dnow");
    build.flag("-msoft-float");

    // --- Position-independent code for static PIE linking ---
    build.flag("-fPIE");
    build.flag("-fno-plt");

    // --- Warnings (Mach kernel quality) ---
    build.flag("-Wall");
    build.flag("-Wextra");
    build.flag("-Wstrict-prototypes");
    build.flag("-Wold-style-definition");
    build.flag("-Wmissing-prototypes");

    // --- C dialect: GNU C99 with GNU89 inline semantics ---
    // Mach relies on GNU89 `extern inline` semantics.  `-fgnu89-inline`
    // overrides the C99 inline rules so that `extern inline` functions
    // are emitted in every translation unit (the legacy GCC behavior).
    build.flag("-std=gnu99");
    build.flag("-fgnu89-inline");

    // --- Disable optimizations that break kernel assumptions ---
    // Mach predates C99 and performs type punning that violates strict
    // aliasing rules.  Disabling it prevents miscompilation.
    build.flag("-fno-strict-aliasing");
    // Stack protector requires libc support (__stack_chk_fail) which the
    // kernel does not provide.
    build.flag("-fno-stack-protector");
    // Preserve frame pointers for accurate backtraces.
    build.flag("-fno-omit-frame-pointer");
    // Disable tail-call optimization so backtraces are accurate.
    build.flag("-fno-optimize-sibling-calls");
    // Mach defines its own `log()` macro/function — disable GCC's builtin.
    build.flag("-fno-builtin-log");

    // --- Debug / optimization ---
    build.flag("-g");
    build.flag("-O2");

    // --- Include paths: mach/ root for #include lookups ---
    build.include("mach");

    // --- Recursively discover and compile all .c files ---
    let mach_dir = Path::new("mach");
    if mach_dir.is_dir() {
        add_c_files(&mut build, mach_dir);
    }

    // --- Produce libchord_c.a ---
    build.compile("chord_c");
}

/// Recursively walk `dir`. Add every `.c` file to the compilation;
/// register every `.c` and `.h` file for rebuild detection.
fn add_c_files(build: &mut cc::Build, dir: &Path) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(e) => {
            println!("cargo::warning=Failed to read {}: {}", dir.display(), e);
            return;
        }
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            add_c_files(build, &path);
        } else {
            match path.extension().and_then(|e| e.to_str()) {
                Some("c") => {
                    build.file(&path);
                    println!("cargo::rerun-if-changed={}", path.display());
                }
                Some("h") => {
                    // Headers aren't compiled, but changes should trigger rebuild
                    println!("cargo::rerun-if-changed={}", path.display());
                }
                _ => {}
            }
        }
    }
}
