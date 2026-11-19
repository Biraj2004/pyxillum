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

        public Action<int, int>? StatsCallback { get; set; }
        public bool IsSuspended { get; set; } = false;

        private const int MaxDiodes = 120;
        private int _activeDiodeCount = 60;

        private readonly Ellipse[] _outerGlow = new Ellipse[MaxDiodes];
        private readonly Ellipse[] _innerGlow = new Ellipse[MaxDiodes];
        private readonly Ellipse[] _coreDiodes = new Ellipse[MaxDiodes];

        private readonly SolidColorBrush[] _outerBrushes = new SolidColorBrush[MaxDiodes];
        private readonly SolidColorBrush[] _innerBrushes = new SolidColorBrush[MaxDiodes];
        private readonly SolidColorBrush[] _coreBrushes = new SolidColorBrush[MaxDiodes];
        private readonly SolidColorBrush[] _strokeBrushes = new SolidColorBrush[MaxDiodes];

        private readonly Color _offBodyColor = Color.FromArgb(255, 0x1F, 0x29, 0x37);
        private readonly Color _offStrokeColor = Color.FromArgb(100, 0x37, 0x41, 0x51);

        private DateTime _lastRenderTime = DateTime.MinValue;
        private DateTime _fpsWindowStart = DateTime.MinValue;
        private int _frameCount = 0;
        private int _currentFps = 60;
        private bool _isRenderingSubscribed = false;

        public LedStripMonitor()
        {
            this.Height = 40;
            this.HorizontalAlignment = HorizontalAlignment.Stretch;

            for (int i = 0; i < MaxDiodes; i++)
            {
                _outerBrushes[i] = new SolidColorBrush(Colors.Transparent);
                _innerBrushes[i] = new SolidColorBrush(Colors.Transparent);
                _coreBrushes[i] = new SolidColorBrush(_offBodyColor);
                _strokeBrushes[i] = new SolidColorBrush(_offStrokeColor);

                // Layer 1: Outer ambient glow (Soft, 0.12 opacity)
                _outerGlow[i] = new Ellipse
                {
                    Width = 24,
                    Height = 24,
                    Fill = _outerBrushes[i],
                    Visibility = Visibility.Collapsed
                };

                // Layer 2: Inner ambient glow (Medium, 0.35 opacity)
                _innerGlow[i] = new Ellipse
                {
                    Width = 16,
                    Height = 16,
                    Fill = _innerBrushes[i],
                    Visibility = Visibility.Collapsed
                };

                // Layer 3: Core circular physical diode
                _coreDiodes[i] = new Ellipse
                {
                    Width = 9,
                    Height = 9,
                    Fill = _coreBrushes[i],
                    Stroke = _strokeBrushes[i],
                    StrokeThickness = 1,
                    Visibility = Visibility.Visible
                };

                this.Children.Add(_outerGlow[i]);
                this.Children.Add(_innerGlow[i]);
                this.Children.Add(_coreDiodes[i]);
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

        public void StartRendering()
        {
            if (!_isRenderingSubscribed && this.Visibility == Visibility.Visible)
            {
                CompositionTarget.Rendering += OnCompositionRendering;
                _isRenderingSubscribed = true;
                _fpsWindowStart = DateTime.UtcNow;
                _frameCount = 0;
            }
        }

        public void StopRendering()
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
            if (width <= 0 || height <= 0) return;

            int count = Math.Clamp(_activeDiodeCount, 1, MaxDiodes);
            double step = width / (double)count;
            double centerY = height / 2.0;

            for (int i = 0; i < MaxDiodes; i++)
            {
                if (i < count)
                {
                    _coreDiodes[i].Visibility = Visibility.Visible;

                    double centerX = (i * step) + (step / 2.0);

                    Canvas.SetLeft(_outerGlow[i], centerX - 12);
                    Canvas.SetTop(_outerGlow[i], centerY - 12);

                    Canvas.SetLeft(_innerGlow[i], centerX - 8);
                    Canvas.SetTop(_innerGlow[i], centerY - 8);

                    Canvas.SetLeft(_coreDiodes[i], centerX - 4.5);
                    Canvas.SetTop(_coreDiodes[i], centerY - 4.5);
                }
                else
                {
                    _outerGlow[i].Visibility = Visibility.Collapsed;
                    _innerGlow[i].Visibility = Visibility.Collapsed;
                    _coreDiodes[i].Visibility = Visibility.Collapsed;
                }
            }
        }

        private void OnCompositionRendering(object? sender, object e)
        {
            if (IsSuspended || this.Visibility != Visibility.Visible || this.ActualWidth <= 0 || this.ActualHeight <= 0)
            {
                return;
            }

            var now = DateTime.UtcNow;
            if ((now - _lastRenderTime).TotalMilliseconds < 16) // ~60 FPS cap
            {
                return;
            }
            _lastRenderTime = now;

            _frameCount++;
            if ((now - _fpsWindowStart).TotalSeconds >= 1.0)
            {
                _currentFps = _frameCount;
                _frameCount = 0;
                _fpsWindowStart = now;
                StatsCallback?.Invoke(_activeDiodeCount, _currentFps);
            }

            if (FrameSource == null) return;
            var frame = FrameSource();
            if (frame == null || frame.Length == 0) return;

            int targetCount = Math.Clamp((int)frame.Length, 1, MaxDiodes);
            if (targetCount != _activeDiodeCount)
            {
                _activeDiodeCount = targetCount;
                LayoutDiodes(this.ActualWidth, this.ActualHeight);
            }

            for (int i = 0; i < _activeDiodeCount; i++)
            {
                int rIdx = i * 3;
                if (rIdx + 2 < frame.PixelsRgb.Length)
                {
                    byte r = frame.PixelsRgb[rIdx];
                    byte g = frame.PixelsRgb[rIdx + 1];
                    byte b = frame.PixelsRgb[rIdx + 2];

                    if (r == 0 && g == 0 && b == 0)
                    {
                        // LED is off
                        _outerGlow[i].Visibility = Visibility.Collapsed;
                        _innerGlow[i].Visibility = Visibility.Collapsed;
                        _coreBrushes[i].Color = _offBodyColor;
                        _strokeBrushes[i].Color = _offStrokeColor;
                    }
                    else
                    {
                        // LED is lit with ambient neon glow
                        _outerGlow[i].Visibility = Visibility.Visible;
                        _innerGlow[i].Visibility = Visibility.Visible;

                        _outerBrushes[i].Color = Color.FromArgb((byte)(255 * 0.12), r, g, b);
                        _innerBrushes[i].Color = Color.FromArgb((byte)(255 * 0.35), r, g, b);
                        _coreBrushes[i].Color = Color.FromArgb(255, r, g, b);
                        _strokeBrushes[i].Color = Color.FromArgb(180, 255, 255, 255);
                    }
                }
            }
        }
    }
}
