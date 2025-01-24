use std::{
    fs::{create_dir_all, File},
    io::{BufRead, Write},
    path::Path,
};

use rand::{rngs::StdRng, Rng, SeedableRng};

use super::fetcher::{fetch_memes, Meme};

const ERROR_IMAGE: &str =
    "https://res.cloudinary.com/dyegt26ww/image/upload/v1718390339/base_sonia_hbg5s4.png";
const SAVE_FILE_NAME: &str = "saved_links.json";
const SAVES_URL: &str = "memes_urls";

async fn get_new_memes() -> Vec<Meme> {
    let memes_urls_file =
        File::open("memes_urls/memes_urls.txt").expect("Could not open memes_urls.txt");
    let reader = std::io::BufReader::new(memes_urls_file);
    let memes_urls: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    println!("{:?}", memes_urls);

    let memes;
    if let Ok(memes_res) = fetch_memes(memes_urls).await {
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

async fn remember() -> Vec<Meme> {
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
    pub to_visit: Vec<usize>,
}

impl UrlManager {
    // This function is intended to create or recuperate the memes that we already fetch
    // So the bot don't have to do the web scrapping all the time
    pub async fn new() -> Self {
        let url = format!("{SAVES_URL}/{SAVE_FILE_NAME}");
        let path = Path::new(&url);

        let memes;
        let to_visit;
        if path.exists() {
            memes = remember().await;
        } else {
            let dir_path = Path::new(SAVES_URL);
            create_dir_all(dir_path).unwrap();
            memes = get_new_memes().await;
        }

        to_visit = (0..memes.len()).collect();
        Self { memes, to_visit }
    }

    //This will save all memes in the manager as the states of the already send memes
    pub fn save_state(&self) -> std::io::Result<()> {
        let dir_path = Path::new(SAVES_URL);
        if !dir_path.exists() {
            create_dir_all(dir_path)?;
        }

        let file_path = dir_path.join(SAVE_FILE_NAME);
        let file = File::create(file_path)?;
        let mut file = std::io::BufWriter::new(file);
        write!(file, "{}", serde_json::to_string(&self.memes).unwrap())?;
        Ok(())
    }

    /// This function will return a meme that hasn't been sent yet
    /// If all memes have been sent, it will fetch new memes
    /// If it fails to fetch new memes, it will return an error image
    /// If it fails to fetch new memes and there is no error image, it will return the last meme
    ///
    /// # Example
    /// ```rust
    /// let mut url_manager = UrlManager::new().await;
    /// let meme = url_manager.get_meme().await;
    /// ```
    ///
    /// # Panics
    /// This function will panic if it fails to fetch new memes and there is no error image
    ///
    /// # Returns
    /// This function will return a Meme struct
    pub async fn get_meme(&mut self) -> Meme {
        let mut rng = StdRng::from_entropy();

        // This is the case when get_new_memes fails
        if self.memes.len() == 1 && self.memes[0].send {
            return self.memes[0].clone();
        }

        // println!("{:?}", self.to_visit);
        // println!("{:?}", self.memes);

        // This ensures that we don't send the same meme twice

        //Here I create a new index to visit and remove it from the list
        let new_index = rng.gen_range(0..self.to_visit.len());

        //Here I get an Index to access the meme vector
        let new_meme_index = self.to_visit.remove(new_index);

        //Here I set the meme as send
        self.memes[new_meme_index].send = true;

        //Here I return the meme
        let to_return = self.memes[new_meme_index].clone();

        // This is made to refresh memes selection and prevent infinite loop
        if self.to_visit.is_empty() {
            let new_memes = get_new_memes().await;
            self.to_visit = (0..new_memes.len()).collect();
            self.memes = new_memes;
        }

        to_return
    }
}
