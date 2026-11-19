using System.Collections.Generic;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Navigation;
using Pyxillum.Core;
using Pyxillum.ViewModels;

namespace Pyxillum.Views
{
    public class EffectItem
    {
        public byte Code { get; set; }
        public string Name { get; set; } = string.Empty;
        public string Category { get; set; } = string.Empty;
    }

    public partial class DynamicPage : Page
    {
        public AppViewModel? ViewModel { get; private set; }

        private readonly List<EffectItem> _allEffects = new();

        public DynamicPage()
        {
            this.InitializeComponent();

            // Populate representative effect catalogue samples
            _allEffects.Add(new EffectItem { Code = 1, Name = "Rainbow Flow", Category = "Rainbow" });
            _allEffects.Add(new EffectItem { Code = 2, Name = "Prismatic Cycle", Category = "Rainbow" });
            _allEffects.Add(new EffectItem { Code = 3, Name = "Blazing Embers", Category = "Fire" });
            _allEffects.Add(new EffectItem { Code = 4, Name = "Campfire Flicker", Category = "Fire" });
            _allEffects.Add(new EffectItem { Code = 5, Name = "Ocean Wave", Category = "Water" });
            _allEffects.Add(new EffectItem { Code = 6, Name = "Deep Sea Pulse", Category = "Water" });
            _allEffects.Add(new EffectItem { Code = 7, Name = "Cosmic Starlight", Category = "Stars" });
            _allEffects.Add(new EffectItem { Code = 8, Name = "Shooting Meteor", Category = "Chase" });
            _allEffects.Add(new EffectItem { Code = 9, Name = "Ambient Aurora", Category = "Ambient" });
            _allEffects.Add(new EffectItem { Code = 10, Name = "Neon Scanner", Category = "Chase" });

            EffectsGridView.ItemsSource = _allEffects;
        }

        protected override void OnNavigatedTo(NavigationEventArgs e)
        {
            base.OnNavigatedTo(e);
            if (e.Parameter is AppViewModel vm)
            {
                ViewModel = vm;

                DynamicSpeedSlider.ValueChanged += (s, args) =>
                {
                    ViewModel.Dispatch(new PyxillumAction.SetSpeed((byte)args.NewValue));
                };

                DynamicBrightnessSlider.ValueChanged += (s, args) =>
                {
                    ViewModel.Dispatch(new PyxillumAction.SetBrightness((byte)args.NewValue));
                };

                AutoCycleToggle.Toggled += (s, args) =>
                {
                    ViewModel.Dispatch(new PyxillumAction.ToggleAutoCycle());
                };

                EffectsGridView.SelectionChanged += (s, args) =>
                {
                    if (EffectsGridView.SelectedItem is EffectItem selected)
                    {
                        ViewModel.Dispatch(new PyxillumAction.SelectDynamicEffect(selected.Code));
                    }
                };
            }
        }
    }
}
