use thin::{
    Config,
    Grammar,
    Symbol,
};


#[test]
fn create_grammar() {
    let cfg = Config::new();
    let _ = Grammar::new(cfg);
}

#[test]
fn set_start() {
    let mut g: Grammar = Grammar::new(Config::new()).unwrap();

    let sym: Symbol = g.new_symbol().unwrap();
    assert!(g.set_start(sym).unwrap() == sym);

    assert!(g.get_start().unwrap() == sym);
}