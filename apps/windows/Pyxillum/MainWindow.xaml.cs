using System;
using Microsoft.UI;
using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Pyxillum.Core;
using Pyxillum.ViewModels;
using Windows.Graphics;

namespace Pyxillum
{
    public partial class MainWindow : Window
    {
        public AppViewModel ViewModel { get; }
        public Frame MainContentFrame => ContentFrame;

        private AppWindow? _appWindow;
        private string? _lastErrorDetails;

        public MainWindow()
        {
            this.InitializeComponent();

            ViewModel = new AppViewModel();

            // Enable Windows 11 Mica backdrop
            this.SystemBackdrop = new MicaBackdrop();

            // Configure Native Title Bar
            this.ExtendsContentIntoTitleBar = true;
            this.SetTitleBar(AppTitleBar);

            // Configure AppWindow & 80% Centered Geometry
            ConfigureWindowGeometry();

            // Connect Live LED Strip Monitor frame pulling and stats callback
            LiveStripMonitor.FrameSource = () => ViewModel.GetCurrentFrame();
            LiveStripMonitor.StatsCallback = (leds, fps) =>
            {
                DispatcherQueue.TryEnqueue(() =>
                {
                    StripStatsText.Text = $"{leds} LEDs · {fps} FPS";
                    ToolTipService.SetToolTip(StripStatsText, $"{leds} LEDs · {fps} FPS");
                });
            };

            // Adaptive NavigationView based on window width
            this.SizeChanged += (s, e) => UpdateNavigationViewAdaptive(e.Size.Width);
            UpdateNavigationViewAdaptive(this.Bounds.Width);

            // Wire Header & Connection Event Listeners
            ConnectionBadgeButton.Click += (s, e) =>
            {
                NavView.SelectedItem = NavView.MenuItems[0];
                NavigateTo("Devices");
            };
            HeaderScanButton.Click += (s, e) =>
            {
                NavView.SelectedItem = NavView.MenuItems[0];
                NavigateTo("Devices");
                ViewModel.StartScan();
            };

            // Wire UI Event Listeners
            MasterPowerButton.Click += (s, e) => ViewModel.TogglePower();
            MasterBrightnessSlider.ValueChanged += (s, e) => ViewModel.SetBrightness((byte)e.NewValue);
            OpenBluetoothSettingsButton.Click += (s, e) => ViewModel.OpenBluetoothSettings();
            ErrorNotificationBar.Closed += (s, e) => ViewModel.DismissError();

            CopyErrorDetailsButton.Click += (s, e) =>
            {
                if (!string.IsNullOrEmpty(_lastErrorDetails))
                {
                    var dataPackage = new Windows.ApplicationModel.DataTransfer.DataPackage();
                    dataPackage.SetText(_lastErrorDetails);
                    Windows.ApplicationModel.DataTransfer.Clipboard.SetContent(dataPackage);
                }
            };

            OpenLogsButton.Click += (s, e) =>
            {
                Diagnostics.Logger.OpenLogDirectory();
            };

            NavView.ItemInvoked += NavView_ItemInvoked;
            if (Diagnostics.AppConfig.IconSheet)
            {
                NavigateTo("IconSheet");
            }
            else
            {
                NavView.SelectedItem = NavView.MenuItems[0];
                NavigateTo("Devices");
            }

            // Observe ViewModel changes for Shell Badges
            ViewModel.PropertyChanged += ViewModel_PropertyChanged;
            UpdateShellState();
        }

        public void DisplayGlobalError(string message, string details)
        {
            _lastErrorDetails = details;
            ErrorNotificationBar.Message = message;
            ErrorNotificationBar.IsOpen = true;
            OpenLogsButton.Visibility = Visibility.Visible;
        }

        public void NavigateTo(string tag)
        {
            switch (tag)
            {
                case "Devices":
                    ViewModel.Navigate(ScreenId.DeviceHub);
                    ContentFrame.Navigate(typeof(Views.DevicesPage), ViewModel);
                    break;
                case "Dynamic":
                    ViewModel.Navigate(ScreenId.Dynamic);
                    ContentFrame.Navigate(typeof(Views.DynamicPage), ViewModel);
                    break;
                case "Solid":
                    ViewModel.Navigate(ScreenId.Solid);
                    ContentFrame.Navigate(typeof(Views.SolidPage), ViewModel);
                    break;
                case "Music":
                    ViewModel.Navigate(ScreenId.Music);
                    ContentFrame.Navigate(typeof(Views.MusicPage), ViewModel);
                    break;
                case "Diy":
                    ViewModel.Navigate(ScreenId.Diy);
                    ContentFrame.Navigate(typeof(Views.DiyPage), ViewModel);
                    break;
                case "Settings":
                    ViewModel.Navigate(ScreenId.HardwareSettings);
                    ContentFrame.Navigate(typeof(Views.SettingsPage), ViewModel);
                    break;
                case "IconSheet":
                    ContentFrame.Navigate(typeof(Views.IconSheetPage));
                    break;
            }
        }

        private void ConfigureWindowGeometry()
        {
            var hwnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
            var windowId = Microsoft.UI.Win32Interop.GetWindowIdFromWindow(hwnd);
            _appWindow = AppWindow.GetFromWindowId(windowId);

            if (_appWindow != null)
            {
                Diagnostics.WindowStateManager.InitializeWindow(this, _appWindow, hwnd);
            }
        }

        public void UpdateNavigationViewAdaptive(double width)
        {
            if (width < 1100)
            {
                if (NavView.PaneDisplayMode != NavigationViewPaneDisplayMode.LeftCompact)
                {
                    NavView.PaneDisplayMode = NavigationViewPaneDisplayMode.LeftCompact;
                    NavView.IsPaneOpen = false;
                }
            }
            else
            {
                if (NavView.PaneDisplayMode != NavigationViewPaneDisplayMode.Left)
                {
                    NavView.PaneDisplayMode = NavigationViewPaneDisplayMode.Left;
                    NavView.IsPaneOpen = true;
                }
            }
        }

        private void NavView_ItemInvoked(NavigationView sender, NavigationViewItemInvokedEventArgs args)
        {
            if (args.InvokedItemContainer is NavigationViewItem item && item.Tag is string tag)
            {
                NavigateTo(tag);
            }
        }

        private void ViewModel_PropertyChanged(object? sender, System.ComponentModel.PropertyChangedEventArgs e)
        {
            if (e.PropertyName == nameof(ViewModel.Connection) ||
                e.PropertyName == nameof(ViewModel.IsPreviewMode) ||
                e.PropertyName == nameof(ViewModel.IsBluetoothAvailable) ||
                e.PropertyName == nameof(ViewModel.HasError))
            {
                UpdateShellState();
            }
        }

        private void UpdateShellState()
        {
            // Connection Status
            switch (ViewModel.Connection.Kind)
            {
                case ConnectionStatusKind.Connected:
                    ConnectionProgressRing.IsActive = false;
                    ConnectionProgressRing.Visibility = Visibility.Collapsed;
                    ConnectionIndicator.Visibility = Visibility.Visible;
                    ConnectionIndicator.Fill = (Brush)Application.Current.Resources["BrushSuccess"];
                    ConnectionStatusText.Text = string.IsNullOrEmpty(ViewModel.Connection.Name) ? "Connected" : ViewModel.Connection.Name;
                    DisconnectedBanner.Visibility = Visibility.Collapsed;
                    MasterControlsPanel.Opacity = 1.0;
                    ToolTipService.SetToolTip(MasterControlsPanel, null);
                    PreviewBadge.Visibility = Visibility.Collapsed;
                    break;
                case ConnectionStatusKind.Connecting:
                    ConnectionIndicator.Visibility = Visibility.Collapsed;
                    ConnectionProgressRing.IsActive = true;
                    ConnectionProgressRing.Visibility = Visibility.Visible;
                    ConnectionStatusText.Text = string.IsNullOrEmpty(ViewModel.Connection.Name) ? "Connecting..." : $"Connecting to {ViewModel.Connection.Name}...";
                    DisconnectedBanner.Visibility = Visibility.Collapsed;
                    MasterControlsPanel.Opacity = 0.8;
                    ToolTipService.SetToolTip(MasterControlsPanel, null);
                    PreviewBadge.Visibility = Visibility.Collapsed;
                    break;
                case ConnectionStatusKind.Scanning:
                    ConnectionIndicator.Visibility = Visibility.Collapsed;
                    ConnectionProgressRing.IsActive = true;
                    ConnectionProgressRing.Visibility = Visibility.Visible;
                    ConnectionStatusText.Text = "Scanning...";
                    DisconnectedBanner.Visibility = Visibility.Visible;
                    MasterControlsPanel.Opacity = 0.6;
                    ToolTipService.SetToolTip(MasterControlsPanel, "Preview mode: modifies the virtual strip only (connect a controller to apply to LEDs)");
                    PreviewBadge.Visibility = ViewModel.IsPreviewMode ? Visibility.Visible : Visibility.Collapsed;
                    break;
                default:
                    ConnectionProgressRing.IsActive = false;
                    ConnectionProgressRing.Visibility = Visibility.Collapsed;
                    ConnectionIndicator.Visibility = Visibility.Visible;
                    ConnectionIndicator.Fill = (Brush)Application.Current.Resources["BrushWarning"];
                    ConnectionStatusText.Text = "Disconnected";
                    DisconnectedBanner.Visibility = Visibility.Visible;
                    MasterControlsPanel.Opacity = 0.6;
                    ToolTipService.SetToolTip(MasterControlsPanel, "Preview mode: modifies the virtual strip only (connect a controller to apply to LEDs)");
                    PreviewBadge.Visibility = ViewModel.IsPreviewMode ? Visibility.Visible : Visibility.Collapsed;
                    break;
            }

            // Bluetooth Unavailable InfoBar
            BluetoothWarningBar.IsOpen = !ViewModel.IsBluetoothAvailable;

            // Error Bar
            if (ViewModel.HasError && !string.IsNullOrEmpty(ViewModel.ErrorMessage))
            {
                ErrorNotificationBar.Message = ViewModel.ErrorMessage;
                ErrorNotificationBar.IsOpen = true;
            }
            else
            {
                ErrorNotificationBar.IsOpen = false;
            }
        }
    }
}
