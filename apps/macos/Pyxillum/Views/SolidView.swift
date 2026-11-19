import SwiftUI
import PyxillumCore

struct SolidView: View {
    @EnvironmentObject var viewModel: AppViewModel

    @State private var red: Double = 255
    @State private var green: Double = 59
    @State private var blue: Double = 48
    @State private var kelvin: Double = 4000

    var body: some View {
        ScrollView {
            VStack(alignment: .leading, spacing: 20) {
                VStack(alignment: .leading, spacing: 4) {
                    Text("Solid Colour")
                        .font(.title2)
                        .fontWeight(.bold)
                    Text("Continuous chromatic hues, colour temperature, and architectural swatches")
                        .font(.subheadline)
                        .foregroundColor(.secondary)
                }

                HStack(alignment: .top, spacing: 24) {
                    // Custom Colour Channels
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Custom Colour Channels")
                            .font(.headline)

                        HStack {
                            Circle()
                                .fill(Color(red: red / 255.0, green: green / 255.0, blue: blue / 255.0))
                                .frame(width: 60, height: 60)
                            Text(String(format: "#%02X%02X%02X", Int(red), Int(green), Int(blue)))
                                .font(.system(.title3, design: .monospaced))
                                .fontWeight(.bold)
                        }

                        VStack(alignment: .leading) {
                            Text("Red (\(Int(red)))").font(.caption)
                            Slider(value: $red, in: 0...255, step: 1) { _ in updateColor() }
                        }
                        VStack(alignment: .leading) {
                            Text("Green (\(Int(green)))").font(.caption)
                            Slider(value: $green, in: 0...255, step: 1) { _ in updateColor() }
                        }
                        VStack(alignment: .leading) {
                            Text("Blue (\(Int(blue)))").font(.caption)
                            Slider(value: $blue, in: 0...255, step: 1) { _ in updateColor() }
                        }
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color(NSColor.controlBackgroundColor))
                    .cornerRadius(12)

                    // Kelvin & Swatches
                    VStack(alignment: .leading, spacing: 16) {
                        Text("Colour Temperature")
                            .font(.headline)
                        Text("\(Int(kelvin)) K")
                            .font(.subheadline)
                            .foregroundColor(.secondary)

                        Slider(value: $kelvin, in: 2700...6500, step: 50) { _ in
                            viewModel.dispatch(.setColorTemperature(kelvin: UInt32(kelvin)))
                        }

                        Text("Architectural Swatches")
                            .font(.headline)
                            .padding(.top, 8)

                        LazyVGrid(columns: [GridItem(.adaptive(minimum: 90))], spacing: 8) {
                            swatchButton("Amber", r: 255, g: 120, b: 0)
                            swatchButton("Warm", r: 255, g: 200, b: 140)
                            swatchButton("Red", r: 255, g: 0, b: 0)
                            swatchButton("Green", r: 0, g: 255, b: 0)
                            swatchButton("Blue", r: 0, g: 0, b: 255)
                            swatchButton("Cyan", r: 0, g: 255, b: 255)
                            swatchButton("Magenta", r: 255, g: 0, b: 255)
                        }
                    }
                    .frame(maxWidth: .infinity)
                    .padding()
                    .background(Color(NSColor.controlBackgroundColor))
                    .cornerRadius(12)
                }
            }
            .padding(24)
        }
    }

    private func swatchButton(_ name: String, r: UInt8, g: UInt8, b: UInt8) -> some View {
        Button(name) {
            red = Double(r)
            green = Double(g)
            blue = Double(b)
            updateColor()
        }
        .buttonStyle(.bordered)
    }

    private func updateColor() {
        viewModel.dispatch(.setSolidColor(r: UInt8(red), g: UInt8(green), b: UInt8(blue)))
    }
}
