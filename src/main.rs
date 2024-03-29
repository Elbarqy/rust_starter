#[macro_use]
extern crate rocket;

use rocket::http::ContentType;
use rocket::Data;
use rocket_multipart_form_data::{
    mime, MultipartFormData, MultipartFormDataField, MultipartFormDataOptions, Repetition,
};

#[post("/", data = "<data>")]
async fn index(content_type: &ContentType, data: Data<'_>) -> &'static str {
    let mut options = MultipartFormDataOptions::with_multipart_form_data_fields(vec![
        MultipartFormDataField::file("photo")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        MultipartFormDataField::file("photo2")
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap(),
        MultipartFormDataField::raw("fingerprint").size_limit(4096),
        MultipartFormDataField::text("name"),
        MultipartFormDataField::text("email").repetition(Repetition::fixed(3)),
        MultipartFormDataField::text("email"),
    ]);

    let mut multipart_form_data = MultipartFormData::parse(content_type, data, options)
        .await
        .unwrap();
    let photo = multipart_form_data.files.get("photo"); // Use the get method to preserve file fields from moving out of the MultipartFormData instance in order to delete them automatically when the MultipartFormData instance is being dropped
    let photo2 = multipart_form_data.files.get("photo2"); // Use the get method to preserve file fields from moving out of the MultipartFormData instance in order to delete them automatically when the MultipartFormData instance is being dropped
    let fingerprint = multipart_form_data.raw.remove("fingerprint"); // Use the remove method to move raw fields out of the MultipartFormData instance (recommended)
    let name = multipart_form_data.texts.remove("name"); // Use the remove method to move text fields out of the MultipartFormData instance (recommended)
    let email = multipart_form_data.texts.remove("email");

    println!("{:#?}", photo);
    println!("{:#?}", photo2);
    if let Some(file_fields) = photo {
        let file_field = &file_fields[0]; // Because we only put one "photo" field to the allowed_fields, the max length of this file_fields is 1.

        let _content_type = &file_field.content_type;
        let _file_name = &file_field.file_name;
        let _path = &file_field.path;
        println!("{:#?}", _path)
        // You can now deal with the uploaded file.
    }

    if let Some(mut raw_fields) = fingerprint {
        let raw_field = raw_fields.remove(0); // Because we only put one "fingerprint" field to the allowed_fields, the max length of this raw_fields is 1.

        let _content_type = raw_field.content_type;
        let _file_name = raw_field.file_name;
        let _raw = raw_field.raw;

        // You can now deal with the raw data.
    }

    if let Some(mut text_fields) = name {
        let text_field = text_fields.remove(0); // Because we only put one "text" field to the allowed_fields, the max length of this text_fields is 1.

        let _content_type = text_field.content_type;
        let _file_name = text_field.file_name;
        let _text = text_field.text;

        // You can now deal with the text data.
    }

    if let Some(text_fields) = email {
        for text_field in text_fields {
            // We put "email" field to the allowed_fields for two times and let the first time repeat for 3 times, so the max length of this text_fields is 4.
            let _content_type = text_field.content_type;
            let _file_name = text_field.file_name;
            let _text = text_field.text;

            // You can now deal with the text data.
        }
    }

    "ok"
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
