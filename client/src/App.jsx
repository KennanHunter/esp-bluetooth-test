import { useState } from "react";
import "./App.css";

function App() {
  const [device, setDevice] = useState(null);
  const [error, setError] = useState("");
  const [status, setStatus] = useState("");

  const requestDevice = async () => {
    try {
      setError("");
      setStatus('Scanning for "knitting machine"...');

      const device = await navigator.bluetooth.requestDevice({
        // filters: [
        //   { namePrefix: 'knitting' }
        // ],
      });

      setDevice(device);
      setStatus(`Found: ${device.name || "Unknown Device"}`);
    } catch (err) {
      setError(err.message);
      setStatus("");
    }
  };

  const connectToDevice = async () => {
    if (!device) return;

    try {
      setStatus("Connecting to GATT server...");
      const server = await device.gatt.connect();

      // Get the custom service
      const service = await server.getPrimaryService(
        "12345678-1234-5678-1234-567812345678"
      );

      // Get characteristics
      const nameChar = await service.getCharacteristic(
        "12345678-1234-5678-1234-567812345679"
      );
      const versionChar = await service.getCharacteristic(
        "12345678-1234-5678-1234-56781234567a"
      );
      const statusChar = await service.getCharacteristic(
        "12345678-1234-5678-1234-56781234567b"
      );

      // Read device_name
      const nameValue = await nameChar.readValue();
      const nameText = new TextDecoder().decode(nameValue);

      // Read device_version
      const versionValue = await versionChar.readValue();
      const versionText = new TextDecoder().decode(versionValue);

      // Read device_status
      const statusValue = await statusChar.readValue();
      const statusByte = statusValue.getUint8(0);

      setStatus(
        `Connected! Name: ${nameText}, Version: ${versionText}, Status: ${statusByte}`
      );
      console.log("Device info:", {
        name: nameText,
        version: versionText,
        status: statusByte,
      });
    } catch (err) {
      setError(err.message);
      setStatus("");
    }
  };

  return (
    <div className="app">
      <h1>Bluetooth Device Scanner</h1>
      <button onClick={requestDevice} className="scan-btn">
        Scan for Devices
      </button>

      {device && (
        <div className="device">
          <h3>Device Found</h3>
          <p>
            <strong>Name:</strong> {device.name || "Unknown"}
          </p>
          <p>
            <strong>ID:</strong> {device.id}
          </p>
          <button onClick={connectToDevice}>Connect to GATT</button>
        </div>
      )}

      {status && <p className="status">{status}</p>}
      {error && <p className="error">Error: {error}</p>}
    </div>
  );
}

export default App;
