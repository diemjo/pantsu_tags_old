use std::path::Path;
use reqwest::blocking::{Client, Response, multipart};
use select::document::Document;
use select::predicate::{Attr, Name};
use crate::common::error::Error;
use super::SauceMatch;

const IQDB_ADDRESS: &str = "https://gelbooru.iqdb.org/";

pub fn find_sauce(image_path: &Path) -> Result<Vec<SauceMatch>, Error> {
    let client = Client::new();
    let form = multipart::Form::new()
        .file("file", image_path).or_else(
        |err| {
            let path = String::from(image_path.to_str().unwrap_or_default());
            Err(Error::ImageNotFound(err, path))
        })?;
    let response = client.post(IQDB_ADDRESS)
        .multipart(form)
        .send()?;
    if !response.status().is_success() {
        return Err(Error::BadResponse(format!("status code {}", String::from(response.status().as_str()))));
    }

    let response = response.text()?;
    let html = Document::from(response.as_str());
    extract_sauce( &html)
}

fn extract_sauce(html: &Document) -> Result<Vec<SauceMatch>, Error> {
    let mut res: Vec<SauceMatch> = Vec::new();
    let mut pages = html.find(Attr("id", "pages"));
    let pages = match pages.next() {
        Some(pages) => pages,
        None => return Ok(res),
    };

    for (idx, img_match) in pages.children().enumerate() {
        if idx == 0 {
            continue; // skip uploaded image
        }

        let mut sauce_link: Option<String> = None;
        let mut sauce_similarity: Option<f32> = None;
        for (idx, node) in img_match.find(Name("tr")).enumerate() {
            match idx {
                0 | 2 => continue,
                1 => {
                    let td_or_th = node.first_child();
                    if td_or_th.is_none() {
                        continue;
                    }
                    let td_or_th = td_or_th.unwrap();

                    if td_or_th.is(Name("th")) {
                        break;
                    }

                    let link = td_or_th.first_child();
                    if link.is_none() {
                        break;
                    }
                    let href = link.unwrap().attr("href");
                    if href.is_none() {
                        break;
                    }
                    let href = href.unwrap();
                    sauce_link = if href.starts_with("//") {
                        Some("https:".to_string() + href)
                    } else {
                        Some(href.to_string())
                    };
                },
                3 => {
                    let td = node.first_child();
                    if td.is_none() {
                        continue;
                    }
                    let td = td.unwrap();
                    let text = td.text();
                    let similarity = text.split('%').collect::<Vec<&str>>()[0];
                    sauce_similarity = match similarity.parse::<f32>() {
                        Ok(similarity) => Ok(Some(similarity)),
                        Err(e) => Err(Error::BadResponse(String::from("Conversion to float failed"))),
                    }?;
                }
                _ => break,
            }
        }
        if let (Some(link), Some(similarity)) = (sauce_link, sauce_similarity) {
            let sauce_match = SauceMatch {
                link,
                similarity,
            };
            res.push(sauce_match);
        }
    }

    Ok(res)
}
