package Person;

import java.time.LocalDate;
//use final to prevent inheritance
public class Person {
    public String FullName;
   public String FirstName;
   public String LastName;
   public int Age;
   public String Description;
   public final LocalDate Met = LocalDate.now();
    private int oldAge;
    public Person(){

    }
    public Person(String firstName,String lastName, int age){
        Age = age;
        FirstName = firstName;
        LastName = lastName;
    }
    public void Birthday(){
        oldAge = Age;
       Age++;
    }
    public String GetFullName(boolean populate){
        String name = FirstName + " " + LastName;
        if (populate){
            FullName = name;
        }
        return name;
    }
    public static Person GetDefault(){
        return new Person();
    }

    public static Person GetDefault(String firstName,String lastName,int age){
        Person person = new Person(firstName,lastName,age);
        person.GetFullName(true);
        return person;
    }
    public void Print(){
        System.out.println("First: " + FirstName + "\nLast: " + LastName + "\nFull: " + FullName + "\nAge: " + Age + "\nMet: " + Met.toString());
    }

    private int AddAges(){
        return oldAge + Age;
    }

    public void Hello(){
        System.out.println("Hi");
    }
}
