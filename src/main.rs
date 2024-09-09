use shapefile_to_geojson::convert_shapefile_to_geojson;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    convert_shapefile_to_geojson("./data/W05-08_08-g_RiverNode", "output.geojson").await?;
    Ok(())
}
