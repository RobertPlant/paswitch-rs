# AGENTS.md

## Code style

`rustfmt` defaults, no `rustfmt.toml`. Run `cargo fmt`, `cargo clippy --all-targets -- -D warnings` and `cargo test` before committing (CI checks all three).

- One module per concern (`pulse.rs`, `paswitch.rs`, `commands.rs`), with the `mod` lines at the top of `main.rs`.
- Never `unwrap()` or `expect()` on anything from outside the program (`pactl` output, user input). Use `?`.
  Some still exist in `pulse.rs` and `main.rs`: replace them when you're in that code anyway. Tests may unwrap.
- New error types are a `thiserror` enum per module, each message a readable sentence, with the underlying
  error kept via `#[from]` / `#[source]`. `anyhow` only in `main`, adding `.context(...)` as errors pass up.
- Check every command's exit status. `Command::output()` is `Ok` when the program exits 1: check
  `status.success()` and put stderr in the error.
- `match` on results, early `return Err(...)` for guards, `Ok(())` last.
- Name conditions before branching on them: `let is_default = sink.name == default;` then `if is_default {`.
- Enums for modes, carrying their data, rather than a flag next to an `Option`. No `_ =>` on our own enums,
  list every variant.
- No `mut` unless there's no other way. Take `&T`, return the new value. Short iterator chains (up to ~3 steps)
  over loops that push.
- Never change a value once it's made, and never through a `&mut` you were handed: a helper that takes
  `&mut Lines` and advances it is editing the caller's state. Build the new value and return it.
- Values first, then effects: work out every value the effects need together, then a blank line, then the
  `pactl` calls and prints. Only a value that needs an effect's result comes after it.
- A list is plural and each item singular, closures included: `groups.find_map(|group| ...)`, never `g`, `t`
  or `s`. No abbreviations.
- A function's name is a verb and its object: `parse_sink_id`, `list_sinks`, `find_sink`. Not a bare noun
  (`sink_id`, `key_value`) or a noun and a preposition. `parse` for text turned into a value, `find` for a
  search that can miss, `read` for terminal input, `is_` / `has_` for a predicate.
- No banner comments over groups of functions. Functions that share a role go in their own module.
- Dispatch with `match`, never a map of functions keyed by name.
- Functions under ~50 lines. Pure logic in plain functions, the `pactl` and terminal calls in the command layer.
- clap derive, with a `///` doc on every argument. Otherwise comments are rare one-line notes on why.
- Tests in a `#[cfg(test)] mod tests` at the bottom of the file, named `test_<what_it_shows>`, one `assert_eq!`
  each.
- No em-dashes in code, comments or docs. Use a comma, hyphen or full stop.
- Commit messages are one line, past tense, full stop, under about 60 characters: `Added a --version flag.`
  Say what changed, not how. A body only for the why the diff can't show.
