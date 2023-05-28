// use std::collections::{HashMap};
// use actix_web::{App, get ,HttpServer, web, Responder, HttpResponse, http};


// pub enum Sizes {
//     Filename10k,
//     Filename20k,
//     Filename50k,
//     Filename100k,
//     Filename1m,
//     Filename2m
// }

// // #[get("/send-file/{variant}")]
// pub fn send_file(file_size: Sizes) {
//     let mut filename: String = "".to_string();

//     match file_size {
//         Sizes::Filename10k => {
//             filename = "./10k";
//         },
//         Sizes::Filename20k => {
//             filename = "./20k";
//             filename = "./20k";

//         },
//         Sizes::Filename50k => {

//         },
//         Sizes::Filename100k => {

//         },
//         Sizes::Filename1m => {

//         },
//         Sizes::Filename2m => {

//         },
//     }

// }