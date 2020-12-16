use std::path::PathBuf;

use neon::context::{Context, ModuleContext};
use neon::object::Object;
use neon::result::NeonResult;
use neon::types::{JsArray, JsObject, JsString};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(bin_name = "npm init neon")]
struct Opt {
    name: Option<String>,
    #[structopt(short, long)]
    submodule: Option<PathBuf>,
}

fn cli(argv: Vec<String>) {
    let opts = Opt::from_iter(argv);
    println!("{:?}", opts);
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    let global = cx.global();

    let process = global.get(&mut cx, "process")?
        .downcast_or_throw::<JsObject, _>(&mut cx)?;

    let argv = process.get(&mut cx, "argv")?
        .downcast_or_throw::<JsArray, _>(&mut cx)?
        .to_vec(&mut cx)?
        .into_iter()
        .skip(1)
        .map(|v| {
            v.downcast_or_throw::<JsString, _>(&mut cx)
                .map(|v| v.value(&mut cx))
        })
        .collect::<Result<_, _>>()?;

    cli(argv);

    Ok(())
}
