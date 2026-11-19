import SwiftUI
import PyxillumCore

struct DiyView: View {
    @EnvironmentObject var viewModel: AppViewModel

    @State private var stepCount: Int = 6
    @State private var activeStep: Int = 0
    @State private var speed: Double = 16

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                HStack {
                    VStack(alignment: .leading, spacing: 4) {
                        Text("DIY Custom Sequencer")
                            .font(.title2)
                            .fontWeight(.bold)
                        Text("Design multi-step custom keyframe lighting sequences")
                            .font(.subheadline)
                            .foregroundColor(.secondary)
                    }
                    Spacer()
                    Button {
                        viewModel.dispatch(.toggleDiyPreview)
                    } label: {
                        Label("Play Preview", systemImage: "play.fill")
                    }
                    .buttonStyle(.borderedProminent)
                }

                // Timeline
                VStack(alignment: .leading, spacing: 12) {
                    Text("Timeline Sequence (Up to 14 Nodes)")
                        .font(.headline)

                    ScrollView(.horizontal, showsIndicators: false) {
                        HStack(spacing: 8) {
                            ForEach(0..<stepCount, id: \.self) { idx in
                                Button {
                                    activeStep = idx
                                } label: {
                                    VStack {
                                        Text("#\(idx + 1)")
                                            .fontWeight(.bold)
                                        Circle()
                                            .fill(idx == activeStep ? Color.blue : Color.gray)
                                            .frame(width: 24, height: 24)
                                    }
                                    .padding(8)
                                    .background(idx == activeStep ? Color.blue.opacity(0.15) : Color(NSColor.controlBackgroundColor))
                                    .cornerRadius(8)
                                }
                                .buttonStyle(.plain)
                            }

                            if stepCount < 14 {
                                Button {
                                    stepCount += 1
                                } label: {
                                    Image(systemName: "plus.circle.fill")
                                        .font(.title2)
                                        .padding()
                                }
                                .buttonStyle(.plain)
                            }
                        }
                    }
                }
                .padding()
                .background(Color(NSColor.controlBackgroundColor))
                .cornerRadius(12)

                // Sequence Speed
                VStack(alignment: .leading, spacing: 8) {
                    Text("Animation Speed")
                        .font(.headline)
                    Slider(value: $speed, in: 1...32, step: 1) { _ in
                        viewModel.dispatch(.setSpeed(speed: UInt8(speed)))
                    }
                }
                .padding()
                .background(Color(NSColor.controlBackgroundColor))
                .cornerRadius(12)
            }
            .padding(24)
        }
    }
}
