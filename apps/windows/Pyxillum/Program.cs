using System;
using System.Threading;
using Microsoft.UI.Dispatching;
using Microsoft.UI.Xaml;
using WinRT;
using Pyxillum.Diagnostics;

namespace Pyxillum
{
    public static class Program
    {
        [STAThread]
        static void Main(string[] args)
        {
            ComWrappersSupport.InitializeComWrappers();

            for (int i = 0; i < args.Length; i++)
            {
                if (args[i] == "--selftest")
                {
                    AppConfig.IsSelftest = true;
                    if (i + 1 < args.Length && !args[i + 1].StartsWith("-"))
                    {
                        AppConfig.SelftestOutDir = args[i + 1];
                    }
                }
                else if (args[i] == "--sim")
                {
                    AppConfig.UseSimulated = true;
                }
                else if (args[i] == "--reset-window")
                {
                    AppConfig.ResetWindow = true;
                }
                else if (args[i] == "--icon-sheet")
                {
                    AppConfig.IconSheet = true;
                }
            }

            Logger.Info($"[Program] Launching Pyxillum Desktop. Selftest: {AppConfig.IsSelftest}, Sim: {AppConfig.UseSimulated}");

            Application.Start((p) =>
            {
                var context = new DispatcherQueueSynchronizationContext(DispatcherQueue.GetForCurrentThread());
                SynchronizationContext.SetSynchronizationContext(context);
                new App();
            });
        }
    }
}
