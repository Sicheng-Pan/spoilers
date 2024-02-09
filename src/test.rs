// use ocrs::{OcrEngine, OcrEngineParams};
// use rten::Model;
// use rten_tensor::NdTensorView;
// use std::fs;
// use xcap::image::{
//     self,
//     imageops::{brighten, contrast},
// };

// #[test]
// fn capture() {}

// #[test]
// fn ocr() {
//     let mut img = image::open("target/wip/sg.jpg").unwrap().into_rgb8();
//     img = brighten(&img, -16);
//     img = contrast(&img, 64.0);

//     let (w, h) = img.dimensions();
//     let layout = img.sample_layout();
//     let chw_tensor = NdTensorView::from_slice(
//         img.as_raw().as_slice(),
//         [h as usize, w as usize, 3],
//         Some([
//             layout.height_stride,
//             layout.width_stride,
//             layout.channel_stride,
//         ]),
//     )
//     .unwrap()
//     .permuted([2, 0, 1]) // HWC => CHW
//     .to_tensor() // Make tensor contiguous, which makes `map` faster
//     .map(|x| *x as f32 / 255.); // Rescale from [0, 255] to [0, 1]

//     let detection_model_data = fs::read("target/wip/dt.rten").unwrap();
//     let recognition_model_data = fs::read("target/wip/rc.rten").unwrap();
//     let detection_model = Model::load(&detection_model_data).unwrap();
//     let recognition_model = Model::load(&recognition_model_data).unwrap();

//     let engine = OcrEngine::new(OcrEngineParams {
//         detection_model: Some(detection_model),
//         recognition_model: Some(recognition_model),
//         ..Default::default()
//     })
//     .unwrap();

//     let ocr_input = engine.prepare_input(chw_tensor.view()).unwrap();

//     // Phase 1: Detect text words
//     let word_rects = engine.detect_words(&ocr_input).unwrap();

//     // Phase 2: Perform layout analysis
//     let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

//     // Phase 3: Recognize text
//     let line_texts = engine.recognize_text(&ocr_input, &line_rects).unwrap();

//     for line in line_texts
//         .iter()
//         .flatten()
//         // Filter likely spurious detections. With future model improvements
//         // this should become unnecessary.
//         .filter(|l| l.to_string().len() > 1)
//     {
//         println!("{}", line);
//     }
// }
