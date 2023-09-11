use anyhow::Result;
use shallot::*;
use shallot_hashmap::*;

fn main() -> Result<()> {
    let mut environment: Environment<HashMapExpression> = Environment::default();
    shallot::builtins::set_environment(&mut environment);
    shallot_hashmap::set_environment(&mut environment);
    run_repl::<HashMapExpression>(&mut environment)
}
