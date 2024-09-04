#![warn(clippy::pedantic, clippy::nursery, clippy::all, clippy::cargo)]
#![allow(clippy::multiple_crate_versions, clippy::module_name_repetitions)]

use std::io::Cursor;

use image::ImageReader;
use image::ImageFormat;
use scraper::{Html, Selector};
use serde_json::json;

pub struct EggersmannUSACom;

impl EggersmannUSACom {
	#[must_use]
	pub fn blog_thumbnail(url: &str) -> (rocket::http::ContentType, Vec<u8>) {
		let page = match reqwest::blocking::get(url.to_string()) {
			Ok(page) => {
				let page = page.text();
				match page {
					Ok(page) => page,
					Err(err) => return (rocket::http::ContentType::JSON, json!(format!("Error getting URL text: {url}, Error: {err}")).to_string().into_bytes()),
				}
			}
			Err(err) => return (rocket::http::ContentType::JSON, json!(format!("Error getting URL: {url}, Error: {err}")).to_string().into_bytes()),
		};

		let document = Html::parse_document(&page);

		//get the div with class entry-thumbnail and get the first img tag inside it and get the src attribute
		let Ok(body) = Selector::parse("body") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing body selector.")).to_string().into_bytes()) };
		let Ok(site) = Selector::parse("div.site") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing site selector.")).to_string().into_bytes()) };
		let Ok(site_inner) = Selector::parse("div.site-inner") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing site_inner selector.")).to_string().into_bytes()) };
		let Ok(site_main) = Selector::parse("div.site-main") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing site_main selector.")).to_string().into_bytes()) };
		let Ok(container) = Selector::parse("div.container") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing container selector.")).to_string().into_bytes()) };
		let Ok(row) = Selector::parse("div.row") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing row selector.")).to_string().into_bytes()) };
		let Ok(main) = Selector::parse("main#site-content") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing main selector.")).to_string().into_bytes()) };
		let Ok(site_content_inner) = Selector::parse("div.site-content-inner") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing site_content_inner selector.")).to_string().into_bytes()) };
		let Ok(page_content) = Selector::parse("div.page-content") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing page_content selector.")).to_string().into_bytes()) };
		let Ok(single_post_detail) = Selector::parse("div.single-post-detail") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing single_post_detail selector.")).to_string().into_bytes()) };
		let Ok(article) = Selector::parse("article") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing article selector.")).to_string().into_bytes()) };
		let Ok(entry_thumbnail) = Selector::parse("div.entry-thumbnail") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing entry_thumbnail selector.")).to_string().into_bytes()) };
		let Ok(a) = Selector::parse("a") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing a selector.")).to_string().into_bytes()) };
		let Ok(img) = Selector::parse("img") else { return (rocket::http::ContentType::JSON, json!(format!("Error parsing img selector.")).to_string().into_bytes()) };
		
                let Some(body) = document.select(&body).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting body")).to_string().into_bytes()) };
		let Some(site) = body.select(&site).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting site")).to_string().into_bytes()) };
		let Some(site_inner) = site.select(&site_inner).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting site_inner")).to_string().into_bytes()) };
		let Some(site_main) = site_inner.select(&site_main).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting site_main")).to_string().into_bytes()) };
		let Some(container) = site_main.select(&container).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting container")).to_string().into_bytes()) };
		let Some(row) = container.select(&row).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting row")).to_string().into_bytes()) };
		let Some(main) = row.select(&main).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting main")).to_string().into_bytes()) };
		let Some(site_content_inner) = main.select(&site_content_inner).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting site_content_inner")).to_string().into_bytes()) };
		let Some(page_content) = site_content_inner.select(&page_content).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting page_content")).to_string().into_bytes()) };
		let Some(single_post_detail) = page_content.select(&single_post_detail).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting single_post_detail")).to_string().into_bytes()) };
		let Some(article) = single_post_detail.select(&article).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting article")).to_string().into_bytes()) };
		let Some(entry_thumbnail) = article.select(&entry_thumbnail).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting entry_thumbnail")).to_string().into_bytes()) };
		let Some(a) = entry_thumbnail.select(&a).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting a")).to_string().into_bytes()) };
		let Some(img) = a.select(&img).next() else { return (rocket::http::ContentType::JSON, json!(format!("Error getting img")).to_string().into_bytes()) };
		let Some(src) = img.value().attr("data-lazy-src") else { return (rocket::http::ContentType::JSON, json!(format!("Error getting src")).to_string().into_bytes()) };

		let client = reqwest::blocking::get(src);
		match client {
			Ok(client) => {
				let client = client.bytes();
				match client {
					Ok(client) => {
						let img = match ImageReader::new(Cursor::new(client)).with_guessed_format() {
							Ok(img) => match img.decode() {
								Ok(img) => img,
								Err(err) => return (rocket::http::ContentType::JSON, json!(format!("error: Error decoding image: {src}, Error: {err}")).to_string().into_bytes()),
							},
							Err(err) => return (rocket::http::ContentType::JSON, json!(format!("error: Error getting format: {src}, Error: {err}")).to_string().into_bytes()),
						};
						let img = img.into_rgba8();
						let mut buffer = Cursor::new(Vec::new());
						match img.write_to(&mut buffer, ImageFormat::Jpeg) {
							Ok(()) => (),
							Err(err) => return (rocket::http::ContentType::JSON, json!(format!("error: Error writing to buffer: {src}, Error: {err}")).to_string().into_bytes()),
						};
						let client = buffer.into_inner();

						(rocket::http::ContentType::JPEG, client)
					}
					Err(err) => (rocket::http::ContentType::JSON, json!(format!("error: Error getting SRC: {src}, Error: {err}")).to_string().into_bytes()),
				}
			}
			Err(err) => (rocket::http::ContentType::JSON, json!(format!("error: Error getting SRC: {src}, Error: {err}")).to_string().into_bytes()),
		}
	}
}
