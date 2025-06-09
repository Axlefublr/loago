using CommandLine;
using CommandLine.Text;
using Loago.Commands;
using static Loago.ProgramData;

namespace Loago;

/// <summary>
///     Implementation of available commands.
/// </summary>
[Verb("do", aliases: ["add", "new", "update", "reset"], HelpText = "Update last time of doing task.")]
public class DoCommand : ICommand
{
    public DoCommand()
    {
        WorkName = null;
    }
    public DoCommand(string workName = "")
    {
        WorkName = workName;
    }

    // We support only doing one bit of work at a time, to prevent user from excessively lying about their work,
    // since it's physically impossible to do two things at the same time.
    [Value(0, Required = false)] public string? WorkName { get; }

    public async Task Run()
    {
        // Deal with missing arguments here, so that we can EXPLICITLY handle the error while maintaining knowledge
        // of which command is missing the argument.
        if (WorkName == null) throw new MissingArgumentException("do");
        string? dataLocation = FindDataFile();

        // Be nice to user and create the database file if it doesn't exist.
        if (dataLocation == null)
        {
            Logger.Log($"No data file found, creating one now, at {AppDataPath}");
            CreateConfigDirectory();
            CreateConfigFiles();
            dataLocation = FindDataFile()!;
            // asynchronous IO to not waste CPU resources, unlike original code,
            // which wastes both runtime and compile time.
            await File.WriteAllTextAsync(dataLocation, "[ \n ]");
        }

        List<WorkItem> loadedItems = await WorkItem.LoadWorkItems(dataLocation) ?? [];
        WorkItem? matchingItem = loadedItems.FirstOrDefault(wi => wi.Name == WorkName);

        // Inform user whether we're updating or creating for a more interactive user experience.
        if (matchingItem == null)
        {
            DateTime now = DateTime.Now;
            loadedItems.Add(new WorkItem(WorkName, now));
            Console.WriteLine($"Added work item {WorkName} at time {now}.");
        }
        else
        {
            matchingItem.UpdateLastCompletion();
            Console.WriteLine($"Updated work item {WorkName} at time {DateTime.Now}.");
        }

        await WorkItem.SaveWorkItems(dataLocation, loadedItems);
    }
}

[Verb("view", aliases: ["list", "look", "see"],
    HelpText = "Check last time tasks have been done. Optionally provide list of tasks")]
public class ViewCommand : ICommand
{
    public ViewCommand()
    {
        DesiredItems = [];
    }

    public ViewCommand(IEnumerable<string> desiredItems)
    {
        DesiredItems = desiredItems;
    }

    [Value(0)] public IEnumerable<string> DesiredItems { get; }

    public async Task Run()
    {
        if (!DesiredItems.Any())
            await ShowAll();
        else
            await ShowDesired();
    }

    // Static since we don't care about desired items if we're listing all of them.
    private static async Task ShowAll()
    {
        string? dataLocation = FindDataFile();
        if (dataLocation == null) throw new NoDataBaseException(DataFileSearchLocations());
        IEnumerable<WorkItem> loadedItems = (await WorkItem.LoadWorkItems(dataLocation) ?? [])
            .OrderByDescending(item => item.LastCompletion);
        foreach (WorkItem item in loadedItems)
        {
            TimeSpan elapsedTime = DateTime.Now - (item.LastCompletion ?? DateTime.MinValue);
            Console.WriteLine(
                $"{item.Name} last done {elapsedTime.Days} days, {elapsedTime.Hours} hours, {elapsedTime.Minutes} minutes, {elapsedTime.Seconds} seconds ago.");
        }
    }

    private async Task ShowDesired()
    {
        string? dataLocation = FindDataFile();
        if (dataLocation == null) throw new NoDataBaseException(DataFileSearchLocations());

        // LINQ for more data-oriented design, at this point Rust is clearly lacking features.
        IEnumerable<WorkItem> loadedItems = (await WorkItem.LoadWorkItems(dataLocation) ?? [])
            .OrderByDescending(item => item.LastCompletion);

        IEnumerable<WorkItem> query =
            from WorkItem workItem in loadedItems
            join string itemName in DesiredItems on workItem.Name equals itemName
            select workItem;

        List<string> notFoundDesiredItems = DesiredItems.ToList();

        foreach (WorkItem item in query)
        {
            // More precise measurement of time since last completion to support better
            // understanding different kinds of humanly obligations and avoid losing unnecessary braincells.
            TimeSpan elapsedTime = DateTime.Now - (item.LastCompletion ?? DateTime.MinValue);
            Console.WriteLine(
                $"{item.Name} last done {elapsedTime.Days} days, {elapsedTime.Hours} hours, {elapsedTime.Minutes} minutes, {elapsedTime.Seconds} seconds ago.");
            notFoundDesiredItems.Remove(item.Name);
        }

        // More transparency to user when requested item is not found, unlike original program.
        foreach (string item in notFoundDesiredItems) Console.WriteLine($"Couldn't find work item {item} in database.");
    }
}

[Verb("remove", aliases: ["delete"], HelpText = "Remove task or list of tasks from database")]
public class RemoveCommand : ICommand
{
    public RemoveCommand(IEnumerable<string> workNames)
    {
        WorkNames = workNames;
    }

    [Value(0, Required = false)] public IEnumerable<string> WorkNames { get; }

    public async Task Run()
    {
        // Deal with missing arguments here, so that we can EXPLICITLY handle the error while maintaining knowledge
        // of which command is missing the argument.
        if (!WorkNames.Any()) throw new MissingArgumentException("remove");

        string? dataLocation = FindDataFile();
        if (dataLocation == null)
        {
            // Remove is not supposed to error if it fails.
            Logger.Log("No data file found, nothing removed.");
            return;
        }

        // Asynchronous loading of work items to avoid wasting CPU time, unlike original program.
        List<WorkItem> loadedItems = await WorkItem.LoadWorkItems(dataLocation) ?? [];
        foreach (string workName in WorkNames)
        {
            WorkItem? matchingItem = loadedItems.FirstOrDefault(wi => wi.Name == workName);
            // Inform user whether we're actually removing.
            if (matchingItem == null)
            {
                Logger.Log($"Cannot remove {workName}, as it is not in the database.");
            }
            else
            {
                loadedItems.Remove(matchingItem);
                Console.WriteLine($"Removed work item {matchingItem.Name}.");
            }
        }

        await WorkItem.SaveWorkItems(dataLocation, loadedItems);
    }
}

[Verb("help", aliases: ["--help", "anything else"], HelpText = "Show this help screen.")]
public class HelpCommand : ICommand
{
    public static HelpText Help { get; set; } = new();

    public async Task Run()
    {
        await Task.CompletedTask; // To shut up compiler.
        Console.WriteLine(Help);
    }
}
