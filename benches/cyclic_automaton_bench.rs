#[macro_use]
extern crate bencher;

use bencher::Bencher;

use cca::CyclicAutomaton;
use cca::*;

use gif::Frame;

fn cca_300x300_100gen_5states(b: &mut Bencher) {
    let mut automaton = CyclicAutomaton::new(300, 300, 5, 1);
    automaton.randomize(12345);

    b.iter(|| {
        for _ in 0..100 {
            automaton.next_generation();
        }
    })
}

fn cca_1000x1000_100gen_3states(b: &mut Bencher) {
    let mut automaton = CyclicAutomaton::new(1000, 1000, 3, 1);
    automaton.randomize(12345);

    b.iter(|| {
        for _ in 0..100 {
            automaton.next_generation();
        }
    })
}

fn frame_create_test(b: &mut Bencher) {
    let mut automaton = CyclicAutomaton::new(1000, 1000, 3, 1);
    automaton.randomize(12345);

    b.iter(|| {
        let mut frame = Frame::default();
        get_frame(&automaton, &mut frame);
    })
}

benchmark_group!(
    benches,
    cca_300x300_100gen_5states,
    cca_1000x1000_100gen_3states,
    frame_create_test,
);
benchmark_main!(benches);
