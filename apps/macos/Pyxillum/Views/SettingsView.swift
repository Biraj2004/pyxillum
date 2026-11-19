import SwiftUI
import PyxillumCore

struct SettingsView: View {
    @EnvironmentObject var viewModel: AppViewModel

    @State private var safetyConfirmed = false
    @State private var pixelCount: Double = 60
    @State private var chipOrder: ChipOrderFfi = .grb

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Hardware Settings")
                        .font(.title2)
                        .fontWeight(.bold)
                    Text("Configure non-volatile EEPROM registers on physical controllers")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }

                // Safety Caution Banner
                VStack(alignment: .leading, spacing: 10) {
                    HStack {
                        Image(systemName: "exclamationmark.triangle.fill")
                            .foregroundColor(.orange)
                        Text("Caution: Non-Volatile Memory Modification")
                            .font(.headline)
                            .foregroundColor(.orange)
                    }
                    Text("Altering these registers writes directly to controller EEPROM. Incorrect parameters may cause desynchronised signal timing or hardware flickering.")
                        .font(.subheadline)
                        .foregroundColor(.secondary)

                    Toggle("I understand this modifies non-volatile controller registers (--i-understand-this-changes-the-device)", isOn: $safetyConfirmed)
                        .fontWeight(.medium)
                }
                .padding()
                .background(Color.orange.opacity(0.1))
                .cornerRadius(12)

                // Strip Topology Parameters
                VStack(alignment: .leading, spacing: 16) {
                    Text("Pixel Strip Parameters")
                        .font(.headline)

                    VStack(alignment: .leading) {
                        Text("Addressable Pixel Count (\(Int(pixelCount)))")
                            .font(.subheadline)
                        Slider(value: $pixelCount, in: 1...300, step: 1)
                    }

                    VStack(alignment: .leading) {
                        Text("Colour Sequence (Channel Calibration)")
                            .font(.subheadline)
                        Picker("", selection: $chipOrder) {
                            Text("RGB").tag(ChipOrderFfi.rgb)
                            Text("RBG").tag(ChipOrderFfi.rbg)
                            Text("GRB (WS2812B Default)").tag(ChipOrderFfi.grb)
                            Text("GBR").tag(ChipOrderFfi.gbr)
                            Text("BRG").tag(ChipOrderFfi.brg)
                            Text("BGR").tag(ChipOrderFfi.bgr)
                        }
                    }

                    Button("Apply Hardware Changes") {
                        if safetyConfirmed {
                            viewModel.dispatch(.configureStrip(
                                pixelCount: UInt16(pixelCount),
                                chipOrder: chipOrder,
                                safetyConfirmed: true
                            ))
                        }
                    }
                    .buttonStyle(.borderedProminent)
                    .disabled(!safetyConfirmed)
                }
                .padding()
                .background(Color(NSColor.controlBackgroundColor))
                .cornerRadius(12)
            }
            .padding(24)
        }
    }
}
