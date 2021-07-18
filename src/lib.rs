// #![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate redis_module;

use redis_module::{CommandFilterContext};

fn audit(ctx: &CommandFilterContext) {
    eprint!("{} {}", ctx.args_get(0), ctx.args_count());
}

redis_module! {
    name: "redisschema",
    version: 001000,
    data_types: [],
    commands: [],
    filters: [
        [audit, 0],
    ],
}
