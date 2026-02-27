// Lifetime borrow test\nfn first_word(s: &str) -> &str {\n    let bytes = s.as_bytes();\n    for (i, &b) in bytes.iter().enumerate() {\n        if b == b\" \" { return &s[..i]; }\n    }\n    s\n}
