#[test]
fn test_create_update(){
    use smart_holmes::smart_home::*; 
    let service_config = ServiceConfig {amount_services: 20};
    let services = SmartHome::generate_services(&service_config);
}