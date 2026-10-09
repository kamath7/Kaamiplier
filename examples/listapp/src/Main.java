import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        ListManager manager = new ListManager();

        System.out.println("Welcome to the Multi-File ListApp!");
        
        while (true) {
            System.out.print("\n> ");
            String input = scanner.nextLine().trim();
            if (input.isEmpty()) continue;

            String[] parts = input.split(" ");
            String command = parts[0].toLowerCase();

            if (command.equals("exit")) {
                System.out.println("Goodbye!");
                break;
            } else if (command.equals("add") && parts.length > 1) {
                manager.add(Integer.parseInt(parts[1]));
            } else if (command.equals("remove") && parts.length > 1) {
                manager.remove(Integer.parseInt(parts[1]));
            } else if (command.equals("list")) {
                manager.printList();
            } else if (command.equals("sum")) {
                manager.printSum();
            } else {
                System.out.println("Unknown command.");
            }
        }
    }
}