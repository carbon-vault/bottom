use nvml_wrapper::enum_wrappers::device::TemperatureSensor;

use super::{
    convert_celsius_to_fahrenheit, convert_celsius_to_kelvin, is_temp_filtered, TempHarvest,
    TemperatureType,
};
use crate::app::Filter;
use crate::data_harvester::nvidia::NVML_DATA;
use crate::utils::error;

pub fn add_nvidia_data(
    temperature_vec: &mut Vec<TempHarvest>, temp_type: &TemperatureType, filter: &Option<Filter>,
) -> error::Result<()> {
    if let Ok(nvml) = &*NVML_DATA {
        if let Ok(gpu_num) = nvml.device_count() {
            for i in 0..gpu_num {
                if let Ok(device) = nvml.device_by_index(i) {
                    if let (Ok(name), Ok(temperature)) =
                        (device.name(), device.temperature(TemperatureSensor::Gpu))
                    {
                        if is_temp_filtered(filter, &name) {
                            let temperature = temperature as f32;
                            let temperature = match temp_type {
                                TemperatureType::Celsius => temperature,
                                TemperatureType::Kelvin => convert_celsius_to_kelvin(temperature),
                                TemperatureType::Fahrenheit => {
                                    convert_celsius_to_fahrenheit(temperature)
                                }
                            };

                            temperature_vec.push(TempHarvest { name, temperature });
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
