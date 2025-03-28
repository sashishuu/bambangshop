pub fn publish(id: usize) -> Result<Product> {
    let product_opt: Option<Product> = ProductRepository::get_by_id(id);
    if product_opt.is_none() {
        return Err(compose_error_response(
            Status::NotFound,
            String::from("Product not found.")
        ));
    }

    let product: Product = product_opt.unwrap();
    NotificationService::notify(&product.product_type, "PROMOTION", product.clone());
    return Ok(product);
}
