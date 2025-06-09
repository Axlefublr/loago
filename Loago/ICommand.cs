namespace Loago.Commands;

/// <summary>
///     Model for asynchronously running command.
/// </summary>
public interface ICommand
{
    public Task Run();
}