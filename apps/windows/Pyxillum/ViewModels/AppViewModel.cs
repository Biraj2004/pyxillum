using System;
using System.Collections.ObjectModel;
using System.Threading.Tasks;
using CommunityToolkit.Mvvm.ComponentModel;
using Microsoft.UI.Dispatching;
using Pyxillum.Core;

namespace Pyxillum.ViewModels
{
    public partial class AppViewModel : ObservableObject, IStateListener
    {
        private readonly Engine _engine;
        private readonly DispatcherQueue _dispatcherQueue;

        [ObservableProperty]
        private ScreenId _currentScreen = ScreenId.DeviceHub;

        [ObservableProperty]
        private ConnectionStatus _connection = new ConnectionStatus(ConnectionStatusKind.Disconnected);

        [ObservableProperty]
        private bool _isBluetoothAvailable = true;

        [ObservableProperty]
        private bool _isPreviewMode = false;

        [ObservableProperty]
        private string? _errorMessage = null;

        [ObservableProperty]
        private bool _hasError = false;

        [ObservableProperty]
        private SolidPageState _solidState = new SolidPageState();

        [ObservableProperty]
        private DynamicPageState _dynamicState = new DynamicPageState();

        [ObservableProperty]
        private MusicPageState _musicState = new MusicPageState();

        [ObservableProperty]
        private DiyPageState _diyState = new DiyPageState();

        [ObservableProperty]
        private StripConfig _stripConfig = new StripConfig();

        [ObservableProperty]
        private float[] _audioSpectrum = new float[24];

        [ObservableProperty]
        private bool _safetyConfirmed = false;

        public ObservableCollection<DiscoveredDevice> DiscoveredDevices { get; } = new();

        public AppViewModel()
        {
            _dispatcherQueue = DispatcherQueue.GetForCurrentThread();

            // Initialise shared core engine (starts in simulated mode if hardware BLE unavailable)
            _engine = new Engine(useSimulated: false);
            _engine.Subscribe(this);

            // Fetch initial immutable state snapshot
            ApplyState(_engine.State);
        }

        public void Dispatch(PyxillumAction action)
        {
            Task.Run(() => _engine.Dispatch(action));
        }

        public StripFrame? GetCurrentFrame()
        {
            return _engine.Frame();
        }

        public void OnStateChanged(AppState newState)
        {
            // Marshal immutable state snapshot updates to UI thread via DispatcherQueue
            _dispatcherQueue.TryEnqueue(() => ApplyState(newState));
        }

        private void ApplyState(AppState state)
        {
            CurrentScreen = state.CurrentScreen;
            Connection = state.Connection;
            IsBluetoothAvailable = state.IsBluetoothAvailable;
            IsPreviewMode = state.IsPreviewMode;
            SolidState = state.SolidPage;
            DynamicState = state.DynamicPage;
            MusicState = state.MusicPage;
            DiyState = state.DiyPage;
            StripConfig = state.StripConfig;

            if (state.ActiveError != null)
            {
                ErrorMessage = state.ActiveError.Message;
                HasError = true;
            }
            else
            {
                ErrorMessage = null;
                HasError = false;
            }

            // Sync discovered devices collection
            DiscoveredDevices.Clear();
            foreach (var dev in state.DiscoveredDevices)
            {
                DiscoveredDevices.Add(dev);
            }
        }

        public void OpenBluetoothSettings()
        {
            _engine.Services.OpenBluetoothSettings();
        }

        public void DismissError()
        {
            Dispatch(new PyxillumAction.DismissError());
        }

        public void TogglePower()
        {
            Dispatch(new PyxillumAction.TogglePower());
        }

        public void SetBrightness(byte brightness)
        {
            Dispatch(new PyxillumAction.SetBrightness(brightness));
        }

        public void SetSpeed(byte speed)
        {
            Dispatch(new PyxillumAction.SetSpeed(speed));
        }

        public void Navigate(ScreenId screen)
        {
            Dispatch(new PyxillumAction.Navigate(screen));
        }
    }
}
