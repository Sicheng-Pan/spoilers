use xcap::Window;

// #[test]
fn capture() {
    let start = std::time::Instant::now();
    let windows = Window::all().unwrap();

    let mut i = 0;

    for window in windows {
        // 最小化的窗口不能截屏
        if window.is_minimized() {
            continue;
        }

        println!(
            "Window: {:?} {:?} {:?}",
            window.title(),
            (window.x(), window.y(), window.width(), window.height()),
            (window.is_minimized(), window.is_maximized())
        );

        let image = window.capture_image().unwrap();
        image
            .save(format!("target/window-{}-{}.png", i, window.title()))
            .unwrap();

        i += 1;
    }

    println!("运行耗时: {:?}", start.elapsed());
}

use image::imageops::{brighten, contrast};
use ocrs::{OcrEngine, OcrEngineParams};
use rten::Model;
use rten_tensor::NdTensorView;
use std::fs;

// #[test]
fn ocr() {
    let mut img = image::open("target/wip/sg.jpg").unwrap().into_rgb8();
    img = brighten(&img, -16);
    img = contrast(&img, 64.0);

    let (w, h) = img.dimensions();
    let layout = img.sample_layout();
    let chw_tensor = NdTensorView::from_slice(
        img.as_raw().as_slice(),
        [h as usize, w as usize, 3],
        Some([
            layout.height_stride,
            layout.width_stride,
            layout.channel_stride,
        ]),
    )
    .unwrap()
    .permuted([2, 0, 1]) // HWC => CHW
    .to_tensor() // Make tensor contiguous, which makes `map` faster
    .map(|x| *x as f32 / 255.); // Rescale from [0, 255] to [0, 1]

    let detection_model_data = fs::read("target/wip/dt.rten").unwrap();
    let recognition_model_data = fs::read("target/wip/rc.rten").unwrap();
    let detection_model = Model::load(&detection_model_data).unwrap();
    let recognition_model = Model::load(&recognition_model_data).unwrap();

    let engine = OcrEngine::new(OcrEngineParams {
        detection_model: Some(detection_model),
        recognition_model: Some(recognition_model),
        ..Default::default()
    })
    .unwrap();

    let ocr_input = engine.prepare_input(chw_tensor.view()).unwrap();

    // Phase 1: Detect text words
    let word_rects = engine.detect_words(&ocr_input).unwrap();

    // Phase 2: Perform layout analysis
    let line_rects = engine.find_text_lines(&ocr_input, &word_rects);

    // Phase 3: Recognize text
    let line_texts = engine.recognize_text(&ocr_input, &line_rects).unwrap();

    for line in line_texts
        .iter()
        .flatten()
        // Filter likely spurious detections. With future model improvements
        // this should become unnecessary.
        .filter(|l| l.to_string().len() > 1)
    {
        println!("{}", line);
    }
}

use pyo3::{prelude::*, types::IntoPyDict};

#[test]
fn translate() {
    Python::with_gil(|py| {
        let timer = std::time::Instant::now();
        let pkg = py.import("argostranslate.package").unwrap();
        let tsl = py.import("argostranslate.translate").unwrap();
        pkg.getattr("update_package_index")
            .unwrap()
            .call0()
            .unwrap();
        let available_pkgs = pkg
            .getattr("get_available_packages")
            .unwrap()
            .call0()
            .unwrap();
        let py_pkgs_var = [("pkgs", available_pkgs)].into_py_dict(py);
        let target_pkg = py
            .eval(
                "next(filter(lambda p: p.from_code == 'en' and p.to_code == 'zh', pkgs))",
                None,
                Some(py_pkgs_var),
            )
            .unwrap();
        pkg.getattr("install_from_path")
            .unwrap()
            .call1((target_pkg.getattr("download").unwrap().call0().unwrap(),))
            .unwrap();
        println!("{}: Initialized", timer.elapsed().as_millis());
        let mut result = tsl
            .getattr("translate")
            .unwrap()
            .call1(("Hello!", "en", "zh"))
            .unwrap()
            .extract::<String>()
            .unwrap();
        println!("{}: {}", timer.elapsed().as_millis(), result);
        result = tsl
            .getattr("translate")
            .unwrap()
            .call1(("I am writing code in Rust!", "en", "zh"))
            .unwrap()
            .extract::<String>()
            .unwrap();
        println!("{}: {}", timer.elapsed().as_millis(), result);
        result = tsl
            .getattr("translate")
            .unwrap()
            .call1(("Goodbye!", "en", "zh"))
            .unwrap()
            .extract::<String>()
            .unwrap();
        println!("{}: {}", timer.elapsed().as_millis(), result);
    });
}
