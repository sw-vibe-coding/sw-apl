//! The shared-variable protocol as the May 1983 manual gives it
//! (chapter 9): offers and the degree of coupling, retraction, access
//! control, and a processor answering what is set.

use apl_d83_shares::{Processor, Shares};
use apl_store::{Memory, Store};
use apl_value::{Array, Number};

/// A processor that takes names beginning with E and answers each set
/// with its value doubled under the same name.
#[derive(Debug, Default)]
struct Echo {
    retracted: Vec<String>,
}

impl Processor for Echo {
    fn accepts(&mut self, p: i64, name: &str) -> bool {
        p == 7 && name.starts_with('E')
    }
    fn set(&mut self, name: &str, value: &Array, _: &mut dyn Store) -> Vec<(String, Array)> {
        let n = value.data.clone();
        let doubled = match n {
            apl_value::Data::Num(v) => v
                .iter()
                .map(|x| Number::from_f64(2.0 * x.as_f64()))
                .collect(),
            apl_value::Data::Char(_) => Vec::new(),
        };
        vec![(name.to_string(), Array::vector(doubled))]
    }
    fn retract(&mut self, name: &str) {
        self.retracted.push(name.to_string());
    }
}

fn shares() -> Shares {
    Shares::with(vec![Box::new(Echo::default())])
}

#[test]
fn an_offer_reports_1_and_an_accepted_one_is_2_when_asked() {
    let mut s = shares();
    assert_eq!(s.offer(7, "E", 0), 1, "offered, not yet matched");
    assert_eq!(s.coupling("E"), 2, "the processor took it");
    assert_eq!(s.offer(7, "E", 0), 2, "a repeated offer only asks");
}

#[test]
fn an_offer_to_a_processor_not_loaded_or_refusing_stays_at_1() {
    let mut s = shares();
    assert_eq!(s.offer(205, "C", 0), 1);
    assert_eq!(s.coupling("C"), 1, "acknowledged, never matched");
    assert_eq!(s.offer(7, "X", 0), 1);
    assert_eq!(s.coupling("X"), 1, "Echo takes only E names");
    assert_eq!(s.coupling("Y"), 0, "never offered");
}

#[test]
fn retraction_gives_the_coupling_before_it_and_tells_the_processor() {
    let mut s = shares();
    s.offer(7, "E", 0);
    s.offer(205, "C", 0);
    assert_eq!(s.retract("E"), 2);
    assert_eq!(s.retract("C"), 1);
    assert_eq!(s.retract("Z"), 0);
    assert_eq!(s.coupling("E"), 0);
}

#[test]
fn a_set_of_a_shared_variable_is_answered_by_its_partner() {
    let mut s = shares();
    let three = Array::vector(vec![Number::Int(3)]);
    assert!(
        s.assigned("E", &three, &mut Memory::default()).is_empty(),
        "not shared yet"
    );
    s.offer(7, "E", 0);
    let reply = s.assigned("E", &three, &mut Memory::default());
    assert_eq!(
        reply,
        vec![("E".to_string(), Array::vector(vec![Number::Int(6)]))]
    );
    s.offer(205, "C", 0);
    assert!(
        s.assigned("C", &three, &mut Memory::default()).is_empty(),
        "no partner, no answer"
    );
}

#[test]
fn access_control_is_set_once_offered() {
    let mut s = shares();
    assert_eq!(s.control("E", None), [false; 4], "not offered");
    assert_eq!(
        s.control("E", Some([true; 4])),
        [false; 4],
        "set only once offered"
    );
    s.offer(7, "E", 0);
    assert_eq!(s.control("E", None), [false; 4], "zeros to begin with");
    let simplex = [false, true, true, false];
    assert_eq!(s.control("E", Some(simplex)), simplex);
    assert_eq!(s.control("E", None), simplex);
    assert_eq!(
        s.control("E", Some([false; 4])),
        [false; 4],
        "back to the minimum"
    );
}

#[test]
fn retracting_everything_frees_every_name() {
    let mut s = shares();
    s.offer(7, "E1", 0);
    s.offer(7, "E2", 3);
    assert_eq!(s.depth("E2"), Some(3), "offered three calls down");
    s.retract_all();
    assert_eq!((s.coupling("E1"), s.depth("E2")), (0, None));
}
