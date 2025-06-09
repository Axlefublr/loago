using System.Text.Json;
using System.Text.Json.Serialization;

namespace Loago;

/// <summary>
///     Class to model work item.
/// </summary>
public class WorkItem
{
    [JsonConstructor]
    public WorkItem(string name, DateTime? lastCompletion = null)
    {
        Name = name;
        LastCompletion = lastCompletion;
    }

    /// <summary>
    ///     Work represented by work item.
    /// </summary>
    public string Name { get; }

    /// <summary>
    ///     Time of last completion of work item. Null if unknown.
    /// </summary>
    public DateTime? LastCompletion { get; private set; }

    /// <summary>
    ///     Loads work items from path asynchronously
    /// </summary>
    /// <param name="path"> Path to json database </param>
    /// <returns>List of loaded work items</returns>
    /// <exception cref="FileNotFoundException"> Specified file does not exist. </exception>
    public static async Task<List<WorkItem>?> LoadWorkItems(string path)
    {
        if (!File.Exists(path)) throw new FileNotFoundException(path);

        await using FileStream openStream = File.OpenRead(path);
        return await JsonSerializer.DeserializeAsync
            (openStream, SourceGenerationContext.Default.ListWorkItem);
    }

    /// <summary>
    ///     Saves work items to json file at path asynchronously.
    /// </summary>
    /// <param name="path"> Path of json file </param>
    /// <param name="workItems"> List of work items to be saved </param>
    public static async Task SaveWorkItems(string path, List<WorkItem> workItems)
    {
        await using FileStream openStream = File.OpenWrite(path);
        openStream.SetLength(0);
        await JsonSerializer.SerializeAsync(openStream, workItems, SourceGenerationContext.Default.ListWorkItem);
    }

    /// <summary>
    ///     Updates last completion time of work item, since LastCompletion setter is private.
    /// </summary>
    public void UpdateLastCompletion()
    {
        LastCompletion = DateTime.Now;
    }

    // For testing purposes
    public static async Task DumpTestWorkItem(string path)
    {
        if (!File.Exists(path)) throw new FileNotFoundException(path);
        List<WorkItem> testItem = [new("Work to be done", DateTime.Now)];
        string serialized = JsonSerializer.Serialize(testItem, SourceGenerationContext.Default.ListWorkItem);
        await File.WriteAllTextAsync(path, serialized);
    }
}

// To enable json serialization/deserialization without reflection for native AOT compilation
[JsonSourceGenerationOptions(WriteIndented = true)]
[JsonSerializable(typeof(List<WorkItem>))]
[JsonSerializable(typeof(WorkItem))]
internal partial class SourceGenerationContext : JsonSerializerContext
{
}