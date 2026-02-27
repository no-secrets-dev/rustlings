use std::sync::mpsc; use std::thread;\nfn main() { let (tx,rx) = mpsc::channel(); thread::spawn(move || tx.send(42).unwrap()); println\!("{}", rx.recv().unwrap()); }
