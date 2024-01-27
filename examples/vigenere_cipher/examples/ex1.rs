use cipher::Cipher;

fn main() {
    let text = "HELLO";
    let key = "RUST";
    let cipher = Cipher::new(key);
    println!("{text} => {:?}", cipher.encrypt(text));
}
