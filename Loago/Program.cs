using CommandLine;
using CommandLine.Text;
using Loago.Commands;

namespace Loago;

public static class Program
{
    public static async Task Main(string[] args)
    {
        Parser parser = new();

        static async Task ExecuteCommand(ICommand cmd)
        {
            await cmd.Run();
        }

        ParserResult<object> result = parser.ParseArguments<DoCommand, ViewCommand, RemoveCommand, HelpCommand>(args);
        HelpText? helpText = new()
        {
            AdditionalNewLineAfterOption = false,
            Heading = "Loago - How long ago program",
            Copyright = "Licensed under the MIT license.",
            AutoHelp = false,
            AutoVersion = false
        };
        helpText.AddVerbs(typeof(DoCommand), typeof(ViewCommand), typeof(RemoveCommand), typeof(HelpCommand));

        Task parseDo = result.WithParsedAsync<DoCommand>(ExecuteCommand);
        Task parseView = result.WithParsedAsync<ViewCommand>(ExecuteCommand);
        Task parseRemove = result.WithParsedAsync<RemoveCommand>(ExecuteCommand);
        Task parseHelp = result.WithParsedAsync<HelpCommand>(async command =>
        {
            HelpCommand.Help = helpText;
            await command.Run();
        });
        Task notParsed = result.WithNotParsedAsync(async errs =>
        {
            HelpCommand.Help = helpText;
            await new HelpCommand().Run();
        });
        try
        {
            // Asynchronous parsing of commands for minimal cpu usage, and maximum exception handling pain.
            // Though at least we properly handle errors, unlike the original program.
            await Task.WhenAll(parseDo, parseView, parseRemove, parseHelp);
        }
        // Using modern facilities to handle errors in a safe and modern way. 
        // This is a clear upgrade over Rust.
        catch (Exception ex)
        {
            await ErrorHandler.HandleError(ex);
        }
    }
}
