use std::{
    env, fs,
    io::{stdout, Write},
};

fn main() {
    let mut args = env::args().skip(1);
    let set = args.next().expect("character set not provided");
    let path = args.next().unwrap_or("words.txt".to_string());
    let mut stdout = stdout().lock();

    fs::read(path)
        .expect("failed to read dictionary file")
        .split(u8::is_ascii_whitespace)
        .filter(|word| word.len() > 3)
        .filter(|word| bee::fits::<32>(word, set.as_bytes()))
        .for_each(|word| {
            stdout
                .write_all(word)
                .and_then(|_| stdout.write_all(b"\n"))
                .expect("failed to write to stdout");
        });
}
