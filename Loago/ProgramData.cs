namespace Loago;


/// <summary>
/// Global data related to program.
/// Added support for a config file for maintainability purposes, even though it's never used by the program.
/// </summary>
public static class ProgramData
{
    // This doesn't configure anything.
    public const string ConfigFileName = "config.json";
    public const string DataFileName = "data.json";

    public static string AppDataPath =>
        Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.ApplicationData), "loago");


    /// <summary>
    /// Gets array of paths to search for data file.
    /// </summary>
    /// <returns>Array of paths</returns>
    public static string[] DataFileSearchLocations()
    {
        return [Path.Combine(Directory.GetCurrentDirectory(), DataFileName), Path.Combine(AppDataPath, DataFileName)];
    }

    /// <summary>
    /// Gets array of paths to search for config file.
    /// Config file currently not used.
    /// </summary>
    /// <returns>Array of paths</returns>
    public static string[] ConfigFileSearchLocations()
    {
        return
        [
            Path.Combine(Directory.GetCurrentDirectory(), ConfigFileName), Path.Combine(AppDataPath, ConfigFileName)
        ];
    }

    /// <summary>
    /// Attempts to find database file
    /// </summary>
    /// <returns>Path to database file, or null if not found</returns>
    public static string? FindDataFile()
    {
        foreach (string file in DataFileSearchLocations())
            if (File.Exists(file))
                return file;
        return null;
    }


    /// <summary>
    /// Attempts to find config file
    /// </summary>
    /// <returns>Path to config file, or null if not found</returns>
    public static string? FindConfigFile()
    {
        foreach (string file in ConfigFileSearchLocations())
            if (File.Exists(file))
                return file;
        return null;
    }

    /// <summary>
    /// Creates config directory at default appdata path.
    /// </summary>
    public static void CreateConfigDirectory()
    {
        Directory.CreateDirectory(AppDataPath);
    }
    /// <summary>
    /// Creates empty data and config files at default appdata path.
    /// </summary>
    public static void CreateConfigFiles()
    {
        using StreamWriter stream1 = File.CreateText(Path.Combine(AppDataPath, DataFileName));
        using StreamWriter stream2 = File.CreateText(Path.Combine(AppDataPath, ConfigFileName));
    }
}