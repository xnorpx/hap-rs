// this file is auto-generated by hap-codegen

use async_trait::async_trait;
use serde::Serialize;
use serde_json::json;

use crate::{
    characteristic::{
        AsyncCharacteristicCallbacks,
        Characteristic,
        CharacteristicCallbacks,
        Format,
        HapCharacteristic,
        HapCharacteristicSetup,
        HapType,
        OnReadFn,
        OnReadFuture,
        OnUpdateFn,
        OnUpdateFuture,
        Perm,
        Unit,
    },
    pointer,
    Result,
};

// TODO - re-check MaximumDataLength
/// Digital Zoom characteristic.
#[derive(Debug, Default, Serialize)]
pub struct DigitalZoomCharacteristic(Characteristic<f32>);

impl DigitalZoomCharacteristic {
    /// Creates a new Digital Zoom characteristic.
    pub fn new(id: u64, accessory_id: u64) -> Self {
        #[allow(unused_mut)]
        let mut c = Self(Characteristic::<f32> {
            id,
            accessory_id,
            hap_type: HapType::DigitalZoom,
            format: Format::Float,
            perms: vec![
				Perm::Events,
				Perm::PairedRead,
				Perm::PairedWrite,
            ],
			step_value: Some(0.1 as f32),
            ..Default::default()
        });

        if let Some(ref min_value) = &c.0.min_value {
            c.0.value = min_value.clone();
        } else if let Some(ref valid_values) = &c.0.valid_values {
            if valid_values.len() > 0 {
                c.0.value = valid_values[0].clone();
            }
        }

        c
    }
}

#[async_trait]
impl HapCharacteristic for DigitalZoomCharacteristic {
    fn get_id(&self) -> u64 { HapCharacteristic::get_id(&self.0) }

    fn set_id(&mut self, id: u64) { HapCharacteristic::set_id(&mut self.0, id) }

    fn get_type(&self) -> HapType { HapCharacteristic::get_type(&self.0) }

    fn set_type(&mut self, hap_type: HapType) { HapCharacteristic::set_type(&mut self.0, hap_type) }

    fn get_format(&self) -> Format { HapCharacteristic::get_format(&self.0) }

    fn set_format(&mut self, format: Format) { HapCharacteristic::set_format(&mut self.0, format) }

    fn get_perms(&self) -> Vec<Perm> { HapCharacteristic::get_perms(&self.0) }

    fn set_perms(&mut self, perms: Vec<Perm>) { HapCharacteristic::set_perms(&mut self.0, perms) }

    fn get_description(&self) -> Option<String> { HapCharacteristic::get_description(&self.0) }

    fn set_description(&mut self, description: Option<String>) {
        HapCharacteristic::set_description(&mut self.0, description)
    }

    fn get_event_notifications(&self) -> Option<bool> { HapCharacteristic::get_event_notifications(&self.0) }

    fn set_event_notifications(&mut self, event_notifications: Option<bool>) {
        HapCharacteristic::set_event_notifications(&mut self.0, event_notifications)
    }

    async fn get_value(&mut self) -> Result<serde_json::Value> { HapCharacteristic::get_value(&mut self.0).await }

    async fn set_value(&mut self, value: serde_json::Value) -> Result<()> {
        HapCharacteristic::set_value(&mut self.0, value).await
    }

    fn get_unit(&self) -> Option<Unit> { HapCharacteristic::get_unit(&self.0) }

    fn set_unit(&mut self, unit: Option<Unit>) { HapCharacteristic::set_unit(&mut self.0, unit) }

    fn get_max_value(&self) -> Option<serde_json::Value> { HapCharacteristic::get_max_value(&self.0).map(|v| json!(v)) }

    fn set_max_value(&mut self, max_value: Option<serde_json::Value>) -> Result<()> {
        HapCharacteristic::set_max_value(&mut self.0, max_value)
    }

    fn get_min_value(&self) -> Option<serde_json::Value> { HapCharacteristic::get_min_value(&self.0).map(|v| json!(v)) }

    fn set_min_value(&mut self, min_value: Option<serde_json::Value>) -> Result<()> {
        HapCharacteristic::set_min_value(&mut self.0, min_value)
    }

    fn get_step_value(&self) -> Option<serde_json::Value> {
        HapCharacteristic::get_step_value(&self.0).map(|v| json!(v))
    }

    fn set_step_value(&mut self, step_value: Option<serde_json::Value>) -> Result<()> {
        HapCharacteristic::set_step_value(&mut self.0, step_value)
    }

    fn get_max_len(&self) -> Option<u16> { HapCharacteristic::get_max_len(&self.0) }

    fn set_max_len(&mut self, max_len: Option<u16>) { HapCharacteristic::set_max_len(&mut self.0, max_len) }

    fn get_max_data_len(&self) -> Option<u32> { HapCharacteristic::get_max_data_len(&self.0) }

    fn set_max_data_len(&mut self, max_data_len: Option<u32>) {
        HapCharacteristic::set_max_data_len(&mut self.0, max_data_len)
    }

    fn get_valid_values(&self) -> Option<Vec<serde_json::Value>> { HapCharacteristic::get_valid_values(&self.0) }

    fn set_valid_values(&mut self, valid_values: Option<Vec<serde_json::Value>>) -> Result<()> {
        HapCharacteristic::set_valid_values(&mut self.0, valid_values)
    }

    fn get_valid_values_range(&self) -> Option<[serde_json::Value; 2]> {
        HapCharacteristic::get_valid_values_range(&self.0)
    }

    fn set_valid_values_range(&mut self, valid_values_range: Option<[serde_json::Value; 2]>) -> Result<()> {
        HapCharacteristic::set_valid_values_range(&mut self.0, valid_values_range)
    }

    fn get_ttl(&self) -> Option<u64> { HapCharacteristic::get_ttl(&self.0) }

    fn set_ttl(&mut self, ttl: Option<u64>) { HapCharacteristic::set_ttl(&mut self.0, ttl) }

    fn get_pid(&self) -> Option<u64> { HapCharacteristic::get_pid(&self.0) }

    fn set_pid(&mut self, pid: Option<u64>) { HapCharacteristic::set_pid(&mut self.0, pid) }
}

impl HapCharacteristicSetup for DigitalZoomCharacteristic {
    fn set_event_emitter(&mut self, event_emitter: Option<pointer::EventEmitter>) {
        HapCharacteristicSetup::set_event_emitter(&mut self.0, event_emitter)
    }
}

impl CharacteristicCallbacks<f32> for DigitalZoomCharacteristic {
    fn on_read(&mut self, f: Option<impl OnReadFn<f32>>) { CharacteristicCallbacks::on_read(&mut self.0, f) }

    fn on_update(&mut self, f: Option<impl OnUpdateFn<f32>>) { CharacteristicCallbacks::on_update(&mut self.0, f) }
}

impl AsyncCharacteristicCallbacks<f32> for DigitalZoomCharacteristic {
    fn on_read_async(&mut self, f: Option<impl OnReadFuture<f32>>) {
        AsyncCharacteristicCallbacks::on_read_async(&mut self.0, f)
    }

    fn on_update_async(&mut self, f: Option<impl OnUpdateFuture<f32>>) {
        AsyncCharacteristicCallbacks::on_update_async(&mut self.0, f)
    }
}
