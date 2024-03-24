pub struct TecRoute {
pub struct PreferentialRoute {
    origin_id: String,
    origin_city: String,
    origin_state_code: String,
    origin_country_code: String,
    destination_id: String,
    destination_city: String,
    destination_state_code: String,
    destination_country_code: String,
    pfr_type_code: String,
    route_number: String,
    special_area_description: String,
    altitude_description: String,
    aircraft: String,
    hours: String,
    route_dir_description: String,
    designator: String,
    nar_type: String,
    inland_fac_fix: String,
    coastal_fix: String,
    destination: String,
    route_string: String
}