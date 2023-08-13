use std::{io::Write, path::PathBuf};

use reqwest::{blocking, Error};
use serde::Deserialize;

const ARCHIVE_URL: &str = "https://www.bing.com/HPImageArchive.aspx?format=js&n=8";
const BASE_URL: &str = "https://bing.com";

#[derive(Deserialize, Debug)]
pub struct Data {
    pub startdate: String,
    pub fullstartdate: String,
    pub enddate: String,
    pub url: String,
    pub urlbase: String,
    pub copyright: String,
    pub copyrightlink: String,
    pub title: String,
}

#[derive(Deserialize, Debug)]
pub struct Json {
    pub images: Vec<Data>,
}

#[derive(Deserialize, Debug)]
pub struct Images {
    pub json: Json,
    pub current: u8,
}

impl Images {
    pub fn new() -> Result<Images, Error> {
        let images: Json = blocking::get(ARCHIVE_URL)?.json::<Json>().unwrap();

        Ok(Self {
            json: images,
            current: 0,
        })
    }

    pub fn image_url(endpoint: String) -> String {
        let mut url = BASE_URL.to_owned();
        url.push_str(&endpoint);
        url
    }

    pub fn fetch_image(url: String) -> Result<bytes::Bytes, Error> {
        Ok(blocking::get(url)?.bytes()?)
    }

    pub fn save_image(
        &self,
        path: PathBuf,
        buf: bytes::Bytes,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut file = std::fs::File::create(path).unwrap();

        file.write_all(&buf)?;

        Ok(())
    }

    pub fn download_image(&self) {
        let curr_endpoint = &self.json.images[self.current as usize].url;
        let img_buffer = Images::fetch_image(Images::image_url(curr_endpoint.to_string())).unwrap();

        let _ = self.save_image(self.image_path(), img_buffer);
    }

    pub fn image_path(&self) -> PathBuf {
        let pictures_dir = xdg_user::pictures().unwrap();
        let date = &self.json.images[self.current as usize].fullstartdate;

        match pictures_dir {
            Some(mut d) => {
                d.push(date);
                d.set_extension("jpg");

                return d;
            }
            None => {
                panic!("Cannot save image")
            }
        };
    }
}
