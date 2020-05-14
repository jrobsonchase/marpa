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

  fn traverse_glade(&self, glade: &mut Glade, state: Self::ParseState) -> Result<(Self::ParseTree, Self::ParseState)>;
}

#[derive(Debug, Clone)]
struct Nidset {
  nids: Vec<i32>,
  id: usize
}

#[derive(Debug, Clone)]
struct Powerset {
  symches: Vec<usize>,
}
impl Powerset {
  // how many symches are in this powerset
  pub fn count(&self) -> usize {
    self.symches.len()
  }
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
  fn intset_id(&mut self, mut ids: Vec<i32>) -> (usize, Vec<i32>) {
    ids.sort();
    let intset_id = self.intset_by_key.entry(ids.clone())
      .or_insert(self.next_inset_id+1);
    if *intset_id > self.next_inset_id {
      self.next_inset_id += 1;
    }
    (*intset_id, ids)
  }

  fn obtain_nidset(&mut self, nids: Vec<i32>) -> &Nidset {
    let (id, nids)           = self.intset_id(nids);
    let nidset = self.nidset_by_id.entry(id).or_insert_with(|| {
      Nidset {
        id,
        nids
      }
    });
    &*nidset
  }

  fn obtain_powerset(&mut self, symches: Vec<usize>) -> Powerset {
    Powerset {
      symches
    }
  }

  pub fn new(mut recce: Recognizer) -> Result<Self> {
    // Initialize all usual thin:: structs here, we'll need them
    let mut bocage = Bocage::new(&recce)?;
    let mut ordering = bocage.get_ordering().expect(
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
      // TODO: Is this used anywhere?
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

  pub fn traverse<PT,PS>(&mut self, init_state: PS, traverser: Box<dyn Traverser<ParseTree=PT, ParseState=PS>>) -> Result<(PT, PS)> {
    let peak = self.peak()?;
    let peak_glade = self.glade_obtain(peak)?;
    traverser.traverse_glade(peak_glade, init_state)
  }

  fn peak(&mut self) -> Result<usize> {
    let mut bocage = &mut self.bocage;
    let augment_or_node_id = bocage.top_or_node()?;
    let augment_and_node_id = self.or_nodes[augment_or_node_id as usize].nids[0];
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
    let mut source_data = Vec::new();
    {
      let base_nidset = self.nidset_by_id.get(&glade_id).unwrap();
      for source_nid in base_nidset.nids.iter() {
        let sort_ix = self.nid_sort_ix(*source_nid)?;
        source_data.push((sort_ix, *source_nid ));
      }
    }
    source_data.sort_by_key(|k| k.0);
    let mut nid_ix = 0;
    let (mut sort_ix_of_this_nid, this_nid) = source_data[nid_ix];
    let mut this_nid_opt = Some(this_nid);
    nid_ix += 1;
    let mut nids_with_current_sort_ix = Vec::new();
    let mut current_sort_ix           = sort_ix_of_this_nid;
    let mut symch_ids                 = Vec::new();
    'NID: loop {
      if sort_ix_of_this_nid != current_sort_ix {
        // Currently only whole id break logic
        let nidset_for_sort_ix = self.obtain_nidset(nids_with_current_sort_ix.clone());
        symch_ids.push(nidset_for_sort_ix.id);
        nids_with_current_sort_ix = Vec::new();
        current_sort_ix           = sort_ix_of_this_nid;
      }
      if this_nid_opt.is_none() {
        break;
      }
      nids_with_current_sort_ix.push(this_nid_opt.unwrap());
      nid_ix += 1;
      if let Some((sort_ix_of_this_nid, this_nid)) = source_data.get(nid_ix-1) {
        this_nid_opt = Some(*this_nid);
        continue 'NID;
      }
      this_nid_opt            = None;
      sort_ix_of_this_nid = -2;
    }
    let choicepoint_powerset = self.obtain_powerset(symch_ids);
    // TODO: choicepoint needs to be a dedicated object here??
    // choicepoint.[Marpa::R2::Internal::Choicepoint::ASF] = $asf;
    // choicepoint.[Marpa::R2::Internal::Choicepoint::FACTORING_STACK] = undef;
    // Check if choicepoint already seen?
    let mut symches     = Vec::new();
    let symch_count = choicepoint_powerset.count();
    'SYMCH: for symch_ix in 0..symch_count {
      dbg!(symch_ix);
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
    Ok(dbg!(glade))
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
    Ok(-token_id -3)
  }
}

const NID_LEAF_BASE : i32 = -43;
/// Range from -1 to -42 reserved for special values
fn and_node_to_nid(offset: i32) -> i32 { NID_LEAF_BASE - offset }
/// Range from -1 to -42 reserved for special values
fn nid_to_and_node(offset: i32) -> i32 { NID_LEAF_BASE - offset }


#[derive(Debug, Clone)]
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