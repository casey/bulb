use crate::common::*;

#[group]
#[commands(ping, uptime)]
struct General;

#[command]
fn ping(context: &mut Context, msg: &Message) -> CommandResult {
  msg.reply(context, "Pong!")?;

  Ok(())
}

#[command]
fn uptime(context: &mut Context, msg: &Message) -> CommandResult {
  let start = Data::start(context);

  let uptime = start.elapsed();

  msg.reply(context, format!("{} seconds", uptime.as_secs()))?;

  Ok(())
}
