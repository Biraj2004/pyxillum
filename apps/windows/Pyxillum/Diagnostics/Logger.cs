using System;
using System.IO;
using System.Text;

namespace Pyxillum.Diagnostics
{
    public enum LogLevel
    {
        Debug,
        Info,
        Warn,
        Error
    }

    public static class Logger
    {
        private static readonly object _syncLock = new();
        private static string? _logDirectory;

        public static string LogDirectory
        {
            get
            {
                if (_logDirectory == null)
                {
                    string localAppData = Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData);
                    _logDirectory = Path.Combine(localAppData, "Pyxillum", "logs");
                    try
                    {
                        Directory.CreateDirectory(_logDirectory);
                    }
                    catch
                    {
                        // Fallback to temp directory if LocalAppData is inaccessible
                        _logDirectory = Path.Combine(Path.GetTempPath(), "Pyxillum", "logs");
                        Directory.CreateDirectory(_logDirectory);
                    }
                }
                return _logDirectory;
            }
        }

        public static void Debug(string message) => Log(LogLevel.Debug, message);
        public static void Info(string message) => Log(LogLevel.Info, message);
        public static void Warn(string message) => Log(LogLevel.Warn, message);
        public static void Error(string message, Exception? ex = null) => Log(LogLevel.Error, message, ex);

        public static void Log(LogLevel level, string message, Exception? ex = null)
        {
            string timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss.fff");
            int threadId = Environment.CurrentManagedThreadId;
            string levelTag = level.ToString().ToUpperInvariant().PadRight(5);

            var sb = new StringBuilder();
            sb.Append($"[{timestamp}] [{levelTag}] [Thread:{threadId}] {message}");

            if (ex != null)
            {
                sb.AppendLine();
                sb.Append($"Exception: {ex.GetType().FullName}: {ex.Message}");
                sb.AppendLine();
                sb.Append(ex.StackTrace);

                var inner = ex.InnerException;
                while (inner != null)
                {
                    sb.AppendLine();
                    sb.Append($"Inner Exception: {inner.GetType().FullName}: {inner.Message}");
                    sb.AppendLine();
                    sb.Append(inner.StackTrace);
                    inner = inner.InnerException;
                }
            }

            string logEntry = sb.ToString();

            System.Diagnostics.Debug.WriteLine(logEntry);

            try
            {
                lock (_syncLock)
                {
                    string fileName = $"pyxillum-{DateTime.Now:yyyy-MM-dd}.log";
                    string filePath = Path.Combine(LogDirectory, fileName);
                    File.AppendAllText(filePath, logEntry + Environment.NewLine, Encoding.UTF8);
                }
            }
            catch
            {
                // Never crash the application if logging file I/O fails
            }
        }

        public static void OpenLogDirectory()
        {
            try
            {
                if (Directory.Exists(LogDirectory))
                {
                    System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
                    {
                        FileName = "explorer.exe",
                        Arguments = $"\"{LogDirectory}\"",
                        UseShellExecute = true
                    });
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to open log folder: {ex.Message}");
            }
        }
    }
}
