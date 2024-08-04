use std::vec;

use extism_pdk::*;
use fluentci_pdk::dag;

pub mod helpers;

#[plugin_fn]
pub fn install(_args: String) -> FnResult<String> {
    let stdout = helpers::setup_mina()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn setup(args: String) -> FnResult<String> {
    helpers::setup_mina()?;
    let stdout = dag()
        .flox()?
        .with_exec(vec!["mina", "setup", &args])?
        .stdout()?;
    Ok(stdout)
}

#[plugin_fn]
pub fn deploy(args: String) -> FnResult<String> {
    helpers::setup_mina()?;
    let stdout = dag()
        .flox()?
        .with_exec(vec!["mina", "deploy", &args])?
        .stdout()?;
    Ok(stdout)
}
