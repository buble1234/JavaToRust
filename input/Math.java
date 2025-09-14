package demo;

public class Math {
    public static final String APP = "App";
    public static final int N = 4;

    public int counter;
    public String message;
    public boolean flag;

    public static String hello(String who) {
        return "hello " + who;
    }

    public static int sumVar(int... xs) {
        int s = 0;
        for (int v : xs) s += v;
        return s;
    }

    public void increment() {
        counter = counter + 1;
    }

    public String getMessage() {
        return "Счетчик: " + counter;
    }

    public void checkEven() {
        if (counter % 2 == 0) {
            System.out.println("Четное: " + counter);
        } else {
            System.out.println("Нечетное: " + counter);
        }
    }

    public String combineMessages() {
        return message + " - " + counter;
    }

    public enum Color {
        RED, GREEN, BLUE
    }

    public static void main(String[] args) {
        int a = 10, b = 5;
        int sum = a + b;
        System.out.println("sum = " + sum);

        for (int i = 0; i < N; i++) {
            System.out.println("for i = " + i);
        }

        int j = 0;
        while (j < 3) {
            System.out.println("while j = " + j);
            j = j + 1;
        }

        int[] arr = new int[3];
        arr = 2;
        arr[1] = 4;
        arr[2] = 6;
        for (int t = 0; t < arr.length; t++) {
            System.out.println("arr[" + t + "] = " + arr[t]);
        }

        Math m = new Math();
        m.counter = 2;
        m.message = "hello";
        System.out.println("greet = " + m.getMessage());
        m.checkEven();
        System.out.println("combine = " + m.combineMessages());

        System.out.println("static hello bare = " + hello("neo"));
        System.out.println("static hello via class = " + Math.hello("neo"));
        System.out.println("static hello via obj = " + m.hello("neo"));

        switch (Color.RED) {
            case RED: System.out.println("enum switch RED"); break;
            case GREEN: System.out.println("enum switch GREEN"); break;
            case BLUE: System.out.println("enum switch BLUE"); break;
        }

        System.out.println("varargs sum(1,2,3,4) = " + sumVar(1,2,3,4));

        int max2 = (a > b) ? a : b;
        System.out.println("ternary max2 = " + max2);
    }
}
