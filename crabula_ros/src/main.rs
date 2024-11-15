use safe_drive::{context::Context, error::DynError};

fn main() -> Result<(), DynError> {
  let ctx = Context::new()?;
    Ok(())
}