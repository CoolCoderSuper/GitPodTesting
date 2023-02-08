package Java.Notes;

import java.util.*;

public class Main {
    public static void main(String[] args) {
        System.out.println("Notes App");
        var objScanner = new Scanner(System.in);
        var cont = true;
        while (cont){
            System.out.print("> ");
            var command = objScanner.nextLine();
            switch (command) {
                case "exit":
                    cont = false;
                    break;
                case "help":
                    System.out.println("list: lists all the available notes");
                    System.out.println("add: adds a new note");
                    System.out.println("remove: removes a note from the list");
                    System.out.println("exit: exits the program");
                    System.out.println("help: this list");
                    break;
                default:
                    System.out.println("Invalid command");
                    break;
            }
        }
        objScanner.close();
    }
}