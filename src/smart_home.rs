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
    pub fn new(config: SmartHomeConfig) -> SmartHome{
        let mut services = Vec::new();
        let mut devices = Vec::new();
        let mut dependencies = Vec::new();
        for i in 0..config.service_config.amount_services{
            services.push(Service::new(i));
        }
    
        for _ in 0..config.device_config.amount_devices{
            let sample: Vec<_> = services
                .choose_multiple(&mut rand::thread_rng(), config.device_config.services_per_device)
                .cloned()
                .collect();
            let mut updates: Vec<Update> = Vec::new();
            let update = Update::map_to_update(&sample);
            updates.push(update);
            for i in 0..config.update_config.amount_updates{
                updates.push(Update::generate_new_update(&updates[i], &services));
            }
            let device = Device::new(sample, updates);
            devices.push(device);
            
        }
    
        while dependencies.len() < config.dependency_config.amount_dependencies{
            let dependency_devices: Vec<_> = devices
                .choose_multiple(&mut rand::thread_rng(), config.dependency_config.device_per_dependency)
                .cloned()
                .collect();
            let dependency_services: Vec<_> = services
                .choose_multiple(&mut rand::thread_rng(), config.dependency_config.service_per_dependency)
                .cloned()
                .collect();
            let dependency = Dependency::new(dependency_devices, dependency_services);
            if dependency.is_fullfilled(){
                dependencies.push(dependency);
            }
        }
        SmartHome{services, devices, dependencies}
    }
}