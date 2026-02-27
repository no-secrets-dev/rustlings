struct Excerpt<'a> { part: &'a str }\nfn main() { let s = String::from("hello world"); let e = Excerpt { part: &s[..5] }; println\!("{}", e.part); }
