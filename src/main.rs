fn main() {
    println!("Hello, world!");



    // References as Values

    // The preceding examples shown in basics shows a pretty typical use for references. Allowing functions to access or manipulate a structure without taking ownership. But references are more flexible than that.



    // Rust References Versus C++ References

    // In C++ references are created implicitly by conversion, and dereferenced implicitly too:
    // C++ code!
    int x = 10;
    int &r = x; // initialization creates reference implicitly
    assert(r == 10); // implicitly dereference r to see x's value
    r = 20; // stores 20 in x, r itself still points to x

    // In Rust, references are created explicitly with the & operator, and dereferenced explicitly with the * operator:

    // Back to Rust code from this point onward
    let x = 10;
    let r = &x; // &x is a shared reference to x
    assert!(*r == 10); // explicitly dereference r

    // To create a mutable reference, use the &mut operator:
    let mut y = 32;
    let m = &mut y; // &mut y is a mutable reference to y
    *m += 32; // explicitly dereference m to set y's value
    assert!(*m == 64); // and to see y's new value

    // Looking back, when we fixed the show function to take the table of artists by reference instead of by value, we never had to use the * operator. Why is that?

    // Since references are so widely used in Rust, the . operator implicitly dereferences its left operand, if needed:
    struct Anime { name: &'static str, bechdel_pass: bool };
    let aria = Anime { name: "Aria: The Animation", bechdel_pass: true };
    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "Aria: The Animation");

    // Equivalent to the above, but without the dereference written out:
    assert_eq!((*anime_ref).name, "Aria: The Animation");

    // The println! macro used in the show function expands to code that uses the . operator, so it takes advantage of this implicit dereference as well.

    // The . operator can also implicitly borrow a reference to its left operand, if needed for a method call. For example, Vec's sort method takes a mutable reference to the vector, so the two calls shown here are equivalent:
    let mut v = vec![1973, 1968];
    v.sort() // implicitly borrows a mutable reference to v
    (&mut v).sort(); // equivalent; much uglier

    // In Rust, we use the & and * operators to create and follow references, with the exception of the . operator, which borrows and dereferences implicitly.



    // Assigning References

    // Assigning to a Rust reference makes it point at a new value:
    let x = 10;
    let y = 20;
    let mut r = &x;

    if b { r = &y; }

    assert!(*r ==10 || *r == 20);

    // The reference r initially points to x. But if b is true, the code points it at y instead (see page 160 for diagram).

}
