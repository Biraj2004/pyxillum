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
                UpdateListVisibility();

                ScanButton.Click += (s, args) =>
                {
                    ScanRing.IsActive = true;
                    ScanRing.Visibility = Visibility.Visible;
                    ViewModel.Dispatch(new PyxillumAction.ScanDevices());
                };
            }
        }

        private void DiscoveredDevices_CollectionChanged(object? sender, NotifyCollectionChangedEventArgs e)
        {
            UpdateListVisibility();
        }

        private void UpdateListVisibility()
        {
            if (ViewModel != null && ViewModel.DiscoveredDevices.Count > 0)
            {
                EmptyStateCard.Visibility = Visibility.Collapsed;
                DeviceListView.Visibility = Visibility.Visible;
            }
            else
            {
                EmptyStateCard.Visibility = Visibility.Visible;
                DeviceListView.Visibility = Visibility.Collapsed;
            }
        }
    }
}
