#[post("/<id>/publish")]
pub fn publish(id: usize) -> Result<Json<Product>> {
    return match ProductService::publish(id) {
        Ok(f) => Ok(Json::from(f)),
        Err(e) => Err(e),
    };
}
