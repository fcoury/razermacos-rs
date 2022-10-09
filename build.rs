use serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
enum RazerDeviceType {
    #[serde(rename = "mouse")]
    Mouse,
    #[serde(rename = "mousedock")]
    MouseDock,
    #[serde(rename = "mousemat")]
    MouseMat,
    #[serde(rename = "keyboard")]
    Keyboard,
    #[serde(rename = "accessory")]
    Accessory,
    #[serde(rename = "headphone")]
    Headphone,
    #[serde(rename = "egpu")]
    EGpu,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct RazerDevice {
    name: String,
    #[serde(deserialize_with = "from_hex")]
    product_id: u16,
    main_type: RazerDeviceType,
    image: String,
    features: Option<Vec<String>>,
    features_missing: Option<Vec<String>>,
    features_config: Option<serde_json::Value>,
}

fn from_hex<'de, D>(deserializer: D) -> Result<u16, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    // do better hex decoding than this
    u16::from_str_radix(&s[2..], 16).map_err(D::Error::custom)
}

fn main() {
    let razer_path = PathBuf::from(env::var("OUT_DIR").unwrap()).join("razer_devices.rs");
    let files = fs::read_dir("src/devices").unwrap();

    let mut contents = format!(
        r#"
    use serde::{{Deserialize, Serialize}};
    use lazy_static::lazy_static;

    #[derive(Serialize, Deserialize, Clone, Copy, Debug)]
    pub enum RazerDeviceType {{
        #[serde(rename = "mouse")]
        Mouse,
        #[serde(rename = "mousedock")]
        MouseDock,
        #[serde(rename = "mousemat")]
        MouseMat,
        #[serde(rename = "keyboard")]
        Keyboard,
        #[serde(rename = "accessory")]
        Accessory,
        #[serde(rename = "headphone")]
        Headphone,
        #[serde(rename = "egpu")]
        EGpu,
    }}

    pub struct RazerDeviceDescriptor {{
        pub name: String,
        pub product_id: u16,
        pub main_type: RazerDeviceType,
        pub image: String,
        pub features: Option<Vec<String>>,
        pub features_missing: Option<Vec<String>>,
        pub features_config: Option<serde_json::Value>,
    }}

    lazy_static! {{
        pub static ref RAZER_DEVICES: [RazerDeviceDescriptor; {}] = [
    "#,
        files.count(),
    );

    let files = fs::read_dir("src/devices").unwrap();
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        let data = fs::read_to_string(path).unwrap();
        let res = serde_json::from_str::<RazerDevice>(&data).unwrap();

        let features = match res.features {
            Some(features) => format!(
                "Some(vec![{}])",
                features
                    .iter()
                    .map(|feature| format!(r#""{}".to_string()"#, feature))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            None => "None".to_string(),
        };

        let features_missing = match res.features_missing {
            Some(features_missing) => format!(
                "Some(vec![{}])",
                features_missing
                    .iter()
                    .map(|feature| format!(r#""{}".to_string()"#, feature))
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
            None => "None".to_string(),
        };

        let features_config = match res.features_config {
            Some(features_config) => format!("Some(serde_json::json!({}))", features_config),
            None => "None".to_string(),
        };

        contents.push_str(&format!(
            r#"
            RazerDeviceDescriptor {{
                name: "{}".to_string(),
                product_id: {},
                main_type: RazerDeviceType::Mouse,
                image: "{}".to_string(),
                features: {},
                features_missing: {},
                features_config: {},
            }},
        "#,
            res.name, res.product_id, res.image, features, features_missing, features_config,
        ));
    }
    contents.push_str(
        r#"
        ];
    }

    // impl RazerDeviceDescriptor {
    //     pub fn find(product_id: u16) -> Option<Self> {
    //         let devices = RAZER_DEVICES.clone();
    //         if let Some(device) = devices
    //             .iter()
    //             .find(|device| device.product_id == product_id)
    //         {
    //             return Some(device.clone());
    //         }

    //         None
    //     }
    // }

    pub fn find_descriptor<'a>(product_id: u16) -> Option<&'a RazerDeviceDescriptor> {
        if let Some(device) = RAZER_DEVICES
            .iter()
            .find(|device| device.product_id == product_id)
        {
            return Some(device.clone());
        }

        None
    }
    "#,
    );

    // eprintln!("razer_path = {:?}", &razer_path);
    fs::write(razer_path, contents).unwrap();
}
