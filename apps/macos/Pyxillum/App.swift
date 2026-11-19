import SwiftUI
import PyxillumCore

@main
struct PyxillumApp: App {
    @StateObject private var viewModel = AppViewModel()

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(viewModel)
                .frame(minWidth: 900, minHeight: 600)
        }
        .windowStyle(.titleBar)
        .windowToolbarStyle(.unified)

        // Native macOS Settings Scene (accessible via Cmd+,)
        Settings {
            SettingsView()
                .environmentObject(viewModel)
                .frame(width: 550, height: 420)
        }
    }
}
