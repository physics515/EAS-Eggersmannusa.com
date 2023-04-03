use image::io::Reader as ImageReader;
use image::ImageOutputFormat;
use scraper::{Html, Selector};
use serde_json::json;
use std::io::Cursor;

pub struct EggersmannUSACom;

impl EggersmannUSACom {
	pub fn blog_thumbnail(url: String) -> (rocket::http::ContentType, Vec<u8>) {
		let page = match reqwest::blocking::get(url.clone()) {
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
		let body = match Selector::parse("body") {
			Ok(body) => body,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing body selector.")).to_string().into_bytes()),
		};

		let site = match Selector::parse("div.site") {
			Ok(site) => site,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing site selector.")).to_string().into_bytes()),
		};

		let site_inner = match Selector::parse("div.site-inner") {
			Ok(site_inner) => site_inner,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing site_inner selector.")).to_string().into_bytes()),
		};

		let site_main = match Selector::parse("div.site-main") {
			Ok(site_main) => site_main,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing site_main selector.")).to_string().into_bytes()),
		};

		let container = match Selector::parse("div.container") {
			Ok(container) => container,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing container selector.")).to_string().into_bytes()),
		};

		let row = match Selector::parse("div.row") {
			Ok(row) => row,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing row selector.")).to_string().into_bytes()),
		};

		let main = match Selector::parse("main#site-content") {
			Ok(main) => main,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing main selector.")).to_string().into_bytes()),
		};

		let site_content_inner = match Selector::parse("div.site-content-inner") {
			Ok(site_content_inner) => site_content_inner,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing site_content_inner selector.")).to_string().into_bytes()),
		};

		let page_content = match Selector::parse("div.page-content") {
			Ok(page_content) => page_content,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing page_content selector.")).to_string().into_bytes()),
		};

		let single_post_detail = match Selector::parse("div.single-post-detail") {
			Ok(single_post_detail) => single_post_detail,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing single_post_detail selector.")).to_string().into_bytes()),
		};

		let article = match Selector::parse("article") {
			Ok(article) => article,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing article selector.")).to_string().into_bytes()),
		};

		let entry_thumbnail = match Selector::parse("div.entry-thumbnail") {
			Ok(entry_thumbnail) => entry_thumbnail,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing entry_thumbnail selector.")).to_string().into_bytes()),
		};

		let a = match Selector::parse("a") {
			Ok(a) => a,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing a selector.")).to_string().into_bytes()),
		};

		let img = match Selector::parse("img") {
			Ok(img) => img,
			Err(_) => return (rocket::http::ContentType::JSON, json!(format!("Error parsing img selector.")).to_string().into_bytes()),
		};

		let body = match document.select(&body).next() {
			Some(body) => body,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting body")).to_string().into_bytes()),
		};

		let site = match body.select(&site).next() {
			Some(site) => site,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting site")).to_string().into_bytes()),
		};

		let site_inner = match site.select(&site_inner).next() {
			Some(site_inner) => site_inner,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting site_inner")).to_string().into_bytes()),
		};

		let site_main = match site_inner.select(&site_main).next() {
			Some(site_main) => site_main,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting site_main")).to_string().into_bytes()),
		};

		let container = match site_main.select(&container).next() {
			Some(container) => container,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting container")).to_string().into_bytes()),
		};

		let row = match container.select(&row).next() {
			Some(row) => row,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting row")).to_string().into_bytes()),
		};

		let main = match row.select(&main).next() {
			Some(main) => main,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting main")).to_string().into_bytes()),
		};

		let site_content_inner = match main.select(&site_content_inner).next() {
			Some(site_content_inner) => site_content_inner,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting site_content_inner")).to_string().into_bytes()),
		};

		let page_content = match site_content_inner.select(&page_content).next() {
			Some(page_content) => page_content,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting page_content")).to_string().into_bytes()),
		};

		let single_post_detail = match page_content.select(&single_post_detail).next() {
			Some(single_post_detail) => single_post_detail,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting single_post_detail")).to_string().into_bytes()),
		};

		let article = match single_post_detail.select(&article).next() {
			Some(article) => article,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting article")).to_string().into_bytes()),
		};

		let entry_thumbnail = match article.select(&entry_thumbnail).next() {
			Some(entry_thumbnail) => entry_thumbnail,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting entry_thumbnail")).to_string().into_bytes()),
		};

		let a = match entry_thumbnail.select(&a).next() {
			Some(a) => a,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting a")).to_string().into_bytes()),
		};

		let img = match a.select(&img).next() {
			Some(img) => img,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting img")).to_string().into_bytes()),
		};

		let src = match img.value().attr("data-lazy-src") {
			Some(src) => src,
			None => return (rocket::http::ContentType::JSON, json!(format!("Error getting src")).to_string().into_bytes()),
		};

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
						match img.write_to(&mut buffer, ImageOutputFormat::Jpeg(100)) {
							Ok(_) => (),
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

