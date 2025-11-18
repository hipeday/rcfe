use rcfe::ByteSequence;

#[test]
fn test_next() {
    // Basic test case
    let from = ByteSequence::from("abc");
    let next_seq = from.next();
    assert_eq!(next_seq, ByteSequence::from("abd"));

    // Test case with 0xff bytes
    let from = ByteSequence::from(b"ab\xff\xff" as &[u8]); // [0x61,0x62,0xff,0xff]
    let next_seq = from.next();
    assert_eq!(next_seq.as_bytes(), b"ac");

    // Test case where all bytes are 0xff
    let from = ByteSequence::from(b"\xff\xff" as &[u8]); // [0xff,0xff]
    let next_seq = from.next();
    assert_eq!(next_seq.as_bytes(), ByteSequence::empty().as_bytes());
}