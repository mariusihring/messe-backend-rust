use rand::Rng;

pub async fn create_salt(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let string: String = (0..length)
        .map(|_| rng.gen_range(b'a'..b'z' + 1) as char)
        .collect();
    return string;
}
