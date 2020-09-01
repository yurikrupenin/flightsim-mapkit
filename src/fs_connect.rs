use simconnect;

use std::mem::transmute_copy;

const DATA_ID_COORDS_STRUCT: u32 = 0;

pub struct CoordStruct {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
}

pub fn init_simconnect() -> Result<simconnect::SimConnector, &'static str> {
    let mut fs_connect = simconnect::SimConnector::new();

    if !fs_connect.connect("Flightsim Mapkit") {
        return Err("Couldn't connect to Flight Simulator.")
    }

    // Describe data definition (DATA_ID_COORDS_STRUCT)
    // as a packed structure containing latitude, longitude,
    // and altitude of user's plane, each represented as 64-bit float
    fs_connect.add_data_definition(0, "PLANE LATITUDE", "Degrees",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX);

    fs_connect.add_data_definition(0, "PLANE LONGITUDE", "Degreese",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX);

    fs_connect.add_data_definition(0, "PLANE ALTITUDE", "Feet",
        simconnect::SIMCONNECT_DATATYPE_SIMCONNECT_DATATYPE_FLOAT64, u32::MAX);

    fs_connect.request_data_on_sim_object(0, DATA_ID_COORDS_STRUCT, 0,
        simconnect::SIMCONNECT_PERIOD_SIMCONNECT_PERIOD_SIM_FRAME);

    Ok(fs_connect)
}

pub fn update(connector: &simconnect::SimConnector) -> Option<CoordStruct>{
    match connector.get_next_message() {
        Ok(simconnect::DispatchResult::SimobjectData(data)) => {
            unsafe {
                match (*data).dwDefineID {
                    // Now we can retrieve actual state of
                    // our plane's coordinates structure
                    // we described earlier.
                    DATA_ID_COORDS_STRUCT => {
                        Some(transmute_copy(&(*data).dwData))
                    },

                    _ => None
                }
            }
        },
        _ => None
    }
}