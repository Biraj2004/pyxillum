using System.Collections.Generic;
using Microsoft.UI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Navigation;
using Microsoft.UI.Xaml.Shapes;
using Pyxillum.Core;
using Pyxillum.ViewModels;

namespace Pyxillum.Views
{
    public class MusicModeItem
    {
        public byte Code { get; set; }
        public string Name { get; set; } = string.Empty;
        public string CodeText => $"Mode {Code}";
    }

    public partial class MusicPage : Page
    {
        public AppViewModel? ViewModel { get; private set; }

        private readonly Rectangle[] _spectrumBars = new Rectangle[24];
        private readonly List<MusicModeItem> _modes = new();

        public MusicPage()
        {
            this.InitializeComponent();

            for (int i = 0; i < 24; i++)
            {
                _spectrumBars[i] = new Rectangle
                {
                    Width = 8,
                    Height = 20,
                    Fill = new SolidColorBrush(Colors.Cyan),
                    RadiusX = 2,
                    RadiusY = 2
                };
                SpectrumCanvas.Children.Add(_spectrumBars[i]);
            }

            for (byte i = 1; i <= 18; i++)
            {
                _modes.Add(new MusicModeItem { Code = i, Name = $"Acoustic Track {i}" });
            }
            MusicModesGrid.ItemsSource = _modes;

            SpectrumCanvas.SizeChanged += (s, e) => LayoutSpectrum(e.NewSize.Width, e.NewSize.Height);
        }

        private void LayoutSpectrum(double width, double height)
        {
            if (width <= 0) return;
            double step = width / 24.0;
            for (int i = 0; i < 24; i++)
            {
                Canvas.SetLeft(_spectrumBars[i], i * step + (step - 8) / 2.0);
                Canvas.SetTop(_spectrumBars[i], height - _spectrumBars[i].Height);
            }
        }

        protected override void OnNavigatedTo(NavigationEventArgs e)
        {
            base.OnNavigatedTo(e);
            if (e.Parameter is AppViewModel vm)
            {
                ViewModel = vm;

                AudioSourceCombo.SelectionChanged += (s, args) =>
                {
                    var source = AudioSourceCombo.SelectedIndex switch
                    {
                        1 => AudioInput.InternalMic,
                        2 => AudioInput.ExternalMic,
                        _ => AudioInput.Player,
                    };
                    ViewModel.Dispatch(new PyxillumAction.SelectAudioInput(source));
                };

                SensitivitySlider.ValueChanged += (s, args) =>
                {
                    ViewModel.Dispatch(new PyxillumAction.SetSensitivity((byte)args.NewValue));
                };

                MusicModesGrid.SelectionChanged += (s, args) =>
                {
                    if (MusicModesGrid.SelectedItem is MusicModeItem item)
                    {
                        ViewModel.Dispatch(new PyxillumAction.SelectMusicMode(item.Code));
                    }
                };
            }
        }
    }
}
