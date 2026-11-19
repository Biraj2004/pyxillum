using System;
using System.IO;
using System.Runtime.InteropServices;
using System.Text.Json;
using Microsoft.UI;
using Microsoft.UI.Windowing;
using Windows.Graphics;

namespace Pyxillum.Diagnostics
{
    public class WindowState
    {
        public int X { get; set; } = -1;
        public int Y { get; set; } = -1;
        public int Width { get; set; } = 1280;
        public int Height { get; set; } = 720;
        public bool IsMaximised { get; set; } = false;
    }

    public static class WindowStateManager
    {
        public const int MinWidthEpx = 1000;
        public const int MinHeightEpx = 650;
        public const int DefaultWidthEpx = 1280;
        public const int DefaultHeightEpx = 720;

        private static string SettingsFilePath =>
            Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "Pyxillum", "window_state.json");

        private static SubclassProc? _subclassProc;
        private static AppWindow? _currentAppWindow;
        private static IntPtr _currentHwnd = IntPtr.Zero;

        [StructLayout(LayoutKind.Sequential)]
        private struct POINT
        {
            public int x;
            public int y;
        }

        [StructLayout(LayoutKind.Sequential)]
        private struct MINMAXINFO
        {
            public POINT ptReserved;
            public POINT ptMaxSize;
            public POINT ptMaxPosition;
            public POINT ptMinTrackSize;
            public POINT ptMaxTrackSize;
        }

        private delegate IntPtr SubclassProc(IntPtr hWnd, uint uMsg, IntPtr wParam, IntPtr lParam, UIntPtr uIdSubclass, UIntPtr dwRefData);

        [DllImport("comctl32.dll", SetLastError = true)]
        private static extern bool SetWindowSubclass(IntPtr hWnd, SubclassProc pfnSubclass, UIntPtr uIdSubclass, UIntPtr dwRefData);

        [DllImport("comctl32.dll", SetLastError = true)]
        private static extern IntPtr DefSubclassProc(IntPtr hWnd, uint uMsg, IntPtr wParam, IntPtr lParam);

        [DllImport("user32.dll")]
        private static extern uint GetDpiForWindow(IntPtr hwnd);

        public static void InitializeWindow(MainWindow window, AppWindow appWindow, IntPtr hwnd)
        {
            _currentAppWindow = appWindow;
            _currentHwnd = hwnd;

            // Subclass Win32 HWND to intercept WM_GETMINMAXINFO (0x0024)
            _subclassProc = (h, msg, wp, lp, id, refData) =>
            {
                if (msg == 0x0024) // WM_GETMINMAXINFO
                {
                    var mmi = Marshal.PtrToStructure<MINMAXINFO>(lp);
                    uint dpi = GetDpiForWindow(h);
                    double scale = dpi > 0 ? (dpi / 96.0) : 1.0;
                    int minW = (int)Math.Ceiling(MinWidthEpx * scale);
                    int minH = (int)Math.Ceiling(MinHeightEpx * scale);

                    mmi.ptMinTrackSize.x = Math.Max(mmi.ptMinTrackSize.x, minW);
                    mmi.ptMinTrackSize.y = Math.Max(mmi.ptMinTrackSize.y, minH);

                    Marshal.StructureToPtr(mmi, lp, false);
                    return IntPtr.Zero;
                }
                return DefSubclassProc(h, msg, wp, lp);
            };

            SetWindowSubclass(hwnd, _subclassProc, (UIntPtr)101, UIntPtr.Zero);

            // Restore geometry or center default
            RestoreOrCenter(appWindow, hwnd);

            // Hook size change & closing events to persist geometry
            appWindow.Closing += (s, e) => SaveState(appWindow, hwnd);
            window.Closed += (s, e) => SaveState(appWindow, hwnd);
        }

        public static void ResetToDefault()
        {
            if (_currentAppWindow != null && _currentHwnd != IntPtr.Zero)
            {
                ApplyDefaultGeometry(_currentAppWindow, _currentHwnd);
            }
        }

        private static void RestoreOrCenter(AppWindow appWindow, IntPtr hwnd)
        {
            if (AppConfig.ResetWindow)
            {
                Logger.Info("AppConfig.ResetWindow requested; applying default 1280x720 centered geometry.");
                ApplyDefaultGeometry(appWindow, hwnd);
                return;
            }

            var state = LoadState();
            if (state == null || state.Width <= 0 || state.Height <= 0)
            {
                Logger.Info("No valid persisted window state found; applying default 1280x720 centered geometry.");
                ApplyDefaultGeometry(appWindow, hwnd);
                return;
            }

            uint dpi = GetDpiForWindow(hwnd);
            double scale = dpi > 0 ? (dpi / 96.0) : 1.0;

            int targetW = Math.Max((int)(state.Width * scale), (int)(MinWidthEpx * scale));
            int targetH = Math.Max((int)(state.Height * scale), (int)(MinHeightEpx * scale));
            int targetX = state.X;
            int targetY = state.Y;

            // Check if saved position intersects with any active display
            var testRect = new RectInt32(targetX, targetY, targetW, targetH);
            var matchedArea = DisplayArea.GetFromRect(testRect, DisplayAreaFallback.None);

            if (matchedArea == null || targetX < -10000 || targetY < -10000)
            {
                Logger.Warn("Persisted window position is off-screen; recentering on primary display.");
                ApplyDefaultGeometry(appWindow, hwnd);
                return;
            }

            appWindow.MoveAndResize(new RectInt32(targetX, targetY, targetW, targetH));

            if (state.IsMaximised && appWindow.Presenter is OverlappedPresenter presenter)
            {
                presenter.Maximize();
            }
        }

        private static void ApplyDefaultGeometry(AppWindow appWindow, IntPtr hwnd)
        {
            uint dpi = GetDpiForWindow(hwnd);
            double scale = dpi > 0 ? (dpi / 96.0) : 1.0;
            var windowId = Microsoft.UI.Win32Interop.GetWindowIdFromWindow(hwnd);
            var displayArea = DisplayArea.GetFromWindowId(windowId, DisplayAreaFallback.Primary)
                              ?? DisplayArea.Primary;
            var workArea = displayArea.WorkArea;

            int defaultW = (int)(DefaultWidthEpx * scale);
            int defaultH = (int)(DefaultHeightEpx * scale);
            int defaultX = workArea.X + (workArea.Width - defaultW) / 2;
            int defaultY = workArea.Y + (workArea.Height - defaultH) / 2;

            if (appWindow.Presenter is OverlappedPresenter presenter && presenter.State == OverlappedPresenterState.Maximized)
            {
                presenter.Restore();
            }

            appWindow.MoveAndResize(new RectInt32(defaultX, defaultY, defaultW, defaultH));
        }

        public static void SaveState(AppWindow appWindow, IntPtr hwnd)
        {
            try
            {
                uint dpi = GetDpiForWindow(hwnd);
                double scale = dpi > 0 ? (dpi / 96.0) : 1.0;

                var state = new WindowState();

                if (appWindow.Presenter is OverlappedPresenter presenter)
                {
                    if (presenter.State == OverlappedPresenterState.Minimized)
                    {
                        return; // Do not overwrite geometry with minimized coords
                    }

                    state.IsMaximised = (presenter.State == OverlappedPresenterState.Maximized);
                }

                var pos = appWindow.Position;
                var size = appWindow.Size;

                if (!state.IsMaximised)
                {
                    state.X = pos.X;
                    state.Y = pos.Y;
                    state.Width = (int)(size.Width / scale);
                    state.Height = (int)(size.Height / scale);
                }
                else
                {
                    // Preserve previous unmaximised geometry
                    var existing = LoadState();
                    if (existing != null)
                    {
                        state.X = existing.X;
                        state.Y = existing.Y;
                        state.Width = existing.Width;
                        state.Height = existing.Height;
                    }
                }

                var dir = Path.GetDirectoryName(SettingsFilePath);
                if (!string.IsNullOrEmpty(dir) && !Directory.Exists(dir))
                {
                    Directory.CreateDirectory(dir);
                }

                var json = JsonSerializer.Serialize(state, new JsonSerializerOptions { WriteIndented = true });
                File.WriteAllText(SettingsFilePath, json);
                Logger.Debug($"Saved window geometry: {state.Width}x{state.Height} at ({state.X}, {state.Y}), Maximised: {state.IsMaximised}");
            }
            catch (Exception ex)
            {
                Logger.Warn($"Failed to persist window geometry: {ex.Message}");
            }
        }

        private static WindowState? LoadState()
        {
            try
            {
                if (File.Exists(SettingsFilePath))
                {
                    var json = File.ReadAllText(SettingsFilePath);
                    return JsonSerializer.Deserialize<WindowState>(json);
                }
            }
            catch (Exception ex)
            {
                Logger.Warn($"Failed to load window state: {ex.Message}");
            }
            return null;
        }
    }
}
