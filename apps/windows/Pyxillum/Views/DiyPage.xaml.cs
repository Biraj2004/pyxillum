using System.Collections.Generic;
using Microsoft.UI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Navigation;
using Microsoft.UI.Xaml.Shapes;
using Pyxillum.Core;
using Pyxillum.ViewModels;
using Windows.UI;

namespace Pyxillum.Views
{
    public partial class DiyPage : Page
    {
        public AppViewModel? ViewModel { get; private set; }

        private readonly List<Button> _stepButtons = new();
        private int _selectedStepIndex = 0;

        public DiyPage()
        {
            this.InitializeComponent();

            for (int i = 0; i < 6; i++)
            {
                AddNodeButton(i);
            }
        }

        private void AddNodeButton(int index)
        {
            var btn = new Button
            {
                Content = $"#{index + 1}",
                Width = 50,
                Height = 50,
                CornerRadius = new CornerRadius(8),
                Margin = new Thickness(4)
            };
            int captureIdx = index;
            btn.Click += (s, e) => SelectStep(captureIdx);
            _stepButtons.Add(btn);
            TimelineNodesPanel.Children.Add(btn);
        }

        private void SelectStep(int index)
        {
            _selectedStepIndex = index;
            for (int i = 0; i < _stepButtons.Count; i++)
            {
                _stepButtons[i].BorderThickness = (i == index) ? new Thickness(2) : new Thickness(1);
            }
        }

        protected override void OnNavigatedTo(NavigationEventArgs e)
        {
            base.OnNavigatedTo(e);
            if (e.Parameter is AppViewModel vm)
            {
                ViewModel = vm;

                PlayPauseButton.Click += (s, args) =>
                {
                    ViewModel.Dispatch(new PyxillumAction.ToggleDiyPreview());
                };

                DiySpeedSlider.ValueChanged += (s, args) =>
                {
                    ViewModel.Dispatch(new PyxillumAction.SetSpeed((byte)args.NewValue));
                };

                DiySwatchRed.Click += (s, args) => ApplyStepColor(255, 0, 0);
                DiySwatchGreen.Click += (s, args) => ApplyStepColor(0, 255, 0);
                DiySwatchBlue.Click += (s, args) => ApplyStepColor(0, 0, 255);
                DiySwatchAmber.Click += (s, args) => ApplyStepColor(255, 120, 0);
                DiySwatchCyan.Click += (s, args) => ApplyStepColor(0, 255, 255);
                DiySwatchMagenta.Click += (s, args) => ApplyStepColor(255, 0, 255);

                AddStepButton.Click += (s, args) =>
                {
                    if (_stepButtons.Count < 14)
                    {
                        AddNodeButton(_stepButtons.Count);
                    }
                };

                RemoveStepButton.Click += (s, args) =>
                {
                    if (_stepButtons.Count > 1)
                    {
                        var last = _stepButtons[_stepButtons.Count - 1];
                        _stepButtons.RemoveAt(_stepButtons.Count - 1);
                        TimelineNodesPanel.Children.Remove(last);
                        if (_selectedStepIndex >= _stepButtons.Count)
                        {
                            SelectStep(_stepButtons.Count - 1);
                        }
                    }
                };

                DiyRetryButton.Click += (s, args) =>
                {
                    DiyErrorBar.IsOpen = false;
                    ViewModel.Dispatch(new PyxillumAction.ToggleDiyPreview());
                };

                EmptyAddStepButton.Click += (s, args) =>
                {
                    AddNodeButton(_stepButtons.Count);
                    DiyEmptyCard.Visibility = Visibility.Collapsed;
                };
            }
        }

        private void ApplyStepColor(byte r, byte g, byte b)
        {
            DiyColorPreview.Fill = new SolidColorBrush(Color.FromArgb(255, r, g, b));
            ViewModel?.Dispatch(new PyxillumAction.UpdateDiyStep((byte)_selectedStepIndex, r, g, b));
        }
    }
}
