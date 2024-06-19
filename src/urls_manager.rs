use std::{collections::HashSet, fs::File, io::Write, path::Path};

use rand::{rngs::StdRng, Rng, SeedableRng};

use crate::fetcher::{fetch_memes, Meme};

const ERROR_IMAGE: &str =
    "https://res.cloudinary.com/dyegt26ww/image/upload/v1718390339/base_sonia_hbg5s4.png";
const SAVE_FILE_NAME: &str = "last_save.json";
const SAVES_URL: &str = "memes_urls";

async fn get_new_memes() -> Vec<Meme> {
    let memes;

    if let Ok(memes_res) = fetch_memes().await {
        memes = memes_res;
    } else {
        println!("Error fetching memes");
        memes = vec![Meme {
            url: ERROR_IMAGE.to_string(),
            text: "Could not find any meme".to_string(),
            send: false,
        }];
    }

    memes
}

async fn remeber() -> Vec<Meme> {
    let url = format!("{SAVES_URL}/{SAVE_FILE_NAME}");
    let path = Path::new(&url);
    let mut memes = Vec::new();
    match File::open(path) {
        Ok(file) => {
            if let Ok(old_memes) = serde_json::from_reader(file) {
                memes = old_memes;
            }
        }
        Err(why) => {
            println!("{why:?}");
            memes = get_new_memes().await;
        }
    }

    memes
}

pub struct UrlManager {
    pub memes: Vec<Meme>,
    pub visited: HashSet<usize>,
}

impl UrlManager {
    pub async fn new() -> Self {
        let url = format!("{SAVES_URL}/{SAVE_FILE_NAME}");
        let path = Path::new(&url);

        let memes;
        let visited;
        if path.exists() {
            memes = remeber().await;
            visited = memes
                .iter()
                .enumerate()
                .filter_map(|(i, m)| if m.send { Some(i) } else { None })
                .collect();
        } else {
            memes = get_new_memes().await;
            visited = HashSet::new();
        }

        Self { memes, visited }
    }

    pub fn save_state(&self) -> std::io::Result<()> {
        let file = File::create(SAVE_FILE_NAME)?;
        let mut file = std::io::BufWriter::new(file);
        write!(file, "{}", serde_json::to_string(&self.memes).unwrap())?;
        Ok(())
    }

    pub async fn get_meme(&mut self) -> Meme {
        let mut rng = StdRng::from_entropy();

        // This is the case when get_new_memes fails
        if self.memes.len() == 1 {
            return self.memes[0].clone();
        }

        let mut new_meme_index;
        loop {
            new_meme_index = rng.gen_range(0..self.memes.len());

            // This prevents to fetch the same meme
            if !self.visited.contains(&new_meme_index) {
                self.visited.insert(new_meme_index);
                self.memes[new_meme_index].send = true;
                return self.memes[new_meme_index].clone();
            }

            // This is made to refresh memes selection and prevent infinite loop
            if self.visited.len() == self.memes.len() {
                self.visited = HashSet::new();
                self.memes = get_new_memes().await;
            }
        }
    }
}
