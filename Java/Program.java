//test java app
package Java;
public class Program
{
    public static void main(String[] args) {
        String message = GetMessage("Bob");
        System.out.println(message);
    }

    public static String GetMessage(String name) {
        return "Hello, " + name;
    }
}
