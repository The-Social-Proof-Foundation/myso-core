module a::b;

fun f() {
    let x = myso::dynamic_field::borrow<vector<u8>, u64>(&parent, b"");
    let x = ::myso::dynamic_field::borrow<vector<u8>, u64>(&parent, b"");
}
