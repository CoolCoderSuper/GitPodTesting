using Newtonsoft.Json;
public class Note
{
    public int Id { get; set; }
    public string Content { get; set; } = "";

    public static List<Note> FromJson(string json) => JsonConvert.DeserializeObject<List<Note>>(json);

    public static string ToJson(List<Note> notes) => JsonConvert.SerializeObject(notes);
}