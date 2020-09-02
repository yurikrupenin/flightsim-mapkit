#![windows_subsystem = "windows"]

#[cfg(feature="position-update")]
mod fs_connect;

#[cfg(feature="position-update")]
use std::{
    thread,
    time:: { Duration, SystemTime },
};

use gpx;
use gpx::{ Gpx, Track, TrackSegment };

use std::{ fs::File, io::BufReader };

use tinyfiledialogs;
use web_view::*;

fn main() {
    let webview = web_view::builder()
    .title("Flightsim Mapkit")
    .content(Content::Html(HTML))
    .size(800, 600)
    .resizable(true)
    .debug(true)
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

                // ...but update our map only once in a second as it could be expensive.
                match fs_connect::update(&fs_connection) {
                    Some(coords) => {
                        if time_diff >= Duration::from_secs(1) {
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

    match gpx::read(reader) {
        Ok(contents) => { gpx = contents; },
        Err(_) => {
            tinyfiledialogs::message_box_ok(
                "Flightsim Mapkit",
                "Cannot open file: format not recognized.",
                tinyfiledialogs::MessageBoxIcon::Error);
                return None
        },
    }

    println!("Number of tracks in a file is {}", gpx.tracks.len());
    println!("Number of segments in the first track is {}", gpx.tracks[0].segments.len());
    println!("And finally, number of points in the first segment is {}",
        gpx.tracks[0].segments[0].points.len());


    webview.handle().dispatch(move |webview| {
        webview.eval("test_something()")
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


const HTML: &str = include_str!("mapwidget.html");

