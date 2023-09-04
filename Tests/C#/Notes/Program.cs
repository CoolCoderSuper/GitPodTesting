Console.WriteLine("Notes");

if (!File.Exists("notes.json"))
{
    File.WriteAllText("notes.json", "[]");
}

var status = true;
var notes = Note.FromJson(File.ReadAllText("notes.json"));
while (status)
{
    Console.Write("> ");
    var input = Console.ReadLine();
    if (input == "exit")
    {
        status = false;
    }
    else if (input.StartsWith("add"))
    {
        var content = input.Split(" ")[1];
        var note = new Note
        {
            Id = notes.Count + 1,
            Content = content
        };
        notes.Add(note);
        File.WriteAllText("notes.json", Note.ToJson(notes));
    }
    else if (input == "list")
    {
        foreach (var note in notes)
        {
            Console.WriteLine($"{note.Id}: {note.Content}");
        }
    }
    else if (input.StartsWith("remove"))
    {
        var id = int.Parse(input.Split(" ")[1]);
        notes.RemoveAll(note => note.Id == id);
        File.WriteAllText("notes.json", Note.ToJson(notes));
    }
    else if (input == "help")
    {
        Console.WriteLine("add <content> - Add a note");
        Console.WriteLine("list - List all notes");
        Console.WriteLine("remove <id> - Remove a note");
        Console.WriteLine("exit - Exit the program");
    }
    else
    {
        Console.WriteLine($"Inavlid command: {input}");
    }
}