use crate::common::*;

pub(crate) struct Data;

impl Data {
  pub(crate) fn initialize(client: &mut Client) {
    let mut data = client.data.write();

    let start = Instant::now();

    data.insert::<Start>(start);
  }

  pub(crate) fn start(context: &mut Context) -> Instant {
    let data = context.data.read();
    *data.get::<Start>().unwrap()
  }
}

struct Start;

impl TypeMapKey for Start {
  type Value = Instant;
}
