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
                ErrorRetryButton.Click += (s, args) => TriggerScan();
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

            ViewModel.Dispatch(new PyxillumAction.ScanDevices());
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

            ScanRing.IsActive = false;
            ScanRing.Visibility = Visibility.Collapsed;

            if (ViewModel.HasError && !string.IsNullOrEmpty(ViewModel.ErrorMessage))
            {
                PageErrorBar.Message = ViewModel.ErrorMessage;
                PageErrorBar.IsOpen = true;
            }
            else
            {
                PageErrorBar.IsOpen = false;
            }

            if (ViewModel.DiscoveredDevices.Count > 0)
            {
                EmptyStateCard.Visibility = Visibility.Collapsed;
                LoadingStateCard.Visibility = Visibility.Collapsed;
                DeviceListView.Visibility = Visibility.Visible;
            }
            else
            {
                DeviceListView.Visibility = Visibility.Collapsed;
                LoadingStateCard.Visibility = Visibility.Collapsed;
                EmptyStateCard.Visibility = Visibility.Visible;
            }
        }
    }
}
