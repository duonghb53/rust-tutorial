use serde::{Serialize, Deserialize};
use std::{
    fs::File,
    io::BufReader,
    path::Path,
    error::Error
};
use std::io::prelude::*;
use serde_json::json;

fn main() {
    println!("Hello, world!");
    let mut f = File::open("./src/data.json").unwrap();
    let mut buffer = [0; 10000];

    // read up to 1000 bytes
    let n = f.read(&mut buffer[..]).unwrap();

    // println!("The bytes: {:?}", &buffer[..n]);

    let unique_one_metadata: Option<UniqueOneBaseMetadata> = match serde_json::from_slice(&buffer[..n]) {
        Ok(metadata) => metadata,
        Err(err) => {
            println!("Failed to parse data to unique_one base metadata {:?}", err);
            None
        },
    };
    println!("unique_one metadata is : {:?}", unique_one_metadata);
    let unique_one_metadata = unique_one_metadata.unwrap();

    let title = (unique_one_metadata.name.len() != 0).then_some(unique_one_metadata.name);
		let description =
			(unique_one_metadata.description.len() != 0).then_some(unique_one_metadata.description);
		let image = (unique_one_metadata.image.len() != 0).then_some(unique_one_metadata.image);

		let extra = json!({
			"externalUrl": unique_one_metadata.external_url,
			"alternativeImage": unique_one_metadata.alternative_image,
			"imageMimeType": unique_one_metadata.image_mime_type,
			"animationUrl": unique_one_metadata.animation_url,
			"alternativeAnimationUrl": unique_one_metadata.alternative_animation_url,
			"animationMimeType": unique_one_metadata.animation_mime_type,
		});

		// parse unique_one base metadata to nep171 format
		let metadata = Nep171TokenMetadata {
			title,
			description,
			media: image,
			media_hash: None,
			copies: None,
			issued_at: None,
			expires_at: None,
			starts_at: None,
			updated_at: None,
			extra: Some(extra.to_string()),
			reference: None,
			reference_hash: None,
		};
		println!("After, the Nep171 media data is {:?} ", metadata);
}


#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
struct UniqueOneBaseMetadata {
	// NFT name, required
	name: String,
	// General notes, abstracts, or summaries about the contents of an NFT.
	#[serde(default)]
	description: String,

	#[serde(default)]
	image: String,

	#[serde(default)]
	external_url: String,

	#[serde(default)]
	alternative_image: String,

	#[serde(default)]
	image_mime_type: String,

	#[serde(default)]
	animation_url: String,

	#[serde(default)]
	alternative_animation_url: String,

	#[serde(default)]
	animation_mime_type: String,

    #[serde(default)]
    attributes: Vec<Attribute>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Attribute {
	// display_type: String,
	trait_type: String,
	value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nep171TokenMetadata {
	// ex. "Arch Nemesis: Mail Carrier" or "Parcel #5055"
	pub title: Option<String>,
	// free-form description
	pub description: Option<String>,
	// URL to associated media, preferably to decentralized, content-addressed storage
	pub media: Option<String>,
	// Base64-encoded sha256 hash of content referenced by the `media` field. Required if `media`
	// is included.
	pub media_hash: Option<Vec<u8>>,
	// number of copies of this set of metadata in existence when token was minted.
	pub copies: Option<u64>,
	// ISO 8601 datetime when token was issued or minted
	pub issued_at: Option<String>,
	// ISO 8601 datetime when token expires
	pub expires_at: Option<String>,
	// ISO 8601 datetime when token starts being valid
	pub starts_at: Option<String>,
	// ISO 8601 datetime when token was last updated
	pub updated_at: Option<String>,
	// anything extra the NFT wants to store on-chain. Can be stringified JSON.
	pub extra: Option<String>,
	// URL to an off-chain JSON file with more info.
	pub reference: Option<String>,
	// Base64-encoded sha256 hash of JSON from reference field. Required if `reference` is
	// included.
	pub reference_hash: Option<Vec<u8>>,
}