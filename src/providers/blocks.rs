use std::rc::Rc;
use std::collections::BTreeMap;

use yew::prelude::*;
use yew::html::Scope;

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

pub type BlocksContext = Rc<Blocks>;

async fn subscribe_blocks(api: Api, link: &Scope<BlocksProvider>) -> Result<(), String> {
  let mut sub = api.client().subscribe_blocks().await
    .map_err(|e| e.to_string())?;
  while let Some(header) = sub
    .next()
    .await
    .transpose()
    .map_err(|e| e.to_string())?
  {
    link.send_message(Msg::NewHeader(header));
  }
  Ok(())
}

pub enum Msg {
  BackendContextUpdated(BackendContext),
  NewHeader(Header),
  SubscriptionError(String),
}

pub struct BlocksProvider {
  backend: BackendContext,
  _context_listener: ContextHandle<BackendContext>,
  blocks: Blocks,
}

impl BlocksProvider {
  fn subscribe_blocks(&mut self, ctx: &Context<Self>) {
    if let Some(api) = self.backend.api() {
      let link = ctx.link().clone();
      wasm_bindgen_futures::spawn_local(async move {
        if let Err(err) = subscribe_blocks(api, &link).await {
          link.send_message(Msg::SubscriptionError(err.to_string()));
        }
      });
    }
  }
}

#[derive(Properties, Debug, PartialEq)]
pub struct BlocksProviderProps {
  pub children: Children,
}

impl Component for BlocksProvider {
  type Message = Msg;
  type Properties = BlocksProviderProps;

  fn create(ctx: &Context<Self>) -> Self {
    let (backend, context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::BackendContextUpdated))
            .expect("No Backend Context Provided");
    let mut provider = Self {
      backend,
      _context_listener: context_listener,
      blocks: Blocks::default(),
    };
    provider.subscribe_blocks(ctx);

    provider
  }

  fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
    match msg {
      Msg::BackendContextUpdated(backend) => {
        self.backend = backend;
        self.subscribe_blocks(ctx);
      }
      Msg::NewHeader(header) => {
        self.blocks.new_header(header);
      }
      Msg::SubscriptionError(err) => {
        log::error!("Block Subscription failed: {err:?}");
      }
    }
    true
  }

  fn view(&self, ctx: &Context<Self>) -> Html {
    let blocks = Rc::new(self.blocks.clone());
    html! {
      <ContextProvider<BlocksContext> context={blocks}>
        { ctx.props().children.clone()}
      </ContextProvider<BlocksContext>>
    }
  }
}
