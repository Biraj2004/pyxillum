using System;
using System.Threading.Tasks;
using Microsoft.UI.Xaml;
using Pyxillum.Diagnostics;

namespace Pyxillum
{
    public partial class App : Application
    {
        private MainWindow? _mainWindow;

        public static MainWindow? MainWindowInstance { get; private set; }

        public App()
        {
            this.InitializeComponent();

            // 1. Application-level unhandled exception handler
            this.UnhandledException += App_UnhandledException;

            // 2. AppDomain unhandled exception handler
            AppDomain.CurrentDomain.UnhandledException += CurrentDomain_UnhandledException;

            // 3. TaskScheduler unobserved task exception handler
            TaskScheduler.UnobservedTaskException += TaskScheduler_UnobservedTaskException;

            // 4. Hook XAML binding failure diagnostics
            this.DebugSettings.BindingFailed += DebugSettings_BindingFailed;
        }

        private void App_UnhandledException(object sender, Microsoft.UI.Xaml.UnhandledExceptionEventArgs e)
        {
            Logger.Error($"[Application Unhandled Exception] {e.Message}", e.Exception);
            e.Handled = true;

            ShowGlobalError(e.Message, e.Exception?.ToString() ?? e.Message);
        }

        private void CurrentDomain_UnhandledException(object sender, System.UnhandledExceptionEventArgs e)
        {
            var ex = e.ExceptionObject as Exception;
            string msg = ex?.Message ?? "Uncaught application domain exception";
            Logger.Error($"[AppDomain Unhandled Exception] {msg}", ex);

            ShowGlobalError(msg, ex?.ToString() ?? msg);
        }

        private void TaskScheduler_UnobservedTaskException(object? sender, UnobservedTaskExceptionEventArgs e)
        {
            Logger.Error($"[TaskScheduler Unobserved Exception] {e.Exception.Message}", e.Exception);
            e.SetObserved();

            ShowGlobalError(e.Exception.Message, e.Exception.ToString());
        }

        private void DebugSettings_BindingFailed(object? sender, BindingFailedEventArgs e)
        {
            string msg = $"[XAML Binding Error] {e.Message}";
            Logger.Warn(msg);
            SelftestRunner.RecordBindingError(e.Message);
        }

        private void ShowGlobalError(string message, string details)
        {
            try
            {
                MainWindowInstance?.DispatcherQueue.TryEnqueue(() =>
                {
                    MainWindowInstance.DisplayGlobalError(message, details);
                });
            }
            catch
            {
                // Fallback if dispatcher is torn down
            }
        }

        protected override async void OnLaunched(Microsoft.UI.Xaml.LaunchActivatedEventArgs args)
        {
            _mainWindow = new MainWindow();
            MainWindowInstance = _mainWindow;
            _mainWindow.Activate();

            if (AppConfig.IsSelftest)
            {
                Logger.Info("[App] Launching in --selftest mode");
                int exitCode = await SelftestRunner.RunAsync(_mainWindow, AppConfig.SelftestOutDir);
                Environment.Exit(exitCode);
            }
        }
    }
}
