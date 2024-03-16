fn main() {
    let method = b"GET";
    // the above is a reference to a byte array not a string
    assert_eq!(method, &[b'G', b'E', b'T']);
}
