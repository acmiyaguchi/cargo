pub use self::cargo_clean::clean;
pub use self::cargo_compile::{compile, CompileOptions};
pub use self::cargo_read_manifest::{read_manifest,read_package,read_packages};
pub use self::cargo_rustc::{compile_targets, Compilation};
pub use self::cargo_run::run;
pub use self::cargo_new::{new, NewOptions};
pub use self::cargo_doc::{doc, DocOptions};
pub use self::cargo_generate_lockfile::{generate_lockfile, write_resolve};
pub use self::cargo_generate_lockfile::{update_lockfile, load_lockfile};
pub use self::cargo_test::{run_tests, run_benches, TestOptions};
pub use self::cargo_package::package;
pub use self::cargo_upload::{upload, upload_configuration, UploadConfig};
pub use self::cargo_upload::{upload_login, http_proxy, http_handle};
pub use self::cargo_fetch::{fetch, resolve_and_fetch};
pub use self::cargo_example::{example, ExampleOptions};

mod cargo_clean;
mod cargo_compile;
mod cargo_read_manifest;
mod cargo_rustc;
mod cargo_run;
mod cargo_new;
mod cargo_doc;
mod cargo_generate_lockfile;
mod cargo_test;
mod cargo_package;
mod cargo_upload;
mod cargo_fetch;
mod cargo_example;