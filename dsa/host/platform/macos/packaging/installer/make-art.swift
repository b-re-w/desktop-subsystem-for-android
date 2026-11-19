// Draws the installer's artwork from the Windows setup design: the DSA logo
// (dsa/host/platform/windows/installer/setup-ui/assets/logo.png) and the setup window's
// accent green (0F7B4A). Run it again after the logo changes:
//
//   swift make-art.swift ../../../windows/installer/setup-ui/assets/logo.png resources
//
// It writes background.png and background-dark.png for the Installer's sidebar (drawn at
// its bottom left, unscaled).
import AppKit
import CoreGraphics

let args = CommandLine.arguments
guard args.count == 3, let logo = NSImage(contentsOfFile: args[1])?
    .cgImage(forProposedRect: nil, context: nil, hints: nil)
else {
    FileHandle.standardError.write("usage: make-art.swift <logo.png> <output folder>\n".data(using: .utf8)!)
    exit(2)
}
let out = URL(fileURLWithPath: args[2])
let space = CGColorSpace(name: CGColorSpace.sRGB)!
let accent = (r: 0x0F / 255.0, g: 0x7B / 255.0, b: 0x4A / 255.0)

func write(_ name: String, width: Int, height: Int, draw: (CGContext) -> Void) {
    let ctx = CGContext(data: nil, width: width, height: height, bitsPerComponent: 8, bytesPerRow: 0,
                        space: space, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)!
    ctx.interpolationQuality = .high
    draw(ctx)
    let rep = NSBitmapImageRep(cgImage: ctx.makeImage()!)
    try! rep.representation(using: .png, properties: [:])!.write(to: out.appendingPathComponent(name))
}

// The sidebar: transparent, a soft accent glow in the corner and the logo on it. The
// Installer window is 620 by 418 points; the sidebar is about the left 170.
func sidebar(_ name: String, glow: Double) {
    write(name, width: 170, height: 418) { ctx in
        let gradient = CGGradient(colorsSpace: space, colors: [
            CGColor(srgbRed: accent.r, green: accent.g, blue: accent.b, alpha: glow),
            CGColor(srgbRed: accent.r, green: accent.g, blue: accent.b, alpha: 0),
        ] as CFArray, locations: [0, 1])!
        ctx.drawRadialGradient(gradient, startCenter: CGPoint(x: 60, y: 60), startRadius: 0,
                               endCenter: CGPoint(x: 60, y: 60), endRadius: 200, options: [])
        ctx.draw(logo, in: CGRect(x: 20, y: 20, width: 96, height: 96))
    }
}
sidebar("background.png", glow: 0.18)
sidebar("background-dark.png", glow: 0.35)
