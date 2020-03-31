use crate::common::*;

mod common;
mod data;
mod general;
mod handler;

fn main() {
  let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("token"), Handler)
    .expect("Error creating client");

  client.with_framework(
    StandardFramework::new()
      .configure(|c| c.prefix("!"))
      .group(&general::GENERAL_GROUP),
  );

  Data::initialize(&mut client);

  if let Err(why) = client.start() {
    println!("An error occurred while running the client: {:?}", why);
  }
}
