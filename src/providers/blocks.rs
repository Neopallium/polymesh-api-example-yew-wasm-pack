use std::collections::BTreeMap;

use leptos::*;

use crate::providers::backend::*;
use crate::providers::backend::client::*;

pub const MAX_BLOCKS: usize = 70;

#[derive(Clone, PartialEq)]
pub struct BlockItem {
  pub number: BlockNumber,
  pub hash: BlockHash,
}

#[derive(Clone, Default, PartialEq)]
pub struct Blocks {
  pub genesis_hash: BlockHash,
  pub blocks: BTreeMap<BlockNumber, BlockItem>,
}

impl Blocks {
  fn new_header(&mut self, header: Header) {
    let hash = header.hash();
    let number = header.number;
    let item = BlockItem {
      number,
      hash,
    };
    self.blocks.insert(number, item);
    if self.blocks.len() > MAX_BLOCKS {
      self.blocks.pop_first();
    }
  }

  pub fn iter(&self) -> impl Iterator<Item = (&BlockNumber, &BlockItem)> {
    self.blocks.iter()
  }
}

async fn subscribe_blocks(api: Api, set_blocks: WriteSignal<Blocks>) -> Result<(), String> {
  let mut sub = api.client().subscribe_blocks().await
    .map_err(|e| e.to_string())?;
  while let Some(header) = sub
    .next()
    .await
    .transpose()
    .map_err(|e| e.to_string())?
  {
    set_blocks.update(|blocks| blocks.new_header(header));
  }
  Ok(())
}

#[component]
pub fn BlocksProvider(children: Children) -> impl IntoView {
  let (blocks, set_blocks) = create_signal(Blocks::default());
  let (backend_state, _) = use_backend_state();
  
  create_effect(move |_| {
    if let BackendState::Connected(api) = backend_state.get() {
      let set_blocks = set_blocks.clone();
      spawn_local(async move {
        if let Err(err) = subscribe_blocks(api, set_blocks).await {
          log::error!("Block Subscription failed: {err:?}");
        }
      });
    }
  });

  provide_context(blocks);
  
  children()
}

pub fn use_blocks() -> ReadSignal<Blocks> {
  use_context::<ReadSignal<Blocks>>().expect("Blocks context")
}
