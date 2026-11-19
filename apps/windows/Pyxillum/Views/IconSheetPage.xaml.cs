using System.Collections.Generic;
using Microsoft.UI.Xaml.Controls;

namespace Pyxillum.Views
{
    public class IconItem
    {
        public string Name { get; set; } = "";
        public string Unicode { get; set; } = "";
        public string Glyph { get; set; } = "";
        public string Description { get; set; } = "";
    }

    public partial class IconSheetPage : Page
    {
        public IconSheetPage()
        {
            this.InitializeComponent();

            var icons = new List<IconItem>
            {
                new() { Name = "icon.device", Unicode = "\\uE702", Glyph = "\uE702", Description = "Bluetooth Radio" },
                new() { Name = "icon.dynamic", Unicode = "\\uE706", Glyph = "\uE706", Description = "Sparkles / Flare" },
                new() { Name = "icon.solid", Unicode = "\\uE790", Glyph = "\uE790", Description = "Paint Palette" },
                new() { Name = "icon.music", Unicode = "\\uEC4F", Glyph = "\uEC4F", Description = "Music Note" },
                new() { Name = "icon.diy", Unicode = "\\uE81C", Glyph = "\uE81C", Description = "Timeline / Sliders" },
                new() { Name = "icon.settings", Unicode = "\\uE713", Glyph = "\uE713", Description = "Settings Gear" },
                new() { Name = "icon.power", Unicode = "\\uE7E8", Glyph = "\uE7E8", Description = "Power Button" },
                new() { Name = "icon.brightness", Unicode = "\\uE706", Glyph = "\uE706", Description = "Brightness Flare" },
                new() { Name = "icon.refresh", Unicode = "\\uE72C", Glyph = "\uE72C", Description = "Refresh Clockwise" },
                new() { Name = "icon.search", Unicode = "\\uE721", Glyph = "\uE721", Description = "Search Lens" },
                new() { Name = "icon.play", Unicode = "\\uE768", Glyph = "\uE768", Description = "Play Fill" },
                new() { Name = "icon.pause", Unicode = "\\uE769", Glyph = "\uE769", Description = "Pause Fill" },
                new() { Name = "icon.waveform", Unicode = "\\uE95B", Glyph = "\uE95B", Description = "Audio Waveform" },
                new() { Name = "icon.warning", Unicode = "\\uE7BA", Glyph = "\uE7BA", Description = "Warning Triangle" },
                new() { Name = "icon.error", Unicode = "\\uEA39", Glyph = "\uEA39", Description = "Error Badge" },
                new() { Name = "icon.check", Unicode = "\\uE73E", Glyph = "\uE73E", Description = "Check Mark" }
            };

            IconGridView.ItemsSource = icons;
        }
    }
}
