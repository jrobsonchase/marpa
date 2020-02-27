use crate::thin::Tree;
use std::collections::HashMap;
use std::rc::Rc;
use crate::result::Result;
use crate::thin::{
    Bocage,
    Grammar,
    Order,
    Recognizer};

pub type Traverser = Rc<dyn Fn(Glade) -> Result<()>>;

struct Nidset {
  nids: Vec<usize>,
  id: usize
}

pub struct ASF {
  next_inset_id: usize,
  nidset_by_id: HashMap<usize, Nidset>,
  powerset_by_id: HashMap<usize, Nidset>,
  glades: HashMap<usize, Glade>,
  intset_by_key: HashMap<Vec<usize>, usize>,
  or_nodes: Vec<Nidset>,
  recce: Recognizer,
  bocage: Bocage,
  ordering: Order,
}

impl ASF {
  fn intset_id(&mut self, mut ids: Vec<usize>) -> usize {
    ids.sort();
    let intset_id = self.intset_by_key.entry(ids)
      .or_insert(self.next_inset_id+1);
    if *intset_id > self.next_inset_id {
      self.next_inset_id += 1;
    }
    *intset_id
  }

  fn obtain_nidset(&mut self, nids: Vec<usize>) -> &Nidset {
    let id           = self.intset_id(nids.clone());
    let nidset = self.nidset_by_id.entry(id).or_insert_with(|| {
      Nidset {
        id,
        nids
      }
    });
    &*nidset
  }

  pub fn new(mut recce: Recognizer) -> Result<Self> {
    recce.start_input()?;
    // Initialize all usual thin:: structs here, we'll need them
    let bocage = Bocage::new(recce.clone())?;
    let ordering = Order::new(bocage.clone())?;
    // my $ordering = $recce->ordering_get();
    let or_nodes = Vec::new();
    // OR_NODE: for ( my $or_node_id = 0;; $or_node_id++ ) {
    //     my @and_node_ids =
    //         $ordering->or_node_and_node_count($or_node_id);
    //     last OR_NODE if not scalar @and_node_ids;
    //     my @sorted_and_node_ids = map { $_->[-1] } sort { $a <=> $b } map {
    //         [ ( $bocage->_marpa_b_and_node_predecessor($_) // -1 ), $_ ]
    //     } @and_node_ids;
    //     $or_nodes->[$or_node_id] = \@and_node_ids;
    // } ## end OR_NODE: for ( my $or_node_id = 0;; $or_node_id++ )
    Ok(ASF {
      next_inset_id: 0,
      nidset_by_id: HashMap::new(),
      powerset_by_id: HashMap::new(),
      glades: HashMap::new(),
      intset_by_key: HashMap::new(),
      or_nodes,
      recce,
      bocage,
      ordering
    })
  }

  fn peak(&mut self) -> Result<usize> {
    let mut bocage = &mut self.bocage;
    let augment_or_node_id = bocage.top_or_node()?;
    let augment_and_node_id = self.or_nodes[augment_or_node_id as usize].id;
    let start_or_node_id = bocage.and_node_cause(augment_and_node_id as i32)?;
    let base_nidset = self.obtain_nidset(vec![start_or_node_id as usize]);
    let glade_id = base_nidset.id;
    let mut glade = self.glades.get_mut(&glade_id).unwrap();
    (*glade).registered = true;
    self.glade_obtain(glade_id);
    Ok(glade_id)
  }

  fn glade_obtain(&mut self, glade_id: usize)  {}
}


pub struct Glade {
  id: usize,
  registered: bool,
  symches: Vec<usize>,
}

impl Glade {
  pub fn rule_id(&self) -> usize {
    self.id
  }
}