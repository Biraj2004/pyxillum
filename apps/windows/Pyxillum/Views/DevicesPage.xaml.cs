using System;
using System.Collections.Specialized;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Navigation;
using Pyxillum.Core;
using Pyxillum.ViewModels;

namespace Pyxillum.Views
{
    public partial class DevicesPage : Page
    {
        public AppViewModel? ViewModel { get; private set; }

        public DevicesPage()
        {
            this.InitializeComponent();
        }

        protected override void OnNavigatedTo(NavigationEventArgs e)
        {
            base.OnNavigatedTo(e);
            if (e.Parameter is AppViewModel vm)
            {
                ViewModel = vm;
                DeviceListView.ItemsSource = ViewModel.DiscoveredDevices;
                ViewModel.DiscoveredDevices.CollectionChanged += DiscoveredDevices_CollectionChanged;
                ViewModel.PropertyChanged += ViewModel_PropertyChanged;
                UpdateListVisibility();

                ScanButton.Click += (s, args) => TriggerScan();
                EmptyScanButton.Click += (s, args) => TriggerScan();
                WizardButton.Click += async (s, args) =>
                {
                    if (ViewModel == null) return;
                    var dialog = new HardwareWizardDialog(ViewModel);
                    dialog.XamlRoot = this.XamlRoot;
                    await dialog.ShowAsync();
                };
                ErrorRetryButton.Click += (s, args) =>
                {
                    PageErrorBar.IsOpen = false;
                    ViewModel?.DismissError();
                    TriggerScan();
                };
            }
        }

        private void ConnectButton_Click(object sender, RoutedEventArgs e)
        {
            if (sender is Button btn && btn.Tag is string address && ViewModel != null)
            {
                LoadingStatusText.Text = $"Connecting to {address}...";
                LoadingStateCard.Visibility = Visibility.Visible;
                DeviceListView.Visibility = Visibility.Collapsed;
                EmptyStateCard.Visibility = Visibility.Collapsed;
                PageErrorBar.IsOpen = false;
                ViewModel.ConnectDevice(address);
            }
        }

        private void TriggerScan()
        {
            if (ViewModel == null) return;
            ScanRing.IsActive = true;
            ScanRing.Visibility = Visibility.Visible;
            LoadingStatusText.Text = "Scanning for BLE devices...";
            LoadingStateCard.Visibility = Visibility.Visible;
            EmptyStateCard.Visibility = Visibility.Collapsed;
            DeviceListView.Visibility = Visibility.Collapsed;
            PageErrorBar.IsOpen = false;

            ViewModel.StartScan();
        }

        private void ViewModel_PropertyChanged(object? sender, System.ComponentModel.PropertyChangedEventArgs e)
        {
            if (e.PropertyName == nameof(AppViewModel.Connection) ||
                e.PropertyName == nameof(AppViewModel.HasError) ||
                e.PropertyName == nameof(AppViewModel.ErrorMessage))
            {
                UpdateListVisibility();
            }
        }

        private void DiscoveredDevices_CollectionChanged(object? sender, NotifyCollectionChangedEventArgs e)
        {
            UpdateListVisibility();
        }

        private void UpdateListVisibility()
        {
            if (ViewModel == null) return;

            // Update scanning / connecting state cards
            switch (ViewModel.Connection.Kind)
            {
                case ConnectionStatusKind.Scanning:
                    ScanRing.IsActive = true;
                    ScanRing.Visibility = Visibility.Visible;
                    LoadingStatusText.Text = "Scanning for BLE devices...";
                    LoadingStateCard.Visibility = Visibility.Visible;
                    EmptyStateCard.Visibility = Visibility.Collapsed;
                    DeviceListView.Visibility = Visibility.Collapsed;
                    PageErrorBar.IsOpen = false;
                    return;

                case ConnectionStatusKind.Connecting:
                    ScanRing.IsActive = false;
                    ScanRing.Visibility = Visibility.Collapsed;
                    LoadingStatusText.Text = string.IsNullOrEmpty(ViewModel.Connection.Name)
                        ? "Connecting to controller..."
                        : $"Connecting to {ViewModel.Connection.Name}...";
                    LoadingStateCard.Visibility = Visibility.Visible;
                    EmptyStateCard.Visibility = Visibility.Collapsed;
                    DeviceListView.Visibility = Visibility.Collapsed;
                    PageErrorBar.IsOpen = false;
                    return;

                default:
                    ScanRing.IsActive = false;
                    ScanRing.Visibility = Visibility.Collapsed;
                    LoadingStateCard.Visibility = Visibility.Collapsed;
                    break;
            }

            // Error display with localized string resolution
            if (ViewModel.HasError && !string.IsNullOrEmpty(ViewModel.ErrorMessage))
            {
                var errorInfo = Diagnostics.StringCatalog.ResolveErrorInfo(ViewModel.ErrorKey, ViewModel.ErrorMessage);
                PageErrorBar.Title = errorInfo.Title;
                PageErrorBar.Message = errorInfo.Message;
                ErrorRetryButton.Content = errorInfo.ActionLabel;
                PageErrorBar.IsOpen = true;
            }
            else
            {
                PageErrorBar.IsOpen = false;
            }

            // Controllers list or empty state
            if (ViewModel.DiscoveredDevices.Count > 0)
            {
                EmptyStateCard.Visibility = Visibility.Collapsed;
                DeviceListView.Visibility = Visibility.Visible;
            }
            else
            {
                DeviceListView.Visibility = Visibility.Collapsed;
                EmptyStateCard.Visibility = Visibility.Visible;
            }
        }
    }
}
