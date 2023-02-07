import Person.*;

import java.time.*;
import java.util.*;
import java.time.format.DateTimeFormatter;

public class Main {
    static boolean testMode = false;
    static boolean javaCool = true;
    static boolean vbCool = true;
    static boolean csCool = true;
    static boolean vbTheBest = false;
    static boolean personCool;
    static boolean personSuperCool;
    static boolean listTestMode = false;
    static boolean hashMapTestMode = false;
    static boolean hashSetTestMode = true;

    public static void main(String[] args) {
        if (hashSetTestMode){
            HashSet<String> set = new HashSet<String>();

        }
        if (hashMapTestMode){
            HashMap<String, String> map = new HashMap<String, String>();
            map.put("joe", "Joseph");
            map.put("bob", "Coder");
            System.out.println(map);
            System.out.println(map.size());
            for (String valString : map.values()) {
                System.out.println(valString);
            }
            for (String keyString : map.keySet()) {
                System.out.println(String.format("Key: %s, Value: %s", keyString, map.get(keyString)));
            }
            var name = map.get("joe");
            System.out.println(name);
            map.remove("joe");
            System.out.println(map);
            map.put("joe", "Humus");
            System.out.println(map);
            map.put("joe", "Hello");
            System.out.println(map);
            map.putIfAbsent("joe", "Bye");
            System.out.println(map);
        }
        if (listTestMode){
            ArrayList<String> names = new ArrayList<String>();
            names.add("Bob");
            names.add("Joe");
            names.add("Rob");
            System.out.println(names);
            System.out.println(names.get(0));
            names.set(0, "Bill");
            System.out.println(names.get(0));
            System.out.println("loop");
            for (String n : names){
                System.out.println(n);
            }
            names.remove("Rob");
            names.remove(0);
            System.out.println(names);
            System.out.println(names.size());
            names.clear();
        }
        if (testMode) {
            // print to console
            System.out.println("Welcome to Java!");
            // math operations
            System.out.println("4 * 5 / 4 % 34 + 1 - 23 * -12 ^ 4 = " + (4 * 5 / 4 % 34 + 1 - 23 * -12 ^ 4));
            // math class for more complex operations
            System.out.println("Square Root: " + Math.sqrt(25));
            System.out.println("Exponent: " + Math.pow(2, 2));
            // type casting
            int cool = 3;
            double superCool = (double) cool;
            System.out.println("Type Cast: " + superCool);
            // if statements
            if (vbCool && vbTheBest) {
                personCool = true;
                personSuperCool = true;
            } else if (javaCool || vbCool || csCool) {
                personCool = true;
                personSuperCool = false;
            } else {
                personCool = false;
                personSuperCool = false;
            }
            System.out.println("Person.Person Cool: " + personCool);
            System.out.println("Person.Person Super Cool: " + personSuperCool);
            // switch
            int age = 11;
            switch (age) {
                case 10:
                    System.out.println(1);
                    break;
                case 20:
                    System.out.println(2);
                    break;
                default:
                    System.out.println("Unknown");
            }
            // while
            int c = 0;
            while (c < 5) {
                System.out.println("Waiting...");
                c++;
                sleep(1000);
            }
            // for loop
            String[] first_names = { "Bob", "Joe", "Rob" };
            boolean skip = false;
            for (int i = 0; i < first_names.length; i++) {
                if (skip)
                    continue;
                System.out.println(first_names[i]);
            }
            // for each loop
            String[] last_names = { "Munch", "Guenther", "Martin" };
            boolean exit = false;
            for (String last_name : last_names) {
                System.out.println(last_name);
                if (exit)
                    break;
            }
            // date time
            System.out.println(GetTimeString());
            System.out.println(GetDateString());
            System.out.println(GetDateTimeString());
            LocalDateTime time = LocalDateTime.now();
            DateTimeFormatter format = DateTimeFormatter.ofPattern("dd-MM-yyyy HH:mm:ss");
            System.out.println(format.format(time));
            // classes
            Person person = new Person("Joe", "Guenther", 10);
            person.Description = "Cool guy!";
            person.Print();
            System.out.println("Potential Full: " + person.GetFullName(true));
            person.Birthday();
            person.Print();
            SuperCoolPerson superCoolPerson = new SuperCoolPerson();
            superCoolPerson.Hello();
            // interfaces
            Animal dog = new Dog();
            dog.sound();
            Dog dog1 = new Dog();
            dog1.Level = FerocityLevel.HIGH;
            System.out.println(dog1.Level);
            System.out.println(dog1.Level.default_val());
            // user input
            Scanner objScanner = new Scanner(System.in);
            System.out.println("Enter name: ");
            String name = objScanner.nextLine();
            System.out.println("Hello " + name);
            objScanner.close();
        }
    }

    private static void sleep(long millis) {
        try {
            Thread.sleep(millis);
        } catch (InterruptedException ex) {

        }
    }

    private static String GetTimeString() {
        return LocalTime.now().toString();
    }

    private static String GetDateString() {
        return LocalDate.now().toString();
    }

    private static String GetDateTimeString() {
        return LocalDateTime.now().toString();
    }

}