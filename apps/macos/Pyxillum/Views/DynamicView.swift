import SwiftUI
import PyxillumCore

struct DynamicView: View {
    @EnvironmentObject var viewModel: AppViewModel

    private let sampleEffects: [(UInt8, String, String)] = [
        (1, "Rainbow Flow", "Rainbow"),
        (2, "Prismatic Cycle", "Rainbow"),
        (3, "Blazing Embers", "Fire"),
        (4, "Campfire Flicker", "Fire"),
        (5, "Ocean Wave", "Water"),
        (6, "Deep Sea Pulse", "Water"),
        (7, "Cosmic Starlight", "Stars"),
        (8, "Shooting Meteor", "Chase"),
        (9, "Ambient Aurora", "Ambient"),
        (10, "Neon Scanner", "Chase")
    ]

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Dynamic Effects")
                        .font(.title2)
                        .fontWeight(.bold)
                    Text("Select from 146 on-chip addressable animation patterns")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }

                HStack(spacing: 24) {
                    VStack(alignment: .leading, spacing: 6) {
                        Text("Animation Velocity (Speed)")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Slider(
                            value: Binding(
                                get: { Double(viewModel.state.dynamicPage.speed) },
                                set: { viewModel.dispatch(.setSpeed(speed: UInt8($0))) }
                            ),
                            in: 1...32,
                            step: 1
                        )
                    }

                    VStack(alignment: .leading, spacing: 6) {
                        Text("Auto Cycle Loop")
                            .font(.caption)
                            .foregroundColor(.secondary)
                        Toggle("", isOn: Binding(
                            get: { viewModel.state.dynamicPage.autoCycle },
                            set: { _ in viewModel.dispatch(.toggleAutoCycle) }
                        ))
                        .labelsHidden()
                    }
                }
                .padding()
                .background(Color(NSColor.controlBackgroundColor))
                .cornerRadius(12)

                Text("Effect Catalogue")
                    .font(.headline)

                LazyVGrid(columns: [GridItem(.adaptive(minimum: 180))], spacing: 12) {
                    ForEach(sampleEffects, id: \.0) { effect in
                        Button {
                            viewModel.dispatch(.selectDynamicEffect(code: effect.0))
                        } label: {
                            VStack(alignment: .leading, spacing: 4) {
                                Text(effect.1)
                                    .fontWeight(.semibold)
                                Text(effect.2)
                                    .font(.caption)
                                    .foregroundColor(.secondary)
                            }
                            .frame(maxWidth: .infinity, alignment: .leading)
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
