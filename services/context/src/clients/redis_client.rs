use std::error::Error;

use redis::{Client, Commands};

use crate::models::context::Context;

pub fn set_context(redis_client: &Client, context: &Context) -> Result<(), Box<dyn Error>> {
    let serialized_context = serde_json::to_string(context)?;
    let mut conn = redis_client.get_connection()?;
    conn.set(&context.user_id, serialized_context)?;
    Ok(())
}

pub fn get_context(redis_client: &Client, key: &str) -> Result<Context, Box<dyn Error>> {
    let mut conn = redis_client.get_connection()?;
    let serialized_context: Option<String> = conn.get(key)?;
    let context: Context = match serialized_context {
        Some(serialized) => serde_json::from_str(&serialized)?,
        None => return Err("Context not found".into()),
    };
    Ok(context)
}
