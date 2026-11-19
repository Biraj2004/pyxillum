import SwiftUI
import PyxillumCore

struct LedStripMonitorView: View {
    @EnvironmentObject var viewModel: AppViewModel

    var body: some View {
        TimelineView(.animation(minimumInterval: 1.0 / 60.0)) { _ in
            let frame = viewModel.getCurrentFrame()
            HStack(spacing: 4) {
                Text("Strip Monitor")
                    .font(.caption2)
                    .foregroundColor(.secondary)
                    .padding(.trailing, 8)

                GeometryReader { geo in
                    let count = min(60, Int(frame?.length ?? 60))
                    let step = geo.size.width / CGFloat(count)
                    Canvas { context, size in
                        for i in 0..<count {
                            let rect = CGRect(
                                x: CGFloat(i) * step + (step - 6) / 2.0,
                                y: (size.height - 12) / 2.0,
                                width: 6,
                                height: 12
                            )
                            var color = Color.black
                            if let f = frame, f.pixelsRgb.count >= (i * 3 + 3) {
                                let r = Double(f.pixelsRgb[i * 3]) / 255.0
                                let g = Double(f.pixelsRgb[i * 3 + 1]) / 255.0
                                let b = Double(f.pixelsRgb[i * 3 + 2]) / 255.0
                                color = Color(red: r, green: g, blue: b)
                            }
                            context.fill(Path(roundedRect: rect, cornerRadius: 2), with: .color(color))
                        }
                    }
                }
            }
            .frame(height: 24)
            .padding(.horizontal, 16)
            .padding(.vertical, 4)
            .background(Color(NSColor.windowBackgroundColor))
        }
    }
}
