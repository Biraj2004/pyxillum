using System;
using System.Collections.Generic;

namespace Pyxillum.Diagnostics
{
    /// <summary>
    /// Canonical source for localized user-facing strings adhering to British / Indian English.
    /// Mirrored from design/strings.toml.
    /// </summary>
    public static class StringCatalog
    {
        private static readonly Dictionary<string, string> Strings = new(StringComparer.OrdinalIgnoreCase)
        {
            // [app]
            ["app.name"] = "Pyxillum",
            ["app.tagline"] = "Native Pixel LED Controller",
            ["app.honest_capability"] = "Implemented, untested on hardware",

            // [common]
            ["common.cancel"] = "Cancel",
            ["common.confirm"] = "Confirm",
            ["common.save"] = "Save",
            ["common.retry"] = "Retry",
            ["common.dismiss"] = "Dismiss",
            ["common.copy"] = "Copy details",
            ["common.power"] = "Power",
            ["common.speed"] = "Speed",
            ["common.brightness"] = "Brightness",
            ["common.scan"] = "Scan Devices",
            ["common.scanning"] = "Scanning...",
            ["common.connect"] = "Connect",
            ["common.disconnect"] = "Disconnect",
            ["common.connected"] = "Connected",
            ["common.disconnected"] = "Disconnected",
            ["common.connecting"] = "Connecting...",

            // [devices]
            ["devices.empty_title"] = "No Controllers Discovered",
            ["devices.empty_desc"] = "Ensure your SP621E controller is powered on, within range, and not connected to companion mobile applications.",
            ["devices.scanning_banner"] = "Scanning for nearby SP621E Bluetooth Low Energy peripherals...",
            ["devices.open_bluetooth_settings"] = "Open Windows Bluetooth Settings",

            // [errors]
            ["error.bluetooth_unavailable"] = "Bluetooth adapter is powered off or unavailable.",
            ["error.scan_failed"] = "Failed to scan for Bluetooth Low Energy controllers.",
            ["error.connection_failed"] = "Unable to establish GATT connection with controller.",
            ["error.device_disconnected"] = "Controller connection lost.",
            ["error.command_failed"] = "Failed to deliver command to controller.",
            ["error.safety_gate_required"] = "Safety confirmation required to alter non-volatile hardware configuration.",
            ["error.preset_not_found"] = "Requested scene preset not found.",
            ["error.persistence_failed"] = "Failed to persist configuration to local storage.",
            ["error.invalid_parameter"] = "Specified parameter value is out of permitted range.",
            ["error.generic"] = "An unexpected operating error occurred."
        };

        public static string Get(string key, string fallback = "")
        {
            if (Strings.TryGetValue(key, out var val))
            {
                return val;
            }
            return string.IsNullOrEmpty(fallback) ? key : fallback;
        }

        public static (string Title, string Message, string ActionLabel) ResolveErrorInfo(string? errorKey, string? rawMessage)
        {
            string key = errorKey ?? string.Empty;
            string message = Strings.TryGetValue(key, out var localizedMsg)
                ? localizedMsg
                : (!string.IsNullOrEmpty(rawMessage) ? rawMessage : Strings["error.generic"]);

            return key switch
            {
                "error.bluetooth_unavailable" => ("Bluetooth Adapter Unavailable", message, "Open Settings"),
                "error.scan_failed" => ("Device Scan Failed", message, "Retry scan"),
                "error.connection_failed" => ("Connection Failed", message, "Retry connect"),
                "error.device_disconnected" => ("Controller Disconnected", message, "Reconnect"),
                "error.safety_gate_required" => ("Safety Confirmation Required", message, "Dismiss"),
                _ => ("Operating Error", message, "Dismiss")
            };
        }
    }
}
