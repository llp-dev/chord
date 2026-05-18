**clap_builder > builder > command**

# Module: builder::command

## Contents

**Structs**

- [`Command`](#command) - Build a command-line interface.

---

## clap_builder::builder::command::Command

*Struct*

Build a command-line interface.

This includes defining arguments, subcommands, parser behavior, and help output.
Once all configuration is complete,
the [`Command::get_matches`] family of methods starts the runtime-parsing
process. These methods then return information about the user supplied
arguments (or lack thereof).

When deriving a [`Parser`][crate::Parser], you can use
[`CommandFactory::command`][crate::CommandFactory::command] to access the
`Command`.

- [Basic API][crate::Command#basic-api]
- [Application-wide Settings][crate::Command#application-wide-settings]
- [Command-specific Settings][crate::Command#command-specific-settings]
- [Subcommand-specific Settings][crate::Command#subcommand-specific-settings]
- [Reflection][crate::Command#reflection]

# Examples

```no_run
# use clap_builder as clap;
# use clap::{Command, Arg};
let m = Command::new("My Program")
    .author("Me, me@mail.com")
    .version("1.0.2")
    .about("Explains in brief what the program does")
    .arg(
        Arg::new("in_file")
    )
    .after_help("Longer explanation to appear after the options when \
                 displaying the help information from --help or -h")
    .get_matches();

// Your program logic starts here...
```
[`Command::get_matches`]: Command::get_matches()

**Methods:**

- `fn get_display_name(self: &Self) -> Option<&str>` - Get the name of the binary.
- `fn get_bin_name(self: &Self) -> Option<&str>` - Get the name of the binary.
- `fn set_bin_name<impl Into<String>>(self: & mut Self, name: impl Trait)` - Set binary name. Uses `&mut self` instead of `self`.
- `fn get_name(self: &Self) -> &str` - Get the name of the cmd.
- `fn get_name_and_visible_aliases(self: &Self) -> Vec<&str>` - Get all known names of the cmd (i.e. primary name and visible aliases).
- `fn get_version(self: &Self) -> Option<&str>` - Get the version of the cmd.
- `fn get_long_version(self: &Self) -> Option<&str>` - Get the long version of the cmd.
- `fn get_display_order(self: &Self) -> usize` - Get the placement within help
- `fn get_author(self: &Self) -> Option<&str>` - Get the authors of the cmd.
- `fn get_short_flag(self: &Self) -> Option<char>` - Get the short flag of the subcommand.
- `fn get_long_flag(self: &Self) -> Option<&str>` - Get the long flag of the subcommand.
- `fn get_about(self: &Self) -> Option<&StyledStr>` - Get the help message specified via [`Command::about`].
- `fn get_long_about(self: &Self) -> Option<&StyledStr>` - Get the help message specified via [`Command::long_about`].
- `fn is_flatten_help_set(self: &Self) -> bool` - Get the custom section heading specified via [`Command::flatten_help`].
- `fn get_next_help_heading(self: &Self) -> Option<&str>` - Get the custom section heading specified via [`Command::next_help_heading`].
- `fn get_visible_aliases(self: &Self) -> impl Trait` - Iterate through the *visible* aliases for this subcommand.
- `fn get_visible_short_flag_aliases(self: &Self) -> impl Trait` - Iterate through the *visible* short aliases for this subcommand.
- `fn get_visible_long_flag_aliases(self: &Self) -> impl Trait` - Iterate through the *visible* long aliases for this subcommand.
- `fn get_all_aliases(self: &Self) -> impl Trait` - Iterate through the set of *all* the aliases for this subcommand, both visible and hidden.
- `fn get_all_short_flag_aliases(self: &Self) -> impl Trait` - Iterate through the set of *all* the short aliases for this subcommand, both visible and hidden.
- `fn get_all_long_flag_aliases(self: &Self) -> impl Trait` - Iterate through the set of *all* the long aliases for this subcommand, both visible and hidden.
- `fn get_aliases(self: &Self) -> impl Trait` - Iterate through the *hidden* aliases for this subcommand.
- `fn get_color(self: &Self) -> ColorChoice` - Should we color the output?
- `fn get_styles(self: &Self) -> &Styles` - Return the current `Styles` for the `Command`
- `fn get_subcommands(self: &Self) -> impl Trait` - Iterate through the set of subcommands, getting a reference to each.
- `fn get_subcommands_mut(self: & mut Self) -> impl Trait` - Iterate through the set of subcommands, getting a mutable reference to each.
- `fn has_subcommands(self: &Self) -> bool` - Returns `true` if this `Command` has subcommands.
- `fn get_subcommand_help_heading(self: &Self) -> Option<&str>` - Returns the help heading for listing subcommands.
- `fn get_subcommand_value_name(self: &Self) -> Option<&str>` - Returns the subcommand value name.
- `fn get_before_help(self: &Self) -> Option<&StyledStr>` - Returns the help heading for listing subcommands.
- `fn get_before_long_help(self: &Self) -> Option<&StyledStr>` - Returns the help heading for listing subcommands.
- `fn get_after_help(self: &Self) -> Option<&StyledStr>` - Returns the help heading for listing subcommands.
- `fn get_after_long_help(self: &Self) -> Option<&StyledStr>` - Returns the help heading for listing subcommands.
- `fn find_subcommand<impl AsRef<std::ffi::OsStr>>(self: &Self, name: impl Trait) -> Option<&Command>` - Find subcommand such that its name or one of aliases equals `name`.
- `fn find_subcommand_mut<impl AsRef<std::ffi::OsStr>>(self: & mut Self, name: impl Trait) -> Option<& mut Command>` - Find subcommand such that its name or one of aliases equals `name`, returning
- `fn get_groups(self: &Self) -> impl Trait` - Iterate through the set of groups.
- `fn get_arguments(self: &Self) -> impl Trait` - Iterate through the set of arguments.
- `fn get_positionals(self: &Self) -> impl Trait` - Iterate through the *positionals* arguments.
- `fn get_opts(self: &Self) -> impl Trait` - Iterate through the *options*.
- `fn get_arg_conflicts_with(self: &Self, arg: &Arg) -> Vec<&Arg>` - Get a list of all arguments the given argument conflicts with.
- `fn is_no_binary_name_set(self: &Self) -> bool` - Report whether [`Command::no_binary_name`] is set
- `fn is_dont_delimit_trailing_values_set(self: &Self) -> bool` - Report whether [`Command::dont_delimit_trailing_values`] is set
- `fn is_disable_version_flag_set(self: &Self) -> bool` - Report whether [`Command::disable_version_flag`] is set
- `fn is_propagate_version_set(self: &Self) -> bool` - Report whether [`Command::propagate_version`] is set
- `fn is_next_line_help_set(self: &Self) -> bool` - Report whether [`Command::next_line_help`] is set
- `fn is_disable_help_flag_set(self: &Self) -> bool` - Report whether [`Command::disable_help_flag`] is set
- `fn is_disable_help_subcommand_set(self: &Self) -> bool` - Report whether [`Command::disable_help_subcommand`] is set
- `fn is_disable_colored_help_set(self: &Self) -> bool` - Report whether [`Command::disable_colored_help`] is set
- `fn is_arg_required_else_help_set(self: &Self) -> bool` - Report whether [`Command::arg_required_else_help`] is set
- `fn is_allow_missing_positional_set(self: &Self) -> bool` - Report whether [`Command::allow_missing_positional`] is set
- `fn is_hide_set(self: &Self) -> bool` - Report whether [`Command::hide`] is set
- `fn is_subcommand_required_set(self: &Self) -> bool` - Report whether [`Command::subcommand_required`] is set
- `fn is_allow_external_subcommands_set(self: &Self) -> bool` - Report whether [`Command::allow_external_subcommands`] is set
- `fn get_external_subcommand_value_parser(self: &Self) -> Option<&super::ValueParser>` - Configured parser for values passed to an external subcommand
- `fn is_args_conflicts_with_subcommands_set(self: &Self) -> bool` - Report whether [`Command::args_conflicts_with_subcommands`] is set
- `fn is_subcommand_precedence_over_arg_set(self: &Self) -> bool` - Report whether [`Command::subcommand_precedence_over_arg`] is set
- `fn is_subcommand_negates_reqs_set(self: &Self) -> bool` - Report whether [`Command::subcommand_negates_reqs`] is set
- `fn is_multicall_set(self: &Self) -> bool` - Report whether [`Command::multicall`] is set
- `fn new<impl Into<Str>>(name: impl Trait) -> Self` - Creates a new instance of an `Command`.
- `fn arg<impl Into<Arg>>(self: Self, a: impl Trait) -> Self` - Adds an [argument] to the list of valid possibilities.
- `fn args<impl Into<Arg>, impl IntoIterator<Item = impl Into<Arg>>>(self: Self, args: impl Trait) -> Self` - Adds multiple [arguments] to the list of valid possibilities.
- `fn mut_arg<F, impl AsRef<str>>(self: Self, arg_id: impl Trait, f: F) -> Self` - Allows one to mutate an [`Arg`] after it's been added to a [`Command`].
- `fn mut_args<F>(self: Self, f: F) -> Self` - Allows one to mutate all [`Arg`]s after they've been added to a [`Command`].
- `fn mut_group<F, impl AsRef<str>>(self: Self, arg_id: impl Trait, f: F) -> Self` - Allows one to mutate an [`ArgGroup`] after it's been added to a [`Command`].
- `fn mut_subcommand<F, impl AsRef<str>>(self: Self, name: impl Trait, f: F) -> Self` - Allows one to mutate a [`Command`] after it's been added as a subcommand.
- `fn mut_subcommands<F>(self: Self, f: F) -> Self` - Allows one to mutate all [`Command`]s after they've been added as subcommands.
- `fn group<impl Into<ArgGroup>>(self: Self, group: impl Trait) -> Self` - Adds an [`ArgGroup`] to the application.
- `fn groups<impl Into<ArgGroup>, impl IntoIterator<Item = impl Into<ArgGroup>>>(self: Self, groups: impl Trait) -> Self` - Adds multiple [`ArgGroup`]s to the [`Command`] at once.
- `fn subcommand<impl Into<Command>>(self: Self, subcmd: impl Trait) -> Self` - Adds a subcommand to the list of valid possibilities.
- `fn subcommands<impl Into<Self>, impl IntoIterator<Item = impl Into<Self>>>(self: Self, subcmds: impl Trait) -> Self` - Adds multiple subcommands to the list of valid possibilities.
- `fn defer(self: Self, deferred: fn(...)) -> Self` - Delay initialization for parts of the `Command`
- `fn debug_assert(self: Self)` - Catch problems earlier in the development cycle.
- `fn error<impl fmt::Display>(self: & mut Self, kind: ErrorKind, message: impl Trait) -> Error` - Custom error message for post-parsing validation
- `fn get_matches(self: Self) -> ArgMatches` - Parse [`env::args_os`], [exiting][Error::exit] on failure.
- `fn get_matches_mut(self: & mut Self) -> ArgMatches` - Parse [`env::args_os`], [exiting][Error::exit] on failure.
- `fn try_get_matches(self: Self) -> ClapResult<ArgMatches>` - Parse [`env::args_os`], returning a [`clap::Result`] on failure.
- `fn get_matches_from<I, T>(self: Self, itr: I) -> ArgMatches` - Parse the specified arguments, [exiting][Error::exit] on failure.
- `fn try_get_matches_from<I, T>(self: Self, itr: I) -> ClapResult<ArgMatches>` - Parse the specified arguments, returning a [`clap::Result`] on failure.
- `fn try_get_matches_from_mut<I, T>(self: & mut Self, itr: I) -> ClapResult<ArgMatches>` - Parse the specified arguments, returning a [`clap::Result`] on failure.
- `fn print_help(self: & mut Self) -> io::Result<()>` - Prints the short help message (`-h`) to [`io::stdout()`].
- `fn print_long_help(self: & mut Self) -> io::Result<()>` - Prints the long help message (`--help`) to [`io::stdout()`].
- `fn render_help(self: & mut Self) -> StyledStr` - Render the short help message (`-h`) to a [`StyledStr`]
- `fn render_long_help(self: & mut Self) -> StyledStr` - Render the long help message (`--help`) to a [`StyledStr`].
- `fn render_version(self: &Self) -> String` - Version message rendered as if the user ran `-V`.
- `fn render_long_version(self: &Self) -> String` - Version message rendered as if the user ran `--version`.
- `fn render_usage(self: & mut Self) -> StyledStr` - Usage statement
- `fn short_flag<impl IntoResettable<char>>(self: Self, short: impl Trait) -> Self` - Sets the short version of the subcommand flag without the preceding `-`.
- `fn long_flag<impl Into<Str>>(self: Self, long: impl Trait) -> Self` - Sets the long version of the subcommand flag without the preceding `--`.
- `fn alias<impl IntoResettable<Str>>(self: Self, name: impl Trait) -> Self` - Sets a hidden alias to this subcommand.
- `fn short_flag_alias<impl IntoResettable<char>>(self: Self, name: impl Trait) -> Self` - Add an alias, which functions as  "hidden" short flag subcommand
- `fn long_flag_alias<impl IntoResettable<Str>>(self: Self, name: impl Trait) -> Self` - Add an alias, which functions as a "hidden" long flag subcommand.
- `fn aliases<impl Into<Str>, impl IntoIterator<Item = impl Into<Str>>>(self: Self, names: impl Trait) -> Self` - Sets multiple hidden aliases to this subcommand.
- `fn short_flag_aliases<impl IntoIterator<Item = char>>(self: Self, names: impl Trait) -> Self` - Add aliases, which function as "hidden" short flag subcommands.
- `fn long_flag_aliases<impl Into<Str>, impl IntoIterator<Item = impl Into<Str>>>(self: Self, names: impl Trait) -> Self` - Add aliases, which function as "hidden" long flag subcommands.
- `fn visible_alias<impl IntoResettable<Str>>(self: Self, name: impl Trait) -> Self` - Sets a visible alias to this subcommand.
- `fn visible_short_flag_alias<impl IntoResettable<char>>(self: Self, name: impl Trait) -> Self` - Add an alias, which functions as  "visible" short flag subcommand
- `fn visible_long_flag_alias<impl IntoResettable<Str>>(self: Self, name: impl Trait) -> Self` - Add an alias, which functions as a "visible" long flag subcommand.
- `fn visible_aliases<impl Into<Str>, impl IntoIterator<Item = impl Into<Str>>>(self: Self, names: impl Trait) -> Self` - Sets multiple visible aliases to this subcommand.
- `fn visible_short_flag_aliases<impl IntoIterator<Item = char>>(self: Self, names: impl Trait) -> Self` - Add aliases, which function as *visible* short flag subcommands.
- `fn visible_long_flag_aliases<impl Into<Str>, impl IntoIterator<Item = impl Into<Str>>>(self: Self, names: impl Trait) -> Self` - Add aliases, which function as *visible* long flag subcommands.
- `fn display_order<impl IntoResettable<usize>>(self: Self, ord: impl Trait) -> Self` - Set the placement of this subcommand within the help.
- `fn hide(self: Self, yes: bool) -> Self` - Specifies that this [`subcommand`] should be hidden from help messages
- `fn subcommand_required(self: Self, yes: bool) -> Self` - If no [`subcommand`] is present at runtime, error and exit gracefully.
- `fn allow_external_subcommands(self: Self, yes: bool) -> Self` - Assume unexpected positional arguments are a [`subcommand`].
- `fn external_subcommand_value_parser<impl IntoResettable<super::ValueParser>>(self: Self, parser: impl Trait) -> Self` - Specifies how to parse external subcommand arguments.
- `fn args_conflicts_with_subcommands(self: Self, yes: bool) -> Self` - Specifies that use of an argument prevents the use of [`subcommands`].
- `fn subcommand_precedence_over_arg(self: Self, yes: bool) -> Self` - Prevent subcommands from being consumed as an arguments value.
- `fn subcommand_negates_reqs(self: Self, yes: bool) -> Self` - Allows [`subcommands`] to override all requirements of the parent command.
- `fn multicall(self: Self, yes: bool) -> Self` - Multiple-personality program dispatched on the binary name (`argv[0]`)
- `fn subcommand_value_name<impl IntoResettable<Str>>(self: Self, value_name: impl Trait) -> Self` - Sets the value name used for subcommands when printing usage and help.
- `fn subcommand_help_heading<impl IntoResettable<Str>>(self: Self, heading: impl Trait) -> Self` - Sets the help heading used for subcommands when printing usage and help.
- `fn no_binary_name(self: Self, yes: bool) -> Self` - Specifies that the parser should not assume the first argument passed is the binary name.
- `fn ignore_errors(self: Self, yes: bool) -> Self` - Try not to fail on parse errors, like missing option values.
- `fn args_override_self(self: Self, yes: bool) -> Self` - Replace prior occurrences of arguments rather than error
- `fn dont_delimit_trailing_values(self: Self, yes: bool) -> Self` - Disables the automatic [delimiting of values][Arg::value_delimiter] after `--` or when [`Arg::trailing_var_arg`]
- `fn color(self: Self, color: ColorChoice) -> Self` - Sets when to color output.
- `fn styles(self: Self, styles: Styles) -> Self` - Sets the [`Styles`] for terminal output
- `fn term_width(self: Self, width: usize) -> Self` - Sets the terminal width at which to wrap help messages.
- `fn max_term_width(self: Self, width: usize) -> Self` - Limit the line length for wrapping help when using the current terminal's width.
- `fn disable_version_flag(self: Self, yes: bool) -> Self` - Disables `-V` and `--version` flag.
- `fn propagate_version(self: Self, yes: bool) -> Self` - Specifies to use the version of the current command for all [`subcommands`].
- `fn next_line_help(self: Self, yes: bool) -> Self` - Places the help string for all arguments and subcommands on the line after them.
- `fn disable_help_flag(self: Self, yes: bool) -> Self` - Disables `-h` and `--help` flag.
- `fn disable_help_subcommand(self: Self, yes: bool) -> Self` - Disables the `help` [`subcommand`].
- `fn disable_colored_help(self: Self, yes: bool) -> Self` - Disables colorized help messages.
- `fn help_expected(self: Self, yes: bool) -> Self` - Panic if help descriptions are omitted.
- `fn hide_possible_values(self: Self, yes: bool) -> Self` - Tells `clap` *not* to print possible values when displaying help information.
- `fn infer_long_args(self: Self, yes: bool) -> Self` - Allow partial matches of long arguments or their [aliases].
- `fn infer_subcommands(self: Self, yes: bool) -> Self` - Allow partial matches of [subcommand] names and their [aliases].
- `fn build(self: & mut Self)` - Prepare for introspecting on all included [`Command`]s
- `fn name<impl Into<Str>>(self: Self, name: impl Trait) -> Self` - (Re)Sets the program's name.
- `fn bin_name<impl IntoResettable<String>>(self: Self, name: impl Trait) -> Self` - Overrides the runtime-determined name of the binary for help and error messages.
- `fn display_name<impl IntoResettable<String>>(self: Self, name: impl Trait) -> Self` - Overrides the runtime-determined display name of the program for help and error messages.
- `fn author<impl IntoResettable<Str>>(self: Self, author: impl Trait) -> Self` - Sets the author(s) for the help message.
- `fn about<impl IntoResettable<StyledStr>>(self: Self, about: impl Trait) -> Self` - Sets the program's description for the short help (`-h`).
- `fn long_about<impl IntoResettable<StyledStr>>(self: Self, long_about: impl Trait) -> Self` - Sets the program's description for the long help (`--help`).
- `fn after_help<impl IntoResettable<StyledStr>>(self: Self, help: impl Trait) -> Self` - Free-form help text for after auto-generated short help (`-h`).
- `fn after_long_help<impl IntoResettable<StyledStr>>(self: Self, help: impl Trait) -> Self` - Free-form help text for after auto-generated long help (`--help`).
- `fn before_help<impl IntoResettable<StyledStr>>(self: Self, help: impl Trait) -> Self` - Free-form help text for before auto-generated short help (`-h`).
- `fn before_long_help<impl IntoResettable<StyledStr>>(self: Self, help: impl Trait) -> Self` - Free-form help text for before auto-generated long help (`--help`).
- `fn version<impl IntoResettable<Str>>(self: Self, ver: impl Trait) -> Self` - Sets the version for the short version (`-V`) and help messages.
- `fn long_version<impl IntoResettable<Str>>(self: Self, ver: impl Trait) -> Self` - Sets the version for the long version (`--version`) and help messages.
- `fn override_usage<impl IntoResettable<StyledStr>>(self: Self, usage: impl Trait) -> Self` - Overrides the `clap` generated usage string for help and error messages.
- `fn override_help<impl IntoResettable<StyledStr>>(self: Self, help: impl Trait) -> Self` - Overrides the `clap` generated help message (both `-h` and `--help`).
- `fn help_template<impl IntoResettable<StyledStr>>(self: Self, s: impl Trait) -> Self` - Sets the help template to be used, overriding the default format.
- `fn flatten_help(self: Self, yes: bool) -> Self` - Flatten subcommand help into the current command's help
- `fn next_help_heading<impl IntoResettable<Str>>(self: Self, heading: impl Trait) -> Self` - Set the default section heading for future args.
- `fn next_display_order<impl IntoResettable<usize>>(self: Self, disp_ord: impl Trait) -> Self` - Change the starting value for assigning future display orders for args.
- `fn arg_required_else_help(self: Self, yes: bool) -> Self` - Exit gracefully if no arguments are present (e.g. `$ myprog`).
- `fn allow_missing_positional(self: Self, yes: bool) -> Self` - Allows one to implement two styles of CLIs where positionals can be used out of order.

**Trait Implementations:**

- **From**
  - `fn from(cmd: &Command) -> Self`
- **Clone**
  - `fn clone(self: &Self) -> Command`
- **Index**
  - `fn index(self: &Self, key: &Id) -> &<Self as >::Output`
- **Debug**
  - `fn fmt(self: &Self, f: & mut $crate::fmt::Formatter) -> $crate::fmt::Result`
- **Display**
  - `fn fmt(self: &Self, f: & mut fmt::Formatter) -> fmt::Result`
- **Default**
  - `fn default() -> Self`



