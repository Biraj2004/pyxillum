using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Navigation;
using Pyxillum.Core;
using Pyxillum.ViewModels;

namespace Pyxillum.Views
{
    public partial class SettingsPage : Page
    {
        public AppViewModel? ViewModel { get; private set; }

        public SettingsPage()
        {
            this.InitializeComponent();
        }

        protected override void OnNavigatedTo(NavigationEventArgs e)
        {
            base.OnNavigatedTo(e);
            if (e.Parameter is AppViewModel vm)
            {
                ViewModel = vm;

                SafetyConfirmCheckBox.Checked += (s, args) =>
                {
                    ApplyHardwareButton.IsEnabled = true;
                };

                SafetyConfirmCheckBox.Unchecked += (s, args) =>
                {
                    ApplyHardwareButton.IsEnabled = false;
                };

                PixelCountSlider.ValueChanged += (s, args) =>
                {
                    PixelCountNumberBox.Value = args.NewValue;
                };

                PixelCountNumberBox.ValueChanged += (s, args) =>
                {
                    if (!double.IsNaN(args.NewValue))
                    {
                        PixelCountSlider.Value = args.NewValue;
                    }
                };

                ApplyHardwareButton.Click += (s, args) =>
                {
                    if (SafetyConfirmCheckBox.IsChecked == true)
                    {
                        ushort pixelCount = (ushort)PixelCountSlider.Value;
                        var chipOrder = ChipOrderCombo.SelectedIndex switch
                        {
                            0 => ChipOrder.Rgb,
                            1 => ChipOrder.Rbg,
                            2 => ChipOrder.Grb,
                            3 => ChipOrder.Gbr,
                            4 => ChipOrder.Brg,
                            5 => ChipOrder.Bgr,
                            _ => ChipOrder.Grb
                        };

                        ViewModel.Dispatch(new PyxillumAction.ConfigureStrip(
                            pixelCount,
                            chipOrder,
                            safetyConfirmed: true
                        ));
                    }
                };

                SettingsRetryButton.Click += (s, args) =>
                {
                    SettingsErrorBar.IsOpen = false;
                };

                GoToDevicesButton.Click += (s, args) =>
                {
                    ViewModel.Navigate(ScreenId.DeviceHub);
                    Frame.Navigate(typeof(DevicesPage), ViewModel);
                };

                ResetGeometryButton.Click += (s, args) =>
                {
                    Diagnostics.WindowStateManager.ResetToDefault();
                };
            }
        }
    }
}
