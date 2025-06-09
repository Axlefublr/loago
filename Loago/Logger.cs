namespace Loago;

public static class Logger
{
    /// <summary>
    /// Whether logging is enabled. Currently unused, included for maintainability and future-proofing reasons,
    /// if a less verbose program is desired.
    /// </summary>
    public static bool ShouldLog { get; set; } = true;

    /// <summary>
    /// Logs message to stdout if configured to do so.
    /// </summary>
    /// <param name="message">Message to log</param>
    public static void Log(string message)
    {
        if (ShouldLog) Console.WriteLine(message);
    }
}