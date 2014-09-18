use std::os;

use core::Source;
use sources::PathSource;
use ops;
use util::{CargoResult, ProcessError};

pub struct ExampleOptions<'a> {
    pub compile_opts: ops::CompileOptions<'a>,
}

pub fn example(manifest_path: &Path,
                 options: &mut ExampleOptions,
                 test_args: &[String]) -> CargoResult<Option<ProcessError>> {
    let mut source = try!(PathSource::for_path(&manifest_path.dir_path()));
    try!(source.update());
    let package = try!(source.get_root_package());

    let mut compile = try!(ops::compile(manifest_path, &mut options.compile_opts));    
    compile.tests.sort();

    let cwd = os::getcwd();
    for exe in compile.tests.iter() {
        let to_display = match exe.path_relative_from(&cwd) {
            Some(path) => path,
            None => exe.clone(),
        };
        let cmd = compile.process(exe, &package).args(test_args);
        try!(options.compile_opts.shell.concise(|shell| {
            shell.status("Running", to_display.display().to_string())
        }));
        try!(options.compile_opts.shell.verbose(|shell| {
            shell.status("Running", cmd.to_string())
        }));
        match cmd.exec() {
            Ok(()) => {}
            Err(e) => return Ok(Some(e))
        }
    }

    Ok(None)
}

/*
pub fn list_examples(manifest_path: &Path) -> CargoResult<Option<ProcessError>> {
    Ok(None)
}
*/