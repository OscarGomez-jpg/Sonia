use std::{
    fs::{create_dir_all, File},
    io::Write,
    path::Path,
};

use rand::{rngs::StdRng, Rng, SeedableRng};

use super::fetcher::{fetch_memes, Meme};

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
            send: true,
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
    pub visited: Vec<usize>,
}

impl UrlManager {
    // This function is intended to create or recuperate the memes that we already fetch
    // So the bot don't have to do the web scrapping all the time
    pub async fn new() -> Self {
        let url = format!("{SAVES_URL}/{SAVE_FILE_NAME}");
        let path = Path::new(&url);

        let memes;
        let visited;
        if path.exists() {
            memes = remeber().await;
            visited = (0..memes.len()).collect();
        } else {
            create_dir_all(path).unwrap();
            memes = get_new_memes().await;
            visited = Vec::new();
        }

        Self { memes, visited }
    }

    //This will save all memes in the manager as the states of the already send memes
    pub fn save_state(&self) -> std::io::Result<()> {
        let url = format!("{SAVES_URL}/{SAVE_FILE_NAME}");
        let path = Path::new(&url);
        let file = File::create(path)?;
        let mut file = std::io::BufWriter::new(file);
        write!(file, "{}", serde_json::to_string(&self.memes).unwrap())?;
        Ok(())
    }

    pub async fn get_meme(&mut self) -> Meme {
        let mut rng = StdRng::from_entropy();

        // This is the case when get_new_memes fails
        if self.memes.len() == 1 && self.memes[0].send {
            return self.memes[0].clone();
        }

        let new_index = rng.gen_range(0..self.visited.len());
        let new_meme_index = self.visited.remove(new_index);
        self.memes[new_meme_index].send = true;
        let to_return = self.memes[new_meme_index].clone();

        // This is made to refresh memes selection and prevent infinite loop
        if self.visited.is_empty() {
            let new_memes = get_new_memes().await;
            self.visited = (0..new_memes.len()).collect();
            self.memes = new_memes;
        }

        to_return
    }
}
