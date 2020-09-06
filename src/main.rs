#![windows_subsystem = "windows"]

#[cfg(feature="position-update")]
mod fs_connect;

#[cfg(feature="position-update")]
use std::{
    thread,
    time:: { Duration, SystemTime },
};

use gpx::Gpx;

use log;

use std::{ fs::File, io::BufReader };

use tinyfiledialogs;
use web_view::*;

fn main() {
    env_logger::init();

    let webview = web_view::builder()
    .title("Flightsim Mapkit")
    .content(Content::Html(HTML))
    .size(800, 600)
    .resizable(true)
    .debug(false)
    .user_data(())
    // Setup JS -> Rust calls (GUI buttons basically)
    .invoke_handler(|webview, arg| {
        match arg {
            "load_gpx" => { load_gpx(webview); },
            unimplemented_name => { report_unimplemented_js_call(unimplemented_name) },
        };

        Ok(())
    })
    .build()
    .unwrap();


    // Run SimConnect thread
    #[cfg(feature="position-update")]
    {
        let handle = webview.handle();
        
        thread::spawn(move || {
            let mut last_time = SystemTime::now();

            // TODO: handle failed connections
            let fs_connection = fs_connect::init_simconnect().unwrap();

            // Main loop: consume updated state from game every millisecond...
            loop {
                let time_diff = SystemTime::now().duration_since(last_time).unwrap();

                // ...but only update our position 30 times per second.
                match fs_connect::update(&fs_connection) {
                    Some(coords) => {
                        if time_diff >= Duration::from_millis(33) {
                            handle.dispatch(move |webview| {
                                update_position(webview, coords)
                            }).expect("Unable to update WebView map state!"); 

                            last_time = SystemTime::now();
                        }
                    },
                    None => ()
                }     
                
            thread::sleep(Duration::from_millis(1));
            }

        });
    }

    webview.run().unwrap();
}


#[cfg(feature="position-update")]
fn update_position(webview: &mut WebView<()>, coords: fs_connect::CoordStruct) -> WVResult {
    webview.eval(&format!("updateCoords({}, {}, {})",
        coords.latitude,
        coords.longitude,
        coords.altitude))
}

fn load_gpx(webview: &mut WebView<()>) -> Option<String> {
    let filename = tinyfiledialogs::open_file_dialog("Open GPX file", "", None)?;

    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(file);

    let gpx: Gpx;

    log::debug!("Open GPX file: {}", filename);

    match gpx::read(reader) {
        Ok(contents) => { gpx = contents; },
        Err(_) => {
            log::debug!("File format not recognized!");

            tinyfiledialogs::message_box_ok(
                "Flightsim Mapkit",
                "Cannot open file: format not recognized.",
                tinyfiledialogs::MessageBoxIcon::Error);

                return None
        },
    }

    // TODO Ask user which track he wants to import.
    if gpx.tracks.len() > 1 {
        log::debug!(
            "File contains {} tracks, which is currently unsupported -- using first track only!",
            gpx.tracks.len());
    }

    // Iterate through all the points in track and format them into string.
    let mut points_string = String::new();
    for (seg_no, segment) in gpx.tracks[0].segments.iter().enumerate() {
        log::debug!("Segment {}: {} points.", seg_no, segment.points.len());
        for wp in segment.points.iter() {
            points_string.push_str(&format!("[{}, {}],", wp.point().lng(), wp.point().lat()));
        }
    }

    // Don't need trailing comma
    points_string.pop();

    // Yeah, that should resemble an JSON object.
    let json_call = String::from(&format!("drawRoute(\"[{}]\")", points_string));

    log::debug!("Javascript call string length is {} bytes.", json_call.bytes().len());

    if json_call.bytes().len() > 10000 {
        log::debug!("We might want to optimize that somehow one day...");
    }

    webview.handle().dispatch(move |webview| {
        webview.eval(&json_call)
    }).unwrap();

    Some(filename)
}

fn report_unimplemented_js_call(name: &str) {

    let message: String = format!(
        "Unimplemented call from JS context: \"{}\"!", name);
    
    tinyfiledialogs::message_box_ok(
        "Flightsim Mapkit",
        &message[..],
        tinyfiledialogs::MessageBoxIcon::Error);
}


const HTML: &str = include_str!(concat!(env!("OUT_DIR"), "/index.html"));
