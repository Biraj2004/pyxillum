namespace Pyxillum.Diagnostics
{
    public static class AppConfig
    {
        public static bool IsSelftest { get; set; } = false;
        public static string SelftestOutDir { get; set; } = "dist/selftest_screenshots";
        public static bool UseSimulated { get; set; } = false;
        public static bool ResetWindow { get; set; } = false;
        public static bool IconSheet { get; set; } = false;
    }
}
