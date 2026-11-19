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

        private AppWindow? _appWindow;

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

            // Connect Live LED Strip Monitor frame pulling
            LiveStripMonitor.FrameSource = () => ViewModel.GetCurrentFrame();

            // Wire UI Event Listeners
            MasterPowerButton.Click += (s, e) => ViewModel.TogglePower();
            MasterBrightnessSlider.ValueChanged += (s, e) => ViewModel.SetBrightness((byte)e.NewValue);
            OpenBluetoothSettingsButton.Click += (s, e) => ViewModel.OpenBluetoothSettings();
            ErrorNotificationBar.Closed += (s, e) => ViewModel.DismissError();

            NavView.ItemInvoked += NavView_ItemInvoked;
            NavView.SelectedItem = NavView.MenuItems[0];

            // Observe ViewModel changes for Shell Badges
            ViewModel.PropertyChanged += ViewModel_PropertyChanged;
            UpdateShellState();
        }

        private void ConfigureWindowGeometry()
        {
            var hwnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
            var windowId = Microsoft.UI.Win32Interop.GetWindowIdFromWindow(hwnd);
            _appWindow = AppWindow.GetFromWindowId(windowId);

            if (_appWindow != null)
            {
                var displayArea = DisplayArea.GetFromWindowId(windowId, DisplayAreaFallback.Primary);
                if (displayArea != null)
                {
                    var workArea = displayArea.WorkArea;
                    int targetWidth = (int)(workArea.Width * 0.80);
                    int targetHeight = (int)(workArea.Height * 0.80);
                    int posX = workArea.X + (workArea.Width - targetWidth) / 2;
                    int posY = workArea.Y + (workArea.Height - targetHeight) / 2;

                    _appWindow.MoveAndResize(new RectInt32(posX, posY, targetWidth, targetHeight));
                }
            }
        }

        private void NavView_ItemInvoked(NavigationView sender, NavigationViewItemInvokedEventArgs args)
        {
            if (args.InvokedItemContainer is NavigationViewItem item && item.Tag is string tag)
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
                }
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
                    ConnectionIndicator.Fill = (Brush)Application.Current.Resources["BrushSuccess"];
                    ConnectionStatusText.Text = ViewModel.Connection.Name ?? "Connected";
                    break;
                case ConnectionStatusKind.Connecting:
                    ConnectionIndicator.Fill = (Brush)Application.Current.Resources["BrushInfo"];
                    ConnectionStatusText.Text = "Connecting...";
                    break;
                case ConnectionStatusKind.Scanning:
                    ConnectionIndicator.Fill = (Brush)Application.Current.Resources["BrushInfo"];
                    ConnectionStatusText.Text = "Scanning...";
                    break;
                default:
                    ConnectionIndicator.Fill = (Brush)Application.Current.Resources["BrushWarning"];
                    ConnectionStatusText.Text = "Disconnected";
                    break;
            }

            // Preview Mode Badge
            PreviewBadge.Visibility = ViewModel.IsPreviewMode ? Visibility.Visible : Visibility.Collapsed;

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
