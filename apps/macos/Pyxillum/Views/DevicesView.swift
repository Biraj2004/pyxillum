import SwiftUI
import PyxillumCore

struct DevicesView: View {
    @EnvironmentObject var viewModel: AppViewModel

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("Device Hub")
                            .font(.title2)
                            .fontWeight(.bold)
                        Text("Discover and manage nearby SP621E Bluetooth controllers")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                    }
                    Spacer()
                    Button {
                        viewModel.dispatch(.scanDevices)
                    } label: {
                        Label("Scan Devices", systemImage: "arrow.clockwise")
                    }
                    .buttonStyle(.borderedProminent)
                }

                if viewModel.state.discoveredDevices.isEmpty {
                    VStack(spacing: 12) {
                        Image(systemName: "point.3.filled.connected.trianglepath.dotted")
                            .font(.system(size: 48))
                            .foregroundColor(.secondary)
                        Text("No Controllers Discovered")
                            .font(.headline)
                        Text("Ensure your SP621E controller is powered on, within range, and not connected to mobile companion applications.")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                            .multilineTextAlignment(.center)
                            .frame(maxWidth: 400)
                    }
                    .frame(maxWidth: .infinity, minHeight: 250)
                    .background(Color(NSColor.controlBackgroundColor))
                    .cornerRadius(12)
                } else {
                    LazyVStack(spacing: 8) {
                        ForEach(viewModel.state.discoveredDevices, id: \.address) { device in
                            HStack {
                                Image(systemName: "lightstrip.fill")
                                    .font(.title2)
                                    .padding(.trailing, 8)
                                VStack(alignment: .leading) {
                                    Text(device.name)
                                        .fontWeight(.semibold)
                                    Text(device.address)
                                        .font(.caption)
                                        .foregroundColor(.secondary)
                                }
                                Spacer()
                                Button("Connect") {
                                    viewModel.dispatch(.connectDevice(address: device.address))
                                }
                                .buttonStyle(.borderedProminent)
                            }
                            .padding()
                            .background(Color(NSColor.controlBackgroundColor))
                            .cornerRadius(8)
                        }
                    }
                }
            }
            .padding(24)
        }
    }
}
