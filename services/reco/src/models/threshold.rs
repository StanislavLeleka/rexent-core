use std::env;

#[derive(Debug)]
pub struct Threshold {
    pub tfidf: f32,
    pub cosine: f32,
    pub accuracy: f32,
}

impl Threshold {
    pub fn from_env() -> Self {
        Self {
            tfidf: env::var("TFIDF").expect("TFIDF not set").parse().unwrap(),
            cosine: env::var("COSINE").expect("COSINE not set").parse().unwrap(),
            accuracy: env::var("ACCURACY")
                .expect("ACCURACY not set")
                .parse()
                .unwrap(),
        }
    }
}
