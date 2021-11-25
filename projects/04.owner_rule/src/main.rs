
/**
 * variables copy
 */
fn _copy() {
    // x and y value stacked memory
    let x: u64 = 2;
    let y = x;
    println!("x copy to y. x={}, y={}", x, y);

    // s1 and s2 value stacked memory
    let s1 = "ab";
    let s2 = s1;
    println!("s1 copy to s2. s1={}, s2={}", s1, s2);

    let _s1 = String::from("test");
    let _s2 = _s1;
    // NG, because _s1 move to _s2(_s1 value allocated heep memory)
    // println!("_s1 copy to _s2. _s1={}, _s2={}", _s1, _s2);

    // OK, because __s1 copy to __s2
    let __s1 = String::from("test");
    let __s2 = __s1.clone();
    println!("__s1 copy to __s2. __s1={}, __s2={}", __s1, __s2);
}

/**
 * Function and Ownership
 */
fn _call_func() {
    let s = String::from("xxxx");
    __sub_str_func(s);  // s is move to function
    // For, follow is NG
    // println!("{}", s);

    let x = 5;
    __sub_int_func(x);
}
fn __sub_str_func(s: String) {
    println!("Param is '{}'", s);
}
fn __sub_int_func(x: u32) {
    println!("Param is '{}'", x);
}

/**
 * Return value and Ownership
 */
fn _return_fn() {
    let s1 = __sub_return_fn();
    let s2 = String::from("pass value");
    let s3 = __sub_pass_value(s2);
    println!("s1 is '{}', s3 is '{}'", s1, s3);
    // NG, because s2 is move to function, s3
    // println!("s2 is '{}'", s2);
}
fn __sub_return_fn() -> String {
    let ret = String::from("return value");
    return ret;
}
fn __sub_pass_value(s: String) -> String {
    return s;
}

/**
 * Refrence and Ownership
 */
fn _ref() {
    let s1 = String::from("pass value");
    let (ret, len) = __no_ref(s1);
    println!("s1 is length:'{}'. (s1 ownership return to ret:{})", len, ret);

    let s2 = String::from("pass value");
    let len = __use_ref(&s2);   // s2 don't move function, because referance parameter.
    println!("s2 is length:'{}'. (s2:'{}')", len, s2);

    let mut s3 = String::from("change");
    __change_ref(&mut s3);
    println!("s3 changed to '{}'", s3);

    let mut s4 = String::from("ref value");
    let r1 = &mut s4;
    println!("r1 = '{}'", r1);
    let r2 = &mut s4;
    // 2018~ no compile error, Non-lexical lifetime(NLL)
    // https://teratail.com/questions/311154
    println!("r2 = '{}'", r2); // When r2 -> r1, error
    // NG
    // println!("r1 = '{}'", r1);

    let mut s5 = String::from("mutable value");
    let rr1 = &s5;
    let rr2 = &s5;
    println!("rr1 = '{}', rr2 = '{}'", rr1, rr2);
    let rr3 = &mut s5;
    // 2018~ no compile error, Non-lexical lifetime(NLL)
    // https://teratail.com/questions/311154
    println!("rr3 = '{}'", rr3);
    // NG
    // println!("rr3 = '{}'", rr2);

}
fn __no_ref(s: String) -> (String, usize) {
    let len = s.len();
    return (s, len);
}
fn __use_ref(s: &String) -> usize {
    s.len()
}
// NG, becauce default reference value can not change
// fn __change_ref(s: &String) {
//     s.push_str(".suffix");
// }
fn __change_ref(s: &mut String) {
    s.push_str(".suffix");
}

// NG: Return value out of scope drop memory
// fn _dangle() -> &String {
//     let s = String::from("dangle!!");
//     &s
// }

/**
 * Slice type data
 *  No ownership data
 */
// May be a bug
fn _get_first_word_index(s: &String) -> usize {
    let wbytes = s.as_bytes();
    for (index, &char) in wbytes.iter().enumerate() {
        if char == b' ' {
            return index;
        }
    }
    return s.len();
}
// improved
fn _get_first_word_pointer(s: &String) -> &str {
    let wbytes = s.as_bytes();
    for (index, &char) in wbytes.iter().enumerate() {
        if char == b' ' {
            return &s[..index]; // as &s[0..index]
        }
    }
    return &s[..]; // as &s[..s.len()]
}

// improved2
fn _get_first_word(s: &str) -> &str {
    let wbytes = s.as_bytes();
    for (index, &char) in wbytes.iter().enumerate() {
        if char == b' ' {
            return &s[..index]; // as &s[0..index]
        }
    }
    return &s[..]; // as &s[..s.len()]
}

fn main() {
    _ref();
    _return_fn();
    _call_func();
    _copy();
    // String type (is not String's literal)
    let mut s = String::from("String type.");
    s.push_str(" 'add string literal'.");
    println!("{}", s);

    // Follow code is bug:
    //   'words' is empty, but first_word_index value is not empty.
    let mut words = String::from("12345 xxxx");
    let first_word_index = _get_first_word_index(&words);
    println!("'{}' of first word index is '{}'", words, first_word_index);
    words.clear();

    // Follow code is compile error:
    //   first_word_point value is immutable.
    let mut words = String::from("123456 yyyy");
    let first_word_pointer = _get_first_word_pointer(&words);
    println!("'{}' of first word is '{}'", words, first_word_pointer);
    words.clear();
    // No error, Maybe, new feature

    // Strin literal is &str(String slice type).
    let lwords = "word01, word02";
    let _1st_word = _get_first_word(lwords);
    println!("'{}' of first word is '{}'", lwords, _1st_word);

    // Other Slice type
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..3]; // 'slice' data type is '&[i32]
    println!("Array slice '{}', '{}'", slice[0], slice[slice.len() - 1]);

} // スコープを抜けるタイミングでメモリが解放
