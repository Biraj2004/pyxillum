using Microsoft.UI;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Navigation;
using Pyxillum.Core;
using Pyxillum.ViewModels;
using Windows.UI;

namespace Pyxillum.Views
{
    public partial class SolidPage : Page
    {
        public AppViewModel? ViewModel { get; private set; }

        public SolidPage()
        {
            this.InitializeComponent();
        }

        protected override void OnNavigatedTo(NavigationEventArgs e)
        {
            base.OnNavigatedTo(e);
            if (e.Parameter is AppViewModel vm)
            {
                ViewModel = vm;

                RedSlider.ValueChanged += (s, args) => UpdateCustomColor();
                GreenSlider.ValueChanged += (s, args) => UpdateCustomColor();
                BlueSlider.ValueChanged += (s, args) => UpdateCustomColor();

                KelvinSlider.ValueChanged += (s, args) =>
                {
                    uint k = (uint)args.NewValue;
                    KelvinReadout.Text = $"{k:N0} K";
                    ViewModel.Dispatch(new PyxillumAction.SetColorTemperature(k));
                };

                SwatchAmber.Click += (s, args) => SetSwatchColor(255, 120, 0);
                SwatchWarm.Click += (s, args) => SetSwatchColor(255, 200, 140);
                SwatchRed.Click += (s, args) => SetSwatchColor(255, 0, 0);
                SwatchGreen.Click += (s, args) => SetSwatchColor(0, 255, 0);
                SwatchBlue.Click += (s, args) => SetSwatchColor(0, 0, 255);
                SwatchCyan.Click += (s, args) => SetSwatchColor(0, 255, 255);
                SwatchMagenta.Click += (s, args) => SetSwatchColor(255, 0, 255);
            }
        }

        private void SetSwatchColor(byte r, byte g, byte b)
        {
            RedSlider.Value = r;
            GreenSlider.Value = g;
            BlueSlider.Value = b;
            UpdateCustomColor();
        }

        private void UpdateCustomColor()
        {
            byte r = (byte)RedSlider.Value;
            byte g = (byte)GreenSlider.Value;
            byte b = (byte)BlueSlider.Value;

            ActiveColorDisc.Fill = new SolidColorBrush(Color.FromArgb(255, r, g, b));
            HexCodeBox.Text = $"#{r:X2}{g:X2}{b:X2}";

            ViewModel?.Dispatch(new PyxillumAction.SetSolidColor(r, g, b));
        }
    }
}
