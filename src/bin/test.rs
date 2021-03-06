use std::io::process::ExitStatus;
use docopt;

use cargo::ops;
use cargo::core::MultiShell;
use cargo::util::{CliResult, CliError, CargoError};
use cargo::util::important_paths::{find_root_manifest_for_cwd};

docopt!(Options, "
Execute all unit and integration tests of a local package

Usage:
    cargo test [options] [--] [<args>...]

Options:
    -h, --help              Print this message
    --no-run                Compile, but don't run tests
    -j N, --jobs N          The number of jobs to run in parallel
    --target TRIPLE         Build for the target triple
    -u, --update-remotes    Deprecated option, use `cargo update` instead
    --manifest-path PATH    Path to the manifest to build tests for
    -v, --verbose           Use verbose output

All of the trailing arguments are passed to the test binaries generated for
filtering tests and generally providing options configuring how they run.
",  flag_jobs: Option<uint>, flag_target: Option<String>,
    flag_manifest_path: Option<String>)

pub fn execute(options: Options, shell: &mut MultiShell) -> CliResult<Option<()>> {
    let root = try!(find_root_manifest_for_cwd(options.flag_manifest_path));
    shell.set_verbose(options.flag_verbose);

    let mut ops = ops::TestOptions {
        no_run: options.flag_no_run,
        compile_opts: ops::CompileOptions {
            update: options.flag_update_remotes,
            env: "test",
            shell: shell,
            jobs: options.flag_jobs,
            target: options.flag_target.as_ref().map(|s| s.as_slice()),
            dev_deps: true,
        },
    };

    let err = try!(ops::run_tests(&root, &mut ops,
                                  options.arg_args.as_slice()).map_err(|err| {
        CliError::from_boxed(err, 101)
    }));
    match err {
        None => Ok(None),
        Some(err) => {
            Err(match err.exit {
                Some(ExitStatus(i)) => CliError::new("", i as uint),
                _ => CliError::from_boxed(err.mark_human(), 101)
            })
        }
    }
}
