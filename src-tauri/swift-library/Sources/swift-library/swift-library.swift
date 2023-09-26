func run_print_dialog(view: UnsafeRawPointer, ns_window: UnsafeRawPointer, data: RustVec<UInt8>) {
    // let window = Unmanaged<NSWindow>.fromOpaque(ns_window).takeUnretainedValue()
    // let window = Unmanaged<NSWindow>.fromOpaque(ns_window).takeUnretainedValue()
    let data = Data(data)
    let appDelegate = AppDelegate(data: data)
    NSApplication.shared.delegate = appDelegate
    NSApplication.shared.run()
    print("hello from swift")
}


import PDFKit

class AppDelegate: NSObject, NSApplicationDelegate {
    var window: NSWindow!
    var document: PDFDocument!

    init(data: Data) {
    
        document = PDFDocument(data: data)
        super.init()
    }
    
    func applicationDidFinishLaunching(_ aNotification: Notification) {
        // Create a window
        let windowRect = NSRect(x: 0, y: 0, width: 800, height: 600)
        window = NSWindow(contentRect: windowRect, styleMask: [.titled, .closable, .miniaturizable, .resizable], backing: .buffered, defer: false)
        
        let printOps = document!.printOperation(for: NSPrintInfo.shared , scalingMode: PDFPrintScalingMode.pageScaleToFit, autoRotate: false)

        printOps?.runModal(for:window , delegate:nil , didRun: nil , contextInfo: nil)

    }

    // func applicationWillFinishLaunching(_ notification: Notification) {
    //     // Load a PDF document
    //     let pdfURL = URL(fileURLWithPath: "/Users/michaelaboah/Desktop/DeleteExample.pdf") //else {
    //    
    //     var pdfData: Data?
    //
    //     do {
    //         pdfData = try Data(contentsOf: pdfURL)
    //     } catch {
    //         print("Failed")
    //     }
    //
    //
    // }

    func applicationDidBecomeActive(_ notification: Notification) {
        let printOps = document.printOperation(for: NSPrintInfo.shared , scalingMode: PDFPrintScalingMode.pageScaleToFit, autoRotate: false)

        printOps?.runModal(for:window , delegate:self , didRun: nil , contextInfo: nil)
        
    }
}


  
