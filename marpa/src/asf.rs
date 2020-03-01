use crate::thin::Tree;
use std::collections::HashMap;
use std::rc::Rc;
use crate::result::Result;
use crate::thin::{
    Bocage,
    Grammar,
    Order,
    Recognizer};

pub trait Traverser {
  type ParseTree;
  type ParseState;

  fn traverse_glade(&self, glade: Glade, state: Self::ParseState) -> Result<Self::ParseTree>;
}

const NID_LEAF_BASE : i32 = -43;

struct Nidset {
  nids: Vec<i32>,
  id: usize
}

pub struct ASF {
  next_inset_id: usize,
  factoring_max: usize,
  nidset_by_id: HashMap<usize, Nidset>,
  powerset_by_id: HashMap<usize, Nidset>,
  glades: HashMap<usize, Glade>,
  intset_by_key: HashMap<Vec<i32>, usize>,
  or_nodes: Vec<Nidset>,
  recce: Recognizer,
  bocage: Bocage,
  ordering: Order,
}

impl ASF {
  fn intset_id(&mut self, mut ids: Vec<i32>) -> usize {
    ids.sort();
    let intset_id = self.intset_by_key.entry(ids)
      .or_insert(self.next_inset_id+1);
    if *intset_id > self.next_inset_id {
      self.next_inset_id += 1;
    }
    *intset_id
  }

  fn obtain_nidset(&mut self, nids: Vec<i32>) -> &Nidset {
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
    let mut ordering = recce.ordering_get().expect(
      "An attempt was make to create an ASF for a null parse\n
          A null parse is a successful parse of a zero-length string\n
          ASF's are not defined for null parses");
    let mut or_nodes = Vec::new();
    let mut or_node_id = 0;
    loop {
      let mut and_node_ids = ordering.or_node_and_node_ids(or_node_id);
      if and_node_ids.is_empty() {break;}
      let mut build_and_node_ids = and_node_ids.iter().map(|id|
        (( bocage.and_node_predecessor(*id).unwrap_or(-1)), id)).collect::<Vec<_>>();
      // build_and_node_ids.sort();
      // let sorted_and_node_ids = build_and_node_ids.into_iter().map(|(k,v)| v).collect();
      or_nodes.insert(or_node_id,
         Nidset { id: or_node_id, nids: and_node_ids });
      or_node_id += 1;
    }

    Ok(ASF {
      next_inset_id: 0,
      nidset_by_id: HashMap::new(),
      powerset_by_id: HashMap::new(),
      glades: HashMap::new(),
      intset_by_key: HashMap::new(),
      factoring_max: 42,
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
    let base_nidset = self.obtain_nidset(vec![start_or_node_id]);
    let glade_id = base_nidset.id;
    // Cannot "obtain" the glade if it is not registered
    let mut glade = self.glades.entry(glade_id).or_insert(Glade::default());
    (*glade).registered = true;
    self.glade_obtain(glade_id);
    Ok(glade_id)
  }

  fn glade_obtain(&mut self, glade_id: usize) -> Result<&mut Glade> {
    let factoring_max = self.factoring_max;
    let mut glade  = self.glades.get(&glade_id)
                  .expect("Attempt to use an invalid glade");
    if !glade.registered {
      panic!("attempt to use an unregistered glade with ID: {}", glade_id);
    }
    // Return the glade if it is already set up
    if !glade.symches.is_empty() {
      Ok(self.glades.get_mut(&glade_id).unwrap())
    } else {
      self.compute_symches(glade_id)
    }
  }

  fn compute_symches(&mut self, glade_id: usize) -> Result<&mut Glade> {
    let base_nidset = self.nidset_by_id.get(&glade_id).unwrap();
    // let choicepoint;
    // let choicepoint_powerset;
    {
      let mut source_data = Vec::new();
      for source_nid in base_nidset.nids.iter() {
        let sort_ix = self.nid_sort_ix(*source_nid)?;
        source_data.push((sort_ix, source_nid ));
      }
      source_data.sort_by_key(|k| k.0);
      let mut nid_ix = 0;
//         my ( $sort_ix_of_this_nid, $this_nid ) =
//             @{ $sorted_source_data[ $nid_ix++ ] };
//         my @nids_with_current_sort_ix = ();
//         my $current_sort_ix           = $sort_ix_of_this_nid;
//         my @symch_ids                 = ();
//         'NID: while (1) {

//             if ( $sort_ix_of_this_nid != $current_sort_ix ) {

//                 # Currently only whole id break logic
//                 my $nidset_for_sort_ix = Marpa::R2::Nidset->obtain( $asf,
//                     @nids_with_current_sort_ix );
//                 push @symch_ids, $nidset_for_sort_ix->id();
//                 @nids_with_current_sort_ix = ();
//                 $current_sort_ix           = $sort_ix_of_this_nid;
//             } ## end if ( $sort_ix_of_this_nid != $current_sort_ix )
//             last NID if not defined $this_nid;
//             push @nids_with_current_sort_ix, $this_nid;
//             my $sorted_entry = $sorted_source_data[ $nid_ix++ ];
//             if ( defined $sorted_entry ) {
//                 ( $sort_ix_of_this_nid, $this_nid ) = @{$sorted_entry};
//                 next NID;
//             }
//             $this_nid            = undef;
//             $sort_ix_of_this_nid = -2;
//         } ## end NID: while (1)
//         $choicepoint_powerset = Marpa::R2::Powerset->obtain( $asf, @symch_ids );
//         $choicepoint->[Marpa::R2::Internal::Choicepoint::ASF] = $asf;
//         $choicepoint->[Marpa::R2::Internal::Choicepoint::FACTORING_STACK] =
//             undef;
    }

    // Check if choicepoint already seen?
    let mut symches     = Vec::new();
    // let symch_count = choicepoint_powerset.count();
    'SYMCH: for symch_ix in 0 .. 0 { // ..  symch_count
      // choicepoint.factoring_stack = Vec::new();
//         my $symch_nidset = $choicepoint_powerset->nidset($asf, $symch_ix);
//         my $choicepoint_nid = $symch_nidset->nid(0);
//         my $symch_rule_id = nid_rule_id($asf, $choicepoint_nid) // -1;

//         # Initial undef indicates no factorings omitted
//         my @factorings = ( $symch_rule_id, undef );

//         # For a token
//         # There will not be multiple factorings or nids,
//         # it is assumed, for a token
//         if ( $symch_rule_id < 0 ) {
//             my $base_nidset = Marpa::R2::Nidset->obtain( $asf, $choicepoint_nid );
//             my $glade_id    = $base_nidset->id();

//             $asf->[Marpa::R2::Internal::ASF::GLADES]->[$glade_id]
//                 ->[Marpa::R2::Internal::Glade::REGISTERED] = 1;
//             push @factorings, [$glade_id];
//             push @symches, \@factorings;
//             next SYMCH;
//         } ## end if ( $symch_rule_id < 0 )

//         my $symch = $choicepoint_powerset->nidset($asf, $symch_ix);
//         my $nid_count = $symch->count();
//         my $factorings_omitted;
//         FACTORINGS_LOOP:
//         for ( my $nid_ix = 0; $nid_ix < $nid_count; $nid_ix++ ) {
//             $choicepoint_nid = $symch_nidset->nid($nid_ix);
//             first_factoring($choicepoint, $choicepoint_nid);
//             my $factoring = glade_id_factors($choicepoint);

//             FACTOR: while ( defined $factoring ) {
//                 if ( scalar @factorings > $factoring_max ) {

//                     # update factorings omitted flag
//                     $factorings[1] = 1;
//                     last FACTORINGS_LOOP;
//                 }
//                 my @factoring = ();
//                 for (
//                     my $item_ix = $#{$factoring};
//                     $item_ix >= 0;
//                     $item_ix--
//                     )
//                 {
//                     push @factoring, $factoring->[$item_ix];
//                 } ## end for ( my $item_ix = $#{$factoring}; $item_ix >= 0; ...)
//                 push @factorings, \@factoring;
//                 next_factoring($choicepoint, $choicepoint_nid);
//                 $factoring = glade_id_factors($choicepoint);
//             } ## end FACTOR: while ( defined $factoring )
//         } ## end FACTORINGS_LOOP: for ( my $nid_ix = 0; $nid_ix < $nid_count; $nid_ix...)
//         push @symches, \@factorings;
    }
    let mut glade  = self.glades.get_mut(&glade_id)
                  .expect("Attempt to use an invalid glade");
    glade.symches = symches;
    glade.id = glade_id;
    Ok(glade)
  }

  fn glade_is_visited(&self, glade_id: usize) -> bool {
    match self.glades.get(&glade_id) {
      None => false,
      Some(glade) => glade.visited
    }
  }

  fn nid_sort_ix(&self, nid: i32) -> Result<i32> {
    let grammar = self.recce.grammar();
    let bocage    = &self.bocage;
    if nid >= 0 {
        let irl_id = bocage.or_node_irl(nid)?;
        return grammar.source_xrl(irl_id);
    }
    let and_node_id  = nid_to_and_node(nid);
    let token_nsy_id = bocage.and_node_symbol(and_node_id)?;
    let token_id     = grammar.source_xsy(token_nsy_id)?;

    // -2 is reserved for 'end of data'
    Ok(-token_id - 3)
  }
}

/// Range from -1 to -42 reserved for special values
fn and_node_to_nid(offset: i32) -> i32 { -offset + NID_LEAF_BASE }
/// Range from -1 to -42 reserved for special values
fn nid_to_and_node(offset: i32) -> i32 { -offset + NID_LEAF_BASE }



pub struct Glade {
  id: usize,
  registered: bool,
  visited: bool,
  symches: Vec<usize>,
}

impl Default for Glade {
  fn default() -> Self {
    Glade {
      id: 0,
      registered: false,
      visited: false,
      symches: Vec::new()
    }
  }
}

impl Glade {
  pub fn rule_id(&self) -> usize {
    self.id
  }
}