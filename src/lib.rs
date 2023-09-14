#![feature(macro_metavar_expr)]
use std::fmt::Display;

use anyhow::{anyhow, bail, Context, Result};
use shallot::*;

#[derive(PartialEq, Debug, Clone)]
pub struct HashMap<E: LispExpression>(std::collections::hash_map::HashMap<Symbol, E>);

impl<E: LispExpression> Atom<E> for HashMap<E> {
    fn sized_name() -> &'static str
    where
        Self: Sized,
    {
        "hash map"
    }

    fn name(&self) -> &'static str {
        "hash map"
    }

    fn call(&self, arguments: &[E], env: &mut Environment<E>) -> Result<E> {
        if arguments.len() != 1 {
            bail!("Indexing into hash map takes exactly one argument")
        }
        let key = arguments[0].eval(env)?;
        let key: &Symbol = key.try_into_atom()?;
        self.0
            .get(key)
            .cloned()
            .ok_or_else(|| anyhow!("Key {} missing in hash map", key))
    }
}

impl<E: LispExpression> Display for HashMap<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut all_variables = self.0.keys().collect::<Vec<_>>();
        all_variables.sort();
        let longest_var_length = all_variables.iter().map(|s| s.len()).max().unwrap_or(0);
        let mut first = true;
        for symbol in &all_variables {
            if !first {
                writeln!(f)?;
            }
            first = false;
            // Note: these values exist in our map for sure
            let value = self.0.get(symbol).unwrap();
            let symbol = &symbol.0;
            write!(f, "{symbol:>longest_var_length$} -> {value}")?;
        }
        Ok(())
    }
}

pub fn get_environment<E>(arguments: &[E], env: &mut Environment<E>) -> Result<E>
where
    E: LispExpression + ToAndFrom<HashMap<E>>,
{
    if !arguments.is_empty() {
        bail!("Function get_environment expects no parameters")
    }
    Ok(HashMap(env.inner.clone()).into())
}

pub fn mapping<E>(arguments: &[E], _env: &mut Environment<E>) -> Result<E>
where
    E: LispExpression + ToAndFrom<HashMap<E>>,
{
    if arguments.len() % 2 != 0 {
        bail!("Constructing a mapping requires an even number of arguments");
    }
    let mut result = HashMap(std::collections::hash_map::HashMap::new());
    for i in 0..arguments.len() / 2 {
        let key: &Symbol = arguments[2 * i].try_into_atom().with_context(|| {
            anyhow!(
                "Even arguments to construct mapping must be symbols (see {})",
                arguments[2 * i]
            )
        })?;
        let value = &arguments[2 * i + 1];
        result.0.insert(key.clone(), value.clone());
    }

    Ok(result.into())
}

create_layer!(
   over
   | atoms HashMap<Expression>
   | builtins "get_environment" -> BuiltinFunction::new("get_environment", get_environment),
            "mapping" -> BuiltinFunction::new("mapping", mapping)
);
