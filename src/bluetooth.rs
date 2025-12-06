use embassy_futures::join::join;
use esp_radio::ble::controller::BleConnector;
use log::info;
use trouble_host::prelude::*;

const CONNECTIONS_MAX: usize = 1;
const L2CAP_CHANNELS_MAX: usize = 3;

pub async fn ble_gatt_task(
    connector: ExternalController<BleConnector<'static>, 20>,
    mac_bytes: [u8; 6],
) {
    info!("========== BLE GATT Server Starting ==========");
    info!(
        "Device Bluetooth Address: {:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
        mac_bytes[0], mac_bytes[1], mac_bytes[2], mac_bytes[3], mac_bytes[4], mac_bytes[5]
    );

    // run_ble(connector, mac_bytes).await;
}

// pub async fn run_ble<C>(controller: C, mac_bytes: [u8; 6])
// where
//     C: trouble_host::Controller,
// {
//     let address = Address::random(mac_bytes);
//     info!("BLE Random Address: {:?}", address);

//     let mut resources: trouble_host::HostResources<
//         _,
//         CONNECTIONS_MAX,
//         L2CAP_CHANNELS_MAX,
//     > = trouble_host::HostResources::new();

//     let stack = trouble_host::new(controller, &mut resources).set_random_address(address);
//     let Host {
//         mut peripheral,
//         runner,
//         ..
//     } = stack.build();

//     // Prepare advertisement data
//     let mut adv_data = [0; 31];
//     let adv_len = AdStructure::encode_slice(
//         &[AdStructure::Flags(
//             LE_GENERAL_DISCOVERABLE | BR_EDR_NOT_SUPPORTED,
//         )],
//         &mut adv_data[..],
//     )
//     .unwrap();

//     let mut scan_data = [0; 31];
//     let scan_len = AdStructure::encode_slice(
//         &[AdStructure::CompleteLocalName(b"knitting machine")],
//         &mut scan_data[..],
//     )
//     .unwrap();

//     info!("Starting BLE advertising...");

//     let _ = join(runner.run(), async {
//         loop {
//             match peripheral
//                 .advertise(
//                     &Default::default(),
//                     Advertisement::ConnectableScannableUndirected {
//                         adv_data: &adv_data[..adv_len],
//                         scan_data: &scan_data[..scan_len],
//                     },
//                 )
//                 .await
//             {
//                 Ok(advertiser) => {
//                     info!("✓ Advertising started!");
//                     match advertiser.accept().await {
//                         Ok(_conn) => {
//                             info!("✓ Device connected!");
//                         }
//                         Err(e) => {
//                             info!("Accept error: {:?}", e);
//                         }
//                     }
//                 }
//                 Err(e) => {
//                     info!("✗ Advertising failed: {:?}", e);
//                 }
//             }
//         }
//     })
//     .await;
// }
