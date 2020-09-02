#![windows_subsystem = "windows"]

#[cfg(feature="position-update")]
mod fs_connect;

#[cfg(feature="position-update")]
use std::{
    thread,
    time:: { Duration, SystemTime },
};

use web_view::*;

fn main() {
    let webview = web_view::builder()
    .title("Flightsim Mapkit")
    .content(Content::Html(HTML))
    .size(800, 600)
    .resizable(true)
    .debug(true)
    .user_data(())
    .invoke_handler(|_webview, _arg| Ok(()))
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

                            println!("Updated map view!");
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


const HTML: &str = include_str!("mapwidget.html");

