using System.Text.Json;

namespace Loago;

public class NoDataBaseException : Exception
{
    public NoDataBaseException(string[] searchedPaths) : base("Couldn't load work items, since database wasn't found. ")
    {
        SearchedPaths = searchedPaths;
    }

    public string[] SearchedPaths { get; }
}

// Unfortunately the command line argument parser's error handling
// doesn't specify which command is missing an argument. To make error-handling
// more specific, we track which command is missing arguments.
public class MissingArgumentException : Exception
{
    public MissingArgumentException(string command) : base($"Missing argument to command {command}.")
    {
        Command = command;
    }

    public string Command { get; }
}

// Centralized error handling static class to cleanly deal with errors, unlike original program,
// which completely forgets to handle errors at all. Frankly, this is an accomplishment in Rust.
public static class ErrorHandler
{
    // Interactive error handling for best UX
    private static async Task TryCreateNewDatabaseInteractive()
    {
        Console.WriteLine(
            "Would you like to create a backup of the current database and create a new blank one? (Y/n)");
        char c = Console.ReadKey().KeyChar;
        if (c == 'y')
        {
            Console.WriteLine();
            string? dataBaseFile = ProgramData.FindDataFile();
            if (dataBaseFile == null) return;
            try
            {
                File.Move(dataBaseFile, dataBaseFile + ".bak");
            }
            catch (IOException)
            {
                Console.WriteLine("Could not create backup, aborting.");
                return;
            }

            ProgramData.CreateConfigFiles();
            await WorkItem.SaveWorkItems(dataBaseFile, []);
            Console.WriteLine("Created new blank database.");
        }
        else
        {
            Console.WriteLine();
            Console.WriteLine("Operation aborted.");
        }
    }
    
    // Interactive error handling for best UX
    private static async Task TryCreateDatabaseInteractive()
    {
        Console.WriteLine("Would you like to create a database at the default location? (Y/n)");
        char c = Console.ReadKey().KeyChar;
        if (c == 'y')
        {
            Console.WriteLine();
            ProgramData.CreateConfigFiles();
            await WorkItem.SaveWorkItems(ProgramData.FindDataFile()!, []);
            Console.WriteLine("Created new blank database.");
        }
        else
        {
            Console.WriteLine();
            Console.WriteLine("Operation aborted.");
        }
    }

    // Functional-style omega-match... sorry, switch, error handling.
    // We report failed error code only on unrecoverable and/or unforeseen errors,
    // unlike original code, which mostl<y doesn't report anything.
    public static async Task HandleError(Exception error)
    {
        switch (error)
        {
            case AggregateException aggregate:
            {
                List<Task> handleTasks = new();
                foreach (Exception inner in aggregate.InnerExceptions)
                    handleTasks.Add(HandleError(inner));
                await Task.WhenAll(handleTasks);
                break;
            }
            case JsonException jex:
            {
                Console.WriteLine("Cannot deserialize JSON, likely corrupted database.");
                Console.WriteLine("Details:");
                Console.WriteLine(jex.Message);
                await TryCreateNewDatabaseInteractive();
                break;
            }
            case PlatformNotSupportedException pnex:
            {
                Console.WriteLine("Platform not supported.");
                Console.WriteLine($"Details: {pnex.Message}");
                Console.WriteLine("Exiting.");
                Environment.Exit(-1);
                break;
            }
            case UnauthorizedAccessException uaex:
            {
                Console.WriteLine("Cannot access database file due to missing permissions.");
                Console.WriteLine($"Details: {uaex.Message}");
                Console.WriteLine("Exiting.");
                Environment.Exit(-1);
                break;
            }
            case NoDataBaseException ndbex:
            {
                Console.WriteLine("Cannot load work items, database wasn't found.");
                Console.WriteLine($"Searched paths: {ndbex.SearchedPaths}");
                await TryCreateDatabaseInteractive();
                break;
            }
            case MissingArgumentException maex:
            {
                Console.WriteLine(
                    $"You forgot an argument to your command {maex.Command}. If you need help, pass --help.");
                break;
            }
            default:
            {
                Console.WriteLine("Unknown error encountered.");
                Console.WriteLine($"Error: {error.GetType()}");
                Console.WriteLine($"Details:  {error.Message}");
                Console.WriteLine("Aborting.");
                Environment.Exit(-1);
                break;
            }
        }
    }
}