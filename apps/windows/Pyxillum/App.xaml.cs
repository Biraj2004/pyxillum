using System;
using Microsoft.UI.Xaml;

namespace Pyxillum
{
    public partial class App : Application
    {
        private Window? _mainWindow;

        public static Window? MainWindowInstance { get; private set; }

        public App()
        {
            this.InitializeComponent();
            this.UnhandledException += App_UnhandledException;
        }

        private void App_UnhandledException(object sender, Microsoft.UI.Xaml.UnhandledExceptionEventArgs e)
        {
            // Prevent silent termination on background thread dispatch errors
            System.Diagnostics.Debug.WriteLine($"[Pyxillum App Exception] {e.Message}\n{e.Exception}");
            e.Handled = true;
        }

        protected override void OnLaunched(Microsoft.UI.Xaml.LaunchActivatedEventArgs args)
        {
            _mainWindow = new MainWindow();
            MainWindowInstance = _mainWindow;
            _mainWindow.Activate();
        }
    }
}
