using System;
using System.Collections.Generic;
using System.IO;
using System.Runtime.InteropServices.WindowsRuntime;
using System.Text;
using System.Threading.Tasks;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using Windows.Graphics.Imaging;
using Windows.Storage.Streams;
using Windows.Foundation;

namespace Pyxillum.Diagnostics
{
    public class AuditDefect
    {
        public string PageName { get; set; } = string.Empty;
        public string WindowSize { get; set; } = string.Empty;
        public string Criteria { get; set; } = string.Empty;
        public string Description { get; set; } = string.Empty;
        public string ElementDetails { get; set; } = string.Empty;
        public string ScreenshotName { get; set; } = string.Empty;
    }

    public static class SelftestRunner
    {
        private static readonly List<string> _bindingErrors = new();
        private static readonly object _bindingLock = new();

        public static void RecordBindingError(string error)
        {
            lock (_bindingLock)
            {
                _bindingErrors.Add(error);
            }
        }

        public static async Task<int> RunAsync(MainWindow mainWindow, string outDir)
        {
            Logger.Info($"[Selftest] Starting Selftest audit. Target output directory: {outDir}");

            // Ensure output directory exists
            Directory.CreateDirectory(outDir);

            var (widths, heights) = GetTargetSizes();
            var pageTags = new[]
            {
                ("Device Hub", "Devices", typeof(Views.DevicesPage)),
                ("Dynamic FX", "Dynamic", typeof(Views.DynamicPage)),
                ("Solid Colour", "Solid", typeof(Views.SolidPage)),
                ("Music Rhythm", "Music", typeof(Views.MusicPage)),
                ("DIY Sequencer", "Diy", typeof(Views.DiyPage)),
                ("Hardware Settings", "Settings", typeof(Views.SettingsPage))
            };

            var defects = new List<AuditDefect>();
            int totalTests = 0;

            for (int s = 0; s < widths.Length; s++)
            {
                int width = widths[s];
                int height = heights[s];
                string sizeStr = $"{width}x{height}";

                Logger.Info($"[Selftest] Configuring layout window size: {sizeStr}");

                // Enforce exact layout canvas size on root grid for deterministic audit
                if (mainWindow.Content is FrameworkElement rootFe)
                {
                    rootFe.Width = width;
                    rootFe.Height = height;
                    rootFe.UpdateLayout();
                }

                foreach (var (pageTitle, tag, pageType) in pageTags)
                {
                    totalTests++;
                    string safeTitle = pageTitle.Replace(" ", "_");
                    string screenshotFileName = $"{safeTitle}_{sizeStr}.png";
                    string screenshotFilePath = Path.Combine(outDir, screenshotFileName);

                    Logger.Info($"[Selftest] Auditing {pageTitle} at {sizeStr}");

                    // Navigate frame
                    mainWindow.NavigateTo(tag);
                    await Task.Delay(150); // allow dispatcher & layout to settle
                    mainWindow.Content.UpdateLayout();

                    // Capture visual screenshot
                    try
                    {
                        await CaptureScreenshotAsync(mainWindow.Content, screenshotFilePath);
                    }
                    catch (Exception ex)
                    {
                        Logger.Error($"[Selftest] Failed to render screenshot for {pageTitle} at {sizeStr}", ex);
                    }

                    // Validate Visual Tree
                    var pageDefects = ValidateVisualTree(
                        mainWindow,
                        mainWindow.MainContentFrame.Content as Page,
                        pageTitle,
                        width,
                        height,
                        screenshotFileName);

                    defects.AddRange(pageDefects);
                }
            }

            // Write comprehensive audit report to docs/v2-audit.md
            string auditReportPath = Path.Combine(Directory.GetCurrentDirectory(), "docs", "v2-audit.md");
            WriteAuditReport(auditReportPath, totalTests, defects);

            Logger.Info($"[Selftest] Completed audit. Total runs: {totalTests}, Defects found: {defects.Count}");
            Logger.Info($"[Selftest] Audit report generated at: {auditReportPath}");

            return defects.Count > 0 ? 1 : 0;
        }

        private static (int[] widths, int[] heights) GetTargetSizes()
        {
            return (
                new[] { 1000, 1280, 1536, 1920, 2560 },
                new[] { 650,  720,  832,  1030, 1440 }
            );
        }

        private static async Task CaptureScreenshotAsync(UIElement element, string filePath)
        {
            var rtb = new RenderTargetBitmap();
            await rtb.RenderAsync(element);
            var pixelBuffer = await rtb.GetPixelsAsync();
            byte[] pixels = pixelBuffer.ToArray();

            using var inMemoryStream = new InMemoryRandomAccessStream();
            var encoder = await BitmapEncoder.CreateAsync(BitmapEncoder.PngEncoderId, inMemoryStream);
            encoder.SetPixelData(
                BitmapPixelFormat.Bgra8,
                BitmapAlphaMode.Premultiplied,
                (uint)rtb.PixelWidth,
                (uint)rtb.PixelHeight,
                96, 96,
                pixels);
            await encoder.FlushAsync();

            inMemoryStream.Seek(0);
            using var fileStream = File.Create(filePath);
            await inMemoryStream.AsStreamForRead().CopyToAsync(fileStream);
        }

        private static List<AuditDefect> ValidateVisualTree(
            MainWindow window,
            Page? page,
            string pageName,
            int windowWidth,
            int windowHeight,
            string screenshotName)
        {
            var list = new List<AuditDefect>();
            string sizeStr = $"{windowWidth}x{windowHeight}";

            // Check (a): Page content area has no children or ActualHeight < 100
            if (page == null)
            {
                list.Add(new AuditDefect
                {
                    PageName = pageName,
                    WindowSize = sizeStr,
                    Criteria = "Criterion (a) - Empty / Null Page",
                    Description = "ContentFrame.Content is null or failed to navigate",
                    ElementDetails = "Frame.Content",
                    ScreenshotName = screenshotName
                });
                return list;
            }

            if (page.ActualHeight < 100)
            {
                list.Add(new AuditDefect
                {
                    PageName = pageName,
                    WindowSize = sizeStr,
                    Criteria = "Criterion (a) - Page ActualHeight < 100",
                    Description = $"Page ActualHeight is {page.ActualHeight:F1}px (less than minimum 100px)",
                    ElementDetails = page.GetType().Name,
                    ScreenshotName = screenshotName
                });
            }

            if (page.Content == null)
            {
                list.Add(new AuditDefect
                {
                    PageName = pageName,
                    WindowSize = sizeStr,
                    Criteria = "Criterion (a) - Missing Root Content",
                    Description = "Page.Content is null",
                    ElementDetails = page.GetType().Name,
                    ScreenshotName = screenshotName
                });
            }

            // Check (e): Hooked XAML binding failures
            lock (_bindingLock)
            {
                if (_bindingErrors.Count > 0)
                {
                    foreach (var err in _bindingErrors)
                    {
                        list.Add(new AuditDefect
                        {
                            PageName = pageName,
                            WindowSize = sizeStr,
                            Criteria = "Criterion (e) - XAML Binding Failure",
                            Description = err,
                            ElementDetails = "DebugSettings.BindingFailed",
                            ScreenshotName = screenshotName
                        });
                    }
                    _bindingErrors.Clear();
                }
            }

            // Check visual tree nodes recursively
            WalkVisualTree(page, window, pageName, windowWidth, windowHeight, sizeStr, screenshotName, list);

            return list;
        }

        private static void WalkVisualTree(
            DependencyObject current,
            MainWindow window,
            string pageName,
            int windowWidth,
            int windowHeight,
            string sizeStr,
            string screenshotName,
            List<AuditDefect> list)
        {
            if (current is FrameworkElement fe && fe.Visibility == Visibility.Visible)
            {
                // Check (c): TextBlock label with IsTextTrimmed = true and no tooltip
                if (fe is TextBlock tb)
                {
                    if (tb.IsTextTrimmed)
                    {
                        var tip = ToolTipService.GetToolTip(tb);
                        if (tip == null || string.IsNullOrWhiteSpace(tip.ToString()))
                        {
                            list.Add(new AuditDefect
                            {
                                PageName = pageName,
                                WindowSize = sizeStr,
                                Criteria = "Criterion (c) - Trimmed Text Without ToolTip",
                                Description = $"Text label \"{tb.Text}\" is visually trimmed but lacks a ToolTip",
                                ElementDetails = $"TextBlock (Text='{tb.Text}', ActualWidth={tb.ActualWidth:F1})",
                                ScreenshotName = screenshotName
                            });
                        }
                    }
                }

                // Check (d): Horizontal scrollbar in page ScrollViewer
                if (fe is ScrollViewer sv)
                {
                    if (sv.ComputedHorizontalScrollBarVisibility == Visibility.Visible ||
                        (sv.HorizontalScrollBarVisibility != ScrollBarVisibility.Disabled && sv.ExtentWidth > sv.ViewportWidth + 0.5))
                    {
                        list.Add(new AuditDefect
                        {
                            PageName = pageName,
                            WindowSize = sizeStr,
                            Criteria = "Criterion (d) - Horizontal Scrollbar Active",
                            Description = $"ScrollViewer has horizontal overflow (ExtentWidth={sv.ExtentWidth:F1}px > ViewportWidth={sv.ViewportWidth:F1}px)",
                            ElementDetails = $"ScrollViewer (Name='{sv.Name}')",
                            ScreenshotName = screenshotName
                        });
                    }
                }

                // Check (b): Element bounds extend beyond window client area (tolerance 0.5 px)
                if (fe.ActualWidth > 0 && fe.ActualHeight > 0 && fe.IsLoaded)
                {
                    try
                    {
                        var transform = fe.TransformToVisual(window.Content);
                        var pos = transform.TransformPoint(new Point(0, 0));
                        double rightEdge = pos.X + fe.ActualWidth;

                        // Exclude elements inside horizontal ScrollViewer panels
                        if (rightEdge > windowWidth + 0.5 && !IsInsideHorizontalScrollViewer(fe))
                        {
                            list.Add(new AuditDefect
                            {
                                PageName = pageName,
                                WindowSize = sizeStr,
                                Criteria = "Criterion (b) - Clipping Beyond Window Bounds",
                                Description = $"Element bounds extend past window right edge: right edge {rightEdge:F1}px exceeds {windowWidth}px by {rightEdge - windowWidth:F1}px",
                                ElementDetails = $"{fe.GetType().Name} (Name='{fe.Name}', Width={fe.ActualWidth:F1})",
                                ScreenshotName = screenshotName
                            });
                        }
                    }
                    catch
                    {
                        // Transform calculation may fail if element is unparented
                    }
                }

                // Check (f): Any element with an enlarging hover state
                if (fe.RenderTransform is CompositeTransform ct && (ct.ScaleX > 1.0 || ct.ScaleY > 1.0))
                {
                    list.Add(new AuditDefect
                    {
                        PageName = pageName,
                        WindowSize = sizeStr,
                        Criteria = "Criterion (f) - Enlarging Transform State",
                        Description = $"Element has scale expansion transform (ScaleX={ct.ScaleX}, ScaleY={ct.ScaleY}) violating hover rule",
                        ElementDetails = $"{fe.GetType().Name} (Name='{fe.Name}')",
                        ScreenshotName = screenshotName
                    });
                }
                else if (fe.RenderTransform is ScaleTransform st && (st.ScaleX > 1.0 || st.ScaleY > 1.0))
                {
                    list.Add(new AuditDefect
                    {
                        PageName = pageName,
                        WindowSize = sizeStr,
                        Criteria = "Criterion (f) - Enlarging Transform State",
                        Description = $"Element has scale expansion transform (ScaleX={st.ScaleX}, ScaleY={st.ScaleY}) violating hover rule",
                        ElementDetails = $"{fe.GetType().Name} (Name='{fe.Name}')",
                        ScreenshotName = screenshotName
                    });
                }
            }

            int count = VisualTreeHelper.GetChildrenCount(current);
            for (int i = 0; i < count; i++)
            {
                var child = VisualTreeHelper.GetChild(current, i);
                WalkVisualTree(child, window, pageName, windowWidth, windowHeight, sizeStr, screenshotName, list);
            }
        }

        private static bool IsInsideHorizontalScrollViewer(FrameworkElement element)
        {
            DependencyObject? parent = VisualTreeHelper.GetParent(element);
            while (parent != null)
            {
                if (parent is ScrollViewer sv && sv.HorizontalScrollBarVisibility != ScrollBarVisibility.Disabled)
                {
                    return true;
                }
                parent = VisualTreeHelper.GetParent(parent);
            }
            return false;
        }

        private static void WriteAuditReport(string filePath, int totalTests, List<AuditDefect> defects)
        {
            var sb = new StringBuilder();
            sb.AppendLine("# Pyxillum Windows Native App Visual & Layout Audit");
            sb.AppendLine();
            sb.AppendLine($"**Audit Date**: {DateTime.Now:yyyy-MM-dd HH:mm:ss}  ");
            sb.AppendLine($"**Environment**: Windows 11 (x86_64) | WinUI 3 (Windows App SDK 1.6) | .NET 8  ");
            sb.AppendLine($"**Total Screen & Geometry Combinations Audited**: {totalTests}  ");
            sb.AppendLine($"**Total Detected Defects**: {defects.Count}  ");
            sb.AppendLine($"**Audit Outcome**: {(defects.Count == 0 ? "PASSED (Clean)" : "FAILED (Defects Detected)")}  ");
            sb.AppendLine();
            sb.AppendLine("---");
            sb.AppendLine();
            sb.AppendLine("## 1. Executive Summary of Observed Problems & Root Causes");
            sb.AppendLine();
            sb.AppendLine("### 1.1 Blank Device Hub Page & Initial Navigation Failure");
            sb.AppendLine("- **Observed Behaviour**: On application startup, the entire main content area is completely blank. The sidebar selection indicates 'Device Hub', but no controls, empty cards, or headers are rendered.");
            sb.AppendLine("- **Root Cause**: `MainWindow.xaml.cs` initialised `NavView.SelectedItem = NavView.MenuItems[0]`, but in WinUI 3, programmatically setting `SelectedItem` during startup does *not* invoke `ItemInvoked` or `SelectionChanged` callbacks. Consequently, `ContentFrame.Navigate` was never executed during initialization, leaving `ContentFrame.Content == null`.");
            sb.AppendLine();
            sb.AppendLine("### 1.2 Button and Navigation Item Hover Enlargement");
            sb.AppendLine("- **Observed Behaviour**: Navigation items and buttons visibly enlarge/grow on cursor hover (e.g. 'Solid Colour' nav item increases in size).");
            sb.AppendLine("- **Root Cause**: Default or explicit WinUI pointer-over visual state transitions apply scale transforms or margin expansions. Hard hover rule requires removing all scale transforms and restricting hover effects strictly to background and border colour transitions.");
            sb.AppendLine();
            sb.AppendLine("### 1.3 Inconsistent and Erroneous Iconography");
            sb.AppendLine("- **Observed Behaviour**: Solid Colour displayed a '+' icon (`\\uE710`), Dynamic FX displayed a paint palette (`\\uE790`), DIY Sequencer displayed an upload-to-screen icon (`\\uE7B5`), and Device Hub displayed a generic broadcast glyph (`\\uE703`).");
            sb.AppendLine("- **Root Cause**: Hardcoded glyphs in `MainWindow.xaml` did not reference `design/icons.toml` authoritative mapping. Authoritative Segoe Fluent mappings: Solid Colour (`\\uE790` Palette), Dynamic FX (`\\uE706` Sparkles), DIY Sequencer (`\\uE81C` Timeline/Sliders), Device Hub (`\\uE702` Bluetooth), Music (`\\uEC4F` Music Note), Settings (`\\uE713` Gear).");
            sb.AppendLine();
            sb.AppendLine("### 1.4 Disconnected Header with Live Controls & Missing Preview Mode Guidance");
            sb.AppendLine("- **Observed Behaviour**: The global header displayed a yellow 'Disconnected' indicator, but the brightness slider and power toggle appeared active without indicating whether commands affected hardware or the simulator.");
            sb.AppendLine("- **Root Cause**: Shell lacked explicit Preview Mode badge, contextual explanation banner ('Preview mode - changes affect the virtual strip only'), and single authoritative connection status location with quick scan invocation.");
            sb.AppendLine();
            sb.AppendLine("### 1.5 Virtual Strip Monitor Visual Deficiencies");
            sb.AppendLine("- **Observed Behaviour**: LED strip monitor rendered black rectangular ticks without diode illumination, glow diffusion, adaptive diode counts, or frame rate captions.");
            sb.AppendLine("- **Root Cause**: `LedStripMonitor.cs` hardcoded 60 black `Rectangle` shapes with fixed dimensions (6x12) without drawing circular diodes, ambient neon glow diffusion, or active pixel count text.");
            sb.AppendLine();
            sb.AppendLine("### 1.6 Layout Clipping & Tooltip Omissions");
            sb.AppendLine("- **Observed Behaviour**: Narrow viewport testing (1000x650) caused horizontal scrollbars or clipped text labels without tooltips.");
            sb.AppendLine("- **Root Cause**: Fixed column widths in grid scaffolds and lack of `IsTextTrimmed` detection with attached `ToolTipService.ToolTip`.");
            sb.AppendLine();
            sb.AppendLine("---");
            sb.AppendLine();
            sb.AppendLine("## 2. Detailed Visual Tree Layout Defect Log");
            sb.AppendLine();

            if (defects.Count == 0)
            {
                sb.AppendLine("Zero layout, visual, or binding defects detected. All visual tree criteria passed.");
            }
            else
            {
                sb.AppendLine("| Page | Window Size | Category | Description | Element Details | Screenshot |");
                sb.AppendLine("| :--- | :---: | :--- | :--- | :--- | :--- |");
                foreach (var d in defects)
                {
                    sb.AppendLine($"| **{d.PageName}** | `{d.WindowSize}` | {d.Criteria} | {d.Description} | `{d.ElementDetails}` | [`{d.ScreenshotName}`](../dist/selftest_screenshots/{d.ScreenshotName}) |");
                }
            }

            sb.AppendLine();
            sb.AppendLine("---");
            sb.AppendLine();
            sb.AppendLine("## 3. Rendered Screenshots Catalogue");
            sb.AppendLine();
            sb.AppendLine("Screenshots captured via `RenderTargetBitmap` for each page across all five standardized audit sizes (1000x650, 1280x720, 1536x832, 1920x1030, 2560x1440) are staged in `dist/selftest_screenshots/`.");

            File.WriteAllText(filePath, sb.ToString(), Encoding.UTF8);
        }
    }
}
