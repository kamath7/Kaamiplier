import java.util.ArrayList;
import java.util.Scanner;

public class Main {
    public static void main(String[] args) {
        Scanner scanner = new Scanner(System.in);
        ArrayList<Integer> numbers = new ArrayList<>();

        System.out.println("Welcome to ListApp!");
        System.out.println("Commands: add <num>, list, remove <num>, sum, exit");

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
                int val = Integer.parseInt(parts[1]);
                numbers.add(val);
                System.out.println("Added " + val);
            } else if (command.equals("remove") && parts.length > 1) {
                int val = Integer.parseInt(parts[1]);
                numbers.remove(Integer.valueOf(val));
                System.out.println("Removed " + val);
            } else if (command.equals("list")) {
                for (int n : numbers) {
                    System.out.println(n);
                }
            } else if (command.equals("sum")) {
                int sum = 0;
                for (int n : numbers) {
                    sum += n;
                }
                System.out.println(sum);
            } else {
                System.out.println("Unknown command.");
            }
        }
    }
}