import SwiftUI
import PyxillumCore

struct ContentView: View {
    @EnvironmentObject var viewModel: AppViewModel

    var body: some View {
        NavigationSplitView {
            List(selection: $viewModel.selectedScreen) {
                NavigationLink(value: ScreenId.deviceHub) {
                    Label("Device Hub", systemImage: "point.3.filled.connected.trianglepath.dotted")
                }
                NavigationLink(value: ScreenId.dynamic) {
                    Label("Dynamic FX", systemImage: "sparkles")
                }
                NavigationLink(value: ScreenId.solid) {
                    Label("Solid Colour", systemImage: "paintpalette")
                }
                NavigationLink(value: ScreenId.music) {
                    Label("Music Rhythm", systemImage: "waveform")
                }
                NavigationLink(value: ScreenId.diy) {
                    Label("DIY Sequencer", systemImage: "slider.horizontal.3")
                }
                NavigationLink(value: ScreenId.hardwareSettings) {
                    Label("Hardware Settings", systemImage: "gearshape")
                }
            }
            .navigationTitle("Pyxillum")
            .listStyle(.sidebar)
        } detail: {
            VStack(spacing: 0) {
                // Top Global Connection & Status Header
                HStack {
                    Circle()
                        .fill(connectionColor)
                        .frame(width: 8, height: 8)
                    Text(connectionText)
                        .font(.subheadline)
                        .fontWeight(.medium)

                    if viewModel.state.isPreviewMode {
                        Text("Preview Mode")
                            .font(.caption)
                            .fontWeight(.semibold)
                            .padding(.horizontal, 6)
                            .padding(.vertical, 2)
                            .background(Color.yellow.opacity(0.2))
                            .foregroundColor(.yellow)
                            .cornerRadius(4)
                    }

                    Spacer()

                    // Master Brightness Slider
                    Image(systemName: "sun.max")
                        .foregroundColor(.secondary)
                    Slider(
                        value: Binding(
                            get: { Double(viewModel.state.dynamicPage.brightness) },
                            set: { viewModel.dispatch(.setBrightness(brightness: UInt8($0))) }
                        ),
                        in: 0...255
                    )
                    .frame(width: 120)

                    // Master Power Button
                    Button {
                        viewModel.dispatch(.togglePower)
                    } label: {
                        Image(systemName: "power")
                            .font(.system(size: 14, weight: .bold))
                    }
                    .buttonStyle(.borderedProminent)
                }
                .padding(.horizontal, 16)
                .padding(.vertical, 10)
                .background(Color(NSColor.controlBackgroundColor))

                Divider()

                // Main Content View
                Group {
                    switch viewModel.selectedScreen {
                    case .deviceHub:
                        DevicesView()
                    case .dynamic:
                        DynamicView()
                    case .solid:
                        SolidView()
                    case .music:
                        MusicView()
                    case .diy:
                        DiyView()
                    case .hardwareSettings:
                        SettingsView()
                    }
                }
                .frame(maxWidth: .infinity, maxHeight: .infinity)

                Divider()

                // Bottom 60 FPS LED Strip Monitor Canvas
                LedStripMonitorView()
            }
        }
    }

    private var connectionColor: Color {
        switch viewModel.state.connection.kind {
        case .connected: return .green
        case .connecting, .scanning: return .blue
        default: return .orange
        }
    }

    private var connectionText: String {
        switch viewModel.state.connection.kind {
        case .connected: return viewModel.state.connection.name ?? "Connected"
        case .connecting: return "Connecting..."
        case .scanning: return "Scanning..."
        default: return "Disconnected"
        }
    }
}
