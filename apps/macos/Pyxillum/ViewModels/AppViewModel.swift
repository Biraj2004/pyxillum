import SwiftUI
import PyxillumCore

final class NativeStateListener: StateListenerCallback {
    private let onUpdate: (AppStateFfi) -> Void

    init(onUpdate: @escaping (AppStateFfi) -> Void) {
        self.onUpdate = onUpdate
    }

    func onStateChanged(newState: AppStateFfi) {
        DispatchQueue.main.async {
            self.onUpdate(newState)
        }
    }
}

final class NativePlatformServices: PlatformServicesCallback {
    func openBluetoothSettings() {
        if let url = URL(string: "x-apple.systempreferences:com.apple.preferences.Bluetooth") {
            NSWorkspace.shared.open(url)
        }
    }

    func appDataDir() -> String {
        FileManager.default.urls(for: .applicationSupportDirectory, in: .userDomainMask).first?.path ?? ""
    }

    func hasAudioPermission() -> Bool {
        true
    }
}

@MainActor
public final class AppViewModel: ObservableObject {
    @Published public var state: AppStateFfi
    @Published public var selectedScreen: ScreenId = .deviceHub
    @Published public var safetyConfirmed: Bool = false

    private var engine: EngineFfi?
    private var listener: NativeStateListener?

    public init() {
        // Initial state placeholder
        self.state = AppStateFfi(
            connection: ConnectionStatusFfi(kind: .disconnected, address: nil, name: nil, message: nil),
            isBluetoothAvailable: true,
            discoveredDevices: [],
            currentScreen: .deviceHub,
            solidPage: SolidPageStateFfi(selectedColorR: 255, selectedColorG: 59, selectedColorB: 48, kelvin: 4000, customEffect: .solid, customSpeed: 16),
            dynamicPage: DynamicPageStateFfi(selectedPatternCode: 1, speed: 16, brightness: 200, autoCycle: false),
            musicPage: MusicPageStateFfi(selectedMode: 1, sensitivity: 60, inputSource: .player),
            diyPage: DiyPageStateFfi(activeStepIndex: 0, stepsCount: 6, pattern: .breathe, speed: 16, isPreviewing: false),
            stripConfig: StripConfigFfi(pixelCount: 60, chipOrder: .grb),
            isPreviewMode: false,
            activeError: nil
        )

        let services = NativePlatformServices()
        do {
            let eng = try createEngine(simulated: false, services: services)
            self.engine = eng
            let l = NativeStateListener { [weak self] newState in
                self?.state = newState
                self?.selectedScreen = newState.currentScreen
            }
            self.listener = l
            eng.subscribe(listener: l)
            self.state = eng.state()
        } catch {
            print("[Pyxillum macOS] Failed to initialise EngineFfi: \(error)")
        }
    }

    public func dispatch(_ action: ActionFfi) {
        Task {
            engine?.dispatch(action: action)
        }
    }

    public func getCurrentFrame() -> StripFrameFfi? {
        engine?.frame()
    }
}
