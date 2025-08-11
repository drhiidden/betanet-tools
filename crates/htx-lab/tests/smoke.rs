#[test]
fn smoke_htx_lab() {
    let s = htx_lab::simulate_paths(3, 20);
    assert_eq!(s.len(), 3);
    let c = htx_lab::generate_example_control_stream();
    assert!(!c.is_empty());
    let enc = htx_lab::encode_frame(b"x");
    let dec = htx_lab::decode_frame(&enc).unwrap();
    assert_eq!(dec, b"x");
}
