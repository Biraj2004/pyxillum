import SwiftUI
import PyxillumCore

struct MusicView: View {
    @EnvironmentObject var viewModel: AppViewModel

    @State private var selectedSource = 0
    @State private var sensitivity: Double = 60

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Music Rhythm")
                        .font(.title2)
                        .fontWeight(.bold)
                    Text("Acoustic rhythm tracking and spectrum reactive lighting")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }

                HStack(spacing: 24) {
                    VStack(alignment: .leading, spacing: 6) {
                        Text("Audio Input Source").font(.caption).foregroundColor(.secondary)
                        Picker("", selection: $selectedSource) {
                            Text("System Audio").tag(0)
                            Text("Controller Internal Mic").tag(1)
                            Text("Host Auxiliary Mic").tag(2)
                        }
                        .labelsHidden()
                        .onChange(of: selectedSource) { val in
                            let src: AudioInputFfi = (val == 1) ? .internalMic : (val == 2 ? .externalMic : .player)
                            viewModel.dispatch(.selectAudioInput(source: src))
                        }
                    }

                    VStack(alignment: .leading, spacing: 6) {
                        Text("Sensor Sensitivity").font(.caption).foregroundColor(.secondary)
                        Slider(value: $sensitivity, in: 1...100, step: 1) { _ in
                            viewModel.dispatch(.setSensitivity(sensitivity: UInt8(sensitivity)))
                        }
                    }
                }
                .padding()
                .background(Color(NSColor.controlBackgroundColor))
                .cornerRadius(12)

                Text("Acoustic Modes (18 Patterns)")
                    .font(.headline)

                LazyVGrid(columns: [GridItem(.adaptive(minimum: 140))], spacing: 12) {
                    ForEach(1...18, id: \.self) { mode in
                        Button {
                            viewModel.dispatch(.selectMusicMode(code: UInt8(mode)))
                        } label: {
                            VStack {
                                Text("Acoustic \(mode)").fontWeight(.semibold)
                                Text("Pattern #\(mode)").font(.caption).foregroundColor(.secondary)
                            }
                            .frame(maxWidth: .infinity)
                            .padding()
                            .background(Color(NSColor.controlBackgroundColor))
                            .cornerRadius(8)
                        }
                        .buttonStyle(.plain)
                    }
                }
            }
            .padding(24)
        }
    }
}
