extern crate serde;
use super::dependency::Dependency;
use super::device::Device;
use super::service::Service;
use super::update::Update;
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;

pub struct ServiceConfig {
    pub amount_services: usize,
}

pub struct DeviceConfig {
    pub amount_devices: usize,
    pub services_per_device: usize,
}

pub struct DependencyConfig {
    pub amount_dependencies: usize,
    pub device_per_dependency: usize,
    pub service_per_dependency: usize,
}

pub struct UpdateConfig {
    pub amount_updates: usize,
}

pub struct SmartHomeConfig {
    service_config: ServiceConfig,
    device_config: DeviceConfig,
    dependency_config: DependencyConfig,
    update_config: UpdateConfig,
}

impl SmartHomeConfig {
    pub fn new(
        service_config: ServiceConfig,
        device_config: DeviceConfig,
        dependency_config: DependencyConfig,
        update_config: UpdateConfig,
    ) -> SmartHomeConfig {
        SmartHomeConfig {
            service_config,
            device_config,
            dependency_config,
            update_config,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SmartHome {
    pub services: Vec<Service>,
    pub devices: Vec<Device>,
    pub dependencies: Vec<Dependency>,
}

impl SmartHome {
    pub fn generate_services(service_config: &ServiceConfig) -> Vec<Service> {
        let mut services = Vec::new();
        for i in 0..service_config.amount_services {
            services.push(Service::new(i));
        }
        services
    }

    pub fn save(&self, filename: String) {
        let writer = BufWriter::new(File::create(filename).unwrap());
        serde_json::to_writer_pretty(writer, &self).unwrap();
    }

    pub fn load(filename: String) -> SmartHome {
        let f = File::open(filename).unwrap();
        let reader = BufReader::new(f);
        let smart_home: SmartHome = serde_json::from_reader(reader).unwrap();
        smart_home
    }

    fn generate_updates(
        update_config: &UpdateConfig,
        services_on_device: &[Service],
        services: &[Service],
    ) -> Vec<Update> {
        let mut updates: Vec<Update> = Vec::new();
        let update = Update::map_to_update(&services_on_device);
        updates.push(update);
        for i in 0..update_config.amount_updates {
            updates.push(Update::generate_new_update(&updates[i], &services));
        }
        updates
    }

    pub fn generate_device(
        id: usize,
        device_config: &DeviceConfig,
        update_config: &UpdateConfig,
        services: &[Service],
    ) -> Device {
        let services_on_device: Vec<_> = services
            .choose_multiple(&mut rand::thread_rng(), device_config.services_per_device)
            .cloned()
            .collect();
        let updates = SmartHome::generate_updates(update_config, &services_on_device, services);
        Device::new(services_on_device, updates, id)
    }

    fn generate_devices(
        device_config: &DeviceConfig,
        update_config: &UpdateConfig,
        services: &[Service],
    ) -> Vec<Device> {
        let mut devices = Vec::new();
        for id in 0..device_config.amount_devices {
            let device = SmartHome::generate_device(id, &device_config, &update_config, services);
            devices.push(device);
        }
        devices
    }

    fn generate_dependencies(
        dependency_config: &DependencyConfig,
        devices: &[Device],
    ) -> Vec<Dependency> {
        let mut dependencies = Vec::new();
        for _ in 0..dependency_config.amount_dependencies {
            let dependency_devices: Vec<_> = devices
                .choose_multiple(
                    &mut rand::thread_rng(),
                    dependency_config.device_per_dependency,
                )
                .cloned()
                .collect();

            let mut service_ids_of_devices = HashSet::new();
            for device in &dependency_devices {
                for service in &device.services {
                    service_ids_of_devices.insert(service.id);
                }
            }
            let mut services_of_devices = Vec::new();
            for i in service_ids_of_devices {
                services_of_devices.push(Service::new(i));
            }
            let dependency_services: Vec<_> = services_of_devices
                .choose_multiple(
                    &mut rand::thread_rng(),
                    dependency_config.service_per_dependency,
                )
                .cloned()
                .collect();
            let dependency =
                Dependency::new(dependency_devices, dependency_services, dependencies.len());
            if dependency.is_fullfilled() {
                dependencies.push(dependency);
            } else {
                println!("NOT FULFILLED");
            }
        }
        dependencies
    }

    pub fn new(config: SmartHomeConfig) -> SmartHome {
        println!(
            "Generating {} services",
            config.service_config.amount_services
        );
        let services = SmartHome::generate_services(&config.service_config);
        println!("Generating {} devices", config.device_config.amount_devices);
        let devices =
            SmartHome::generate_devices(&config.device_config, &config.update_config, &services);
        println!(
            "Generating {} dependencies",
            config.dependency_config.amount_dependencies
        );
        let dependencies = SmartHome::generate_dependencies(&config.dependency_config, &devices);
        SmartHome {
            services,
            devices,
            dependencies,
        }
    }
}
