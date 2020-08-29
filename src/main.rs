use simconnect;

use std::{
    mem::transmute_copy,
    thread,
    time::{ Duration, SystemTime },
};

use web_view::*;

struct CoordStruct {
    latitude: f64,
    longitude: f64,
    altitude: f64,
}

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

    let handle = webview.handle();

    thread::spawn(move || {
        let mut last_time = SystemTime::now();
        let mut fs_connect = simconnect::SimConnector::new();

        fs_connect.connect("Flightsim Mapkit");

        fs_connect.add_data_definition(0, "PLANE LATITUDE", "Degrees",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX);

        fs_connect.add_data_definition(0, "PLANE LONGITUDE", "Degrees",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX);

        fs_connect.add_data_definition(0, "PLANE ALTITUDE", "Feet",
            simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX);
    
        fs_connect.request_data_on_sim_object(0, 0, 0,
            simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME);

        loop {
            match fs_connect.get_next_message() {
                Ok(simconnect::DispatchResult::SimobjectData(data)) => {
                    unsafe {
                        match (*data).dwDefineID {
                            0 => {
                                let sim_data: CoordStruct = transmute_copy(&(*data).dwData);

                                let time_diff = SystemTime::now().duration_since(last_time).unwrap();

                                if time_diff >= Duration::from_secs(1) {
                                    handle.dispatch(move |webview| {
                                        render(webview, sim_data)
                                    }).unwrap();

                                    last_time = SystemTime::now();
                                }

                            },
                            _ => ()
                        }
                    }
                },
                _ => ()
            }

        thread::sleep(Duration::from_millis(1));
        }

    });

    webview.run().unwrap();
}

fn render(webview: &mut WebView<()>, coords: CoordStruct) -> web_view::WVResult {
    webview.eval(&format!("updateCoords({}, {}, {})",
                coords.latitude,
                coords.longitude,
                coords.altitude))
}


const HTML: &str = include_str!("mapwidget.html");

