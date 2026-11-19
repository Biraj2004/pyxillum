using System;
using Microsoft.UI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Shapes;
using Windows.UI;
using Pyxillum.Core;

namespace Pyxillum.Controls
{
    public class LedStripMonitor : Canvas
    {
        public static readonly DependencyProperty FrameSourceProperty =
            DependencyProperty.Register(
                nameof(FrameSource),
                typeof(Func<StripFrame?>),
                typeof(LedStripMonitor),
                new PropertyMetadata(null));

        public Func<StripFrame?>? FrameSource
        {
            get => (Func<StripFrame?>?)GetValue(FrameSourceProperty);
            set => SetValue(FrameSourceProperty, value);
        }

        private readonly Rectangle[] _diodes = new Rectangle[60];
        private readonly SolidColorBrush[] _brushes = new SolidColorBrush[60];
        private DateTime _lastRenderTime = DateTime.MinValue;
        private bool _isRenderingSubscribed = false;

        public LedStripMonitor()
        {
            this.Height = 28;
            this.HorizontalAlignment = HorizontalAlignment.Stretch;

            for (int i = 0; i < _diodes.Length; i++)
            {
                _brushes[i] = new SolidColorBrush(Colors.Black);
                _diodes[i] = new Rectangle
                {
                    Width = 6,
                    Height = 12,
                    RadiusX = 2,
                    RadiusY = 2,
                    Fill = _brushes[i]
                };
                this.Children.Add(_diodes[i]);
            }

            this.Loaded += OnLoaded;
            this.Unloaded += OnUnloaded;
            this.SizeChanged += OnSizeChanged;
        }

        private void OnLoaded(object sender, RoutedEventArgs e)
        {
            StartRendering();
        }

        private void OnUnloaded(object sender, RoutedEventArgs e)
        {
            StopRendering();
        }

        private void StartRendering()
        {
            if (!_isRenderingSubscribed && this.Visibility == Visibility.Visible)
            {
                CompositionTarget.Rendering += OnCompositionRendering;
                _isRenderingSubscribed = true;
            }
        }

        private void StopRendering()
        {
            if (_isRenderingSubscribed)
            {
                CompositionTarget.Rendering -= OnCompositionRendering;
                _isRenderingSubscribed = false;
            }
        }

        private void OnSizeChanged(object sender, SizeChangedEventArgs e)
        {
            LayoutDiodes(e.NewSize.Width, e.NewSize.Height);
        }

        private void LayoutDiodes(double width, double height)
        {
            if (width <= 0) return;

            double diodeWidth = 6;
            double diodeHeight = 12;
            double top = (height - diodeHeight) / 2.0;
            double step = width / (double)_diodes.Length;

            for (int i = 0; i < _diodes.Length; i++)
            {
                double left = (i * step) + ((step - diodeWidth) / 2.0);
                Canvas.SetLeft(_diodes[i], left);
                Canvas.SetTop(_diodes[i], top);
            }
        }

        private void OnCompositionRendering(object? sender, object e)
        {
            // Cap rendering to 60 FPS (16.6ms threshold)
            var now = DateTime.UtcNow;
            if ((now - _lastRenderTime).TotalMilliseconds < 16)
            {
                return;
            }
            _lastRenderTime = now;

            if (FrameSource == null) return;
            var frame = FrameSource();
            if (frame == null || frame.Length == 0) return;

            int pixelCount = Math.Min(frame.Length, _diodes.Length);
            for (int i = 0; i < pixelCount; i++)
            {
                int rIdx = i * 3;
                if (rIdx + 2 < frame.PixelsRgb.Length)
                {
                    byte r = frame.PixelsRgb[rIdx];
                    byte g = frame.PixelsRgb[rIdx + 1];
                    byte b = frame.PixelsRgb[rIdx + 2];
                    _brushes[i].Color = Color.FromArgb(255, r, g, b);
                }
            }
        }
    }
}
