// See https://aka.ms/new-console-template for more information
var dataPath = "notes.json";
while (true){
    
}

public class Note
{
    public int Id {get;set;}

    public string Value {get;set;}

    public static List<Note> GetNotes()
    {
        return new List<Note>();
    }

    public static void Save()
    {

    }
}