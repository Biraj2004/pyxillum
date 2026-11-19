// swift-tools-version: 5.9
import PackageDescription

let package = Package(
    name: "Pyxillum",
    platforms: [
        .macOS(.v13)
    ],
    products: [
        .executable(name: "Pyxillum", targets: ["Pyxillum"]),
        .library(name: "PyxillumCore", targets: ["PyxillumCore"])
    ],
    targets: [
        .target(
            name: "PyxillumCore",
            path: "PyxillumCore",
            sources: ["Generated", "Sources"],
            publicHeadersPath: "Generated"
        ),
        .executableTarget(
            name: "Pyxillum",
            dependencies: ["PyxillumCore"],
            path: "Pyxillum"
        )
    ]
)
