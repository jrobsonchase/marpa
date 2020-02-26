use std::collections::HashMap;

struct Nidset {
  nids: Vec<usize>,
  id: usize
}

pub struct ASF {
  next_inset_id: usize,
  nidset_by_id: HashMap<usize, Nidset>,
  powerset_by_id: HashMap<usize, Nidset>,
  glades: HashMap<usize, Nidset>,
  intset_by_key: HashMap<Vec<usize>, usize>,
}

impl Default for ASF {
  fn default() -> Self {
    ASF {
      next_inset_id: 0,
      nidset_by_id: HashMap::new(),
      powerset_by_id: HashMap::new(),
      glades: HashMap::new(),
      intset_by_key: HashMap::new(),
    }
  }
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

  pub fn new() -> Self {

    // my $ordering = $recce->ordering_get();
    // OR_NODE: for ( my $or_node_id = 0;; $or_node_id++ ) {
    //     my @and_node_ids =
    //         $ordering->or_node_and_node_count($or_node_id);
    //     last OR_NODE if not scalar @and_node_ids;
    //     my @sorted_and_node_ids = map { $_->[-1] } sort { $a <=> $b } map {
    //         [ ( $bocage->_marpa_b_and_node_predecessor($_) // -1 ), $_ ]
    //     } @and_node_ids;
    //     $or_nodes->[$or_node_id] = \@and_node_ids;
    // } ## end OR_NODE: for ( my $or_node_id = 0;; $or_node_id++ )
    ASF::default()
  }
}
