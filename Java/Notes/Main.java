package Java.Notes;

import java.util.*;

public class Main {
    public static void main(String[] args) {
        System.out.println("Notes App");
        var objScanner = new Scanner(System.in);
        var cont = true;
        var notes = new ArrayList<Note>();
        while (cont) {
            System.out.print("> ");
            var command = objScanner.nextLine();
            if (command == "exit") {
                cont = false;
            } else if (command == "help") {
                System.out.println("list: lists all the available notes");
                System.out.println("add: adds a new note");
                System.out.println("remove: removes a note from the list");
                System.out.println("exit: exits the program");
                System.out.println("help: this list");
            } else if (command == "list") {
                System.out.println(notes);
            } else if (command.startsWith("add")) {
                var note = command;
                notes.add(new Note(notes.stream().max(Comparator.comparing(v -> v.Id)).get().Id, note));
            } else if (command == "remove") {

            } else {
                System.out.println("Invalid command");
            }
        }
        objScanner.close();
    }
}