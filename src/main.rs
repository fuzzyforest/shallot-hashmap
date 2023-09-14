use anyhow::Result;
use shallot::*;

fn main() -> Result<()> {
    let mut environment: Environment<shallot_hashmap::Expression> = Environment::default();
    shallot::builtins::set_environment(&mut environment);
    shallot_hashmap::set_environment(&mut environment);
    run_repl(&mut environment)
}
