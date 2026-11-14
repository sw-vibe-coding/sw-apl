use apl_c80_port::port;

const AB: &str = "⍝ sw-apl workspace. Re-executable APL: loading it runs it.
⍝!MODES (A)(B)
⍝!SOURCE sw-apl -- written for this repository.
⍝!DIGITS 10
⍝!WIDTH 120
)WSID T
M←2 3⍴⍳6
∇Z←F X;∆T
∆T←⍴X
Z←∆T∧X∨1
∇
";

#[test]
fn a_definition_becomes_a_def_block_ended_by_an_empty_line() {
    let c = port(AB);
    assert!(c.contains("\n)DEF Z←F X;D.T\n"), "{c}");
    assert!(c.ends_with("Z←D.T&Xv1\n\n"), "{c}");
}

#[test]
fn glyphs_become_apl80_characters() {
    assert!(port(AB).contains("\nM←2 3pi6\n"), "{}", port(AB));
}

#[test]
fn a_delta_helper_is_named_d_dot() {
    assert!(port(AB).contains("\nD.T←pX\n"), "{}", port(AB));
}

#[test]
fn the_heading_is_c_alone_with_its_settings() {
    let c = port(AB);
    let head: Vec<&str> = c.lines().take(6).collect();
    assert_eq!(
        head[0],
        "c SW-APL WORKSPACE, AS APL80 TYPES IT: LOADING IT RUNS IT."
    );
    assert_eq!(head[1], "⍝!MODES (C)");
    assert_eq!(head[2], "⍝!SOURCE sw-apl -- written for this repository.");
    assert_eq!(head[3], "⍝!DIGITS 6");
    assert_eq!(head[4], "⍝!WIDTH 64");
    assert_eq!(head[5], ")WSID T");
}

#[test]
fn indexing_is_in_round_brackets() {
    let c = port("x\nZ←A[I;2]+B[1]\n");
    assert!(c.contains("\nZ←A(I;2)+B(1)\n"), "{c}");
}

#[test]
fn an_axis_is_a_d_before_the_argument() {
    let c = port("x\nZ←+/[1]M\n");
    assert!(c.contains("\nZ←+/1dM\n"), "{c}");
}

#[test]
fn brackets_in_prose_read_as_apl80_writes_them() {
    let c = port("x\n'A[1] IS THE FIRST'\n");
    assert!(c.contains("\n'A(1) IS THE FIRST'\n"), "{c}");
}

#[test]
fn a_sample_keeps_its_notes_and_types_its_lines_as_apl80() {
    let s = apl_c80_port::sample("⍝!MODES (B)\n⍝ A note: ⍴ stays.\n)LOAD 2 CALC\nA[2]←⍴⍳5\n");
    assert_eq!(
        s,
        "⍝!MODES (C)\n⍝ A note: ⍴ stays.\n)LOAD 2 CALC\nA(2)←pi5\n"
    );
}

#[test]
fn a_sample_with_no_modes_line_gains_one() {
    assert_eq!(apl_c80_port::sample("⍳3\n"), "⍝!MODES (C)\ni3\n");
}

#[test]
fn a_branch_on_an_indexed_list_goes_through_a_local() {
    let c = port("x\n∇R←F X\n→(A,B)[X]\nA:R←1\n→0\nB:R←2\n∇\n");
    assert!(
        c.contains("\n)DEF R←F X;D.B\nD.B←A,B\n→D.B(X)\nA:R←1\n"),
        "{c}"
    );
}
