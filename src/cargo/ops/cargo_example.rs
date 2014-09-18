use std::os;

use core::{Source, Target};
use sources::PathSource;
use ops;
use util::{CargoResult, ProcessError, human};

pub struct ExampleOptions<'a> {
    pub list: bool,
    pub example_name: Option<String>,
    pub compile_opts: ops::CompileOptions<'a>,
}

pub fn example(manifest_path: &Path,
 options: &mut ExampleOptions,
 test_args: &[String]) -> CargoResult<Option<ProcessError>> {
    let mut source = try!(PathSource::for_path(&manifest_path.dir_path()));
    try!(source.update());
    let package = try!(source.get_root_package());

    // List all the available binaries
    if options.list {
        let mut bins = package.get_manifest().get_targets().iter().filter(|a| {
        a.is_bin()
    });
        for bin in bins {
            println!("{}", bin.get_name());
        }
    }
    else {
        let mut compile = try!(ops::compile(manifest_path, &mut options.compile_opts));
        let example_name = options.example_name;
        match example_name {
            Some(name) => {
                let cwd = os::getcwd();
                match compile.binaries.iter().find(|x| x.filename_str() == Some(name.as_slice())) {
                    Some(exe) => {
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
                    },
                    None => {}
                }
            },
            None => {}
        }
    }
    Ok(None)
}