import java.util.ArrayList;

public class ListManager {
    private ArrayList<Integer> numbers = new ArrayList<>();

    public void add(int val) {
        numbers.add(val);
        System.out.println("Added " + val);
    }

    public void remove(int val) {
        numbers.remove(Integer.valueOf(val));
        System.out.println("Removed " + val);
    }

    public void printList() {
        for (int n : numbers) {
            System.out.println(n);
        }
    }

    public void printSum() {
        int sum = 0;
        for (int n : numbers) {
            sum += n;
        }
        System.out.println(sum);
    }
}
}
