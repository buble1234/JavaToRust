public class Math {
    public int counter;
    public String message;
    public boolean flag;

    public void increment() {
        counter = counter + 1;
    }

    public String getMessage() {
        return "Счетчик: " + counter;
    }

    public void checkEven() {
        if (counter % 2 == 0) {
            System.out.println("Четное число: " + counter);
        } else {
            System.out.println("Нечетное число: " + counter);
        }
    }

    public String combineMessages() {
        return message + " - " + counter;
    }
}