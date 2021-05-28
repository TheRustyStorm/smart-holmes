use super::service::Service;
use super::device::Device;
use super::dependency::Dependency;
use super::update::Update;
use rand::seq::SliceRandom;

pub struct ServiceConfig{
    pub amount_services: usize,
}

pub struct DeviceConfig{
    pub amount_devices: usize,
    pub services_per_device: usize,
}

pub struct DependencyConfig{
    pub amount_dependencies: usize,
    pub device_per_dependency: usize,
    pub service_per_dependency: usize,
}

pub struct UpdateConfig{
    pub amount_updates:usize,
}

pub struct SmartHomeConfig{
    service_config: ServiceConfig,
    device_config: DeviceConfig,
    dependency_config: DependencyConfig,
    update_config: UpdateConfig,
}

impl SmartHomeConfig{
    pub fn new(service_config: ServiceConfig, device_config: DeviceConfig, dependency_config: DependencyConfig, update_config: UpdateConfig) -> SmartHomeConfig{
        SmartHomeConfig{service_config, device_config, dependency_config, update_config}
    }
}

#[derive(Debug)]
pub struct SmartHome{
    pub services: Vec<Service>,
    pub devices: Vec<Device>,
    pub dependencies: Vec<Dependency>
}

impl SmartHome{

    fn generate_services(service_config: &ServiceConfig) -> Vec<Service>{
        let mut services = Vec::new();
        for i in 0..service_config.amount_services{
            services.push(Service::new(i));
        }
        services
    }

    fn generate_updates(update_config: &UpdateConfig, services_on_device: &[Service], services: &[Service]) -> Vec<Update>{
        let mut updates: Vec<Update> = Vec::new();
        let update = Update::map_to_update(&services_on_device);
        updates.push(update);
        for i in 0..update_config.amount_updates{
            updates.push(Update::generate_new_update(&updates[i], &services));
        }
        updates
    }

    fn generate_device(device_config: &DeviceConfig, update_config: &UpdateConfig, services: &[Service]) -> Device{
        let services_on_device: Vec<_> = services
            .choose_multiple(&mut rand::thread_rng(), device_config.services_per_device)
            .cloned()
            .collect();
        let updates = SmartHome::generate_updates(update_config, &services_on_device, services);
        let device = Device::new(services_on_device, updates);
        device
    }

    fn generate_devices(device_config: &DeviceConfig, update_config: &UpdateConfig, services: &[Service]) -> Vec<Device>{
        let mut devices = Vec::new();
        for _ in 0..device_config.amount_devices{
            let device = SmartHome::generate_device(&device_config, &update_config, services);
            devices.push(device);   
        }
        devices
    }

    fn generate_dependencies(dependency_config: &DependencyConfig, devices: &[Device], services: &[Service]) -> Vec<Dependency>{
        let mut dependencies = Vec::new();
        while dependencies.len() < dependency_config.amount_dependencies{
            let dependency_devices: Vec<_> = devices
                .choose_multiple(&mut rand::thread_rng(), dependency_config.device_per_dependency)
                .cloned()
                .collect();
            let dependency_services: Vec<_> = services
                .choose_multiple(&mut rand::thread_rng(), dependency_config.service_per_dependency)
                .cloned()
                .collect();
            let dependency = Dependency::new(dependency_devices, dependency_services);
            if dependency.is_fullfilled(){
                dependencies.push(dependency);
            }
        }
        dependencies
    }

    pub fn new(config: SmartHomeConfig) -> SmartHome{
        let services = SmartHome::generate_services(&config.service_config);
        let devices = SmartHome::generate_devices(&config.device_config, &config.update_config, &services);
        let dependencies = SmartHome::generate_dependencies(&config.dependency_config, &devices, &services);
        SmartHome{services, devices, dependencies}
    }
}