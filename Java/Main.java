import Person.Person;
import Person.Animal;
import Person.Dog;
import Person.FerocityLevel;
import Person.SuperCoolPerson;
import java.time.LocalTime;
import java.util.Scanner;

public class Main {
    static boolean javaCool = true;
    static  boolean vbCool = true;
    static   boolean csCool = true;
    static  boolean vbTheBest = false;
    static boolean personCool;
    static boolean personSuperCool;

    public static void main(String[] args) {
        //print to console
        System.out.println("Welcome to Java!");
        System.out.println("Enter your name: ");
        //math operations
        System.out.println("4 * 5 / 4 % 34 + 1 - 23 * -12 ^ 4 = " + (4 * 5 / 4 % 34 + 1 - 23 * -12 ^ 4));
        //math class for more complex operations
        System.out.println("Square Root: " + Math.sqrt(25));
        System.out.println("Exponent: " + Math.pow(2,2));
        //type casting
        int cool = 3;
        double superCool = (double) cool;
        System.out.println("Type Cast: " + superCool);
        //if statements
         if (vbCool && vbTheBest){
            personCool = true;
            personSuperCool = true;
        }
       else if(javaCool || vbCool || csCool){
            personCool = true;
            personSuperCool = false;
        }else{
           personCool = false;
           personSuperCool = false;
        }
       System.out.println("Person.Person Cool: " + personCool);
       System.out.println("Person.Person Super Cool: " + personSuperCool);
       //switch
        int age = 11;
        switch (age){
            case 10:
                System.out.println(1);
                break;
            case 20:
                System.out.println(2);
                break;
            default:
                System.out.println("Unknown");
        }
        //while
        int c = 0;
        while (c < 5){
             System.out.println("Waiting...");
             c++;
             sleep(1000);
        }
        //for loop
        String[] first_names = {"Bob","Joe","Rob"};
        boolean skip = false;
        for(int i = 0; i < first_names.length; i++){
            if (skip) continue;
            System.out.println(first_names[i]);
        }
        //for each loop
        String[] last_names = {"Munch","Guenther","Martin"};
        boolean exit = false;
        for(String last_name:last_names){
            System.out.println(last_name);
            if (exit) break;
        }
        //function
        System.out.println(GetTimeString());
        //classes
        Person person = new Person("Joe","Guenther",10);
        person.Description = "Cool guy!";
        person.Print();
        System.out.println("Potential Full: " + person.GetFullName(true));
        person.Birthday();
        person.Print();
        SuperCoolPerson superCoolPerson = new SuperCoolPerson();
        superCoolPerson.Hello();
        //interfaces
        Animal dog = new Dog();
        dog.sound();
        Dog dog1 = new Dog();
        dog1.Level = FerocityLevel.HIGH;
        System.out.println(dog1.Level);
        System.out.println(dog1.Level.default_val());
        //user input
        Scanner objScanner = new Scanner(System.in);
        System.out.println("Enter name: ");
        String name = objScanner.nextLine();
        System.out.println("Hello " + name);
        objScanner.close();
    }

    private static void sleep(long millis){
        try{
            Thread.sleep(millis);
        }catch (InterruptedException ex){

        }
    }

    private static String GetTimeString(){
        return LocalTime.now().toString();
    }

}